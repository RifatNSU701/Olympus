use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct RecommendationRequest { pub product_ids: Vec<Uuid>, pub limit: Option<u8> }

#[derive(Debug, Serialize)]
pub struct RecommendationResponse { pub product_id: Uuid, pub score: f32, pub reason: String }

pub async fn recommend(
    State(state): State<AppState>,
    Json(request): Json<RecommendationRequest>,
) -> Result<Json<Vec<RecommendationResponse>>, StatusCode> {
    if request.product_ids.is_empty() || request.product_ids.len() > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let ids = request.product_ids.iter().map(Uuid::to_string).collect();
    let limit = request.limit.unwrap_or(5).clamp(1, 20);
    let recommendations = state.ai.recommend(ids, limit).await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let response = recommendations.into_iter().filter_map(|item| {
        Uuid::parse_str(&item.product_id).ok().map(|product_id| RecommendationResponse { product_id, score: item.score, reason: item.reason })
    }).collect();
    Ok(Json(response))
}
