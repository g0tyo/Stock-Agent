use stock_agent::analysis::signals::{AnalysisResult, SignalDirection};
use stock_agent::strategy::StrategyConfig;

#[test]
fn strategy_generates_buy_signal_for_bullish_analysis() {
    let config = StrategyConfig::default();
    let analysis = AnalysisResult {
        current_price: 100.0,
        trend: "BULLISH".to_string(),
        momentum: "POSITIVE".to_string(),
        volatility: "MODERATE".to_string(),
        rsi: 62.0,
        macd: 1.5,
        volume_condition: "ABOVE AVERAGE".to_string(),
        overall_score: 8,
        confidence: 80,
        signal: SignalDirection::Buy,
    };
    let decision = config.generate_signal(&analysis);
    assert_eq!(decision.signal, SignalDirection::Buy);
}

#[test]
fn strategy_generates_hold_if_score_is_moderate() {
    let config = StrategyConfig::default();
    let analysis = AnalysisResult {
        current_price: 100.0,
        trend: "NEUTRAL".to_string(),
        momentum: "NEUTRAL".to_string(),
        volatility: "MODERATE".to_string(),
        rsi: 50.0,
        macd: 0.0,
        volume_condition: "NORMAL".to_string(),
        overall_score: 0,
        confidence: 50,
        signal: SignalDirection::Hold,
    };
    let decision = config.generate_signal(&analysis);
    assert_eq!(decision.signal, SignalDirection::Hold);
}
