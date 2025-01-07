# Binance Connector

A Rust-based connector for interacting with the Binance API, designed to facilitate trading operations and data retrieval for cryptocurrency markets.

![Crates.io](https://img.shields.io/crates/v/binance_connector)
![License](https://img.shields.io/github/license/EnosDomingues/binance_connector)

## Table of Contents

- [About](#about)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installing](#installing)
  - [Basic Usage](#basic-usage)
- [License](#license)

## About

Binance Connector is a Rust library that provides a simple interface for interacting with the Binance cryptocurrency exchange's REST API. It aims to simplify the process of executing trades, querying market data, and managing accounts on Binance through a Rust application.

## Features

- Support for Binance REST API (e.g., market data, account management, and trading operations).
- Lightweight and fast with idiomatic Rust design.
- Easy-to-use abstractions for complex API interactions.
- Examples included to help you get started quickly.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust (stable version, use [rustup](https://rustup.rs/) to manage your Rust version)
- Cargo, Rust's package manager
- [Tokio](https://tokio.rs/) runtime for asynchronous programming
- A `.env` file in the root of your project with your Binance API credentials:

  ```env
  # BINANCE
  # Your Binance API Key
  BINANCE_API_KEY='type your api key here'

  # Your Binance API Secret
  BINANCE_API_SECRET='type your api secret here'
  
### Installing

1. Clone the repo:
   ```sh
   git clone https://github.com/EnosDomingues/binance_connector.git
   ```
  
2. Navigate to the project directory:
   ```sh 
   cd binance_connector
    ```
3. Build and run the project: 
    ```sh
    cargo build
    cargo run
    ```
    
### Add dependencies to `Cargo.toml`

To use the Binance Connector library, you'll need to add the required dependencies to your `Cargo.toml` file. Below is the required configuration:

```toml
[dependencies]
binance_connector = "0.1.0" # Replace with the latest version
tokio = { version = "1", features = ["full"] }
dotenv = "0.15" # For loading environment variables from .env
```

### Basic Usage

> The following example demonstrates how to create a **futures limit order** on Binance using the Binance Connector library.

> **⚠️ Important:** This example is specifically for Binance Futures. Ensure you have Futures permissions enabled for your API Key.

```rust
use binance_connector::core::create_order;
use std::error::Error as StdError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    // Create a new limit order to buy 0.002 BTC at a price of 80,000 USDT on Binance Futures
    create_order(binance_connector::types::core::OrderDetails {
        side: String::from("BUY"), // Order side: BUY or SELL
        order_type: String::from("LIMIT"), // Order type: LIMIT, MARKET, etc.
        quantity: 0.002, // Quantity of the asset to trade
        price: Some(80000.0), // Limit price for the order
        time_in_force: Some(String::from("GTX")), // Time in force: GTC, GTX, etc.
        reduce_only: Some(true), // Optional: Reduce-only flag for the order
    }).await?;

    Ok(())
}
```

