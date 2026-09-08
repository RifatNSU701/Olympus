use axum::{extract::State, http::StatusCode, Json};
use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::auth::Claims;

#[derive(Deserialize)]
pub struct RegisterRequest { pub email: String, pub password: String }
#[derive(Deserialize)]
pub struct LoginRequest { pub email: String, pub password: String }
#[derive(Serialize)]
pub struct TokenResponse { pub access_token: String, pub token_type: &'static str }

fn token(claims: Claims, secret: &str) -> Result<String, StatusCode> {
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn register(State((pool, secret)): State<(PgPool, String)>, Json(req): Json<RegisterRequest>) -> Result<(StatusCode, Json<TokenResponse>), StatusCode> {
    if !req.email.contains('@') || req.password.len() < 8 { return Err(StatusCode::BAD_REQUEST); }
    let email = req.email.trim().to_lowercase();
    let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE lower(email) = $1 LIMIT 1").bind(&email).fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if exists.is_some() { return Err(StatusCode::CONFLICT); }
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(req.password.as_bytes(), &salt).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.to_string();
    let id: Uuid = sqlx::query_scalar("INSERT INTO users (email, password_hash, role, status) VALUES ($1, $2, 'BUYER', 'ACTIVE') RETURNING id").bind(&email).bind(hash).fetch_one(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let claims = Claims { sub: id, email, role: "BUYER".into(), exp: (chrono::Utc::now().timestamp() + 3600) as usize };
    Ok((StatusCode::CREATED, Json(TokenResponse { access_token: token(claims, &secret)?, token_type: "Bearer" })))
}

pub async fn login(State((pool, secret)): State<(PgPool, String)>, Json(req): Json<LoginRequest>) -> Result<Json<TokenResponse>, StatusCode> {
    let email = req.email.trim().to_lowercase();
    let row: Option<(Uuid, String, String, String)> = sqlx::query_as("SELECT id, password_hash, role, status FROM users WHERE lower(email) = $1 LIMIT 1").bind(&email).fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let (id, hash, role, status) = row.ok_or(StatusCode::UNAUTHORIZED)?;
    if status != "ACTIVE" { return Err(StatusCode::FORBIDDEN); }
    let parsed = PasswordHash::new(&hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Argon2::default().verify_password(req.password.as_bytes(), &parsed).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let claims = Claims { sub: id, email, role, exp: (chrono::Utc::now().timestamp() + 3600) as usize };
    Ok(Json(TokenResponse { access_token: token(claims, &secret)?, token_type: "Bearer" }))
}
