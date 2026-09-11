use chrono::Utc;

use crate::analysis::signals::analyze_market;
use crate::market_data::MarketDataProvider;
use crate::portfolio::Portfolio;
use crate::strategy::{RiskConfig, StrategyConfig};
use crate::trading::orders::{DecisionLog, OrderProposal};

#[derive(Debug, Clone)]
pub struct TradingEngine<B> {
    pub provider: B,
    pub strategy_cfg: StrategyConfig,
    pub risk_cfg: RiskConfig,
    pub portfolio: Portfolio,
    pub paper_mode: bool,
}

impl<B> TradingEngine<B>
where
    B: MarketDataProvider,
{
    pub fn new(provider: B, portfolio: Portfolio, paper_mode: bool) -> Self {
        Self {
            provider,
            strategy_cfg: StrategyConfig::default(),
            risk_cfg: RiskConfig::default(),
            portfolio,
            paper_mode,
        }
    }

    pub async fn evaluate_symbol(&mut self, symbol: &str) -> Result<Option<DecisionLog>, Box<dyn std::error::Error>> {
        let _quote = self.provider.get_quote(symbol).await?;
        let start = Utc::now() - chrono::Duration::days(45);
        let candles = self.provider.get_historical_data(symbol, start, Utc::now()).await?;
        let analysis = analyze_market(&candles, &self.strategy_cfg);
        let order = self.strategy_cfg.generate_signal(&analysis);

        let position_size = self.risk_cfg.position_size_for_trade(self.portfolio.total_equity(), analysis.current_price, order.stop_loss);
        let decision = DecisionLog {
            symbol: symbol.to_string(),
            timestamp: Utc::now(),
            current_price: analysis.current_price,
            score: analysis.overall_score,
            signal: format!("{:?}", order.signal),
            confidence: order.confidence,
            reason: order.reasoning.clone(),
            position_size,
            stop_loss: order.stop_loss,
            take_profit: order.take_profit,
            risk_reward_ratio: order.risk_reward_ratio,
            executed: false,
            rejection_reason: None,
        };

        if order.signal == crate::analysis::signals::SignalDirection::Hold {
            return Ok(Some(decision));
        }

        let risk_check = self.risk_cfg.validate_trade(self.portfolio.total_equity(), position_size, analysis.current_price, order.stop_loss, 0.0, 0);
        if !risk_check.allowed {
            let mut rejected = decision;
            rejected.executed = false;
            rejected.rejection_reason = Some(risk_check.reasons.join("; "));
            return Ok(Some(rejected));
        }

        let proposal = OrderProposal {
            symbol: symbol.to_string(),
            side: match order.signal {
                crate::analysis::signals::SignalDirection::Buy => "BUY".to_string(),
                crate::analysis::signals::SignalDirection::Sell => "SELL".to_string(),
                crate::analysis::signals::SignalDirection::Hold => "HOLD".to_string(),
            },
            quantity: (position_size / analysis.current_price.max(1.0)).floor(),
            order_type: "MARKET".to_string(),
            entry_price: analysis.current_price,
            stop_loss: order.stop_loss,
            take_profit: order.take_profit,
            rationale: order.reasoning,
            confidence: order.confidence,
            risk_reward_ratio: order.risk_reward_ratio,
            timestamp: Utc::now(),
        };

        let mut final_decision = decision;
        final_decision.executed = true;
        final_decision.position_size = proposal.quantity;
        Ok(Some(final_decision))
    }
}
