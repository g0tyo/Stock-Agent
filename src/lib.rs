pub mod analysis;
pub mod backtest;
pub mod broker;
pub mod config;
pub mod database;
pub mod logging;
pub mod market_data;
pub mod portfolio;
pub mod strategy;
pub mod trading;

pub use analysis::{AnalysisResult, IndicatorBundle, SignalDirection, SignalOutput, analyze_market};
pub use broker::{AccountSnapshot, Broker, BrokerError, OrderRequest, OrderSide, OrderType, LiveBroker, PaperBroker};
pub use market_data::{Candle, MarketDataError, MarketDataProvider, MarketQuote, MockMarketDataProvider};
pub use portfolio::{Portfolio, Position, TradeRecord, TradeSide};
pub use strategy::{RiskConfig, StrategyConfig, StrategyDecision};
