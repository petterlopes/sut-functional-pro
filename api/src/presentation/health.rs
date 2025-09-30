use axum::{routing::get, Router};
pub fn routes() -> Router<std::sync::Arc<crate::AppState>> {
    // Keep the presentation-level probes limited to the `*z` endpoints.
    // The application-level `/health` and `/ready` are defined in `main.rs`
    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/readyz", get(|| async { "ok" }))
}
