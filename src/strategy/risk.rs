#[derive(Debug, Clone)]
pub struct RiskConfig {
    pub risk_per_trade: f64,
    pub max_position_size: f64,
    pub max_portfolio_exposure: f64,
    pub max_daily_loss: f64,
    pub max_trades_per_day: u32,
    pub stop_loss_pct: f64,
    pub take_profit_pct: f64,
    pub max_drawdown: f64,
    pub cooldown_after_losses: u32,
    pub circuit_breaker_enabled: bool,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            risk_per_trade: 0.01,
            max_position_size: 100_000.0,
            max_portfolio_exposure: 0.50,
            max_daily_loss: 0.10,
            max_trades_per_day: 3,
            stop_loss_pct: 0.05,
            take_profit_pct: 0.10,
            max_drawdown: 0.20,
            cooldown_after_losses: 2,
            circuit_breaker_enabled: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RiskCheck {
    pub allowed: bool,
    pub reasons: Vec<String>,
    pub position_size: f64,
    pub risk_amount: f64,
}

impl RiskConfig {
    pub fn position_size_for_trade(&self, equity: f64, entry_price: f64, stop_loss: f64) -> f64 {
        if equity <= 0.0 || entry_price <= 0.0 || stop_loss <= 0.0 || stop_loss == entry_price {
            return 0.0;
        }
        let distance = (entry_price - stop_loss).abs();
        if distance <= 0.0 {
            return 0.0;
        }
        let risk_amount = equity * self.risk_per_trade;
        let position_size = risk_amount / distance;
        position_size.min(self.max_position_size)
    }

    pub fn validate_trade(&self, portfolio_equity: f64, position_size: f64, entry_price: f64, stop_loss: f64, current_drawdown: f64, consecutive_losses: u32) -> RiskCheck {
        let mut reasons = Vec::new();
        let mut allowed = true;

        if portfolio_equity <= 0.0 {
            allowed = false;
            reasons.push("Portfolio equity is not positive.".to_string());
        }
        if position_size <= 0.0 {
            allowed = false;
            reasons.push("Position size is zero or negative.".to_string());
        }
        let max_exposure = portfolio_equity * self.max_portfolio_exposure;
        if position_size * entry_price > max_exposure {
            allowed = false;
            reasons.push("Trade exceeds maximum portfolio exposure.".to_string());
        }
        if current_drawdown >= self.max_drawdown {
            allowed = false;
            reasons.push("Maximum drawdown limit reached.".to_string());
        }
        if consecutive_losses >= self.cooldown_after_losses && self.cooldown_after_losses > 0 {
            allowed = false;
            reasons.push("Cooling down after consecutive losses.".to_string());
        }
        if self.max_daily_loss > 0.0 && current_drawdown >= self.max_daily_loss {
            allowed = false;
            reasons.push("Daily loss limit reached.".to_string());
        }
        let risk_amount = portfolio_equity * self.risk_per_trade;
        if position_size > self.max_position_size {
            allowed = false;
            reasons.push("Trade exceeds maximum allowed position size.".to_string());
        }
        if stop_loss <= 0.0 || stop_loss == entry_price {
            allowed = false;
            reasons.push("Invalid stop loss placement.".to_string());
        }

        RiskCheck {
            allowed,
            reasons,
            position_size: position_size.min(self.max_position_size),
            risk_amount,
        }
    }
}
