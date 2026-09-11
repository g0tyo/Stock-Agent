use crate::market_data::Candle;

pub fn sharpe_ratio(returns: &[f64], risk_free: f64) -> f64 {
    if returns.is_empty() {
        return 0.0;
    }
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / returns.len() as f64;
    let std = variance.sqrt();
    if std == 0.0 {
        return 0.0;
    }
    (mean - risk_free) / std
}

pub fn drawdown(prices: &[f64]) -> f64 {
    if prices.is_empty() {
        return 0.0;
    }
    let mut peak = prices[0];
    let mut max_dd = 0.0;
    for &price in prices {
        if price > peak {
            peak = price;
        }
        let dd = ((peak - price) / peak).max(0.0);
        if dd > max_dd {
            max_dd = dd;
        }
    }
    max_dd
}

pub fn average_trade_return(trades: &[f64]) -> f64 {
    if trades.is_empty() { return 0.0; }
    trades.iter().sum::<f64>() / trades.len() as f64
}

pub fn compute_total_return(starting_capital: f64, ending_capital: f64) -> f64 {
    if starting_capital <= 0.0 {
        return 0.0;
    }
    (ending_capital / starting_capital) - 1.0
}

pub fn annualized_return(total_return: f64, periods: usize) -> f64 {
    if periods == 0 { return 0.0; }
    (1.0 + total_return).powf(1.0 / periods as f64) - 1.0
}

pub fn profit_factor(wins: f64, losses: f64) -> f64 {
    if losses <= 0.0 { return wins.max(1.0); }
    wins / losses.abs()
}

pub fn compute_profit_and_loss(candles: &[Candle]) -> Vec<f64> {
    let mut profits = Vec::new();
    for window in candles.windows(2) {
        let prev = window[0].close;
        let curr = window[1].close;
        let pct = if prev > 0.0 { (curr / prev) - 1.0 } else { 0.0 };
        profits.push(pct);
    }
    profits
}
