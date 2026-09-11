use stock_agent::broker::broker::{OrderRequest, OrderSide, OrderType};
use stock_agent::broker::PaperBroker;
use stock_agent::broker::Broker;
use chrono::Utc;

#[tokio::test]
async fn paper_broker_executes_valid_buy_order() {
    let broker = PaperBroker::new(10_000.0);
    let result = broker.submit_order(OrderRequest {
        id: "abc".to_string(),
        symbol: "AAPL".to_string(),
        side: OrderSide::Buy,
        quantity: 10.0,
        order_type: OrderType::Market,
        limit_price: Some(100.0),
        stop_loss: Some(95.0),
        take_profit: Some(110.0),
        timestamp: Utc::now(),
        rationale: "Bullish setup".to_string(),
        confidence: 70,
        risk_reward_ratio: 2.0,
    }).await;
    assert!(result.is_ok());
}
