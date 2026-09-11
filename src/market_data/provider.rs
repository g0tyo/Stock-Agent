use chrono::{DateTime, Duration, Utc};
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

#[async_trait::async_trait]
pub trait MarketDataProvider {
    async fn get_quote(&self, symbol: &str) -> Result<MarketQuote, MarketDataError>;
    async fn get_historical_data(
        &self,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Candle>, MarketDataError>;
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
