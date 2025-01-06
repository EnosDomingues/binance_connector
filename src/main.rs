use config::config::load_config;
use reqwest::Client;
use types::core::OrderDetails;
use core::orders::new_order;
use std::error::Error as StdError;

mod config;
mod utils;
mod core;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let config: types::config::Config = load_config()?;
    let client: Client = Client::new();

    new_order(OrderDetails {
        side: String::from("BUY"),
        order_type: String::from("LIMIT"),
        quantity: 0.002,
        price: Some(80000.0),
        time_in_force: Some(String::from("GTX")),
        reduce_only: Some(true),
    }, &config, &client).await?;

    Ok(())
}