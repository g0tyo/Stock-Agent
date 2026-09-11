use crate::broker::broker::{AccountSnapshot, Broker, BrokerError, OrderExecution, OrderRequest};
use crate::portfolio::Position;

#[derive(Debug, Clone, Default)]
pub struct LiveBroker;

#[async_trait::async_trait]
impl Broker for LiveBroker {
    async fn get_account(&self) -> Result<AccountSnapshot, BrokerError> {
        Err(BrokerError::new("Live broker is intentionally not implemented. Enable a real broker adapter before using live trading."))
    }

    async fn get_position(&self, _symbol: &str) -> Result<Option<Position>, BrokerError> {
        Err(BrokerError::new("Live broker is intentionally not implemented. Enable a real broker adapter before using live trading."))
    }

    async fn submit_order(&self, _order: OrderRequest) -> Result<OrderExecution, BrokerError> {
        Err(BrokerError::new("Live broker is intentionally not implemented. Enable a real broker adapter before using live trading."))
    }

    async fn cancel_order(&self, _order_id: &str) -> Result<(), BrokerError> {
        Err(BrokerError::new("Live broker is intentionally not implemented. Enable a real broker adapter before using live trading."))
    }
}
