mod auth;
mod auth_api;
mod cart;
mod db;
mod health;
mod products;
mod state;

use crate::{auth::Claims, state::AppState};
use axum::{
    Json, Router,
    extract::Extension,
    middleware,
    routing::{delete, get, post, put},
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Serialize)]
struct Identity {
    id: Uuid,
    email: String,
    role: String,
}
async fn me(Extension(claims): Extension<Claims>) -> Json<Identity> {
    Json(Identity {
        id: claims.sub,
        email: claims.email,
        role: claims.role,
    })
}

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
    let state = AppState { pool, jwt_secret };

    let protected = Router::new()
        .route("/api/v1/auth/me", get(me))
        .route("/api/v1/products", post(products::create))
        .route("/api/v1/cart", get(cart::get))
        .route("/api/v1/cart/items", post(cart::add))
        .route(
            "/api/v1/cart/items/{item_id}",
            put(cart::update).delete(cart::remove),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::require_auth,
        ));

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/api/v1/auth/register", post(auth_api::register))
        .route("/api/v1/auth/login", post(auth_api::login))
        .route("/api/v1/products", get(products::list))
        .route("/api/v1/products/{id}", get(products::get))
        .merge(protected)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr, "Olympus core API listening");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind API listener");
    axum::serve(listener, app).await.expect("serve API");
}
