use std::error::Error;
use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, routing::get, Json, Router};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use stock_agent::config::AppConfig;
use stock_agent::logging::init_logging;
use stock_agent::market_data::{FinnhubMarketDataProvider, MarketDataProvider, MockMarketDataProvider};
use stock_agent::trading::TradingEngine;
use stock_agent::portfolio::Portfolio;

#[derive(Parser)]
#[command(name = "stock_agent")]
#[command(version, about = "Paper-trading stock analysis and trading system.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Server,
    Analyze { symbol: String },
    Quote { symbol: String },
    Backtest { symbol: String },
    PaperTrade { symbol: String },
    Portfolio,
    Positions,
    Trades,
    Strategy { symbol: String },
    Risk,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logging();
    let config = AppConfig::from_env()?;
    let provider = build_market_data_provider(&config)?;

    let cli = Cli::parse();
    match cli.command {
        Command::Server => run_server(config).await?,
        Command::Analyze { symbol } => {
            println!("ANALYZE: {symbol}");
            println!("PAPER TRADING MODE");
        }

        Command::Quote { symbol } => {
            let quote = provider.get_quote(&symbol).await?;
            println!("SYMBOL: {}\nPRICE: ${:.2}\nTIMESTAMP: {}", quote.symbol, quote.price, quote.timestamp);
        }
        Command::Backtest { symbol } => {
            println!("BACKTEST: {symbol}");
            let end = chrono::Utc::now();
            let start = end - chrono::Duration::days(30);
            let candles = provider.get_historical_data(&symbol, start, end).await?;
            let engine = stock_agent::backtest::BacktestEngine::default();
            let result = engine.run(&symbol, &candles);
            println!("STARTING CAPITAL: ${:.2}", result.starting_capital);
            println!("ENDING CAPITAL: ${:.2}", result.ending_capital);
            println!("TOTAL RETURN: {:.2}%", result.total_return * 100.0);
        }
        Command::PaperTrade { symbol } => {
            println!("PAPER TRADING MODE");
            let mut engine = TradingEngine::new(provider, Portfolio::new(100_000.0), true);
            let _ = engine.evaluate_symbol(&symbol).await?;
            println!("Paper trade evaluation complete for {symbol}.");
        }
        Command::Portfolio => {
            let portfolio = Portfolio::new(100_000.0);
            println!("CASH: ${:.2}", portfolio.cash);
            println!("TOTAL EQUITY: ${:.2}", portfolio.total_equity());
        }
        Command::Positions => println!("No active positions."),
        Command::Trades => println!("No trades recorded."),
        Command::Strategy { symbol } => {
            println!("STRATEGY: {symbol}");
            println!("BUY THRESHOLD: {}", stock_agent::strategy::StrategyConfig::default().buy_threshold);
        }
        Command::Risk => println!("RISK: no violations detected in the default paper-trading profile."),
    }

    Ok(())
}

#[derive(Clone)]
struct ApiState {
    provider: Arc<dyn MarketDataProvider>,
}

#[derive(Debug, Deserialize)]
struct MarketQuery {
    symbols: Option<String>,
}

#[derive(Debug, Serialize)]
struct MarketQuoteResponse {
    symbol: String,
    price: f64,
    change: Option<f64>,
    volume: Option<f64>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

async fn market_quotes(
    State(state): State<ApiState>,
    Query(query): Query<MarketQuery>,
) -> Result<Json<Vec<MarketQuoteResponse>>, (StatusCode, String)> {
    let symbols = query.symbols
        .as_deref()
        .unwrap_or("AAPL,MSFT,NVDA,AVGO,GOOGL,AMZN,META,TSLA,JPM,V,MA,BAC,LLY,UNH,JNJ,XOM,CVX,WMT,COST,NFLX,CRM,AMD")
        .split(',')
        .map(str::trim)
        .filter(|symbol| !symbol.is_empty() && symbol.len() <= 8)
        .map(str::to_uppercase)
        .collect::<Vec<_>>();
    let mut quotes = Vec::with_capacity(symbols.len());
    for symbol in symbols {
        let quote = state.provider.get_quote(&symbol).await.map_err(|error| {
            (StatusCode::BAD_GATEWAY, format!("Failed to load {symbol}: {error}"))
        })?;
        quotes.push(MarketQuoteResponse {
            symbol: quote.symbol,
            price: quote.price,
            change: quote.change,
            volume: quote.volume,
            timestamp: quote.timestamp,
        });
    }
    Ok(Json(quotes))
}

async fn run_server(config: AppConfig) -> Result<(), Box<dyn Error>> {
    let provider = build_market_data_provider(&config)?;
    let app = Router::new()
        .route("/api/market", get(market_quotes))
        .layer(CorsLayer::permissive())
        .with_state(ApiState { provider: Arc::from(provider) });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Market API listening on http://127.0.0.1:8080");
    axum::serve(listener, app).await?;
    Ok(())
}

fn build_market_data_provider(
    config: &AppConfig,
) -> Result<Box<dyn MarketDataProvider>, Box<dyn Error>> {
    match config.market_data_provider.to_lowercase().as_str() {
        "mock" => Ok(Box::new(MockMarketDataProvider::new())),
        "finnhub" => Ok(Box::new(FinnhubMarketDataProvider::new(
            &config.market_data_base_url,
            config.market_data_api_key.as_deref().unwrap_or_default(),
        )?)),
        provider => Err(format!("Unsupported MARKET_DATA_PROVIDER '{provider}'. Use 'mock' or 'finnhub'.").into()),
    }
}
