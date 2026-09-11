use serde::{Deserialize, Serialize};

use crate::market_data::Candle;
use crate::strategy::StrategyConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalDirection {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub current_price: f64,
    pub trend: String,
    pub momentum: String,
    pub volatility: String,
    pub rsi: f64,
    pub macd: f64,
    pub volume_condition: String,
    pub overall_score: i32,
    pub confidence: u8,
    pub signal: SignalDirection,
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self {
            current_price: 0.0,
            trend: "NEUTRAL".to_string(),
            momentum: "NEUTRAL".to_string(),
            volatility: "MODERATE".to_string(),
            rsi: 50.0,
            macd: 0.0,
            volume_condition: "NORMAL".to_string(),
            overall_score: 0,
            confidence: 0,
            signal: SignalDirection::Hold,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SignalOutput {
    pub signal: SignalDirection,
    pub score: i32,
    pub confidence: u8,
    pub reasoning: String,
    pub entry_price: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub risk_reward_ratio: f64,
}

pub fn analyze_market(candles: &[Candle], strategy_cfg: &StrategyConfig) -> AnalysisResult {
    use crate::analysis::indicators::aggregate_indicators;

    if candles.is_empty() {
        return AnalysisResult::default();
    }

    let last = candles.last().unwrap();
    let indicators = aggregate_indicators(candles);
    let current_price = last.close;
    let rsi = indicators.rsi_14.unwrap_or(50.0);
    let macd = indicators.macd.unwrap_or(0.0);
    let trend = if let (Some(sma50), Some(sma200)) = (indicators.sma_50, indicators.sma_200) {
        if sma50 > sma200 { "BULLISH".to_string() } else if sma50 < sma200 { "BEARISH".to_string() } else { "NEUTRAL".to_string() }
    } else {
        "NEUTRAL".to_string()
    };
    let momentum = if let Some(momentum_v) = indicators.momentum {
        if momentum_v > 0.02 { "POSITIVE".to_string() } else if momentum_v < -0.02 { "NEGATIVE".to_string() } else { "NEUTRAL".to_string() }
    } else {
        "NEUTRAL".to_string()
    };
    let volatility = if let Some(vol) = indicators.volatility {
        if vol > 0.03 { "HIGH".to_string() } else if vol > 0.015 { "MODERATE".to_string() } else { "LOW".to_string() }
    } else {
        "MODERATE".to_string()
    };
    let volume_condition = if let Some(ratio) = indicators.volume_ratio {
        if ratio > 1.2 { "ABOVE AVERAGE".to_string() } else if ratio < 0.8 { "BELOW AVERAGE".to_string() } else { "NORMAL".to_string() }
    } else {
        "NORMAL".to_string()
    };

    let mut score = 0_i32;
    if current_price > indicators.sma_50.unwrap_or(current_price) { score += 2; }
    else { score -= 2; }
    if let (Some(sma50), Some(sma200)) = (indicators.sma_50, indicators.sma_200) {
        if sma50 > sma200 { score += 2; } else if sma50 < sma200 { score -= 2; }
    }
    if strategy_cfg.rsi_bullish_range.contains(&(rsi as i32)) { score += 2; }
    else if strategy_cfg.rsi_bearish_range.contains(&(rsi as i32)) { score -= 2; }
    if macd > 0.0 { score += 2; } else if macd < 0.0 { score -= 2; }
    if let Some(ratio) = indicators.volume_ratio {
        if ratio > 1.0 { score += 1; } else { score -= 1; }
    }
    if let Some(momentum_v) = indicators.momentum {
        if momentum_v > 0.0 { score += 1; } else if momentum_v < 0.0 { score -= 1; }
    }

    let signal = if score >= strategy_cfg.buy_threshold { SignalDirection::Buy }
                 else if score <= -strategy_cfg.sell_threshold { SignalDirection::Sell }
                 else { SignalDirection::Hold };
    let confidence = ((score.abs() as u8).min(95) + 50).min(95);

    AnalysisResult {
        current_price,
        trend,
        momentum,
        volatility,
        rsi,
        macd,
        volume_condition,
        overall_score: score,
        confidence,
        signal,
    }
}
