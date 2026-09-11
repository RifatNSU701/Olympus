use axum::{extract::{Extension, Path, State}, http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::{auth::Claims, rbac::{authorize, Permission}, state::AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct SellerOrder {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub status: String,
    pub currency: String,
    pub subtotal: Decimal,
    pub shipping_amount: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SellerOrderItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub seller_id: Uuid,
    pub quantity: i32,
    pub unit_price: Decimal,
}

#[derive(Debug, Serialize)]
pub struct SellerOrderDetail { pub order: SellerOrder, pub items: Vec<SellerOrderItem> }

#[derive(Debug, Deserialize)]
pub struct StatusRequest { pub status: String }

pub async fn list(State(state): State<AppState>, Extension(claims): Extension<Claims>) -> Result<Json<Vec<SellerOrder>>, StatusCode> {
    authorize(&claims, Permission::SellerOperations)?;
    sqlx::query_as::<_, SellerOrder>("SELECT DISTINCT o.id,o.buyer_id,o.status,o.currency,o.subtotal,o.shipping_amount,o.tax_amount,o.total_amount,o.created_at FROM orders o JOIN order_items oi ON oi.order_id=o.id WHERE oi.seller_id=$1 ORDER BY o.created_at DESC")
        .bind(claims.sub).fetch_all(&state.pool).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get(State(state): State<AppState>, Extension(claims): Extension<Claims>, Path(id): Path<Uuid>) -> Result<Json<SellerOrderDetail>, StatusCode> {
    authorize(&claims, Permission::SellerOperations)?;
    let order = sqlx::query_as::<_, SellerOrder>("SELECT DISTINCT o.id,o.buyer_id,o.status,o.currency,o.subtotal,o.shipping_amount,o.tax_amount,o.total_amount,o.created_at FROM orders o JOIN order_items oi ON oi.order_id=o.id WHERE o.id=$1 AND oi.seller_id=$2")
        .bind(id).bind(claims.sub).fetch_optional(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    let items = sqlx::query_as::<_, SellerOrderItem>("SELECT id,product_id,seller_id,quantity,unit_price FROM order_items WHERE order_id=$1 AND seller_id=$2 ORDER BY id")
        .bind(id).bind(claims.sub).fetch_all(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(SellerOrderDetail { order, items }))
}

pub async fn update_status(State(state): State<AppState>, Extension(claims): Extension<Claims>, Path(id): Path<Uuid>, Json(req): Json<StatusRequest>) -> Result<Json<SellerOrder>, StatusCode> {
    authorize(&claims, Permission::SellerOperations)?;
    let status = req.status.trim().to_uppercase();
    if !matches!(status.as_str(), "PROCESSING" | "SHIPPED" | "DELIVERED" | "CANCELLED") { return Err(StatusCode::BAD_REQUEST); }
    let result = sqlx::query_as::<_, SellerOrder>("UPDATE orders SET status=$1,updated_at=NOW() WHERE id=$2 AND EXISTS (SELECT 1 FROM order_items WHERE order_id=orders.id AND seller_id=$3) RETURNING id,buyer_id,status,currency,subtotal,shipping_amount,tax_amount,total_amount,created_at")
        .bind(status).bind(id).bind(claims.sub).fetch_optional(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(result))
}
