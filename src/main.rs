mod models;
mod portfolio;
mod api;

use actix_web::{App, HttpServer, web};
use api::{AppState, get_portfolio, buy_asset, sell_asset};
use models::Portfolio;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let portfolio = Portfolio::new("My Portfolio".to_string(), 10000.0);
    let app_state = web::Data::new(AppState {
        portfolio: Mutex::new(portfolio),
    });

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/portfolio", web::get().to(get_portfolio))
            .route("/buy", web::post().to(buy_asset))
            .route("/sell", web::post().to(sell_asset))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}