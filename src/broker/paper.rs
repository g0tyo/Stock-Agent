use chrono::Utc;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::broker::broker::{AccountSnapshot, Broker, BrokerError, OrderExecution, OrderRequest, OrderSide};
use crate::portfolio::Position;

#[derive(Debug)]
pub struct PaperBroker {
    pub cash: Mutex<f64>,
    pub positions: Mutex<HashMap<String, Position>>,
    pub orders: Mutex<HashMap<String, OrderRequest>>,
    pub slippage_bps: f64,
    pub fee_pct: f64,
}

impl PaperBroker {
    pub fn new(cash: f64) -> Self {
        Self {
            cash: Mutex::new(cash),
            positions: Mutex::new(HashMap::new()),
            orders: Mutex::new(HashMap::new()),
            slippage_bps: 5.0,
            fee_pct: 0.001,
        }
    }

    fn effective_price(&self, side: &OrderSide, base_price: f64) -> f64 {
        let slippage = base_price * (self.slippage_bps / 10_000.0);
        match side {
            OrderSide::Buy => base_price + slippage,
            OrderSide::Sell => base_price - slippage,
        }
    }
}

#[async_trait::async_trait]
impl Broker for PaperBroker {
    async fn get_account(&self) -> Result<AccountSnapshot, BrokerError> {
        let cash = *self.cash.lock().map_err(|_| BrokerError::new("Cash lock poisoned."))?;
        let positions = self.positions.lock().map_err(|_| BrokerError::new("Position lock poisoned."))?;
        Ok(AccountSnapshot {
            cash,
            buying_power: cash,
            equity: cash + positions.values().map(|p| p.market_value()).sum::<f64>(),
            positions: positions.len(),
        })
    }

    async fn get_position(&self, symbol: &str) -> Result<Option<Position>, BrokerError> {
        let positions = self.positions.lock().map_err(|_| BrokerError::new("Position lock poisoned."))?;
        Ok(positions.get(symbol).cloned())
    }

    async fn submit_order(&self, order: OrderRequest) -> Result<OrderExecution, BrokerError> {
        let base_price = order.limit_price.unwrap_or(100.0);
        let execution_price = self.effective_price(&order.side, base_price);
        let fee = (order.quantity * execution_price) * self.fee_pct;

        match order.side {
            OrderSide::Buy => {
                let mut cash = self.cash.lock().map_err(|_| BrokerError::new("Cash lock poisoned."))?;
                let total_cost = order.quantity * execution_price + fee;
                if total_cost > *cash {
                    return Err(BrokerError::new("Insufficient cash for paper trade."));
                }
                *cash -= total_cost;

                let mut positions = self.positions.lock().map_err(|_| BrokerError::new("Position lock poisoned."))?;
                let mut existing = positions.get(&order.symbol).cloned().unwrap_or(Position {
                    symbol: order.symbol.clone(),
                    quantity: 0.0,
                    average_cost: 0.0,
                    current_price: execution_price,
                    stop_loss: order.stop_loss,
                    take_profit: order.take_profit,
                    updated_at: Utc::now(),
                });
                let new_qty = existing.quantity + order.quantity;
                let new_avg = ((existing.quantity * existing.average_cost) + (order.quantity * execution_price)) / new_qty.max(1.0);
                existing.quantity = new_qty;
                existing.average_cost = new_avg;
                existing.current_price = execution_price;
                existing.stop_loss = order.stop_loss;
                existing.take_profit = order.take_profit;
                existing.updated_at = Utc::now();
                positions.insert(order.symbol.clone(), existing);

                let mut orders = self.orders.lock().map_err(|_| BrokerError::new("Order lock poisoned."))?;
                orders.insert(order.id.clone(), order.clone());

                Ok(OrderExecution {
                    id: order.id,
                    symbol: order.symbol.clone(),
                    status: "FILLED".to_string(),
                    filled_quantity: order.quantity,
                    average_fill_price: execution_price,
                    fees: fee,
                    message: format!("Paper buy order executed for {} shares of {}.", order.quantity, order.symbol),
                })
            }
            OrderSide::Sell => {
                let mut positions = self.positions.lock().map_err(|_| BrokerError::new("Position lock poisoned."))?;
                if let Some(existing) = positions.get(&order.symbol) {
                    if existing.quantity < order.quantity {
                        return Err(BrokerError::new("Not enough shares to sell in paper mode."));
                    }
                    let mut pos = existing.clone();
                    pos.quantity -= order.quantity;
                    pos.current_price = execution_price;
                    pos.updated_at = Utc::now();
                    if pos.quantity <= 0.0 {
                        positions.remove(&order.symbol);
                    } else {
                        positions.insert(order.symbol.clone(), pos.clone());
                    }
                    let mut cash = self.cash.lock().map_err(|_| BrokerError::new("Cash lock poisoned."))?;
                    *cash += (order.quantity * execution_price) - fee;
                    let mut orders = self.orders.lock().map_err(|_| BrokerError::new("Order lock poisoned."))?;
                    orders.insert(order.id.clone(), order.clone());
                    return Ok(OrderExecution {
                        id: order.id,
                        symbol: order.symbol.clone(),
                        status: "FILLED".to_string(),
                        filled_quantity: order.quantity,
                        average_fill_price: execution_price,
                        fees: fee,
                        message: format!("Paper sell order executed for {} shares of {}.", order.quantity, order.symbol),
                    });
                }
                Err(BrokerError::new("No position to sell in paper mode."))
            }
        }
    }

    async fn cancel_order(&self, order_id: &str) -> Result<(), BrokerError> {
        let mut orders = self.orders.lock().map_err(|_| BrokerError::new("Order lock poisoned."))?;
        if orders.remove(order_id).is_some() {
            Ok(())
        } else {
            Err(BrokerError::new("Order not found."))
        }
    }
}
