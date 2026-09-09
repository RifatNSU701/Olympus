use axum::{extract::{Extension, State}, http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;
use crate::{auth::Claims, state::AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct SellerDashboard {
    pub product_count: i64,
    pub low_stock_count: i64,
    pub order_count: i64,
    pub total_revenue: Decimal,
}

pub async fn dashboard(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SellerDashboard>, StatusCode> {
    if !matches!(claims.role.as_str(), "SELLER" | "ADMIN" | "SUPER_ADMIN") {
        return Err(StatusCode::FORBIDDEN);
    }

    let product_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM products WHERE seller_id=$1 AND status <> 'DELETED'",
    )
    .bind(claims.sub)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let low_stock_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM products WHERE seller_id=$1 AND status='ACTIVE' AND stock <= 5",
    )
    .bind(claims.sub)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let order_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(DISTINCT oi.order_id) FROM order_items oi WHERE oi.seller_id=$1",
    )
    .bind(claims.sub)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_revenue = sqlx::query_scalar::<_, Option<Decimal>>(
        "SELECT COALESCE(SUM(oi.unit_price * oi.quantity), 0) FROM order_items oi JOIN orders o ON o.id=oi.order_id WHERE oi.seller_id=$1 AND o.status IN ('PAID','PROCESSING','SHIPPED','DELIVERED')",
    )
    .bind(claims.sub)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(Decimal::ZERO);

    Ok(Json(SellerDashboard { product_count, low_stock_count, order_count, total_revenue }))
}
