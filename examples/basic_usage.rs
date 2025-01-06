use binance_connector::core::create_order;
use std::error::Error as StdError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    create_order(binance_connector::types::core::OrderDetails {
        side: String::from("BUY"),
        order_type: String::from("LIMIT"),
        quantity: 0.002,
        price: Some(80000.0),
        time_in_force: Some(String::from("GTX")),
        reduce_only: Some(true),
    }).await?;

    Ok(())
}