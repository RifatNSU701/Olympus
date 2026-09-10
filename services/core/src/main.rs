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

use axum::{
    extract::Extension,
    http::{HeaderValue, Method},
    middleware,
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "olympus_core=info,tower_http=info".into()),
        )
        .json()
        .init();
    dotenvy::dotenv().ok();

    let pool = db::connect().await.expect("PostgreSQL connection required");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is required");
    let ai_url =
        std::env::var("OLYMPUS_AI_URL").unwrap_or_else(|_| "http://localhost:8000".into());
    let state = AppState {
        pool,
        jwt_secret: jwt_secret.clone(),
        ai: ai_client::AiClient::new(ai_url),
    };

    let allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".into())
        .split(',')
        .filter_map(|origin| origin.trim().parse::<HeaderValue>().ok())
        .collect::<Vec<_>>();
    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ]);

    let protected = Router::new()
        .route("/api/v1/auth/me", get(auth_api::me))
        .route("/api/v1/products", post(products::create))
        .route("/api/v1/products/{id}", put(products::update))
        .route(
            "/api/v1/products/{id}/stock",
            put(products::update_stock),
        )
        .route("/api/v1/cart", get(cart::get))
        .route("/api/v1/cart/items", post(cart::add))
        .route(
            "/api/v1/cart/items/{item_id}",
            put(cart::update).delete(cart::remove),
        )
        .route("/api/v1/checkout", post(orders::checkout))
        .route("/api/v1/orders", get(order_api::list))
        .route("/api/v1/orders/{id}", get(order_api::get))
        .route(
            "/api/v1/orders/{order_id}/payments",
            post(payments::create),
        )
        .route(
            "/api/v1/payments/{payment_id}/verify",
            post(payment_api::verify),
        )
        .route("/api/v1/seller/dashboard", get(seller_dashboard::dashboard))
        .route("/api/v1/seller/analytics", get(seller_analytics::analytics))
        .route("/api/v1/seller/orders", get(seller_orders::list))
        .route("/api/v1/seller/orders/{id}", get(seller_orders::get))
        .route(
            "/api/v1/seller/orders/{id}/status",
            put(seller_orders::update_status),
        )
        .route("/api/v1/admin/stats", get(admin::stats))
        .route("/api/v1/admin/users", get(admin::list_users))
        .route(
            "/api/v1/admin/users/{id}/status",
            put(admin::update_user_status),
        )
        .layer(Extension(jwt_secret))
        .route_layer(middleware::from_fn(auth::require_auth));

    let request_id = security::request_id_header();
    let app = Router::new()
        .route("/health", get(health::health))
        .route("/health/live", get(health::live))
        .route("/health/ready", get(health::ready))
        .route("/api/v1/auth/register", post(auth_api::register))
        .route("/api/v1/auth/login", post(auth_api::login))
        .route("/api/v1/products", get(products::list))
        .route("/api/v1/products/{id}", get(products::get))
        .route("/api/v1/recommendations", post(ai_api::recommend))
        .merge(protected)
        .with_state(state)
        .layer(cors)
        .layer(PropagateRequestIdLayer::new(request_id.clone()))
        .layer(SetRequestIdLayer::new(request_id, MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(security::security_headers));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr, "Olympus core API listening");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind API listener");
    axum::serve(listener, app).await.expect("serve API");
}
