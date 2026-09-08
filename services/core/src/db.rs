use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn connect() -> Result<PgPool> {
    let url = env::var("DATABASE_URL").context("DATABASE_URL is required")?;
    let max = env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(20);

    PgPoolOptions::new()
        .max_connections(max)
        .connect(&url)
        .await
        .context("failed to connect to PostgreSQL")
}
