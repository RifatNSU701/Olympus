use axum::{extract::{Path, State, Extension}, http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;
use crate::{auth::Claims, state::AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct OrderSummary { pub id: Uuid, pub status: String, pub currency: String, pub subtotal: Decimal, pub shipping_amount: Decimal, pub tax_amount: Decimal, pub total_amount: Decimal, pub created_at: chrono::DateTime<chrono::Utc> }

#[derive(Debug, Serialize, FromRow)]
pub struct OrderItem { pub id: Uuid, pub product_id: Uuid, pub seller_id: Uuid, pub quantity: i32, pub unit_price: Decimal }

#[derive(Debug, Serialize)]
pub struct OrderDetail { pub order: OrderSummary, pub items: Vec<OrderItem> }

pub async fn list(State(state): State<AppState>, Extension(claims): Extension<Claims>) -> Result<Json<Vec<OrderSummary>>, StatusCode> {
    let rows = sqlx::query_as::<_, OrderSummary>("SELECT id,status,currency,subtotal,shipping_amount,tax_amount,total_amount,created_at FROM orders WHERE buyer_id=$1 ORDER BY created_at DESC")
        .bind(claims.sub).fetch_all(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(rows))
}

pub async fn get(State(state): State<AppState>, Extension(claims): Extension<Claims>, Path(id): Path<Uuid>) -> Result<Json<OrderDetail>, StatusCode> {
    let order = sqlx::query_as::<_, OrderSummary>("SELECT id,status,currency,subtotal,shipping_amount,tax_amount,total_amount,created_at FROM orders WHERE id=$1 AND buyer_id=$2")
        .bind(id).bind(claims.sub).fetch_optional(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    let items = sqlx::query_as::<_, OrderItem>("SELECT id,product_id,seller_id,quantity,unit_price FROM order_items WHERE order_id=$1 ORDER BY id")
        .bind(id).fetch_all(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(OrderDetail { order, items }))
}
