use axum::{extract::Extension, extract::State, http::StatusCode, routing::{get, post, patch, delete}, Json, Router};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ListResp<T> { pub items: Vec<T> }

// Minimal DTOs (only what the frontend expects to receive)
#[derive(Serialize, Deserialize)]
struct Localidade { pub incdlocalidade: Option<i64>, pub descricao: Option<String> }

#[derive(Serialize, Deserialize)]
struct GenericItem { pub id: Option<i64>, pub name: Option<String> }

pub fn routes() -> Router<std::sync::Arc<crate::AppState>> {
    Router::new()
        .route("/v1/localidades", get(list_localidades).post(create_localidade))
        .route("/v1/localidades/:id", patch(update_localidade).delete(delete_localidade))
        .route("/v1/departamentos", get(list_departamentos).post(create_departamento))
        .route("/v1/departamentos/:id", patch(update_departamento).delete(delete_departamento))
        .route("/v1/tipos-contato", get(list_generic))
        .route("/v1/origens-contato", get(list_generic))
        .route("/v1/ref-origem-contato", get(list_generic))
        .route("/v1/grupos", get(list_generic))
        .route("/v1/grupo-membros", get(list_generic))
        .route("/v1/responsaveis", get(list_generic))
    .route("/v1/sites", get(list_generic))
    .route("/v1/debug/me", get(debug_me))
}

async fn list_localidades(
    State(_st): State<std::sync::Arc<crate::AppState>>,
    Extension(_claims): Extension<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    // return empty list; frontend expects { items: [...] }
    (StatusCode::OK, Json(serde_json::json!({ "items": [] })))
}

async fn create_localidade(
    State(_st): State<std::sync::Arc<crate::AppState>>,
    Extension(_claims): Extension<serde_json::Value>,
    Json(_body): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::CREATED, Json(serde_json::json!({ "ok": true })))
}

async fn update_localidade(
    State(_st): State<std::sync::Arc<crate::AppState>>,
    Extension(_claims): Extension<serde_json::Value>,
    Json(_body): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({ "ok": true })))
}

async fn delete_localidade(
    State(_st): State<std::sync::Arc<crate::AppState>>,
    Extension(_claims): Extension<serde_json::Value>,
) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn list_departamentos(
    State(_st): State<std::sync::Arc<crate::AppState>>,
    Extension(_claims): Extension<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({ "items": [] })))
}

async fn create_departamento(
    State(_st): State<std::sync::Arc<crate::AppState>>,
    Extension(_claims): Extension<serde_json::Value>,
    Json(_body): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::CREATED, Json(serde_json::json!({ "ok": true })))
}

async fn update_departamento(
    State(_st): State<std::sync::Arc<crate::AppState>>,
    Extension(_claims): Extension<serde_json::Value>,
    Json(_body): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({ "ok": true })))
}

async fn delete_departamento(
    State(_st): State<std::sync::Arc<crate::AppState>>,
    Extension(_claims): Extension<serde_json::Value>,
) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn list_generic(
    State(_st): State<std::sync::Arc<crate::AppState>>,
    Extension(_claims): Extension<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({ "items": [] })))
}

async fn debug_me(
    Extension(claims): Extension<serde_json::Value>,
) -> impl IntoResponse {
    if !crate::shared::has_scope(&claims, "admin.debug") {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Access denied"}))).into_response();
    }
    Json(claims).into_response()
}
