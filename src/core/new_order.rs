use chrono::Utc;
use reqwest::{Client, header::HeaderValue};
use crate::utils::build::query_string;
use crate::utils::signature::generate_signature;
use crate::types::core::{ErrorResponse, OrderResponse, ApiError};
use crate::types::config::Config;

/// Creates an order on Binance Futures API.
///
/// # Arguments
///
/// * `side` - A string slice that holds the side of the order ("BUY" or "SELL").
/// * `order_type` - A string slice that defines the type of the order.
/// * `quantity` - A float specifying the amount of the asset to order.
/// * `price` - An optional float for the price of the order. If None, it's a market order.
/// * `config` - Configuration containing API key and secret.
///
/// # Returns
///
/// * `Ok(())` if the order was successfully created.
/// * `Err(Box<dyn std::error::Error>)` if there was an error creating the order.
pub async fn create_order(
    side: &str,
    order_type: &str,
    quantity: f64,
    price: Option<f64>,
    config: &Config,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp: String = Utc::now().timestamp_millis().to_string();
    let query_string: String = query_string(side, order_type, quantity, price, &timestamp);
    let signature: String = generate_signature(&query_string, &config.secret_key);
    let url: String = format!("https://fapi.binance.com/fapi/v1/order?{}&signature={}", query_string, signature);

    let header_value: HeaderValue = HeaderValue::from_str(&config.api_key).map_err(|e| {
        ApiError::CustomError(format!("Failed to convert API key to HeaderValue: {}", e))
    })?;

    let response: reqwest::Response = client
        .post(&url)
        .header("X-MBX-APIKEY", header_value)
        .send()
        .await?;

    let status: reqwest::StatusCode = response.status();
    let body: String = response.text().await?;

    if status.is_success() {
        let order_response: OrderResponse = serde_json::from_str(&body)
            .map_err(|e| ApiError::CustomError(format!("Failed to deserialize response as OrderResponse: {}", e)))?;
        log::info!("Order created: {:#?}", order_response);
        Ok(())
    } else {
        let error_response: ErrorResponse = serde_json::from_str(&body).map_err(|_| {
            ApiError::CustomError("Failed to deserialize response as ErrorResponse".to_string())
        })?;
        Err(Box::new(ApiError::CustomError(format!("Error creating order: {:?}", error_response))))
    }
}
