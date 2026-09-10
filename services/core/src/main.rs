use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod admin;
mod ai_api;
mod ai_client;
mod auth;
mod auth_api;
mod cart;
mod db;
mod health;
mod order_api;
mod orders;
mod payment_api;
mod payments;
mod products;
mod security;
mod seller_analytics;
mod seller_dashboard;
mod seller_orders;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = state::AppState::from_env().await.expect("failed to initialize application state");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/health/live", get(health::live))
        .route("/health/ready", get(health::ready))
        .route("/api/v1/seller/analytics", get(seller_analytics::analytics))
        .layer(cors)
        .layer(security::security_headers());

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!(%addr, "Olympus core listening");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("failed to bind");
    axum::serve(listener, app).await.expect("server error");
}
