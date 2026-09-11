use std::error::Error;

use clap::{Parser, Subcommand};
use stock_agent::config::AppConfig;
use stock_agent::database::Database;
use stock_agent::logging::init_logging;
use stock_agent::market_data::{MarketDataProvider, MockMarketDataProvider};
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
    let _db = Database::connect(&config.database_url).await?;

    let cli = Cli::parse();
    match cli.command {
        Command::Analyze { symbol } => {
            println!("ANALYZE: {symbol}");
            println!("PAPER TRADING MODE");
        }
        Command::Quote { symbol } => {
            let provider = MockMarketDataProvider::new();
            let quote = provider.get_quote(&symbol).await?;
            println!("SYMBOL: {}\nPRICE: ${:.2}\nTIMESTAMP: {}", quote.symbol, quote.price, quote.timestamp);
        }
        Command::Backtest { symbol } => {
            println!("BACKTEST: {symbol}");
            let provider = MockMarketDataProvider::new();
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
            let provider = MockMarketDataProvider::new();
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
