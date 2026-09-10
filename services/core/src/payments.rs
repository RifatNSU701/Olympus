use axum::{extract::{Extension, Path, State}, http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::{auth::Claims, state::AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub provider: String,
    pub status: String,
    pub amount: Decimal,
    pub currency: String,
    pub provider_reference: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePayment {
    pub provider: String,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(order_id): Path<Uuid>,
    Json(req): Json<CreatePayment>,
) -> Result<(StatusCode, Json<Payment>), StatusCode> {
    let provider = req.provider.trim().to_uppercase();
    if !matches!(provider.as_str(), "BKASH" | "NAGAD" | "STRIPE") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let order = sqlx::query_as::<_, (Uuid, Decimal, String, String)>(
        "SELECT id,total_amount,currency,status FROM orders WHERE id=$1 AND buyer_id=$2 FOR UPDATE",
    )
    .bind(order_id)
    .bind(claims.sub)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    if order.3 == "PAID" {
        return Err(StatusCode::CONFLICT);
    }
    if !matches!(order.3.as_str(), "PENDING_PAYMENT" | "PAYMENT_FAILED") {
        return Err(StatusCode::CONFLICT);
    }

    if sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM payments WHERE order_id=$1 AND status IN ('PENDING','PAID'))",
    )
    .bind(order.0)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::CONFLICT);
    }

    let payment = sqlx::query_as::<_, Payment>(
        "INSERT INTO payments (order_id,provider,status,amount,currency) VALUES ($1,$2,'PENDING',$3,$4) RETURNING id,order_id,provider,status,amount,currency,provider_reference",
    )
    .bind(order.0)
    .bind(&provider)
    .bind(order.1)
    .bind(&order.2)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::CONFLICT)?;

    sqlx::query("UPDATE orders SET status='PENDING_PAYMENT', updated_at=NOW() WHERE id=$1")
        .bind(order.0)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(payment)))
}
