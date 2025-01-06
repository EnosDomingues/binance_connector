/// Builds the query string for the order creation request.
///
/// # Arguments
///
/// * `side` - A string slice that holds the side of the order ("BUY" or "SELL").
/// * `order_type` - A string slice that defines the type of the order.
/// * `quantity` - A float specifying the amount of the asset to order.
/// * `price` - An optional float for the price of the order. If None, it's a market order.
/// * `time_in_force` - An optional string slice representing the time in force of the order, like "GTX" for post-only.
/// * `timestamp` - A string slice representing the current timestamp in milliseconds.
/// * `reduce_only` - An optional string slice for specifying if the order is reduce-only, generally "true" or "false".
///
/// # Returns
///
/// * `String` - The constructed query string ready to be signed and used in the API request.
pub fn query_string(
  side: &str, 
  order_type: &str, 
  quantity: f64, 
  price: Option<f64>, 
  time_in_force: Option<&str>, 
  timestamp: &str,
  reduce_only: Option<bool>
) -> String {
  let time_in_force = time_in_force.unwrap_or("GTC"); // Default to GTC if not specified
  let reduce_only = reduce_only.unwrap_or(false); // Default to false if not specified
  let mut query_string = format!(
      "symbol=BTCUSDT&side={}&type={}&quantity={}&timeInForce={}&timestamp={}&reduceOnly={}",
      side, order_type, quantity, time_in_force, timestamp, reduce_only
  );

  if let Some(price_value) = price {
      query_string.push_str(&format!("&price={}", price_value));
  }

  query_string
}