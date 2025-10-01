use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderName, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod presentation;
mod infra {
    pub mod audit;
    pub mod pg;
    pub mod vault;
}
mod application;
mod domain;
mod infrastructure;
mod shared;

#[derive(Clone)]
pub struct AppState {
    pub pg: sqlx::Pool<sqlx::Postgres>,
    pub vault: Option<infra::vault::VaultClient>,
    pub metrics_token: Option<String>,
    // Clean Architecture repositories
    pub contact_repository: Arc<infrastructure::repositories::PostgresContactRepository>,
    pub org_unit_repository: Arc<infrastructure::repositories::PostgresOrgUnitRepository>,
    pub department_repository: Arc<infrastructure::repositories::PostgresDepartmentRepository>,
    pub user_repository: Arc<infrastructure::repositories::PostgresUserRepository>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();


    let (prometheus_layer, metric_handle) = axum_prometheus::PrometheusMetricLayer::pair();
    let metrics_router = Router::new().route(
        "/metrics",
        get({
            let handle = metric_handle.clone();
            move |State(app): State<Arc<AppState>>, headers: HeaderMap| {
                let handle = handle.clone();
                async move {
                    if let Some(expected) = app.metrics_token.as_deref() {
                        let provided = headers.get("X-Metrics-Token").and_then(|v| v.to_str().ok());
                        if provided != Some(expected) {
                            return StatusCode::UNAUTHORIZED.into_response();
                        }
                    }
                    handle.render().into_response()
                }
            }
        }),
    );

    let fmt_layer = tracing_subscriber::fmt::layer().json();
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();

    let dsn =
        std::env::var("PG_DSN").unwrap_or_else(|_| "postgres://sut:sut@localhost:5432/sut".into());
    let jwks_uri = std::env::var("KEYCLOAK_JWKS").unwrap_or_else(|_| {
        "http://localhost:8081/realms/sut/protocol/openid-connect/certs".into()
    });
    let issuer = std::env::var("KEYCLOAK_ISSUER").ok();
    let audiences = std::env::var("KEYCLOAK_AUDIENCE")
        .map(|v| {
            v.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|_| vec!["sut-frontend".into()]);
    let jwt_leeway = std::env::var("JWT_LEEWAY_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(60);

    let pg = infra::pg::pool(&dsn).await?;
    infra::pg::migrate(&pg).await?;

    presentation::auth::init(presentation::auth::AuthConfig {
        jwks_uri: jwks_uri.clone(),
        issuer,
        audiences,
        leeway_secs: jwt_leeway,
    })
    .await?;

    let vault = match (std::env::var("VAULT_ADDR"), std::env::var("VAULT_TOKEN")) {
        (Ok(addr), Ok(token)) => Some(infra::vault::VaultClient::new(addr, token)),
        _ => None,
    };

    let metrics_token = std::env::var("METRICS_TOKEN").ok();

    // Initialize Clean Architecture repositories
    let contact_repository = Arc::new(infrastructure::repositories::PostgresContactRepository::new(pg.clone()));
    let org_unit_repository = Arc::new(infrastructure::repositories::PostgresOrgUnitRepository::new(pg.clone()));
    let department_repository = Arc::new(infrastructure::repositories::PostgresDepartmentRepository::new(pg.clone()));
    let user_repository = Arc::new(infrastructure::repositories::PostgresUserRepository::new(pg.clone()));

    let state = Arc::new(AppState {
        pg,
        vault,
        metrics_token,
        contact_repository,
        org_unit_repository,
        department_repository,
        user_repository,
    });

    // Spawn a background task to periodically refresh JWKS to keep keys up-to-date
    // and avoid one-off startup races. If refresh fails we'll log and continue.
    let _jwks_uri_clone = jwks_uri.clone();
    tokio::spawn(async move {
        // initial delay before first refresh
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        loop {
            if let Err(e) = presentation::auth::refresh_jwks().await {
                tracing::warn!(error = ?e, "periodic jwks refresh failed");
            }
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    });

    let allowed_origins =
        std::env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "http://localhost:5173".into());
    let origin_values: Vec<HeaderValue> = allowed_origins
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect();
    let cors = if origin_values.is_empty() {
        CorsLayer::new().allow_origin(AllowOrigin::exact(HeaderValue::from_static(
            "http://localhost:5173",
        )))
    } else {
        CorsLayer::new().allow_origin(AllowOrigin::list(origin_values))
    }
    .allow_methods(AllowMethods::list(vec![
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::DELETE,
        Method::OPTIONS,
    ]))
    .allow_headers(AllowHeaders::list(vec![
        header::AUTHORIZATION,
        header::CONTENT_TYPE,
        header::IF_MATCH,
        HeaderName::from_static("x-dev-user"),
    ]))
    .allow_credentials(false);

    let app = Router::new()
        .route("/health", get(|| async move { (StatusCode::OK, "ok") }))
        .route(
            "/ready",
            get({
                let state = state.clone();
                move || {
                    let state = state.clone();
                    async move {
                        // Check Postgres connectivity
                        let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
                            .fetch_one(&state.pg)
                            .await
                            .is_ok();
                        // Check JWKS keys are loaded
                        let jwks_ok = presentation::auth::jwks_has_keys();
                        if db_ok && jwks_ok {
                            (StatusCode::OK, "ok").into_response()
                        } else {
                            let mut msg = String::new();
                            if !db_ok { msg.push_str("db_down;") }
                            if !jwks_ok { msg.push_str("jwks_missing;") }
                            (StatusCode::SERVICE_UNAVAILABLE, msg).into_response()
                        }
                    }
                }
            }),
        )
        .merge(metrics_router)
        .merge(presentation::routes())
        .layer(prometheus_layer)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);

    let addr: std::net::SocketAddr = std::env::var("BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse::<std::net::SocketAddr>()
        .unwrap();
    tracing::info!(%addr, "listening");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
