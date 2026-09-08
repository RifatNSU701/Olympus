mod auth;
mod auth_api;
mod db;
mod health;

use axum::{extract::State, routing::{get, post}, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

async fn me(State(_pool): State<sqlx::PgPool>) -> &'static str { "authenticated" }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "olympus_core=info,tower_http=info".into())).json().init();
    dotenvy::dotenv().ok();
    let pool = db::connect().await.expect("PostgreSQL connection required");
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is required");
    let auth_state = (pool.clone(), secret);

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/api/v1/auth/register", post(auth_api::register))
        .route("/api/v1/auth/login", post(auth_api::login))
        .route("/api/v1/auth/me", get(me))
        .with_state(auth_state)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr, "Olympus core API listening");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind API listener");
    axum::serve(listener, app).await.expect("serve API");
}
