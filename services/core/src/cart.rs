use crate::{auth::Claims, state::AppState};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CartItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub unit_price: Decimal,
    pub available_stock: i32,
    pub quantity: i32,
    pub line_total: Decimal,
}
#[derive(Serialize)]
pub struct CartResponse {
    pub id: Uuid,
    pub items: Vec<CartItemResponse>,
    pub subtotal: Decimal,
}
#[derive(Deserialize)]
pub struct AddItemRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}
#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub quantity: i32,
}

async fn ensure_cart(state: &AppState, buyer_id: Uuid) -> Result<Uuid, StatusCode> {
    sqlx::query_scalar("INSERT INTO carts (buyer_id) VALUES ($1) ON CONFLICT (buyer_id) DO UPDATE SET updated_at = now() RETURNING id")
        .bind(buyer_id).fetch_one(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<CartResponse>, StatusCode> {
    let cart_id = ensure_cart(&state, claims.sub).await?;
    let rows = sqlx::query_as::<_, (Uuid, Uuid, String, Decimal, i32, i32)>("SELECT ci.id, p.id, p.name, p.price, p.stock, ci.quantity FROM cart_items ci JOIN products p ON p.id = ci.product_id WHERE ci.cart_id = $1 ORDER BY ci.id")
        .bind(cart_id).fetch_all(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut subtotal = Decimal::ZERO;
    let items = rows
        .into_iter()
        .map(|(id, product_id, name, price, stock, quantity)| {
            let line_total = price * Decimal::from(quantity);
            subtotal += line_total;
            CartItemResponse {
                id,
                product_id,
                product_name: name,
                unit_price: price,
                available_stock: stock,
                quantity,
                line_total,
            }
        })
        .collect();
    Ok(Json(CartResponse {
        id: cart_id,
        items,
        subtotal,
    }))
}

pub async fn add(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<AddItemRequest>,
) -> Result<Json<CartResponse>, StatusCode> {
    if req.quantity <= 0 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cart_id: Uuid = sqlx::query_scalar("INSERT INTO carts (buyer_id) VALUES ($1) ON CONFLICT (buyer_id) DO UPDATE SET updated_at = now() RETURNING id").bind(claims.sub).fetch_one(&mut *tx).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let stock: Option<i32> = sqlx::query_scalar(
        "SELECT stock FROM products WHERE id = $1 AND status = 'ACTIVE' FOR UPDATE",
    )
    .bind(req.product_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let stock = stock.ok_or(StatusCode::NOT_FOUND)?;
    let existing: Option<i32> = sqlx::query_scalar(
        "SELECT quantity FROM cart_items WHERE cart_id = $1 AND product_id = $2 FOR UPDATE",
    )
    .bind(cart_id)
    .bind(req.product_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let quantity = existing.unwrap_or(0) + req.quantity;
    if quantity > stock {
        return Err(StatusCode::CONFLICT);
    }
    sqlx::query("INSERT INTO cart_items (cart_id, product_id, quantity) VALUES ($1, $2, $3) ON CONFLICT (cart_id, product_id) DO UPDATE SET quantity = EXCLUDED.quantity")
        .bind(cart_id).bind(req.product_id).bind(quantity).execute(&mut *tx).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    get(State(state), Extension(claims)).await
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(item_id): Path<Uuid>,
    Json(req): Json<UpdateItemRequest>,
) -> Result<Json<CartResponse>, StatusCode> {
    if req.quantity <= 0 {
        return remove(State(state), Extension(claims), Path(item_id)).await;
    }
    let result = sqlx::query("UPDATE cart_items AS ci SET quantity = $1 FROM carts AS c, products AS p WHERE ci.id = $2 AND ci.cart_id = c.id AND ci.product_id = p.id AND c.buyer_id = $3 AND p.status = 'ACTIVE' AND $1 <= p.stock")
        .bind(req.quantity).bind(item_id).bind(claims.sub).execute(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if result.rows_affected() == 0 {
        return Err(StatusCode::CONFLICT);
    }
    get(State(state), Extension(claims)).await
}

pub async fn remove(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(item_id): Path<Uuid>,
) -> Result<Json<CartResponse>, StatusCode> {
    sqlx::query("DELETE FROM cart_items AS ci USING carts AS c WHERE ci.id = $1 AND ci.cart_id = c.id AND c.buyer_id = $2").bind(item_id).bind(claims.sub).execute(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    get(State(state), Extension(claims)).await
}
