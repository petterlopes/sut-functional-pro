// ============================================================================
// IMPORTS E DEPENDÊNCIAS
// ============================================================================
// Importação de bibliotecas para gerenciamento de concorrência e HTTP

use std::sync::Arc; // Arc (Atomically Reference Counted) para compartilhamento seguro entre threads

use axum::{
    extract::State, // Para extrair estado compartilhado das requisições
    http::{header, HeaderMap, HeaderName, HeaderValue, Method, StatusCode}, // Tipos HTTP
    response::IntoResponse, // Trait para converter tipos em respostas HTTP
    routing::get,   // Macro para definir rotas GET
    Router,         // Estrutura principal do roteador Axum
};
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer}, // Middleware CORS
    trace::TraceLayer,                                          // Middleware de tracing/logging
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt}; // Sistema de logging estruturado

// ============================================================================
// DECLARAÇÃO DE MÓDULOS - CLEAN ARCHITECTURE
// ============================================================================
// Organização dos módulos seguindo os princípios da Clean Architecture

mod application; // Camada de aplicação (use cases, DTOs)
mod domain; // Camada de domínio (entidades, repositórios, value objects)
mod infra; // Infraestrutura básica (não confundir com infrastructure)
mod infrastructure; // Camada de infraestrutura (implementações concretas)
mod presentation; // Camada de apresentação (controllers, middleware, rotas)
mod shared; // Utilitários e código compartilhado

// ============================================================================
// APP STATE - DEPENDENCY INJECTION CONTAINER
// ============================================================================
// Estrutura que contém todas as dependências compartilhadas da aplicação
// Implementa o padrão Dependency Injection para facilitar testes e manutenção

#[derive(Clone)] // Permite clonagem para compartilhamento entre handlers
pub struct AppState {
    // ===== INFRAESTRUTURA BÁSICA =====
    pub pg: sqlx::Pool<sqlx::Postgres>, // Pool de conexões PostgreSQL para reutilização eficiente
    pub vault: Option<infra::vault::VaultClient>, // Cliente Vault opcional para gerenciamento de secrets
    pub metrics_token: Option<String>, // Token opcional para autenticação de métricas Prometheus
    pub webhook_token: Option<String>, // Token compartilhado para autenticação de webhooks

    // ===== REPOSITÓRIOS - CLEAN ARCHITECTURE =====
    // Implementações concretas dos repositórios injetadas como dependências
    // Arc<T> permite compartilhamento thread-safe sem duplicação de dados
    pub contact_repository: Arc<infrastructure::repositories::PostgresContactRepository>,
    pub org_unit_repository: Arc<infrastructure::repositories::PostgresOrgUnitRepository>,
    pub department_repository: Arc<infrastructure::repositories::PostgresDepartmentRepository>,
    pub user_repository: Arc<infrastructure::repositories::PostgresUserRepository>,
}

// ============================================================================
// FUNÇÃO PRINCIPAL - PONTO DE ENTRADA DA APLICAÇÃO
// ============================================================================
// Função assíncrona que inicializa e configura toda a aplicação

#[tokio::main] // Macro que configura o runtime assíncrono Tokio
async fn main() -> anyhow::Result<()> {
    // Carrega variáveis de ambiente do arquivo .env (opcional)
    dotenvy::dotenv().ok();

    // ============================================================================
    // CONFIGURAÇÃO DE MÉTRICAS PROMETHEUS
    // ============================================================================
    // Configuração do sistema de métricas para observabilidade e monitoramento

    let (prometheus_layer, metric_handle) = axum_prometheus::PrometheusMetricLayer::pair();
    let metrics_router = Router::new().route(
        "/metrics", // Endpoint padrão do Prometheus para coleta de métricas
        get({
            let handle = metric_handle.clone();
            move |State(app): State<Arc<AppState>>, headers: HeaderMap| {
                let handle = handle.clone();
                async move {
                    // Verificação de autenticação para endpoint de métricas
                    // Protege métricas sensíveis com token de autenticação
                    if let Some(expected) = app.metrics_token.as_deref() {
                        let provided = headers.get("X-Metrics-Token").and_then(|v| v.to_str().ok());
                        if provided != Some(expected) {
                            return StatusCode::UNAUTHORIZED.into_response();
                        }
                    }
                    handle.render().into_response() // Retorna métricas em formato Prometheus
                }
            }
        }),
    );

    // ============================================================================
    // CONFIGURAÇÃO DE LOGGING ESTRUTURADO
    // ============================================================================
    // Sistema de logging que produz logs em formato JSON para facilitar análise

    let fmt_layer = tracing_subscriber::fmt::layer().json(); // Formato JSON para logs estruturados
    let filter = tracing_subscriber::EnvFilter::from_default_env(); // Filtros baseados em variáveis de ambiente
    tracing_subscriber::registry()
        .with(filter) // Aplica filtros de nível de log
        .with(fmt_layer) // Aplica formatação JSON
        .init(); // Inicializa o sistema de logging

    // ============================================================================
    // CONFIGURAÇÃO DE VARIÁVEIS DE AMBIENTE
    // ============================================================================
    // Carregamento e configuração de variáveis de ambiente com valores padrão

    // String de conexão PostgreSQL com fallback para desenvolvimento local
    let dsn =
        std::env::var("PG_DSN").unwrap_or_else(|_| "postgres://sut:sut@localhost:5432/sut".into());

    // URL do JWKS (JSON Web Key Set) do Keycloak para validação de JWT
    let jwks_uri = std::env::var("KEYCLOAK_JWKS").unwrap_or_else(|_| {
        "http://localhost:8081/realms/sut/protocol/openid-connect/certs".into()
    });

    // Issuer do JWT (opcional) - quem emitiu o token
    let issuer = std::env::var("KEYCLOAK_ISSUER").ok();

    // Audiences permitidos - aplicações que podem usar o token
    let audiences = std::env::var("KEYCLOAK_AUDIENCE")
        .map(|v| {
            v.split(',') // Suporte a múltiplas audiences separadas por vírgula
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|_| vec!["sut-frontend".into()]); // Padrão para frontend

    // Tolerância de tempo para validação de JWT (em segundos)
    let jwt_leeway = std::env::var("JWT_LEEWAY_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(60); // 60 segundos de tolerância por padrão

    // Identifica??o do ambiente para aplicar pol?ticas de seguran?a diferenciadas
    let is_production_env = matches!(
        std::env::var("RUST_ENV"),
        Ok(ref v) if v.eq_ignore_ascii_case("production")
    );

    // Token para autentica??o de m?tricas (opcional em desenvolvimento, obrigat?rio em produ??o)
    let metrics_token = std::env::var("METRICS_TOKEN").ok();
    if is_production_env && metrics_token.is_none() {
        return Err(anyhow::anyhow!(
            "METRICS_TOKEN must be configured when RUST_ENV=production"
        ));
    }

    // Token compartilhado para autenticar webhooks externos
    let webhook_token = std::env::var("WEBHOOK_SHARED_SECRET").ok();
    if is_production_env && webhook_token.is_none() {
        return Err(anyhow::anyhow!(
            "WEBHOOK_SHARED_SECRET must be configured when RUST_ENV=production"
        ));
    }

    // ============================================================================
    // INICIALIZAÇÃO DO BANCO DE DADOS
    // ============================================================================
    // Criação do pool de conexões e execução de migrações

    let pg = infra::pg::pool(&dsn).await?; // Cria pool de conexões PostgreSQL
    infra::pg::migrate(&pg).await?; // Executa migrações do banco de dados

    // ============================================================================
    // CONFIGURAÇÃO DE AUTENTICAÇÃO JWT
    // ============================================================================
    // Inicialização do sistema de autenticação com Keycloak

    presentation::auth::init(presentation::auth::AuthConfig {
        jwks_uri: jwks_uri.clone(), // URL para buscar chaves públicas
        issuer,                     // Quem emitiu o token (opcional)
        audiences,                  // Aplicações autorizadas a usar o token
        leeway_secs: jwt_leeway,    // Tolerância de tempo para validação
    })
    .await?;

    // ============================================================================
    // CONFIGURAÇÃO DE VAULT (OPCIONAL)
    // ============================================================================
    // Cliente para HashiCorp Vault para gerenciamento de secrets

    let vault = match infra::vault::VaultClient::default() {
        Ok(client) => {
            // Verificar se o Vault está disponível
            match client.health_check().await {
                Ok(true) => {
                    tracing::info!("Vault conectado e funcionando");
                    Some(client)
                }
                Ok(false) => {
                    tracing::warn!("Vault não está saudável, continuando sem ele");
                    None
                }
                Err(e) => {
                    tracing::warn!("Erro ao conectar com Vault: {}, continuando sem ele", e);
                    None
                }
            }
        }
        Err(e) => {
            tracing::warn!(
                "Falha ao inicializar cliente Vault: {}, continuando sem ele",
                e
            );
            None
        }
    };

    // Token para autenticação de métricas (opcional)
    let metrics_token = std::env::var("METRICS_TOKEN").ok();

    // ============================================================================
    // INICIALIZAÇÃO DOS REPOSITÓRIOS - CLEAN ARCHITECTURE
    // ============================================================================
    // Criação das implementações concretas dos repositórios com injeção de dependência

    let contact_repository =
        Arc::new(infrastructure::repositories::PostgresContactRepository::new(pg.clone()));
    let org_unit_repository =
        Arc::new(infrastructure::repositories::PostgresOrgUnitRepository::new(pg.clone()));
    let department_repository =
        Arc::new(infrastructure::repositories::PostgresDepartmentRepository::new(pg.clone()));
    let user_repository = Arc::new(infrastructure::repositories::PostgresUserRepository::new(
        pg.clone(),
    ));

    // ============================================================================
    // CRIAÇÃO DO ESTADO COMPARTILHADO
    // ============================================================================
    // Montagem do AppState com todas as dependências para injeção nos handlers

    let state = Arc::new(AppState {
        pg,                    // Pool de conexões compartilhado
        vault,                 // Cliente Vault (opcional)
        metrics_token,         // Token de métricas (opcional)
        webhook_token,         // Token de webhooks (opcional)
        contact_repository,    // Repositório de contatos
        org_unit_repository,   // Repositório de unidades organizacionais
        department_repository, // Repositório de departamentos
        user_repository,       // Repositório de usuários
    });

    // ============================================================================
    // TAREFA EM BACKGROUND - ATUALIZAÇÃO PERIÓDICA DE JWKS
    // ============================================================================
    // Tarefa assíncrona que atualiza periodicamente as chaves JWT do Keycloak
    // para manter a validação de tokens funcionando mesmo com rotação de chaves

    let _jwks_uri_clone = jwks_uri.clone();
    tokio::spawn(async move {
        // Delay inicial antes da primeira atualização para evitar corridas na inicialização
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        loop {
            // Tenta atualizar as chaves JWKS
            if let Err(e) = presentation::auth::refresh_jwks().await {
                // Log do erro mas continua executando - não é crítico
                tracing::warn!(error = ?e, "periodic jwks refresh failed");
            }
            // Aguarda 60 segundos antes da próxima atualização
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    });

    // ============================================================================
    // CONFIGURAÇÃO CORS (CROSS-ORIGIN RESOURCE SHARING)
    // ============================================================================
    // Configuração de CORS para permitir requisições de diferentes origens

    let allowed_origins = match std::env::var("CORS_ALLOWED_ORIGINS") {
        Ok(val) if !val.trim().is_empty() => val,
        _ if !is_production_env => "http://localhost:5173".into(),
        _ => {
            return Err(anyhow::anyhow!(
                "CORS_ALLOWED_ORIGINS must be configured when RUST_ENV=production"
            ));
        }
    };
    let origin_values: Vec<HeaderValue> = allowed_origins
        .split(',') // Suporte a m?ltiplas origens separadas por v?rgula
        .map(|s| s.trim()) // Remove espa?os em branco
        .filter(|s| !s.is_empty()) // Remove strings vazias
        .filter_map(|origin| HeaderValue::from_str(origin).ok()) // Converte para HeaderValue
        .collect();

    // Configura??o do middleware CORS
    let cors = if origin_values.is_empty() {
        CorsLayer::new().allow_origin(AllowOrigin::exact(HeaderValue::from_static(
            "http://localhost:5173",
        )))
    } else {
        CorsLayer::new().allow_origin(AllowOrigin::list(origin_values))
    }
    .allow_methods(AllowMethods::list(vec![
        Method::GET,     // Leitura de dados
        Method::POST,    // Criação de recursos
        Method::PATCH,   // Atualização parcial
        Method::DELETE,  // Remoção de recursos
        Method::OPTIONS, // Preflight requests do CORS
    ]))
    .allow_headers(AllowHeaders::list(vec![
        header::AUTHORIZATION,                 // Header de autenticação JWT
        header::CONTENT_TYPE,                  // Tipo de conteúdo das requisições
        header::IF_MATCH,                      // Para controle de concorrência otimista
        HeaderName::from_static("x-dev-user"), // Header customizado para desenvolvimento
    ]))
    .allow_credentials(false); // Não permite cookies/credenciais cross-origin

    // ============================================================================
    // CONFIGURAÇÃO DO ROTEADOR PRINCIPAL
    // ============================================================================
    // Montagem do roteador Axum com todas as rotas e middlewares

    let app = Router::new()
        // ===== HEALTH CHECKS =====
        .route("/health", get(|| async move { (StatusCode::OK, "ok") })) // Health check simples
        .route(
            "/ready", // Readiness probe para Kubernetes/containers
            get({
                let state = state.clone();
                move || {
                    let state = state.clone();
                    async move {
                        // Verifica conectividade com PostgreSQL
                        let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
                            .fetch_one(&state.pg)
                            .await
                            .is_ok();
                        // Verifica se as chaves JWKS estão carregadas
                        let jwks_ok = presentation::auth::jwks_has_keys();

                        if db_ok && jwks_ok {
                            (StatusCode::OK, "ok").into_response()
                        } else {
                            // Retorna erro detalhado indicando qual dependência falhou
                            let mut msg = String::new();
                            if !db_ok {
                                msg.push_str("db_down;")
                            }
                            if !jwks_ok {
                                msg.push_str("jwks_missing;")
                            }
                            (StatusCode::SERVICE_UNAVAILABLE, msg).into_response()
                        }
                    }
                }
            }),
        )
        // ===== MERGE DE ROTEADORES =====
        .merge(metrics_router) // Adiciona rotas de métricas Prometheus
        .merge(presentation::routes()) // Adiciona rotas da aplicação (Clean Architecture)
        // ===== MIDDLEWARES (APLICADOS EM ORDEM REVERSA) =====
        .layer(prometheus_layer) // Coleta métricas HTTP
        .layer(TraceLayer::new_for_http()) // Logging de requisições HTTP
        .layer(cors) // Middleware CORS
        // ===== INJEÇÃO DE ESTADO =====
        .with_state(state); // Injeta o AppState em todos os handlers

    // ============================================================================
    // INICIALIZAÇÃO DO SERVIDOR HTTP
    // ============================================================================
    // Configuração do endereço de bind e inicialização do servidor

    let addr: std::net::SocketAddr = std::env::var("BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".into()) // Padrão: todas as interfaces na porta 8080
        .parse::<std::net::SocketAddr>()
        .unwrap();

    // Log do endereço onde o servidor está escutando
    tracing::info!(%addr, "listening");

    // Inicia o servidor HTTP com o roteador configurado
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(()) // Retorna sucesso (nunca alcançado em execução normal)
}
