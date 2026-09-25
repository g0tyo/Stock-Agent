# Stock Agent

Stock Agent is a Rust-based stock analysis and paper-trading system designed around risk-first decision making. It supports PostgreSQL persistence, technical analysis, strategy scoring, and a broker abstraction that can later be implemented for a live brokerage API.

## Features
- PostgreSQL-ready schema and migration files
- Async Rust with Tokio and SQLx
- Mock market-data provider for development and testing
- Optional Finnhub HTTP provider for real quotes and historical candles
- Technical indicators: SMA, EMA, RSI, MACD, ATR, Bollinger Bands, average volume, volatility, and momentum
- Configurable long/short strategy scoring
- Portfolio and position tracking
- Paper trading broker for simulated fills, slippage, and fees
- Safety rails for max daily loss, cooldowns, and drawdown protection
- CLI commands for analysis, quote lookup, portfolio, backtests, and strategy checks

## Quick start
1. Install Rust via rustup.
2. Create a PostgreSQL database named `stock_agent`.
3. Copy `.env.example` to `.env` and fill in values.
4. Run `cargo build` and then `cargo test`.
5. Use the CLI commands listed below.

To use real market data, create a Finnhub API key and set `MARKET_DATA_PROVIDER=finnhub` and `MARKET_DATA_API_KEY=your_key` in `.env`. Keep `PAPER_TRADING=true`; market data and brokerage execution are separate concerns.

## Dashboard
The visual dashboard is in `web/`. Start it with:

```bash
cd web
python3 -m http.server 4173
```

Then open `http://localhost:4173`. The current dashboard is a frontend prototype using representative paper-trading values; connect it to Rust API endpoints when the persistence/API layer is added.

## Example commands
- `cargo run -- analyze AAPL`
- `cargo run -- quote AAPL`
- `cargo run -- backtest AAPL`
- `cargo run -- paper-trade AAPL`
- `cargo run -- portfolio`
- `cargo run -- positions`
- `cargo run -- trades`
- `cargo run -- strategy AAPL`
- `cargo run -- risk`

## Important safety rules
- The system never trades blindly.
- All decisions are scored and reasoned.
- Paper mode is the default and must be used until a live broker is explicitly enabled.
- Stop losses and maximum risk checks are enforced before any order is accepted.

## Project structure
- `src/market_data` market-data abstraction and mock provider
- `src/analysis` technical and statistical analysis
- `src/strategy` scoring and risk rules
- `src/portfolio` portfolio and positions
- `src/broker` broker abstraction and paper/live broker implementations
- `src/trading` execution engine
- `src/backtest` historical simulation engine
- `migrations` PostgreSQL schema and migrations
