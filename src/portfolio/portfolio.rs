use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::portfolio::positions::{Position, TradeRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSnapshot {
    pub timestamp: DateTime<Utc>,
    pub cash: f64,
    pub total_equity: f64,
    pub total_position_value: f64,
    pub daily_pnl: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Portfolio {
    pub cash: f64,
    pub positions: Vec<Position>,
    pub trades: Vec<TradeRecord>,
}

impl Portfolio {
    pub fn new(cash: f64) -> Self {
        Self {
            cash,
            positions: Vec::new(),
            trades: Vec::new(),
        }
    }

    pub fn total_position_value(&self) -> f64 {
        self.positions.iter().map(Position::market_value).sum()
    }

    pub fn total_equity(&self) -> f64 {
        self.cash + self.total_position_value()
    }

    pub fn add_trade(&mut self, trade: TradeRecord) {
        self.trades.push(trade);
    }

    pub fn upsert_position(&mut self, position: Position) {
        if let Some(current) = self.positions.iter_mut().find(|p| p.symbol == position.symbol) {
            *current = position;
        } else {
            self.positions.push(position);
        }
    }

    pub fn position_for(&self, symbol: &str) -> Option<&Position> {
        self.positions.iter().find(|p| p.symbol.eq_ignore_ascii_case(symbol))
    }

    pub fn snapshot(&self) -> PortfolioSnapshot {
        PortfolioSnapshot {
            timestamp: Utc::now(),
            cash: self.cash,
            total_equity: self.total_equity(),
            total_position_value: self.total_position_value(),
            daily_pnl: 0.0,
        }
    }
}
