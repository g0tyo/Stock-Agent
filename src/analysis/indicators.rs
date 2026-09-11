use crate::market_data::Candle;

#[derive(Debug, Clone, Default)]
pub struct IndicatorBundle {
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub sma_200: Option<f64>,
    pub ema_20: Option<f64>,
    pub rsi_14: Option<f64>,
    pub macd: Option<f64>,
    pub macd_signal: Option<f64>,
    pub bollinger_upper: Option<f64>,
    pub bollinger_lower: Option<f64>,
    pub atr: Option<f64>,
    pub avg_volume: Option<f64>,
    pub volume_ratio: Option<f64>,
    pub daily_return: Option<f64>,
    pub volatility: Option<f64>,
    pub momentum: Option<f64>,
}

pub fn sma(values: &[f64], period: usize) -> Option<f64> {
    if values.len() < period {
        return None;
    }
    let window = &values[values.len() - period..];
    let sum: f64 = window.iter().sum();
    Some(sum / period as f64)
}

pub fn ema(values: &[f64], period: usize) -> Option<f64> {
    if values.is_empty() || values.len() < period {
        return None;
    }
    let multiplier = 2.0 / (period as f64 + 1.0);
    let mut out = values[0];
    for v in values.iter().skip(1) {
        out = (v - out) * multiplier + out;
    }
    Some(out)
}

pub fn rsi(values: &[f64], period: usize) -> Option<f64> {
    if values.len() <= period {
        return None;
    }
    let mut gains = 0.0;
    let mut losses = 0.0;
    for i in 1..=(period.min(values.len() - 1)) {
        let diff = values[values.len() - i] - values[values.len() - i - 1];
        if diff >= 0.0 {
            gains += diff;
        } else {
            losses += diff.abs();
        }
    }
    if losses == 0.0 {
        return Some(100.0);
    }
    let rs = gains / losses;
    Some(100.0 - (100.0 / (1.0 + rs)))
}

pub fn macd(values: &[f64], fast: usize, slow: usize, signal: usize) -> (Option<f64>, Option<f64>) {
    let fast_ema = ema(values, fast);
    let slow_ema = ema(values, slow);
    let macd_value = match (fast_ema, slow_ema) {
        (Some(f), Some(s)) => Some(f - s),
        _ => None,
    };
    let signal_value = match macd_value {
        Some(v) => {
            let history = vec![v; signal.min(3)];
            let s = ema(&history, signal.min(3).max(1));
            s
        }
        None => None,
    };
    (macd_value, signal_value)
}

pub fn bollinger_bands(values: &[f64], period: usize, multiplier: f64) -> (Option<f64>, Option<f64>) {
    if values.len() < period {
        return (None, None);
    }
    let window = &values[values.len() - period..];
    let mean = window.iter().sum::<f64>() / window.len() as f64;
    let variance = window.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / window.len() as f64;
    let std_dev = variance.sqrt();
    (Some(mean + std_dev * multiplier), Some(mean - std_dev * multiplier))
}

pub fn atr(candles: &[Candle]) -> Option<f64> {
    if candles.len() < 2 {
        return None;
    }
    let values: Vec<f64> = candles
        .windows(2)
        .map(|w| (w[1].high - w[1].low).max((w[1].high - w[0].close).abs()).max((w[1].low - w[0].close).abs()))
        .collect();
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

pub fn average_volume(candles: &[Candle]) -> Option<f64> {
    if candles.is_empty() {
        return None;
    }
    let avg = candles.iter().map(|c| c.volume).sum::<f64>() / candles.len() as f64;
    Some(avg)
}

pub fn volume_ratio(current_volume: f64, avg_volume: f64) -> Option<f64> {
    if avg_volume <= 0.0 {
        return None;
    }
    Some(current_volume / avg_volume)
}

pub fn daily_returns(candles: &[Candle]) -> Vec<f64> {
    let mut output = Vec::new();
    for window in candles.windows(2) {
        let prev = window[0].close;
        let curr = window[1].close;
        if prev > 0.0 {
            output.push((curr / prev) - 1.0);
        }
    }
    output
}

pub fn volatility(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
    Some(variance.sqrt())
}

pub fn momentum(values: &[f64]) -> Option<f64> {
    if values.len() < 2 {
        return None;
    }
    let start = values[0];
    let end = values[values.len() - 1];
    if start == 0.0 { return None; }
    Some((end / start) - 1.0)
}

pub fn aggregate_indicators(candles: &[Candle]) -> IndicatorBundle {
    let closes: Vec<f64> = candles.iter().map(|c| c.close).collect();
    let _vols: Vec<f64> = candles.iter().map(|c| c.volume).collect();
    let daily = daily_returns(candles);
    let (macd_value, macd_signal_value) = macd(&closes, 12, 26, 9);
    let (upper, lower) = bollinger_bands(&closes, 20, 2.0);
    let rsi_value = rsi(&closes, 14);
    let avg_vol = average_volume(candles);
    let volume_ratio_value = match (candles.last(), avg_vol) {
        (Some(last), Some(avg)) => volume_ratio(last.volume, avg),
        _ => None,
    };
    let momentum_value = momentum(&closes);
    let volatility_value = volatility(&daily);

    IndicatorBundle {
        sma_20: sma(&closes, 20),
        sma_50: sma(&closes, 50),
        sma_200: sma(&closes, 200),
        ema_20: ema(&closes, 20),
        rsi_14: rsi_value,
        macd: macd_value,
        macd_signal: macd_signal_value,
        bollinger_upper: upper,
        bollinger_lower: lower,
        atr: atr(candles),
        avg_volume: avg_vol,
        volume_ratio: volume_ratio_value,
        daily_return: daily.last().copied(),
        volatility: volatility_value,
        momentum: momentum_value,
    }
}
