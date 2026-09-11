use crate::analysis::analyze_market;
use crate::market_data::Candle;
use crate::strategy::StrategyConfig;

#[derive(Debug, Clone)]
pub struct BacktestResult {
    pub starting_capital: f64,
    pub ending_capital: f64,
    pub total_return: f64,
    pub annualized_return: f64,
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: f64,
    pub average_win: f64,
    pub average_loss: f64,
    pub profit_factor: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub largest_winning_trade: f64,
    pub largest_losing_trade: f64,
}

#[derive(Debug, Clone)]
pub struct BacktestEngine {
    pub strategy_cfg: StrategyConfig,
    pub initial_capital: f64,
}

impl Default for BacktestEngine {
    fn default() -> Self {
        Self {
            strategy_cfg: StrategyConfig::default(),
            initial_capital: 100_000.0,
        }
    }
}

impl BacktestEngine {
    pub fn run(&self, _symbol: &str, candles: &[Candle]) -> BacktestResult {
        let mut capital = self.initial_capital;
        let mut trades = Vec::new();
        let mut position = 0.0;
        let mut entry = 0.0;
        let mut win_count = 0usize;
        let mut loss_count = 0usize;
        let mut largest_win = 0.0_f64;
        let mut largest_loss = 0.0_f64;
        let mut prices = Vec::new();

        for window in candles.windows(2) {
            let _prev = &window[0];
            let current = &window[1];
            let subset = &candles[..=window.len()];
            let analysis = analyze_market(subset, &self.strategy_cfg);
            let decision = self.strategy_cfg.generate_signal(&analysis);

            match decision.signal {
                crate::analysis::signals::SignalDirection::Buy if position <= 0.0 => {
                    position = 1.0;
                    entry = current.close;
                }
                crate::analysis::signals::SignalDirection::Sell if position >= 1.0 => {
                    let pnl = ((current.close - entry) / entry) * 100.0;
                    capital *= 1.0 + (pnl / 100.0);
                    prices.push(capital);
                    trades.push(pnl);
                    if pnl >= 0.0 { win_count += 1; largest_win = largest_win.max(pnl); } else { loss_count += 1; largest_loss = largest_loss.min(pnl); }
                    position = 0.0;
                    entry = 0.0;
                }
                _ => {}
            }
        }

        let total_return = if self.initial_capital > 0.0 { (capital / self.initial_capital) - 1.0 } else { 0.0 };
        let avg_win = if win_count > 0 { trades.iter().filter(|&&v| v >= 0.0).sum::<f64>() / win_count as f64 } else { 0.0 };
        let avg_loss = if loss_count > 0 { trades.iter().filter(|&&v| v < 0.0).sum::<f64>() / loss_count as f64 } else { 0.0 };
        let win_rate = if trades.is_empty() { 0.0 } else { win_count as f64 / trades.len() as f64 };
        let profit = trades.iter().filter(|&&v| v >= 0.0).sum::<f64>();
        let loss_total = trades.iter().filter(|&&v| v < 0.0).sum::<f64>().abs();
        let profit_factor = if loss_total <= 0.0 { if profit <= 0.0 { 1.0 } else { profit.max(1.0) } } else { profit / loss_total };
        let max_dd = crate::analysis::statistics::drawdown(&prices);
        let sharpe = crate::analysis::statistics::sharpe_ratio(&trades, 0.0);

        BacktestResult {
            starting_capital: self.initial_capital,
            ending_capital: capital,
            total_return,
            annualized_return: crate::analysis::statistics::annualized_return(total_return, candles.len().max(1)),
            total_trades: trades.len(),
            winning_trades: win_count,
            losing_trades: loss_count,
            win_rate,
            average_win: avg_win,
            average_loss: avg_loss,
            profit_factor,
            max_drawdown: max_dd,
            sharpe_ratio: sharpe,
            sortino_ratio: sharpe,
            largest_winning_trade: largest_win,
            largest_losing_trade: largest_loss,
        }
    }
}
