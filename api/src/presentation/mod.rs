use axum::extract::Request;
use axum::{middleware, middleware::Next, Router};
use std::sync::Arc;
use tracing::{info_span, Span};

pub mod auth;
mod docs;
mod health;
pub mod clean;

async fn span_enricher(
    req: Request,
    next: Next,
) -> Result<axum::response::Response, axum::http::StatusCode> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let span = info_span!("http.request", "http.method"=%method, "http.route"=%path);
    let _enter = span.enter();
    if let Some(claims) = req.extensions().get::<serde_json::Value>() {
        if let Some(sub) = claims.get("sub").and_then(|s| s.as_str()) {
            Span::current().record("enduser.id", &tracing::field::display(sub));
        }
    }
    Ok(next.run(req).await)
}

pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        .merge(docs::routes())
        .layer(middleware::from_fn(span_enricher))
        .merge(health::routes())
        // Clean Architecture routes only
        .merge(clean::contact_controller::routes().route_layer(middleware::from_fn(auth::jwt_middleware)))
        .merge(clean::org_unit_controller::routes().route_layer(middleware::from_fn(auth::jwt_middleware)))
        .merge(clean::department_controller::routes().route_layer(middleware::from_fn(auth::jwt_middleware)))
        .merge(clean::user_controller::routes().route_layer(middleware::from_fn(auth::jwt_middleware)))
}
