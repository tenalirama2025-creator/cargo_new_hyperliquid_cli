🦀 Hyperliquid-rs: High-Performance Perpetual DEX CLI Client🚀 OverviewThis project is a high-performance, asynchronous Command-Line Interface (CLI) client built in Rust for interacting with a perpetual decentralized exchange (DEX) API, similar to Hyperliquid.The primary goal is to demonstrate expertise in low-latency system design, real-time data processing, and the ability to build robust, production-grade infrastructure critical for CEX-level speed in a decentralized environment.✨ Key Features & Job RelevanceThis client focuses on the core components required for high-frequency trading infrastructure:Asynchronous Market Data Consumption: Connects directly to the WebSocket feed to stream L2 Order Book data in real-time using the Tokio runtime and tokio-tungstenite. (Relevant to CEX-level speed and infrastructure).Low-Latency Pre-processing: Implements basic trading logic (mid-price calculation, margin estimation) directly on the live stream, showcasing the ability to handle data quickly. (Relevant to trading systems experience and sharp trade-offs).Infrastructure Connectivity: Demonstrates successful integration with both real-time (WebSocket) and query-based (REST) APIs. (Relevant to Define and evolve architecture).Rust Implementation: Built entirely in Rust, emphasizing system-level control, memory safety, and high computational performance. (Relevant to Writing production-grade code).🛠️ Technologies UsedCategoryTechnologyPurposeLanguageRustSystem-level performance, type safety, and concurrency.Async RuntimetokioHigh-performance, scalable asynchronous event handling.WebSockettokio-tungsteniteLow-level, non-blocking WebSocket connection for market data.HTTP ClientreqwestUsed for querying account balances and general metadata via the REST API.Data Handlingserde, serde_jsonEfficient JSON serialization and deserialization.⚙️ Installation & SetupPrerequisitesRust programming language (latest stable version recommended)Cargo (Rust's package manager, included with Rust installation)StepsClone the Repository:Bashgit clone [Your GitHub Repo Link]
cd hyperliquid-rs
Build the Project:Bashcargo build --release
Run the Client:Bash# Execute the compiled binary
./target/release/hyperliquid-rs
🖥️ Usage & Output ExampleUpon running, the CLI client simultaneously performs two main functions:Market Data Stream (WebSocket): Connects and outputs real-time order book updates.Simple Action (REST API): Executes a one-time call to fetch metadata (placeholder for account balance/login).Expected Output Format:Connecting to WebSocket: wss://api.hyperliquid.xyz/ws
WebSocket handshake successful.
Subscription sent. Reading messages...

Market Data Update for ETH:
  Mid Price: 3205.50
  Best Bid: 5.23 @ 3205.25
  Best Ask: 3.10 @ 3205.75
  **Calculated Margin for 1x pos: 320.5500 USD**

Market Data Update for ETH:
  Mid Price: 3205.60
  Best Bid: 4.88 @ 3205.35
  Best Ask: 2.90 @ 3205.85
  **Calculated Margin for 1x pos: 320.5600 USD**

Executing Simple REST Action (Placeholder)...
Fetched REST Info (Partial):
[
  {
    "universe": "Hyperliquid",
    "status": "Operational",
    "version": "v1.2.3"
  }
]
... (Streaming continues until manually stopped)
🛣️ Future Development & RoadmapTo further evolve this project into a complete trading system:Signing & Order Execution: Implement wallet integration and signature generation for submitting signed orders via the API.Risk Engine: Add more complex position and margin calculations, including liquidation price tracking.CLI Enhancements: Implement argument parsing for coin selection and connection environment (Testnet/Mainnet).✍️ ContributionWhile this is a personal project, feel free to fork, explore, and provide feedback!Venkateshwar Rao N | Github profile link:- tenalirama2025-creator
