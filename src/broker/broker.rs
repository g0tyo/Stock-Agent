use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub order_type: OrderType,
    pub limit_price: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub rationale: String,
    pub confidence: u8,
    pub risk_reward_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSnapshot {
    pub cash: f64,
    pub buying_power: f64,
    pub equity: f64,
    pub positions: usize,
}

#[derive(Debug, Clone)]
pub struct BrokerError(String);

impl BrokerError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl std::fmt::Display for BrokerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for BrokerError {}

#[async_trait::async_trait]
pub trait Broker {
    async fn get_account(&self) -> Result<AccountSnapshot, BrokerError>;
    async fn get_position(&self, symbol: &str) -> Result<Option<crate::portfolio::Position>, BrokerError>;
    async fn submit_order(&self, order: OrderRequest) -> Result<crate::broker::broker::OrderExecution, BrokerError>;
    async fn cancel_order(&self, order_id: &str) -> Result<(), BrokerError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderExecution {
    pub id: String,
    pub symbol: String,
    pub status: String,
    pub filled_quantity: f64,
    pub average_fill_price: f64,
    pub fees: f64,
    pub message: String,
}
