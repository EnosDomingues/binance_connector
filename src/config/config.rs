use dotenv::dotenv;
use std::env;
use log::error;
use std::error::Error as StdError;
use crate::types::config::Config;

pub fn load_config() -> Result<Config, Box<dyn StdError>> {
    dotenv().ok();
    Ok(Config {
        api_key: env::var("BINANCE_API_KEY").map_err(|e| {
            error!("API Key not found: {}", e);
            e
        })?,
        secret_key: env::var("BINANCE_API_SECRET").map_err(|e| {
            error!("API Secret not found: {}", e);
            e
        })?,
    })
}