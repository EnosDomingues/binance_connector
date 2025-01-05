
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub order_id: i64,
    pub symbol: String,
    pub status: String,
    pub client_order_id: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    #[serde(rename = "cumQty")]
    pub cum_qty: String,
    #[serde(rename = "cumQuote")]
    pub cum_quote: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: String,
    pub position_side: String,
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
    pub working_type: String,
    pub price_protect: bool,
    pub orig_type: String,
    pub price_match: String,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
    pub good_till_date: i64,
    pub update_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),
    #[error("Custom error: {0}")]
    CustomError(String),
}

