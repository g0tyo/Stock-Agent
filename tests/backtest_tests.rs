use chrono::Utc;
use stock_agent::backtest::BacktestEngine;
use stock_agent::market_data::Candle;

#[test]
fn backtest_runs_with_basic_history() {
    let now = Utc::now();
    let candles = (0..50)
        .map(|i| Candle {
            symbol: "AAPL".to_string(),
            timestamp: now - chrono::Duration::minutes(i as i64 * 5),
            open: 100.0 + i as f64,
            high: 101.0 + i as f64,
            low: 99.0 + i as f64,
            close: 100.5 + i as f64,
            volume: 1000.0 + i as f64 * 50.0,
        })
        .collect::<Vec<_>>();
    let engine = BacktestEngine::default();
    let result = engine.run("AAPL", &candles);
    assert!(result.starting_capital > 0.0);
    assert!(result.ending_capital >= 0.0);
}
