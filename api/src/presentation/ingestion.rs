use axum::{extract::State, http::HeaderMap, routing::post, Json, Router};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::types::chrono::Utc;

type HmacSha256 = Hmac<Sha256>;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IngestionEvent {
    pub source: String,
    #[serde(rename = "sourceKey")]
    pub source_key: String,
    pub payload: serde_json::Value,
    pub nonce: String,
    pub ts: i64,
}

pub fn routes() -> Router<std::sync::Arc<crate::AppState>> {
    Router::new().route("/v1/ingestion/events", post(receive))
}

async fn receive(
    State(st): State<std::sync::Arc<crate::AppState>>,
    headers: HeaderMap,
    Json(body): Json<IngestionEvent>,
) -> axum::response::Result<axum::Json<serde_json::Value>> {
    let vault = st
        .vault
        .as_ref()
        .ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let secret_value = vault
        .kv_get("webhook")
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let secret_hex = secret_value["data"]["data"]["secret"]
        .as_str()
        .ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let secret_bytes =
        hex::decode(secret_hex).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let sig_hdr = headers
        .get("X-Signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !sig_hdr.starts_with("sha256=") {
        return Err(axum::http::StatusCode::UNAUTHORIZED.into());
    }
    let sig_hex = &sig_hdr[7..];

    let mut mac = HmacSha256::new_from_slice(&secret_bytes)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let body_bytes =
        serde_json::to_vec(&body).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    mac.update(&body_bytes);
    let expected = hex::encode(mac.finalize().into_bytes());
    if expected != sig_hex {
        return Err(axum::http::StatusCode::UNAUTHORIZED.into());
    }

    // Replay window: 5 minutes
    let now = Utc::now().timestamp();
    if (now - body.ts).abs() > 300 {
        return Err(axum::http::StatusCode::UNAUTHORIZED.into());
    }

    // Idempotency (source, nonce)
    let res = sqlx::query(
        "INSERT INTO webhook_receipts (source, nonce) VALUES ($1,$2) ON CONFLICT DO NOTHING",
    )
    .bind(&body.source)
    .bind(&body.nonce)
    .execute(&st.pg)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    if res.rows_affected() == 0 {
        return Ok(Json(serde_json::json!({"status":"duplicate"})));
    }

    Ok(Json(serde_json::json!({"status":"accepted"})))
}
