use axum::{http::HeaderValue, routing::{get, post}, Router};
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer}, trace::TraceLayer};

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

    let allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string())
        .split(',')
        .filter_map(|origin| origin.trim().parse::<HeaderValue>().ok())
        .collect::<Vec<_>>();

    let cors = CorsLayer::new()
        .allow_origins(allowed_origins)
        .allow_headers([axum::http::header::AUTHORIZATION, axum::http::header::CONTENT_TYPE, axum::http::header::ACCEPT, axum::http::HeaderName::from_static("x-request-id")])
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PUT, axum::http::Method::PATCH, axum::http::Method::DELETE, axum::http::Method::OPTIONS])
        .allow_credentials(false);

    let request_id = axum::http::HeaderName::from_static("x-request-id");
    let app = Router::new()
        .route("/health", get(health::health))
        .route("/health/live", get(health::live))
        .route("/health/ready", get(health::ready))
        .route("/api/v1/auth/register", post(auth_api::register))
        .route("/api/v1/auth/login", post(auth_api::login))
        .route("/api/v1/products", get(products::list).post(products::create))
        .route("/api/v1/cart", get(cart::get_cart))
        .route("/api/v1/orders", get(order_api::list).post(order_api::create))
        .route("/api/v1/payments", post(payment_api::create))
        .route("/api/v1/seller/analytics", get(seller_analytics::analytics))
        .route("/api/v1/seller/dashboard", get(seller_dashboard::dashboard))
        .route("/api/v1/seller/orders", get(seller_orders::list))
        .route("/api/v1/admin", get(admin::dashboard))
        .with_state(state)
        .layer(cors)
        .layer(security::security_headers())
        .layer(SetRequestIdLayer::new(request_id.clone(), MakeRequestUuid))
        .layer(PropagateRequestIdLayer::new(request_id))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!(%addr, "Olympus core listening");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("failed to bind");
    axum::serve(listener, app).await.expect("server error");
}
