// ============================================================================
// PRESENTATION LAYER MODULE
// ============================================================================
// Módulo principal da camada de apresentação da Clean Architecture
// Organiza controllers, middleware e utilitários

use axum::extract::Request;
use axum::{middleware, middleware::Next, Router};
use std::sync::Arc;
use tracing::{info_span, Span};

// ===== CORE MODULES =====
pub mod auth; // Autenticação e autorização
pub mod clean;
mod docs; // Documentação da API
mod health; // Health checks // Controllers da Clean Architecture

// ===== UTILITY MODULES =====
pub mod error_mapper; // Mapeamento de erros de domínio para HTTP
pub mod handler_macros; // Macros para handlers CRUD genéricos
pub mod response_helpers; // Helpers para respostas HTTP
pub mod security_headers; // Headers de segurança HTTP
pub mod validation; // Utilitários de validação
pub mod webhooks; // Webhooks para serviços externos

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
        .layer(middleware::from_fn(
            security_headers::security_headers_middleware,
        ))
        .merge(health::routes())
        // Clean Architecture routes with security layers
        .merge(
            clean::contact_controller::routes()
                .route_layer(middleware::from_fn(auth::jwt_middleware))
                .route_layer(middleware::from_fn(
                    auth::require_read_permission_middleware,
                )),
        )
        .merge(
            clean::org_unit_controller::routes()
                .route_layer(middleware::from_fn(auth::jwt_middleware))
                .route_layer(middleware::from_fn(
                    auth::require_read_permission_middleware,
                )),
        )
        .merge(
            clean::department_controller::routes()
                .route_layer(middleware::from_fn(auth::jwt_middleware))
                .route_layer(middleware::from_fn(
                    auth::require_read_permission_middleware,
                )),
        )
        .merge(
            clean::user_controller::routes()
                .route_layer(middleware::from_fn(auth::jwt_middleware))
                .route_layer(middleware::from_fn(auth::require_admin_middleware)),
        )
        // Webhook routes (sem autenticação JWT, mas com validação de token)
        .nest("/v1/webhooks", webhooks::webhook_routes())
}
