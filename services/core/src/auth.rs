use axum::{extract::State, http::{Request, StatusCode}, middleware::Next, response::Response};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

pub async fn require_auth<B>(State(secret): State<String>, mut request: Request<B>, next: Next) -> Result<Response, StatusCode> {
    let header = request.headers().get("authorization").and_then(|v| v.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    let token = header.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation).map_err(|_| StatusCode::UNAUTHORIZED)?;
    request.extensions_mut().insert(data.claims);
    Ok(next.run(request).await)
}

pub async fn check_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1 FROM users WHERE id = $1 AND status = 'ACTIVE'")
        .bind(user_id).fetch_optional(pool).await?.ok_or(sqlx::Error::RowNotFound).map(|_| ())
}
