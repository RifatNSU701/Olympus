use sqlx::PgPool;
use crate::ai_client::AiClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
    pub ai: AiClient,
}
