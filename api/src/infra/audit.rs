use crate::AppState;
use axum::http::StatusCode;

pub async fn log_audit(
    st: &AppState,
    actor_sub: Option<&str>,
    action: &str,
    entity_type: &str,
    entity_id: &str,
    before: Option<serde_json::Value>,
    after: Option<serde_json::Value>,
) -> Result<(), StatusCode> {
    let _ = sqlx::query("INSERT INTO audit_events (actor_sub, action, entity_type, entity_id, before, after) VALUES ($1,$2,$3,$4,$5,$6)")
    .bind(actor_sub.map(|s| s.to_string()))
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(before)
    .bind(after)
    .execute(&st.pg).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
