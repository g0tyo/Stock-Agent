use stock_agent::strategy::RiskConfig;

#[test]
fn position_size_respects_risk_and_cap() {
    let risk_cfg = RiskConfig::default();
    let position_size = risk_cfg.position_size_for_trade(100_000.0, 100.0, 95.0);
    assert!(position_size > 0.0);
    assert!(position_size <= risk_cfg.max_position_size);
}

#[test]
fn validation_rejects_invalid_stop_loss() {
    let risk_cfg = RiskConfig::default();
    let check = risk_cfg.validate_trade(100_000.0, 10.0, 100.0, 100.0, 0.01, 0);
    assert!(!check.allowed);
}
