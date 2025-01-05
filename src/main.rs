use config::config::load_config;
use reqwest::Client;
use core::orders::create_order;
use std::error::Error as StdError;

mod config;
mod utils;
mod core;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let config: types::config::Config = load_config()?;
    let client: Client = Client::new();
    
    create_order("BUY", "LIMIT", 0.002, Some(80000.0), &config, &client).await?;

    Ok(())
}