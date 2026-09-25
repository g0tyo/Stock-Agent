use chrono::{DateTime, Duration, TimeZone, Utc};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

use crate::market_data::models::{Candle, MarketQuote};

#[derive(Debug, Clone)]
pub enum MarketDataError {
    ApiFailure(String),
    ValidationError(String),
    StaleData(String),
    NotFound(String),
    RequestFailed(String),
}

impl std::fmt::Display for MarketDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiFailure(msg) => write!(f, "API failure: {msg}"),
            Self::ValidationError(msg) => write!(f, "Validation error: {msg}"),
            Self::StaleData(msg) => write!(f, "Stale data: {msg}"),
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::RequestFailed(msg) => write!(f, "Request failed: {msg}"),
        }
    }
}

impl Error for MarketDataError {}

#[derive(Debug, Clone)]
pub struct FinnhubMarketDataProvider {
    client: Client,
    base_url: String,
    api_key: String,
}

#[derive(Debug, Deserialize)]
struct FinnhubQuote {
    c: f64,
    t: i64,
}

#[derive(Debug, Deserialize)]
struct FinnhubCandles {
    c: Option<Vec<f64>>,
    h: Option<Vec<f64>>,
    l: Option<Vec<f64>>,
    o: Option<Vec<f64>>,
    t: Option<Vec<i64>>,
    v: Option<Vec<f64>>,
    s: String,
}

impl FinnhubMarketDataProvider {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self, MarketDataError> {
        let api_key = api_key.into();
        if api_key.trim().is_empty() {
            return Err(MarketDataError::ApiFailure("MARKET_DATA_API_KEY is required for Finnhub.".to_string()));
        }
        Ok(Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key,
        })
    }

    async fn parse_response<T: serde::de::DeserializeOwned>(response: reqwest::Response) -> Result<T, MarketDataError> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(MarketDataError::ApiFailure(format!("HTTP {status}: {body}")));
        }
        response.json::<T>().await.map_err(|error| MarketDataError::RequestFailed(error.to_string()))
    }
}

#[async_trait::async_trait]
impl MarketDataProvider for FinnhubMarketDataProvider {
    async fn get_quote(&self, symbol: &str) -> Result<MarketQuote, MarketDataError> {
        let symbol = symbol.to_uppercase();
        let response = self.client
            .get(format!("{}/quote", self.base_url))
            .query(&[("symbol", symbol.as_str()), ("token", self.api_key.as_str())])
            .send()
            .await
            .map_err(|error| MarketDataError::RequestFailed(error.to_string()))?;
        let quote: FinnhubQuote = Self::parse_response(response).await?;
        if !quote.c.is_finite() || quote.c <= 0.0 {
            return Err(MarketDataError::ValidationError(format!("Invalid price returned for {symbol}.")));
        }
        let timestamp = Utc.timestamp_opt(quote.t, 0).single().ok_or_else(|| {
            MarketDataError::ValidationError(format!("Invalid timestamp returned for {symbol}."))
        })?;
        if Utc::now().signed_duration_since(timestamp) > Duration::minutes(15) {
            return Err(MarketDataError::StaleData(format!("Quote for {symbol} is older than 15 minutes.")));
        }
        Ok(MarketQuote { symbol, price: quote.c, timestamp })
    }

    async fn get_historical_data(&self, symbol: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Candle>, MarketDataError> {
        let symbol = symbol.to_uppercase();
        let response = self.client
            .get(format!("{}/stock/candle", self.base_url))
            .query(&[
                ("symbol", symbol.as_str()),
                ("resolution", "5"),
                ("from", &start.timestamp().to_string()),
                ("to", &end.timestamp().to_string()),
                ("token", self.api_key.as_str()),
            ])
            .send()
            .await
            .map_err(|error| MarketDataError::RequestFailed(error.to_string()))?;
        let candles: FinnhubCandles = Self::parse_response(response).await?;
        if candles.s != "ok" {
            return Err(MarketDataError::NotFound(format!("No historical data returned for {symbol}.")));
        }
        let (closes, highs, lows, opens, timestamps, volumes) = match (candles.c, candles.h, candles.l, candles.o, candles.t, candles.v) {
            (Some(c), Some(h), Some(l), Some(o), Some(t), Some(v)) => (c, h, l, o, t, v),
            _ => return Err(MarketDataError::ValidationError("Incomplete candle arrays returned by Finnhub.".to_string())),
        };
        let lengths = [closes.len(), highs.len(), lows.len(), opens.len(), timestamps.len(), volumes.len()];
        if lengths.iter().any(|length| *length != lengths[0]) {
            return Err(MarketDataError::ValidationError("Candle arrays returned by Finnhub have different lengths.".to_string()));
        }
        let mut result = Vec::with_capacity(closes.len());
        for index in 0..closes.len() {
            let timestamp = Utc.timestamp_opt(timestamps[index], 0).single().ok_or_else(|| {
                MarketDataError::ValidationError("Invalid candle timestamp returned by Finnhub.".to_string())
            })?;
            let candle = Candle {
                symbol: symbol.clone(),
                timestamp,
                open: opens[index],
                high: highs[index],
                low: lows[index],
                close: closes[index],
                volume: volumes[index],
            };
            if !candle.is_valid() || !candle.close.is_finite() || !candle.volume.is_finite() {
                return Err(MarketDataError::ValidationError(format!("Invalid candle returned for {symbol}.")));
            }
            result.push(candle);
        }
        Ok(result)
    }
}

#[async_trait::async_trait]
pub trait MarketDataProvider: Send + Sync {
    async fn get_quote(&self, symbol: &str) -> Result<MarketQuote, MarketDataError>;
    async fn get_historical_data(
        &self,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Candle>, MarketDataError>;
}

#[async_trait::async_trait]
impl<T> MarketDataProvider for Box<T>
where
    T: MarketDataProvider + ?Sized,
{
    async fn get_quote(&self, symbol: &str) -> Result<MarketQuote, MarketDataError> {
        self.as_ref().get_quote(symbol).await
    }

    async fn get_historical_data(
        &self,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Candle>, MarketDataError> {
        self.as_ref().get_historical_data(symbol, start, end).await
    }
}

#[derive(Debug, Clone)]
pub struct MockMarketDataProvider;

impl MockMarketDataProvider {
    pub fn new() -> Self {
        Self
    }

    fn seed_candles(symbol: &str, start: DateTime<Utc>, count: usize) -> Vec<Candle> {
        let mut out = Vec::with_capacity(count);
        let mut price = match symbol {
            "AAPL" => 178.0,
            "MSFT" => 335.0,
            "NVDA" => 120.0,
            _ => 50.0,
        };
        let mut ts = start;
        for idx in 0..count {
            let drift = ((idx % 10) as f64 - 4.5) * 1.2;
            let open = price;
            let close = (open + drift).max(1.0);
            let high = (open.max(close) + 2.0).max(close);
            let low = (open.min(close) - 2.0).min(open);
            let volume = 100_000.0 + (idx as f64 * 1_200.0);
            out.push(Candle {
                symbol: symbol.to_string(),
                timestamp: ts,
                open,
                high,
                low,
                close,
                volume,
            });
            price = close;
            ts += Duration::minutes(5);
        }
        out
    }
}

#[async_trait::async_trait]
impl MarketDataProvider for MockMarketDataProvider {
    async fn get_quote(&self, symbol: &str) -> Result<MarketQuote, MarketDataError> {
        let symbol = symbol.to_uppercase();
        let price = match symbol.as_str() {
            "AAPL" => 179.42,
            "MSFT" => 336.64,
            "NVDA" => 121.89,
            _ => 99.99,
        };

        Ok(MarketQuote {
            symbol,
            price,
            timestamp: Utc::now(),
        })
    }

    async fn get_historical_data(
        &self,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Candle>, MarketDataError> {
        let count = ((end - start).num_minutes() / 5).max(1) as usize;
        let candles = Self::seed_candles(symbol, start, count.min(250));
        Ok(candles)
    }
}
