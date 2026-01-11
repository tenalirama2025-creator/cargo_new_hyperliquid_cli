Hyperliquid CLI: High-Performance DeFi Interface in Rust
A lightweight, high-velocity command-line interface (CLI) for interacting with the Hyperliquid decentralized exchange. Built in Rust for maximum memory safety and execution speed, this tool provides programmatic access to HyperCore (trading) and HyperEVM (smart contracts) without the overhead of a GUI.

🚀 Research & Engineering Motivation
Autonomous AI agents are increasingly tasked with operating in high-stakes financial environments. This project was developed to explore:

Low-Latency Agent Control: How efficiently can a command-line interface bridge the gap between an agent's decision and on-chain execution?

Rust for Safety-Critical Systems: Leveraging Rust's ownership model to eliminate common memory vulnerabilities in financial tooling.

Deterministic Execution: Ensuring every CLI command maps precisely to a verifiable on-chain action.

🛠️ Technical Stack
Language: Rust (Stable)

Async Runtime: tokio for non-blocking I/O and high-concurrency API calls.

SDK: Built on top of hypersdk for EIP-712 type-safe signing and WebSocket support.

CLI Framework: clap (Command Line Argument Parser) for intuitive, nested subcommands.

Formatting: tabwriter for clean, human-readable terminal outputs.

✨ Key Features
Market Intelligence: Query perpetual and spot market metadata, leverage limits, and price tick sizes instantly.

Portfolio Monitoring: Real-time tracking of user balances across Spot, Perps, and HyperEVM balances.

DeFi Integrations: Direct hooks into Morpho lending positions and vault equities.

Security First: Implements delegated wallet logic, allowing the CLI to sign trades without exposing the main account's master private key.

🛡️ AI Safety & Control Implications

This project demonstrates technical competency in AI Control and Security, specifically:

Agent Sandbox Monitoring: The CLI serves as a "human-in-the-loop" monitoring tool. It allows researchers to audit and override AI agent actions in real-time within a DeFi "model organism" environment.

Robust Tooling for Red Teaming: Providing a scriptable, reliable interface to stress-test how AI agents handle adversarial market conditions (e.g., sudden volatility or oracle manipulation).

Formal Verification Foundations: By using Rust, the codebase is a step toward "verifiable safety" in the tools used to manage and deploy autonomous frontier models.

🛡️ AI Safety & Monitoring

This repository now includes an Agent Monitor (agent_monitor.py) written in Python. This script demonstrates how the high-performance Rust CLI can be wrapped in a Scalable Oversight framework. By polling the Rust binary, the monitor can act as a "trip-wire," detecting anomalous or high-risk trading behaviors in autonomous agents and triggering safety interventions.

💻 Quick Start
Installation
Bash

# Clone the repository
git clone https://github.com/tenalirama2025-creator/cargo_new_hyperliquid_cli
cd cargo_new_hyperliquid_cli

# Build for release
cargo build --release
Usage Examples
Bash

# List all active perpetual markets
./target/release/hypecli perps

# Check spot balances for a specific address
./target/release/hypecli spot-balances --user <WALLET_ADDRESS>

# Monitor a Morpho lending market
./target/release/hypecli morpho-position --market <MARKET_ID>
