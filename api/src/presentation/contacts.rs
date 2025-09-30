use axum::{
    extract::{Extension, Path, Query, State},
    http::{HeaderMap, StatusCode},
    routing::get,
    Json, Router,
};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Contact {
    pub id: Uuid,
    #[sqlx(rename = "full_name")]
    pub full_name: String,
    pub unit_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub status: Option<String>,
    pub etag: Option<String>,
}

#[derive(Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
}

#[derive(Deserialize)]
pub struct Upsert {
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "unitId")]
    pub unit_id: Option<Uuid>,
    #[serde(rename = "departmentId")]
    pub department_id: Option<Uuid>,
    pub status: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub emails: Option<Vec<String>>,
    #[serde(default)]
    #[allow(dead_code)]
    pub phones: Option<Vec<String>>,
    pub document: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchBody {
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    pub status: Option<String>,
}

pub fn routes() -> Router<std::sync::Arc<crate::AppState>> {
    Router::new()
        .route("/v1/contacts", get(list).post(create))
        .route(
            "/v1/contacts/:id",
            get(get_one).patch(update).delete(delete_one),
        )
        .route(
            "/v1/contacts/:id/document",
            get(get_document).patch(patch_document),
        )
}

async fn list(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
    Query(q): Query<ListParams>,
) -> Json<serde_json::Value> {
    if !crate::shared::has_scope(&claims, "directory.read") {
        return Json(serde_json::json!({"items": [], "nextCursor": null}));
    }
    let rows: Vec<Contact> = sqlx::query_as::<_, Contact>(
        r#"
    SELECT id, full_name, unit_id, department_id, status, etag
    FROM contacts ORDER BY created_at DESC LIMIT $1"#,
    )
    .bind(q.limit.unwrap_or(50))
    .fetch_all(&st.pg)
    .await
    .unwrap_or_default();
    let items: Vec<serde_json::Value> = rows.into_iter().map(|c| serde_json::json!({
    "id": c.id, "fullName": c.full_name, "unitId": c.unit_id, "departmentId": c.department_id, "status": c.status, "etag": c.etag
  })).collect();
    Json(serde_json::json!({ "items": items, "nextCursor": null }))
}

async fn get_one(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
    Path(id): Path<Uuid>,
) -> axum::response::Result<Json<serde_json::Value>> {
    if !crate::shared::has_scope(&claims, "directory.read") {
        return Err(axum::http::StatusCode::FORBIDDEN.into());
    }
    let c: Option<Contact> = sqlx::query_as(
        r#"SELECT id, full_name, unit_id, department_id, status, etag FROM contacts WHERE id=$1"#,
    )
    .bind(id)
    .fetch_optional(&st.pg)
    .await
    .unwrap();
    if let Some(c) = c {
        Ok(Json(serde_json::json!({
          "id": c.id, "fullName": c.full_name, "unitId": c.unit_id, "departmentId": c.department_id, "status": c.status, "etag": c.etag
        })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND.into())
    }
}

async fn create(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
    Json(b): Json<Upsert>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !crate::shared::has_scope(&claims, "directory.write") {
        return Err(StatusCode::FORBIDDEN);
    }
    let mut document_enc: Option<String> = None;
    if let Some(doc) = b.document.clone() {
        if let Some(v) = &st.vault {
            let pt_b64 = base64::engine::general_purpose::STANDARD.encode(doc.as_bytes());
            let ct = v
                .transit_encrypt("pii-doc", &pt_b64)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            document_enc = Some(ct);
        }
    }
    let rec: Contact = sqlx::query_as(
        r#"
    INSERT INTO contacts (id, full_name, unit_id, department_id, status, document)
    VALUES (gen_random_uuid(), $1, $2, $3, COALESCE($4,'ACTIVE'), $5)
    RETURNING id, full_name, unit_id, department_id, status, etag"#,
    )
    .bind(b.full_name)
    .bind(b.unit_id)
    .bind(b.department_id)
    .bind(b.status)
    .bind(document_enc)
    .fetch_one(&st.pg)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({
      "id": rec.id, "fullName": rec.full_name, "unitId": rec.unit_id, "departmentId": rec.department_id, "status": rec.status, "etag": rec.etag
    })))
}

async fn update(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(b): Json<PatchBody>,
) -> Result<(HeaderMap, Json<serde_json::Value>), StatusCode> {
    if !crate::shared::has_scope(&claims, "directory.write") {
        return Err(StatusCode::FORBIDDEN);
    }
    let Some(if_match) = headers.get("If-Match").and_then(|v| v.to_str().ok()) else {
        return Err(StatusCode::PRECONDITION_FAILED);
    };
    let current: Option<(String,)> = sqlx::query_as("SELECT etag FROM contacts WHERE id=$1")
        .bind(id)
        .fetch_optional(&st.pg)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if current.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    if current.as_ref().unwrap().0 != if_match {
        return Err(StatusCode::PRECONDITION_FAILED);
    }

    let full = b.full_name.clone().unwrap_or_default();
    let status = b.status.clone().unwrap_or("ACTIVE".into());

    let rec: Contact = sqlx::query_as(
        r#"
    UPDATE contacts SET full_name = COALESCE(NULLIF($2,''), full_name), status=$3, updated_at=now()
    WHERE id=$1
    RETURNING id, full_name, unit_id, department_id, status, etag
  "#,
    )
    .bind(id)
    .bind(full)
    .bind(status)
    .fetch_one(&st.pg)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut resp_headers = HeaderMap::new();
    if let Some(e) = rec.etag.clone() {
        resp_headers.insert("ETag", axum::http::HeaderValue::from_str(&e).ok().unwrap());
    }
    Ok((
        resp_headers,
        Json(
            serde_json::json!({ "id": rec.id, "fullName": rec.full_name, "unitId": rec.unit_id, "departmentId": rec.department_id, "status": rec.status, "etag": rec.etag }),
        ),
    ))
}

async fn delete_one(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    if !crate::shared::has_scope(&claims, "directory.write") {
        return StatusCode::FORBIDDEN;
    }
    let r = sqlx::query("DELETE FROM contacts WHERE id=$1")
        .bind(id)
        .execute(&st.pg)
        .await;
    match r {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn get_document(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
    Path(id): Path<Uuid>,
) -> axum::response::Result<Json<serde_json::Value>> {
    if !crate::shared::has_scope(&claims, "directory.pii.read") {
        return Err(axum::http::StatusCode::FORBIDDEN.into());
    }
    let row: Option<(Option<String>,)> =
        sqlx::query_as("SELECT document FROM contacts WHERE id=$1")
            .bind(id)
            .fetch_optional(&st.pg)
            .await
            .unwrap();
    if row.is_none() {
        return Err(axum::http::StatusCode::NOT_FOUND.into());
    }
    let ct = row.unwrap().0.unwrap_or_default();
    if ct.is_empty() {
        return Ok(Json(serde_json::json!({ "document": null })));
    }
    let v = st
        .vault
        .as_ref()
        .ok_or_else(|| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let pt_b64 = v
        .transit_decrypt("pii-doc", &ct)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(pt_b64)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let doc =
        String::from_utf8(bytes).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({ "document": doc })))
}

#[derive(Deserialize)]
struct DocPatch {
    document: Option<String>,
}

async fn patch_document(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
    Path(id): Path<Uuid>,
    Json(b): Json<DocPatch>,
) -> Result<(HeaderMap, Json<serde_json::Value>), StatusCode> {
    if !crate::shared::has_scope(&claims, "directory.pii.read") {
        return Err(StatusCode::FORBIDDEN);
    }
    if b.document.is_some() && st.vault.is_none() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let before: Option<serde_json::Value> = sqlx::query_scalar("SELECT row_to_json(c) FROM (SELECT id, full_name, status, unit_id, department_id FROM contacts WHERE id=$1) c")
    .bind(id).fetch_optional(&st.pg).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut enc: Option<String> = None;
    if let Some(doc) = b.document.clone() {
        let vault = st.vault.as_ref().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let pt_b64 = B64.encode(doc.as_bytes());
        enc = Some(
            vault
                .transit_encrypt("pii-doc", &pt_b64)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        );
    }
    let rec: Option<(uuid::Uuid, String, Option<String>)> = sqlx::query_as("UPDATE contacts SET document=COALESCE($2, document), updated_at=now() WHERE id=$1 RETURNING id::uuid, etag, document")
    .bind(id).bind(enc).fetch_optional(&st.pg).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rec.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    let etag = rec.as_ref().unwrap().1.clone();

    // log audit
    let sub = claims.get("sub").and_then(|s| s.as_str());
    crate::infra::audit::log_audit(
        &st,
        sub,
        "PATCH_DOCUMENT",
        "contact",
        &id.to_string(),
        before,
        Some(serde_json::json!({"hasDocument": b.document.is_some()})),
    )
    .await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "ETag",
        axum::http::HeaderValue::from_str(&etag).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    Ok((
        headers,
        Json(serde_json::json!({ "status": "ok", "etag": etag })),
    ))
}
