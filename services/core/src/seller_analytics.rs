use axum::{extract::{Extension, State}, http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::FromRow;
use crate::{auth::Claims, rbac::{authorize, Permission}, state::AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct RevenuePoint {
    pub month: String,
    pub revenue: Decimal,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TopProduct {
    pub name: String,
    pub revenue: Decimal,
    pub units_sold: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CategoryMix {
    pub name: String,
    pub revenue: Decimal,
}

#[derive(Debug, Serialize)]
pub struct SellerAnalytics {
    pub revenue: Decimal,
    pub orders: i64,
    pub average_order_value: Decimal,
    pub revenue_trend: Vec<RevenuePoint>,
    pub top_products: Vec<TopProduct>,
    pub category_mix: Vec<CategoryMix>,
}

pub async fn analytics(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SellerAnalytics>, StatusCode> {
    authorize(&claims, Permission::ViewSellerAnalytics)?;

    let seller_id = claims.sub;
    let revenue = sqlx::query_scalar::<_, Option<Decimal>>(
        "SELECT COALESCE(SUM(oi.unit_price * oi.quantity), 0) FROM order_items oi JOIN orders o ON o.id=oi.order_id WHERE oi.seller_id=$1 AND o.status IN ('PAID','PROCESSING','SHIPPED','DELIVERED')",
    )
    .bind(seller_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(Decimal::ZERO);

    let orders = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(DISTINCT oi.order_id) FROM order_items oi JOIN orders o ON o.id=oi.order_id WHERE oi.seller_id=$1 AND o.status IN ('PAID','PROCESSING','SHIPPED','DELIVERED')",
    )
    .bind(seller_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let average_order_value = if orders > 0 { revenue / Decimal::from(orders) } else { Decimal::ZERO };

    let revenue_trend = sqlx::query_as::<_, RevenuePoint>(
        "SELECT to_char(date_trunc('month', o.created_at), 'YYYY-MM') AS month, COALESCE(SUM(oi.unit_price * oi.quantity), 0) AS revenue FROM order_items oi JOIN orders o ON o.id=oi.order_id WHERE oi.seller_id=$1 AND o.status IN ('PAID','PROCESSING','SHIPPED','DELIVERED') AND o.created_at >= date_trunc('month', now()) - interval '5 months' GROUP BY date_trunc('month', o.created_at) ORDER BY date_trunc('month', o.created_at)",
    )
    .bind(seller_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let top_products = sqlx::query_as::<_, TopProduct>(
        "SELECT p.name, COALESCE(SUM(oi.unit_price * oi.quantity), 0) AS revenue, COALESCE(SUM(oi.quantity), 0)::BIGINT AS units_sold FROM order_items oi JOIN orders o ON o.id=oi.order_id JOIN products p ON p.id=oi.product_id WHERE oi.seller_id=$1 AND o.status IN ('PAID','PROCESSING','SHIPPED','DELIVERED') GROUP BY p.id, p.name ORDER BY revenue DESC LIMIT 10",
    )
    .bind(seller_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let category_mix = sqlx::query_as::<_, CategoryMix>(
        "SELECT COALESCE(c.name, 'Uncategorized') AS name, COALESCE(SUM(oi.unit_price * oi.quantity), 0) AS revenue FROM order_items oi JOIN orders o ON o.id=oi.order_id JOIN products p ON p.id=oi.product_id LEFT JOIN categories c ON c.id=p.category_id WHERE oi.seller_id=$1 AND o.status IN ('PAID','PROCESSING','SHIPPED','DELIVERED') GROUP BY COALESCE(c.name, 'Uncategorized') ORDER BY revenue DESC LIMIT 8",
    )
    .bind(seller_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SellerAnalytics { revenue, orders, average_order_value, revenue_trend, top_products, category_mix }))
}
