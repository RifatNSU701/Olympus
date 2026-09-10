use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Json}};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub async fn live() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthResponse { status: "ok" }))
}

pub async fn ready(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(HealthResponse { status: "ready" })),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse { status: "database_unavailable" }),
        ),
    }
}

pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    ready(State(state)).await
}
