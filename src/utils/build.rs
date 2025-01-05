
/// Builds the query string for the order creation request.
///
/// # Arguments
///
/// * `side` - A string slice that holds the side of the order ("BUY" or "SELL").
/// * `order_type` - A string slice that defines the type of the order.
/// * `quantity` - A float specifying the amount of the asset to order.
/// * `price` - An optional float for the price of the order. If None, it's a market order.
/// * `timestamp` - A string slice representing the current timestamp in milliseconds.
///
/// # Returns
///
/// * `String` - The constructed query string ready to be signed and used in the API request.
pub fn query_string(side: &str, order_type: &str, quantity: f64, price: Option<f64>, timestamp: &str) -> String {
  let mut query_string = format!(
      "symbol=BTCUSDT&side={}&type={}&quantity={}&timeInForce=GTC&timestamp={}",
      side, order_type, quantity, timestamp
  );

  if let Some(price_value) = price {
      query_string.push_str(&format!("&price={}", price_value));
  }

  query_string
}