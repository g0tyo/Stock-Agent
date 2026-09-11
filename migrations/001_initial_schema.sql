CREATE TABLE IF NOT EXISTS stocks (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(16) NOT NULL UNIQUE,
    name VARCHAR(128),
    exchange VARCHAR(64),
    currency VARCHAR(8) DEFAULT 'USD',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS market_data (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(16) NOT NULL REFERENCES stocks(symbol),
    timestamp TIMESTAMPTZ NOT NULL,
    open_price DOUBLE PRECISION NOT NULL,
    high_price DOUBLE PRECISION NOT NULL,
    low_price DOUBLE PRECISION NOT NULL,
    close_price DOUBLE PRECISION NOT NULL,
    volume DOUBLE PRECISION NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (symbol, timestamp)
);

CREATE INDEX IF NOT EXISTS idx_market_data_symbol_time ON market_data(symbol, timestamp DESC);

CREATE TABLE IF NOT EXISTS indicators (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(16) NOT NULL REFERENCES stocks(symbol),
    timestamp TIMESTAMPTZ NOT NULL,
    sma_20 DOUBLE PRECISION,
    sma_50 DOUBLE PRECISION,
    sma_200 DOUBLE PRECISION,
    ema_20 DOUBLE PRECISION,
    rsi_14 DOUBLE PRECISION,
    macd DOUBLE PRECISION,
    macd_signal DOUBLE PRECISION,
    bollinger_upper DOUBLE PRECISION,
    bollinger_lower DOUBLE PRECISION,
    atr DOUBLE PRECISION,
    avg_volume DOUBLE PRECISION,
    volume_ratio DOUBLE PRECISION,
    daily_return DOUBLE PRECISION,
    volatility DOUBLE PRECISION,
    momentum DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (symbol, timestamp)
);

CREATE TABLE IF NOT EXISTS signals (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(16) NOT NULL REFERENCES stocks(symbol),
    timestamp TIMESTAMPTZ NOT NULL,
    signal VARCHAR(16) NOT NULL,
    score INTEGER NOT NULL,
    confidence INTEGER NOT NULL,
    current_price DOUBLE PRECISION NOT NULL,
    stop_loss DOUBLE PRECISION,
    take_profit DOUBLE PRECISION,
    risk_reward_ratio DOUBLE PRECISION,
    reason TEXT,
    executed BOOLEAN DEFAULT FALSE,
    rejection_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(16) NOT NULL REFERENCES stocks(symbol),
    order_id VARCHAR(128) NOT NULL UNIQUE,
    side VARCHAR(16) NOT NULL,
    order_type VARCHAR(32) NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    limit_price DOUBLE PRECISION,
    stop_loss DOUBLE PRECISION,
    take_profit DOUBLE PRECISION,
    status VARCHAR(32) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS trades (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(16) NOT NULL REFERENCES stocks(symbol),
    trade_id VARCHAR(128) NOT NULL UNIQUE,
    side VARCHAR(16) NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    execution_price DOUBLE PRECISION NOT NULL,
    fees DOUBLE PRECISION NOT NULL DEFAULT 0,
    pnl DOUBLE PRECISION,
    rationale TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS positions (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(16) NOT NULL REFERENCES stocks(symbol),
    quantity DOUBLE PRECISION NOT NULL DEFAULT 0,
    average_cost DOUBLE PRECISION NOT NULL DEFAULT 0,
    current_price DOUBLE PRECISION NOT NULL DEFAULT 0,
    stop_loss DOUBLE PRECISION,
    take_profit DOUBLE PRECISION,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (symbol)
);

CREATE TABLE IF NOT EXISTS portfolio_snapshots (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    cash DOUBLE PRECISION NOT NULL,
    total_equity DOUBLE PRECISION NOT NULL,
    total_position_value DOUBLE PRECISION NOT NULL,
    daily_pnl DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS strategy_runs (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(16) NOT NULL REFERENCES stocks(symbol),
    timestamp TIMESTAMPTZ NOT NULL,
    strategy_name VARCHAR(64) NOT NULL,
    score INTEGER NOT NULL,
    signal VARCHAR(16) NOT NULL,
    confidence INTEGER NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS risk_events (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(16) REFERENCES stocks(symbol),
    timestamp TIMESTAMPTZ NOT NULL,
    event_type VARCHAR(64) NOT NULL,
    severity VARCHAR(32) NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS system_logs (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    level VARCHAR(16) NOT NULL,
    component VARCHAR(64) NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
