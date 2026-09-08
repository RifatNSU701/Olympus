use axum::{extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

pub async fn health(State(pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(&pool).await {
        Ok(_) => (StatusCode::OK, "ok"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "database_unavailable"),
    }
}
