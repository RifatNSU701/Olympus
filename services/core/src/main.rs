mod db;
mod health;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "olympus_core=info,tower_http=info".into()))
        .json()
        .init();

    dotenvy::dotenv().ok();
    let pool = db::connect().await.expect("PostgreSQL connection required");

    let app = Router::new()
        .route("/health", get(health::health))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr, "Olympus core API listening");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind API listener");
    axum::serve(listener, app).await.expect("serve API");
}
