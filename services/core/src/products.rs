use axum::{extract::{Path, Query, State}, http::StatusCode, Extension, Json};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{auth::Claims, state::AppState};

#[derive(Debug, Serialize)]
pub struct Product { pub id: Uuid, pub seller_id: Uuid, pub category_id: Option<Uuid>, pub name: String, pub slug: String, pub description: Option<String>, pub price: Decimal, pub stock: i32, pub status: String }

#[derive(Deserialize)]
pub struct ListQuery { pub page: Option<i64>, pub limit: Option<i64>, pub category_id: Option<Uuid> }

#[derive(Deserialize)]
pub struct CreateProduct { pub category_id: Option<Uuid>, pub name: String, pub slug: String, pub description: Option<String>, pub price: Decimal, pub stock: i32 }

#[derive(Deserialize)]
pub struct UpdateProduct { pub category_id: Option<Uuid>, pub name: Option<String>, pub description: Option<String>, pub price: Option<Decimal>, pub status: Option<String> }

#[derive(Deserialize)]
pub struct StockUpdate { pub stock: i32 }

fn seller_allowed(role: &str) -> bool { matches!(role, "SELLER" | "ADMIN" | "SUPER_ADMIN") }

pub async fn list(State(state): State<AppState>, Query(q): Query<ListQuery>) -> Result<Json<Vec<Product>>, StatusCode> {
    let limit = q.limit.unwrap_or(24).clamp(1, 100);
    let page = q.page.unwrap_or(0).max(0);
    let rows = sqlx::query_as::<_, (Uuid,Uuid,Option<Uuid>,String,String,Option<String>,Decimal,i32,String)>("SELECT id,seller_id,category_id,name,slug,description,price,stock,status FROM products WHERE status='ACTIVE' AND ($1::uuid IS NULL OR category_id=$1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
        .bind(q.category_id).bind(limit).bind(page * limit).fetch_all(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(rows.into_iter().map(|r| Product { id:r.0,seller_id:r.1,category_id:r.2,name:r.3,slug:r.4,description:r.5,price:r.6,stock:r.7,status:r.8 }).collect()))
}

pub async fn get(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Product>, StatusCode> {
    let r = sqlx::query_as::<_, (Uuid,Uuid,Option<Uuid>,String,String,Option<String>,Decimal,i32,String)>("SELECT id,seller_id,category_id,name,slug,description,price,stock,status FROM products WHERE id=$1 AND status='ACTIVE'")
        .bind(id).fetch_optional(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(Product { id:r.0,seller_id:r.1,category_id:r.2,name:r.3,slug:r.4,description:r.5,price:r.6,stock:r.7,status:r.8 }))
}

pub async fn create(State(state): State<AppState>, Extension(claims): Extension<Claims>, Json(req): Json<CreateProduct>) -> Result<(StatusCode, Json<Product>), StatusCode> {
    if !seller_allowed(&claims.role) { return Err(StatusCode::FORBIDDEN); }
    if req.name.trim().is_empty() || req.slug.trim().is_empty() || req.name.len()>240 || req.slug.len()>260 || req.price.is_sign_negative() || req.stock<0 { return Err(StatusCode::BAD_REQUEST); }
    let slug=req.slug.trim().to_lowercase();
    if sqlx::query_scalar::<_,Uuid>("SELECT id FROM products WHERE slug=$1 LIMIT 1").bind(&slug).fetch_optional(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.is_some() { return Err(StatusCode::CONFLICT); }
    let r=sqlx::query_as::<_,(Uuid,Uuid,Option<Uuid>,String,String,Option<String>,Decimal,i32,String)>("INSERT INTO products (seller_id,category_id,name,slug,description,price,stock,status) VALUES ($1,$2,$3,$4,$5,$6,$7,'ACTIVE') RETURNING id,seller_id,category_id,name,slug,description,price,stock,status")
        .bind(claims.sub).bind(req.category_id).bind(req.name.trim()).bind(&slug).bind(req.description).bind(req.price).bind(req.stock).fetch_one(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED,Json(Product{id:r.0,seller_id:r.1,category_id:r.2,name:r.3,slug:r.4,description:r.5,price:r.6,stock:r.7,status:r.8})))
}

pub async fn update(State(state): State<AppState>, Extension(claims): Extension<Claims>, Path(id): Path<Uuid>, Json(req): Json<UpdateProduct>) -> Result<Json<Product>, StatusCode> {
    if !seller_allowed(&claims.role) { return Err(StatusCode::FORBIDDEN); }
    let current=sqlx::query_as::<_,(Uuid,Uuid,Option<Uuid>,String,String,Option<String>,Decimal,i32,String)>("SELECT id,seller_id,category_id,name,slug,description,price,stock,status FROM products WHERE id=$1 AND seller_id=$2").bind(id).bind(claims.sub).fetch_optional(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    let name=req.name.unwrap_or(current.3); let description=req.description.or(current.5); let price=req.price.unwrap_or(current.6); let category_id=req.category_id.or(current.2); let status=req.status.unwrap_or(current.8);
    if name.trim().is_empty() || name.len()>240 || price.is_sign_negative() || !matches!(status.as_str(),"DRAFT"|"ACTIVE"|"INACTIVE") { return Err(StatusCode::BAD_REQUEST); }
    let r=sqlx::query_as::<_,(Uuid,Uuid,Option<Uuid>,String,String,Option<String>,Decimal,i32,String)>("UPDATE products SET category_id=$1,name=$2,description=$3,price=$4,status=$5,updated_at=NOW() WHERE id=$6 AND seller_id=$7 RETURNING id,seller_id,category_id,name,slug,description,price,stock,status")
        .bind(category_id).bind(name.trim()).bind(description).bind(price).bind(status).bind(id).bind(claims.sub).fetch_one(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(Product{id:r.0,seller_id:r.1,category_id:r.2,name:r.3,slug:r.4,description:r.5,price:r.6,stock:r.7,status:r.8}))
}

pub async fn update_stock(State(state): State<AppState>, Extension(claims): Extension<Claims>, Path(id): Path<Uuid>, Json(req): Json<StockUpdate>) -> Result<Json<Product>, StatusCode> {
    if !seller_allowed(&claims.role) || req.stock<0 { return Err(StatusCode::BAD_REQUEST); }
    let r=sqlx::query_as::<_,(Uuid,Uuid,Option<Uuid>,String,String,Option<String>,Decimal,i32,String)>("UPDATE products SET stock=$1,updated_at=NOW() WHERE id=$2 AND seller_id=$3 AND reserved_stock <= $1 RETURNING id,seller_id,category_id,name,slug,description,price,stock,status")
        .bind(req.stock).bind(id).bind(claims.sub).fetch_optional(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(Product{id:r.0,seller_id:r.1,category_id:r.2,name:r.3,slug:r.4,description:r.5,price:r.6,stock:r.7,status:r.8}))
}
