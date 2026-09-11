use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::FromRow;
use uuid::Uuid;

use crate::{auth::Claims, state::AppState};

type HmacSha256 = Hmac<Sha256>;

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

#[derive(Debug, Deserialize)]
pub struct PaymentWebhook {
    pub payment_id: Uuid,
    pub provider_reference: String,
    pub status: String,
    pub amount: rust_decimal::Decimal,
}

pub fn normalize_webhook_status(status: &str) -> Option<&'static str> {
    match status.trim().to_uppercase().as_str() {
        "PAID" => Some("PAID"),
        "FAILED" => Some("FAILED"),
        _ => None,
    }
}

pub fn verify_webhook_signature(secret: &[u8], payload: &[u8], signature: &str) -> bool {
    let provided = signature.strip_prefix("sha256=").unwrap_or(signature);
    if provided.len() != 64 || !provided.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return false;
    }
    let Ok(provided_bytes) = hex::decode(provided) else {
        return false;
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(secret) else {
        return false;
    };
    mac.update(payload);
    mac.verify_slice(&provided_bytes).is_ok()
}

pub async fn verify(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(payment_id): Path<Uuid>,
    Json(req): Json<VerifyPayment>,
) -> Result<Json<PaymentStatus>, StatusCode> {
    if std::env::var("OLYMPUS_ALLOW_SIMULATED_PAYMENTS")
        .ok()
        .as_deref()
        != Some("true")
    {
        return Err(StatusCode::NOT_IMPLEMENTED);
    }

    let reference = req.provider_reference.trim();
    if reference.is_empty() || reference.len() > 160 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = state.pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
        tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(updated))
}

pub async fn webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<PaymentWebhook>,
) -> Result<Json<PaymentStatus>, StatusCode> {
    let secret = std::env::var("PAYMENT_WEBHOOK_SECRET")
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let signature = headers
        .get("x-olympus-signature")
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let reference = req.provider_reference.trim();
    if reference.is_empty() || reference.len() > 160 || req.amount.is_sign_negative() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let Some(status) = normalize_webhook_status(&req.status) else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let canonical = serde_json::to_string(&req).map_err(|_| StatusCode::BAD_REQUEST)?;
    if !verify_webhook_signature(secret.as_bytes(), canonical.as_bytes(), signature) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut tx = state.pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let payment = sqlx::query_as::<_, PaymentStatus>(
        "SELECT id,order_id,provider,status,amount,currency,provider_reference FROM payments WHERE id=$1 FOR UPDATE",
    )
    .bind(req.payment_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    if payment.amount != req.amount {
        return Err(StatusCode::BAD_REQUEST);
    }
    if payment.status == status {
        tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(payment));
    }
    if payment.status != "PENDING" {
        return Err(StatusCode::CONFLICT);
    }

    let updated = sqlx::query_as::<_, PaymentStatus>(
        "UPDATE payments SET status=$1,provider_reference=$2,paid_at=CASE WHEN $1='PAID' THEN NOW() ELSE paid_at END,updated_at=NOW() WHERE id=$3 RETURNING id,order_id,provider,status,amount,currency,provider_reference",
    )
    .bind(status)
    .bind(reference)
    .bind(req.payment_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let order_status = if status == "PAID" { "PAID" } else { "PAYMENT_FAILED" };
    sqlx::query("UPDATE orders SET status=$1,updated_at=NOW() WHERE id=$2")
        .bind(order_status)
        .bind(payment.order_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(updated))
}

#[cfg(test)]
mod tests {
    use super::{normalize_webhook_status, verify_webhook_signature};
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    #[test]
    fn webhook_status_is_normalized_and_restricted() {
        assert_eq!(normalize_webhook_status(" paid "), Some("PAID"));
        assert_eq!(normalize_webhook_status("FAILED"), Some("FAILED"));
        assert_eq!(normalize_webhook_status("PENDING"), None);
    }

    #[test]
    fn webhook_signature_accepts_valid_signature() {
        let secret = b"test-secret";
        let payload = br#"{"payment_id":"123","status":"PAID"}"#;
        let mut mac = Hmac::<Sha256>::new_from_slice(secret).unwrap();
        mac.update(payload);
        let signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));
        assert!(verify_webhook_signature(secret, payload, &signature));
        assert!(verify_webhook_signature(secret, payload, signature.trim_start_matches("sha256=")));
    }

    #[test]
    fn webhook_signature_rejects_tampering_and_malformed_values() {
        let secret = b"test-secret";
        let payload = b"payload";
        assert!(!verify_webhook_signature(secret, b"tampered", "00"));
        assert!(!verify_webhook_signature(secret, payload, "sha256=not-hex"));
        assert!(!verify_webhook_signature(secret, payload, "sha256=00"));
    }
}
