use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn connect() -> Result<PgPool> {
    let url = env::var("DATABASE_URL").context("DATABASE_URL is required")?;
    let max = env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(20);

    let pool = PgPoolOptions::new()
        .max_connections(max)
        .connect(&url)
        .await
        .context("failed to connect to PostgreSQL")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to apply database migrations")?;

    Ok(pool)
}
