use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;
use futures::{stream::StreamExt, SinkExt};
use serde_json::json;

// --- STEP 1: Define Data Structures (Simplified) ---
#[derive(Debug, serde::Deserialize)]
struct OrderBookLevel {
    px: String, // Price as a string
    sz: String, // Size as a string
}

#[derive(Debug, serde::Deserialize)]
struct OrderBookData {
    coin: String,
    asks: Vec<OrderBookLevel>,
    bids: Vec<OrderBookLevel>,
    // Add other fields as needed (e.g., sequence, time)
}

// --- STEP 2: Market Data Fetching via WebSocket ---
async fn connect_market_data(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to WebSocket: {}", url);
    
    // Connect to the WebSocket endpoint
    let (ws_stream, _) = connect_async(Url::parse(url)?).await?;
    println!("WebSocket handshake successful.");
    
    // Split the stream into a sender and receiver
    let (mut write, mut read) = ws_stream.split();
    
    // Subscribe message (adapt this JSON payload for Hyperliquid's specific format)
    let subscribe_msg = json!({
        "method": "subscribe",
        "params": {
            "channel": "l2Book",
            "coin": "ETH",
        },
        "id": 1
    }).to_string();

    // Send the subscription message
    write.send(tokio_tungstenite::tungstenite::Message::Text(subscribe_msg)).await?;
    
    println!("Subscription sent. Reading messages...");

    // Loop to read incoming messages
    while let Some(message) = read.next().await {
        match message {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                // Try to parse the message as an OrderBookData update
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                    // Check if it's the specific data structure you want
                    if let Some(data_payload) = data.get("data").and_then(|d| d.get("data")) {
                        if let Ok(order_book) = serde_json::from_value::<OrderBookData>(data_payload.clone()) {
                            println!("\nMarket Data Update for {}:", order_book.coin);
                            
                            // --- STEP 3: Simple Order Calculation/Pre-processing ---
                            if let (Some(best_bid), Some(best_ask)) = (order_book.bids.first(), order_book.asks.first()) {
                                
                                // Example Pre-processing/Calculation: Mid-Price and Simple Margin Calc
                                let bid_px: f64 = best_bid.px.parse()?;
                                let ask_px: f64 = best_ask.px.parse()?;
                                let mid_price = (bid_px + ask_px) / 2.0;

                                // Assuming 10x leverage, rough margin calculation for 1 ETH position
                                let position_size = 1.0; 
                                let leverage = 10.0;
                                let margin_req = (mid_price * position_size) / leverage;

                                println!("  Mid Price: {:.2}", mid_price);
                                println!("  Best Bid: {} @ {}", best_bid.sz, best_bid.px);
                                println!("  Best Ask: {} @ {}", best_ask.sz, best_ask.px);
                                println!("  **Calculated Margin for {}x pos: {:.4} USD**", position_size, margin_req);
                            }
                        } else {
                            // Print unparsed messages for debugging (like heartbeats, initial confirmations)
                            // println!("Raw Message: {}", text); 
                        }
                    }
                }
            },
            Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                println!("WebSocket connection closed.");
                break;
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
            _ => {} // Ignore Ping/Pong/Binary messages
        }
    }

    Ok(())
}

// --- STEP 4: Placeholder for REST API / Simple Action ---
// This would use `reqwest` for wallet balance fetching or placing an order
async fn execute_simple_action() -> Result<(), reqwest::Error> {
    println!("\nExecuting Simple REST Action (Placeholder)...");
    
    // Placeholder for Hyperliquid API base URL (adapt as necessary)
    let url = "https://api.hyperliquid.xyz/info"; 
    
    // Example: Fetching some general information
    let response = reqwest::Client::new()
        .post(url)
        .json(&json!({ "type": "metaAndAssetCtxs" }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("Fetched REST Info (Partial):\n{}", serde_json::to_string_pretty(&response.get(0).unwrap_or(&serde_json::Value::Null)).unwrap());
    
    // In a real scenario, you'd fetch balances, sign a transaction, and submit it here.

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_url = "wss://api.hyperliquid.xyz/ws"; // Check official docs for correct WS URL

    // Run both tasks concurrently
    tokio::select! {
        // Run the WebSocket connection for real-time data
        res = connect_market_data(ws_url) => {
            if let Err(e) = res {
                eprintln!("WebSocket Error: {}", e);
            }
        }
        // Run the REST action for an account action
        res = execute_simple_action() => {
             if let Err(e) = res {
                eprintln!("REST Error: {}", e);
            }
        }
    }
    
    Ok(())
}
