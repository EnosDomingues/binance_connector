use chrono::Utc;
use reqwest::{Client, header::HeaderValue};
use crate::utils::build::query_string;
use crate::utils::signature::generate_signature;
use crate::types::core::{ApiError, ErrorResponse, OrderDetails, OrderResponse};
use crate::types::config::Config;

/// Creates an order on Binance Futures API.
///
/// # Arguments
///
/// * `order_details` - Struct containing the specifics of the order to be created, including options for post-only and reduce-only.
/// * `config` - Configuration containing API key and secret.
///
/// # Returns
///
/// * `Ok(())` if the order was successfully created.
/// * `Err(Box<dyn std::error::Error>)` if there was an error creating the order.
pub async fn new_order(
    order_details: OrderDetails,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let timestamp = Utc::now().timestamp_millis().to_string();
    let query_string = query_string(
        &order_details.side,
        &order_details.order_type,
        order_details.quantity,
        order_details.price,
        order_details.time_in_force.as_deref(),
        &timestamp,
        order_details.reduce_only,
    );
    let signature = generate_signature(&query_string, &config.secret_key);
    let url = format!(
        "https://fapi.binance.com/fapi/v1/order?{}&signature={}",
        query_string, signature
    );

    let header_value = HeaderValue::from_str(&config.api_key).map_err(|e| {
        ApiError::CustomError(format!("Failed to convert API key to HeaderValue: {}", e))
    })?;

    let response = client
        .post(&url)
        .header("X-MBX-APIKEY", header_value)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if status.is_success() {
        let order_response: OrderResponse = serde_json::from_str(&body).map_err(|e| {
            ApiError::CustomError(format!("Failed to deserialize response as OrderResponse: {}", e))
        })?;
        log::info!("Order created: {:#?}", order_response);
        Ok(())
    } else {
        let error_response: ErrorResponse = serde_json::from_str(&body).map_err(|_| {
            ApiError::CustomError("Failed to deserialize response as ErrorResponse".to_string())
        })?;
        Err(Box::new(ApiError::CustomError(format!(
            "Error creating order: {:?}",
            error_response
        ))))
    }
}