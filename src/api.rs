use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use crate::models::{Portfolio};

pub struct AppState {
    pub portfolio: Mutex<Portfolio>,
}

#[derive(Deserialize)]
pub struct TradeRequest {
    symbol: String,
    quantity: f64,
    price: f64,
}

#[derive(Serialize)]
pub struct PortfolioResponse {
    name: String,
    cash_balance: f64,
    assets: Vec<AssetResponse>,
}

#[derive(Serialize)]
pub struct AssetResponse {
    symbol: String,
    quantity: f64,
    purchase_price: f64,
}

pub async fn get_portfolio(){}

pub async fn buy_asset(){}

pub async fn sell_asset(){}
    