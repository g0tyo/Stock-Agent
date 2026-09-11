use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub average_cost: f64,
    pub current_price: f64,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub updated_at: DateTime<Utc>,
}

impl Position {
    pub fn market_value(&self) -> f64 {
        self.quantity * self.current_price
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRecord {
    pub id: String,
    pub symbol: String,
    pub side: TradeSide,
    pub quantity: f64,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
    pub fee: f64,
    pub rationale: String,
}
