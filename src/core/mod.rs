pub mod orders;
pub mod new_order;

use orders::new_order;

use crate::config::config::load_config;
use crate::types::core::OrderDetails;

/// Wrapper function to create an order on Binance Futures API without needing to manage configuration.
///
/// # Arguments
///
/// * `order_details` - Struct containing the specifics of the order to be created, including options for post-only and reduce-only.
///
/// # Returns
///
/// * `Ok(())` if the order was successfully created.
/// * `Err(Box<dyn std::error::Error>)` if there was an error creating the order or loading the configuration.
pub async fn create_order(order_details: OrderDetails) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    new_order(order_details, &config).await
}