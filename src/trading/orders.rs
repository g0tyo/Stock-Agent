use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderProposal {
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub order_type: String,
    pub entry_price: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub rationale: String,
    pub confidence: u8,
    pub risk_reward_ratio: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionLog {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub current_price: f64,
    pub score: i32,
    pub signal: String,
    pub confidence: u8,
    pub reason: String,
    pub position_size: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub risk_reward_ratio: f64,
    pub executed: bool,
    pub rejection_reason: Option<String>,
}
