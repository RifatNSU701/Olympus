use axum::{extract::{Extension, Path, State}, http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::{auth::Claims, audit, rbac::{authorize, Permission}, state::AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct PlatformStats {
    pub users: i64,
    pub sellers: i64,
    pub products: i64,
    pub orders: i64,
    pub gross_sales: Decimal,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminUser {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub role: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserStatus { pub status: String }

pub async fn stats(State(state): State<AppState>, Extension(claims): Extension<Claims>) -> Result<Json<PlatformStats>, StatusCode> {
    authorize(&claims, Permission::ManageUsers)?;
    let users = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users").fetch_one(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let sellers = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE role='SELLER'").fetch_one(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let products = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM products WHERE status <> 'DELETED'").fetch_one(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let orders = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM orders").fetch_one(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let gross_sales = sqlx::query_scalar::<_, Option<Decimal>>("SELECT COALESCE(SUM(total_amount),0) FROM orders WHERE status IN ('PAID','PROCESSING','SHIPPED','DELIVERED')").fetch_one(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.unwrap_or(Decimal::ZERO);
    Ok(Json(PlatformStats { users, sellers, products, orders, gross_sales }))
}

pub async fn list_users(State(state): State<AppState>, Extension(claims): Extension<Claims>) -> Result<Json<Vec<AdminUser>>, StatusCode> {
    authorize(&claims, Permission::ManageUsers)?;
    let users = sqlx::query_as::<_, AdminUser>("SELECT id,email,full_name,role,status FROM users ORDER BY created_at DESC LIMIT 200").fetch_all(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}

pub async fn update_user_status(State(state): State<AppState>, Extension(claims): Extension<Claims>, Path(id): Path<Uuid>, Json(req): Json<UpdateUserStatus>) -> Result<Json<AdminUser>, StatusCode> {
    authorize(&claims, Permission::ManageUsers)?;
    let status = req.status.trim().to_uppercase();
    if !matches!(status.as_str(), "ACTIVE" | "SUSPENDED" | "BANNED") { return Err(StatusCode::BAD_REQUEST); }
    if id == claims.sub { return Err(StatusCode::CONFLICT); }
    let user = sqlx::query_as::<_, AdminUser>("UPDATE users SET status=$1, updated_at=NOW() WHERE id=$2 RETURNING id,email,full_name,role,status").bind(&status).bind(id).fetch_optional(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    audit::record(&state.pool, Some(claims.sub), audit::AuditEvent { action: "USER_STATUS_CHANGED", entity_type: "USER", entity_id: Some(id) }).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(user))
}
