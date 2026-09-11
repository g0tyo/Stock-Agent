use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub market_data_api_key: Option<String>,
    pub broker_api_key: Option<String>,
    pub broker_secret: Option<String>,
    pub paper_trading: bool,
    pub trading_symbols: Vec<String>,
    pub default_risk_per_trade: f64,
    pub max_position_size: f64,
    pub max_portfolio_exposure: f64,
    pub stop_loss_pct: f64,
    pub take_profit_pct: f64,
    pub buy_threshold: i32,
    pub sell_threshold: i32,
    pub max_daily_loss: f64,
    pub trading_frequency_seconds: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: "postgresql://postgres:postgres@localhost:5432/stock_agent".to_string(),
            market_data_api_key: None,
            broker_api_key: None,
            broker_secret: None,
            paper_trading: true,
            trading_symbols: vec!["AAPL".to_string(), "MSFT".to_string()],
            default_risk_per_trade: 0.01,
            max_position_size: 100_000.0,
            max_portfolio_exposure: 0.5,
            stop_loss_pct: 0.05,
            take_profit_pct: 0.10,
            buy_threshold: 6,
            sell_threshold: 6,
            max_daily_loss: 0.10,
            trading_frequency_seconds: 300,
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| Self::default().database_url.clone());
        let market_data_api_key = env::var("MARKET_DATA_API_KEY").ok();
        let broker_api_key = env::var("BROKER_API_KEY").ok();
        let broker_secret = env::var("BROKER_SECRET").ok();
        let paper_trading = env::var("PAPER_TRADING").unwrap_or_else(|_| "true".to_string()).eq_ignore_ascii_case("true");

        Ok(Self {
            database_url,
            market_data_api_key,
            broker_api_key,
            broker_secret,
            paper_trading,
            trading_symbols: env::var("TRADING_SYMBOLS").map(|v| v.split(',').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect()).unwrap_or_else(|_| Self::default().trading_symbols),
            default_risk_per_trade: env::var("RISK_PER_TRADE").ok().and_then(|v| v.parse().ok()).unwrap_or(Self::default().default_risk_per_trade),
            max_position_size: env::var("MAX_POSITION_SIZE").ok().and_then(|v| v.parse().ok()).unwrap_or(Self::default().max_position_size),
            max_portfolio_exposure: env::var("MAX_PORTFOLIO_EXPOSURE").ok().and_then(|v| v.parse().ok()).unwrap_or(Self::default().max_portfolio_exposure),
            stop_loss_pct: env::var("STOP_LOSS_PCT").ok().and_then(|v| v.parse().ok()).unwrap_or(Self::default().stop_loss_pct),
            take_profit_pct: env::var("TAKE_PROFIT_PCT").ok().and_then(|v| v.parse().ok()).unwrap_or(Self::default().take_profit_pct),
            buy_threshold: env::var("BUY_THRESHOLD").ok().and_then(|v| v.parse().ok()).unwrap_or(Self::default().buy_threshold),
            sell_threshold: env::var("SELL_THRESHOLD").ok().and_then(|v| v.parse().ok()).unwrap_or(Self::default().sell_threshold),
            max_daily_loss: env::var("MAX_DAILY_LOSS").ok().and_then(|v| v.parse().ok()).unwrap_or(Self::default().max_daily_loss),
            trading_frequency_seconds: env::var("TRADING_FREQUENCY_SECONDS").ok().and_then(|v| v.parse().ok()).unwrap_or(Self::default().trading_frequency_seconds),
        })
    }
}
