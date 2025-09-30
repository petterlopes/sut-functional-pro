use axum::{
    extract::{Extension, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use sqlx::Row;

#[derive(Deserialize)]
pub struct Params {
    q: String,
    limit: Option<i64>,
    autocomplete: Option<bool>,
}

pub fn routes() -> Router<std::sync::Arc<crate::AppState>> {
    Router::new().route("/v1/search", get(search))
}

fn mk_tsquery(q: &str, autocomplete: bool) -> String {
    let terms: Vec<String> = q
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|t| {
            let mut t = t.to_string();
            t = t.replace("'", "");
            if autocomplete {
                format!("{}:*", t)
            } else {
                t
            }
        })
        .collect();
    if terms.is_empty() {
        return "".into();
    }
    terms.join(" & ")
}

async fn search(
    State(st): State<std::sync::Arc<crate::AppState>>,
    Extension(claims): Extension<serde_json::Value>,
    Query(p): Query<Params>,
) -> impl IntoResponse {
    if !crate::shared::has_scope(&claims, "directory.read") {
        return (axum::http::StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Access denied"}))).into_response();
    }
    let q = p.q.trim().to_string();
    let limit = p.limit.unwrap_or(20);
    let tsq = mk_tsquery(&q, p.autocomplete.unwrap_or(false));
    let like = format!("%{}%", q.to_lowercase());

    let items: Vec<serde_json::Value> = if tsq.is_empty() {
        Vec::new()
    } else {
        sqlx::query(
            r#"
      SELECT id, full_name,
             ts_rank_cd(search_vector, to_tsquery('simple', immutable_unaccent($1))) AS rank,
             similarity(full_name_norm, LOWER(immutable_unaccent($2))) AS s
      FROM contacts
      WHERE search_vector @@ to_tsquery('simple', immutable_unaccent($1))
         OR full_name_norm ILIKE $3
      ORDER BY rank DESC NULLS LAST, s DESC NULLS LAST, full_name ASC
      LIMIT $4
    "#,
        )
        .bind(&tsq)
        .bind(&q)
        .bind(&like)
        .bind(limit)
        .map(|row: sqlx::postgres::PgRow| {
            let id: uuid::Uuid = row.get("id");
            let full_name: String = row.get("full_name");
            serde_json::json!({ "id": id, "fullName": full_name })
        })
        .fetch_all(&st.pg)
        .await
        .unwrap_or_default()
    };

    Json(serde_json::json!({
      "items": items,
      "nextCursor": null
    })).into_response()
}
