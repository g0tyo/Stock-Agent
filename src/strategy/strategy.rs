use serde::{Deserialize, Serialize};

use crate::analysis::signals::{AnalysisResult, SignalDirection, SignalOutput};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub buy_threshold: i32,
    pub sell_threshold: i32,
    pub rsi_bullish_range: std::ops::Range<i32>,
    pub rsi_bearish_range: std::ops::Range<i32>,
    pub sma_fast: usize,
    pub sma_slow: usize,
    pub volume_multiplier: f64,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            buy_threshold: 6,
            sell_threshold: 6,
            rsi_bullish_range: 50..71,
            rsi_bearish_range: 0..40,
            sma_fast: 20,
            sma_slow: 50,
            volume_multiplier: 1.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StrategyDecision {
    pub signal: SignalDirection,
    pub score: i32,
    pub confidence: u8,
    pub reasoning: String,
    pub entry_price: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub risk_reward_ratio: f64,
}

impl StrategyConfig {
    pub fn generate_signal(&self, analysis: &AnalysisResult) -> StrategyDecision {
        let score = analysis.overall_score;
        let signal = if score >= self.buy_threshold {
            SignalDirection::Buy
        } else if score <= -self.sell_threshold {
            SignalDirection::Sell
        } else {
            SignalDirection::Hold
        };

        let reasoning = match signal {
            SignalDirection::Buy => "Bullish trend, positive momentum, and supportive volume are aligned for a long setup.".to_string(),
            SignalDirection::Sell => "Bearish trend, weak momentum, and deteriorating conditions favor a short or defensive action.".to_string(),
            SignalDirection::Hold => "Signal is mixed or insufficiently strong for a trade under current risk rules.".to_string(),
        };

        let stop_loss = analysis.current_price * 0.96;
        let take_profit = analysis.current_price * 1.08;
        let risk_reward_ratio = if stop_loss == 0.0 { 0.0 } else { (take_profit - analysis.current_price).abs() / (analysis.current_price - stop_loss).abs() };

        StrategyDecision {
            signal,
            score,
            confidence: analysis.confidence,
            reasoning,
            entry_price: analysis.current_price,
            stop_loss,
            take_profit,
            risk_reward_ratio,
        }
    }
}

pub fn generate_signal_output(strategy_cfg: &StrategyConfig, analysis: &AnalysisResult) -> SignalOutput {
    let decision = strategy_cfg.generate_signal(analysis);
    SignalOutput {
        signal: decision.signal,
        score: decision.score,
        confidence: decision.confidence,
        reasoning: decision.reasoning,
        entry_price: decision.entry_price,
        stop_loss: decision.stop_loss,
        take_profit: decision.take_profit,
        risk_reward_ratio: decision.risk_reward_ratio,
    }
}
