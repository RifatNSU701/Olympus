use axum::{extract::{Extension, Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::{auth::Claims, state::AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct PaymentStatus {
    pub id: Uuid,
    pub order_id: Uuid,
    pub provider: String,
    pub status: String,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub provider_reference: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyPayment {
    pub provider_reference: String,
    pub success: bool,
}

pub async fn verify(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(payment_id): Path<Uuid>,
    Json(req): Json<VerifyPayment>,
) -> Result<Json<PaymentStatus>, StatusCode> {
    let reference = req.provider_reference.trim();
    if reference.is_empty() || reference.len() > 160 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let payment = sqlx::query_as::<_, PaymentStatus>(
        "SELECT p.id,p.order_id,p.provider,p.status,p.amount,p.currency,p.provider_reference FROM payments p JOIN orders o ON o.id=p.order_id WHERE p.id=$1 AND o.buyer_id=$2 FOR UPDATE",
    )
    .bind(payment_id)
    .bind(claims.sub)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    if payment.status == "PAID" {
        tx.commit()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(payment));
    }
    if payment.status != "PENDING" {
        return Err(StatusCode::CONFLICT);
    }

    let next_status = if req.success { "PAID" } else { "FAILED" };
    let updated = sqlx::query_as::<_, PaymentStatus>(
        "UPDATE payments SET status=$1, provider_reference=$2, paid_at=CASE WHEN $1='PAID' THEN NOW() ELSE paid_at END, updated_at=NOW() WHERE id=$3 RETURNING id,order_id,provider,status,amount,currency,provider_reference",
    )
    .bind(next_status)
    .bind(reference)
    .bind(payment_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let order_status = if req.success { "PAID" } else { "PAYMENT_FAILED" };
    sqlx::query("UPDATE orders SET status=$1, updated_at=NOW() WHERE id=$2")
        .bind(order_status)
        .bind(payment.order_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(updated))
}
