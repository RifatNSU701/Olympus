use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::Claims, state::AppState};

#[derive(Debug, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    pub status: String,
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub category_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct CreateProduct {
    pub category_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
}

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    let limit = q.limit.unwrap_or(24).clamp(1, 100);
    let page = q.page.unwrap_or(0).max(0);
    let offset = page * limit;

    let rows = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        Option<Uuid>,
        String,
        String,
        Option<String>,
        Decimal,
        i32,
        String,
    )>(
        "SELECT id, seller_id, category_id, name, slug, description, price, stock, status FROM products WHERE status = 'ACTIVE' AND ($1::uuid IS NULL OR category_id = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(q.category_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        rows.into_iter()
            .map(|r| Product {
                id: r.0,
                seller_id: r.1,
                category_id: r.2,
                name: r.3,
                slug: r.4,
                description: r.5,
                price: r.6,
                stock: r.7,
                status: r.8,
            })
            .collect(),
    ))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, StatusCode> {
    let r = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        Option<Uuid>,
        String,
        String,
        Option<String>,
        Decimal,
        i32,
        String,
    )>(
        "SELECT id, seller_id, category_id, name, slug, description, price, stock, status FROM products WHERE id = $1 AND status = 'ACTIVE'",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(Product {
        id: r.0,
        seller_id: r.1,
        category_id: r.2,
        name: r.3,
        slug: r.4,
        description: r.5,
        price: r.6,
        stock: r.7,
        status: r.8,
    }))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateProduct>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    if claims.role != "SELLER" && claims.role != "ADMIN" && claims.role != "SUPER_ADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    if req.name.trim().is_empty()
        || req.slug.trim().is_empty()
        || req.name.len() > 240
        || req.slug.len() > 260
        || req.price.is_sign_negative()
        || req.stock < 0
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let slug = req.slug.trim().to_lowercase();
    if sqlx::query_scalar::<_, Uuid>("SELECT id FROM products WHERE slug = $1 LIMIT 1")
        .bind(&slug)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some()
    {
        return Err(StatusCode::CONFLICT);
    }

    let r = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        Option<Uuid>,
        String,
        String,
        Option<String>,
        Decimal,
        i32,
        String,
    )>(
        "INSERT INTO products (seller_id, category_id, name, slug, description, price, stock, status) VALUES ($1,$2,$3,$4,$5,$6,$7,'ACTIVE') RETURNING id, seller_id, category_id, name, slug, description, price, stock, status",
    )
    .bind(claims.sub)
    .bind(req.category_id)
    .bind(req.name.trim())
    .bind(&slug)
    .bind(req.description)
    .bind(req.price)
    .bind(req.stock)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(Product {
            id: r.0,
            seller_id: r.1,
            category_id: r.2,
            name: r.3,
            slug: r.4,
            description: r.5,
            price: r.6,
            stock: r.7,
            status: r.8,
        }),
    ))
}
