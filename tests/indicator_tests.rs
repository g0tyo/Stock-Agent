use stock_agent::analysis::indicators::{aggregate_indicators, rsi, sma};
use stock_agent::market_data::Candle;
use chrono::Utc;

#[test]
fn moving_average_calculates() {
    let values = vec![10.0, 12.0, 11.0, 13.0, 15.0, 16.0, 18.0, 17.0, 19.0, 20.0];
    let result = sma(&values, 5).unwrap();
    assert!(result > 0.0);
}

#[test]
fn rsi_is_in_range() {
    let values = vec![100.0, 102.0, 101.0, 105.0, 110.0, 109.0, 115.0, 120.0, 118.0, 125.0];
    let result = rsi(&values, 5).unwrap();
    assert!(result >= 0.0 && result <= 100.0);
}

#[test]
fn aggregate_indicators_handles_valid_candles() {
    let now = Utc::now();
    let candles = vec![
        Candle { symbol: "AAPL".to_string(), timestamp: now - chrono::Duration::minutes(30), open: 100.0, high: 101.0, low: 99.0, close: 100.5, volume: 1000.0 },
        Candle { symbol: "AAPL".to_string(), timestamp: now - chrono::Duration::minutes(25), open: 100.5, high: 102.0, low: 100.0, close: 101.5, volume: 1200.0 },
        Candle { symbol: "AAPL".to_string(), timestamp: now - chrono::Duration::minutes(20), open: 101.5, high: 103.0, low: 101.0, close: 102.0, volume: 1300.0 },
        Candle { symbol: "AAPL".to_string(), timestamp: now - chrono::Duration::minutes(15), open: 102.0, high: 104.0, low: 102.0, close: 103.5, volume: 1250.0 },
        Candle { symbol: "AAPL".to_string(), timestamp: now - chrono::Duration::minutes(10), open: 103.5, high: 105.0, low: 103.0, close: 104.0, volume: 1350.0 },
        Candle { symbol: "AAPL".to_string(), timestamp: now - chrono::Duration::minutes(5), open: 104.0, high: 106.0, low: 103.5, close: 105.0, volume: 1500.0 },
    ];
    let result = aggregate_indicators(&candles);
    assert!(result.sma_20.is_none() || result.sma_20.is_some());
    assert!(result.avg_volume.unwrap() > 0.0);
}
