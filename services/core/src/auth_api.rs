use crate::{auth::Claims, state::AppState};
use argon2::{Argon2, password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}};
use axum::{Json, extract::{Extension, State}, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: &'static str,
}

#[derive(Serialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub status: String,
    pub full_name: String,
}

fn token(claims: Claims, secret: &str) -> Result<String, StatusCode> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<TokenResponse>), StatusCode> {
    let name = req.full_name.trim();
    if !req.email.contains('@') || req.password.len() < 8 || name.is_empty() || name.len() > 160 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let email = req.email.trim().to_lowercase();
    if sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM users WHERE lower(email) = $1 LIMIT 1",
    )
    .bind(&email)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .is_some()
    {
        return Err(StatusCode::CONFLICT);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let id: Uuid = sqlx::query_scalar("INSERT INTO users (email, password_hash, full_name, role, status) VALUES ($1, $2, $3, 'BUYER', 'ACTIVE') RETURNING id")
        .bind(&email)
        .bind(hash)
        .bind(name)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let claims = Claims {
        sub: id,
        email,
        role: "BUYER".into(),
        exp: (chrono::Utc::now().timestamp() + 3600) as usize,
    };

    Ok((
        StatusCode::CREATED,
        Json(TokenResponse {
            access_token: token(claims, &state.jwt_secret)?,
            token_type: "Bearer",
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, StatusCode> {
    let email = req.email.trim().to_lowercase();
    let row: Option<(Uuid, String, String, String)> = sqlx::query_as(
        "SELECT id, password_hash, role, status FROM users WHERE lower(email) = $1 LIMIT 1",
    )
    .bind(&email)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (id, hash, role, status) = row.ok_or(StatusCode::UNAUTHORIZED)?;
    if status != "ACTIVE" {
        return Err(StatusCode::FORBIDDEN);
    }

    let parsed = PasswordHash::new(&hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = Claims {
        sub: id,
        email,
        role,
        exp: (chrono::Utc::now().timestamp() + 3600) as usize,
    };

    Ok(Json(TokenResponse {
        access_token: token(claims, &state.jwt_secret)?,
        token_type: "Bearer",
    }))
}

pub async fn me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<MeResponse>, StatusCode> {
    let row = sqlx::query_as::<_, (Uuid, String, String, String, String)>(
        "SELECT id, email, role, status, full_name FROM users WHERE id = $1",
    )
    .bind(claims.sub)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(Json(MeResponse {
        id: row.0,
        email: row.1,
        role: row.2,
        status: row.3,
        full_name: row.4,
    }))
}
