use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::Claims, state::AppState};

#[derive(Deserialize)]
pub struct CheckoutRequest {
    pub idempotency_key: String,
    pub shipping_amount: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub status: String,
    pub currency: String,
    pub subtotal: Decimal,
    pub shipping_amount: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
}

pub async fn checkout(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CheckoutRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), StatusCode> {
    let idempotency_key = req.idempotency_key.trim();
    if idempotency_key.is_empty() || idempotency_key.len() > 120 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let shipping = req.shipping_amount.unwrap_or(Decimal::ZERO);
    let tax = req.tax_amount.unwrap_or(Decimal::ZERO);
    if shipping.is_sign_negative() || tax.is_sign_negative() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(existing) = sqlx::query_as::<_, (Uuid, String, String, Decimal, Decimal, Decimal, Decimal)>(
        "SELECT id,status,currency,subtotal,shipping_amount,tax_amount,total_amount FROM orders WHERE buyer_id=$1 AND idempotency_key=$2",
    )
    .bind(claims.sub)
    .bind(idempotency_key)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Ok((
            StatusCode::OK,
            Json(OrderResponse {
                id: existing.0,
                status: existing.1,
                currency: existing.2,
                subtotal: existing.3,
                shipping_amount: existing.4,
                tax_amount: existing.5,
                total_amount: existing.6,
            }),
        ));
    }

    let cart_id: Uuid = sqlx::query_scalar("SELECT id FROM carts WHERE buyer_id=$1 FOR UPDATE")
        .bind(claims.sub)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)?;

    let items = sqlx::query_as::<_, (Uuid, Uuid, Uuid, i32, Decimal, i32)>(
        "SELECT ci.id,p.id,p.seller_id,ci.quantity,p.price,p.stock FROM cart_items ci JOIN products p ON p.id=ci.product_id WHERE ci.cart_id=$1 AND p.status='ACTIVE' FOR UPDATE OF p",
    )
    .bind(cart_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if items.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut subtotal = Decimal::ZERO;
    for (_, _, _, quantity, price, stock) in &items {
        if *quantity > *stock {
            return Err(StatusCode::CONFLICT);
        }
        subtotal += *price * Decimal::from(*quantity);
    }
    let total = subtotal + shipping + tax;

    let order = sqlx::query_as::<_, (Uuid, String, String, Decimal, Decimal, Decimal, Decimal)>(
        "INSERT INTO orders (buyer_id,status,currency,subtotal,shipping_amount,tax_amount,total_amount,idempotency_key) VALUES ($1,'PENDING_PAYMENT','BDT',$2,$3,$4,$5,$6) RETURNING id,status,currency,subtotal,shipping_amount,tax_amount,total_amount",
    )
    .bind(claims.sub)
    .bind(subtotal)
    .bind(shipping)
    .bind(tax)
    .bind(total)
    .bind(idempotency_key)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for (_, product_id, seller_id, quantity, price, _) in items {
        sqlx::query(
            "INSERT INTO order_items (order_id,product_id,seller_id,quantity,unit_price) VALUES ($1,$2,$3,$4,$5)",
        )
        .bind(order.0)
        .bind(product_id)
        .bind(seller_id)
        .bind(quantity)
        .bind(price)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        sqlx::query("UPDATE products SET stock=stock-$1, updated_at=now() WHERE id=$2")
            .bind(quantity)
            .bind(product_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    sqlx::query("DELETE FROM cart_items WHERE cart_id=$1")
        .bind(cart_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(OrderResponse {
            id: order.0,
            status: order.1,
            currency: order.2,
            subtotal: order.3,
            shipping_amount: order.4,
            tax_amount: order.5,
            total_amount: order.6,
        }),
    ))
}
