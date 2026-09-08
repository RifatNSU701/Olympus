use axum::{extract::State, http::StatusCode, response::IntoResponse};
use crate::state::AppState;

pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(&state.pool).await {
        Ok(_) => (StatusCode::OK, "ok"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "database_unavailable"),
    }
}
