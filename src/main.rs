use clap::{Parser, Subcommand};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::connect_async;
use url::Url;

/* #[derive(Debug, serde::Deserialize)]
struct OrderBookLevel {
    px: String,
    sz: String,
} */
/// Hyperliquid-rs: High-Performance Perpetual DEX CLI
#[derive(Parser)]
#[command(name = "hypecli", version = "0.1.0", author = "Venkateshwar Rao Nagala")]
#[command(about = "High-performance Rust CLI for Hyperliquid DEX", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all active perpetual markets with leverage limits
    Perps,
    /// Stream real-time order book via WebSocket
    Stream {
        /// Asset to stream (e.g. ETH, BTC)
        #[arg(short, long, default_value = "ETH")]
        coin: String,
    },
    /// Check spot balances for a wallet address
    SpotBalances {
        /// Wallet address to query
        #[arg(short, long)]
        user: String,
    },
    /// Monitor Morpho lending position
    MorphoPosition {
        /// Market ID to monitor
        #[arg(short, long)]
        market: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Perps => cmd_perps().await?,
        Commands::Stream { coin } => cmd_stream(&coin).await?,
        Commands::SpotBalances { user } => cmd_spot_balances(&user).await?,
        Commands::MorphoPosition { market } => cmd_morpho_position(&market).await?,
    }

    Ok(())
}

async fn cmd_perps() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Fetching Hyperliquid Perpetual Markets...\n");

    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.hyperliquid.xyz/info")
        .json(&json!({ "type": "meta" }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let universe = resp["universe"]
        .as_array()
        .ok_or("No universe field")?;

    println!("{:<6} {:<12} {:<15}", "IDX", "ASSET", "MAX LEVERAGE");
    println!("{}", "-".repeat(35));

    for (i, asset) in universe.iter().enumerate().take(20) {
        let name = asset["name"].as_str().unwrap_or("?");
        let leverage = asset["maxLeverage"].as_u64().unwrap_or(0);
        println!("{:<6} {:<12} {}x", i, name, leverage);
    }

    println!("\n✅ Showing top 20 of {} markets", universe.len());
    Ok(())
}

async fn cmd_stream(coin: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Streaming {} order book (press Ctrl+C to stop)...\n", coin);

    let (ws_stream, _) = connect_async(Url::parse("wss://api.hyperliquid.xyz/ws")?).await?;
    println!("✅ WebSocket connected.");

    let (mut write, mut read) = ws_stream.split();

    let sub = json!({
        "method": "subscribe",
        "subscription": { "type": "l2Book", "coin": coin }
    })
    .to_string();

    write
        .send(tokio_tungstenite::tungstenite::Message::Text(sub))
        .await?;

    let mut count = 0;
    while let Some(msg) = read.next().await {
        match msg {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) {
                    if val["channel"] == "l2Book" {
                        if let Some(levels) = val["data"]["levels"].as_array() {
                            if levels.len() >= 2 {
                                let bids = &levels[0];
                                let asks = &levels[1];
                                if let (Some(best_bid), Some(best_ask)) =
                                    (bids.as_array().and_then(|b| b.first()),
                                     asks.as_array().and_then(|a| a.first()))
                                {
                                    let bid: f64 = best_bid["px"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                                    let ask: f64 = best_ask["px"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                                    let mid = (bid + ask) / 2.0;
                                    let spread = ask - bid;
                                    println!(
                                        "#{:<4} {} | Bid: {:.2}  Ask: {:.2}  Mid: {:.2}  Spread: {:.4}",
                                        count, coin, bid, ask, mid, spread
                                    );
                                    count += 1;
                                    if count >= 10 {
                                        println!("\n✅ 10 updates received. Exiting stream.");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                println!("WebSocket closed.");
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

async fn cmd_spot_balances(user: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("💰 Fetching spot balances for {}...\n", user);

    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.hyperliquid.xyz/info")
        .json(&json!({ "type": "spotClearinghouseState", "user": user }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let balances = resp["balances"].as_array();
    match balances {
        Some(b) if !b.is_empty() => {
            println!("{:<10} {:<20} {:<20}", "COIN", "TOTAL", "HOLD");
            println!("{}", "-".repeat(50));
            for bal in b {
                let coin = bal["coin"].as_str().unwrap_or("?");
                let total = bal["total"].as_str().unwrap_or("0");
                let hold = bal["hold"].as_str().unwrap_or("0");
                println!("{:<10} {:<20} {:<20}", coin, total, hold);
            }
        }
        _ => println!("⚠️  No spot balances found for this address."),
    }

    Ok(())
}

async fn cmd_morpho_position(market: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏦 Morpho Market Monitor: {}\n", market);
    println!("ℹ️  Morpho integration requires on-chain EVM query.");
    println!("    Market ID: {}", market);
    println!("    Chain:     HyperEVM (Chain ID: 999)");
    println!("    Status:    Placeholder — full EVM integration in v0.2.0");
    Ok(())
}