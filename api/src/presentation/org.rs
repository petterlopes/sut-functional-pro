use axum::{
    extract::{Extension, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(sqlx::FromRow, serde::Serialize)]
struct Unit {
    id: Uuid,
    name: String,
    parent_id: Option<Uuid>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
struct Dept {
    id: Uuid,
    unit_id: Uuid,
    name: String,
}

#[derive(Deserialize)]
struct DeptParams {
    #[serde(rename = "unitId")]
    unit_id: Uuid,
}

pub fn routes() -> Router<std::sync::Arc<crate::AppState>> {
    Router::new()
        .route("/v1/org/units", get(units))
        .route("/v1/org/departments", get(departments))
}

async fn units(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
) -> Json<serde_json::Value> {
    if !crate::shared::has_scope(&claims, "directory.read") {
        return Json(serde_json::json!({"error": "Access denied"}));
    }
    let rows: Vec<Unit> = sqlx::query_as("SELECT id, name, parent_id FROM org_units ORDER BY name")
        .fetch_all(&st.pg)
        .await
        .unwrap_or_default();
    Json(serde_json::json!({ "items": rows }))
}

async fn departments(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
    Query(q): Query<DeptParams>,
) -> Json<serde_json::Value> {
    if !crate::shared::has_scope(&claims, "directory.read") {
        return Json(serde_json::json!({"error": "Access denied"}));
    }
    let rows: Vec<Dept> =
        sqlx::query_as("SELECT id, unit_id, name FROM departments WHERE unit_id=$1 ORDER BY name")
            .bind(q.unit_id)
            .fetch_all(&st.pg)
            .await
            .unwrap_or_default();
    Json(serde_json::json!({ "items": rows }))
}
