use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AuditEvent<'a> {
    pub action: &'a str,
    pub entity_type: &'a str,
    pub entity_id: Option<Uuid>,
}

pub async fn record(
    pool: &PgPool,
    actor_id: Option<Uuid>,
    event: AuditEvent<'_>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO audit_logs (actor_id, action, entity_type, entity_id) VALUES ($1, $2, $3, $4)",
    )
    .bind(actor_id)
    .bind(event.action)
    .bind(event.entity_type)
    .bind(event.entity_id)
    .execute(pool)
    .await
    .map(|_| ())
}
