// ============================================================================
// MIDDLEWARE SYSTEM - SISTEMA DE MIDDLEWARE CENTRALIZADO
// ============================================================================
// Sistema unificado para gerenciamento de middleware
// Elimina redundância e centraliza toda a lógica de middleware

use axum::{
    extract::Request,
    http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info_span, Span};

// ============================================================================
// TRAIT BASE PARA MIDDLEWARE
// ============================================================================

#[async_trait::async_trait]
pub trait Middleware: Send + Sync {
    /// Nome do middleware
    fn name(&self) -> &str;

    /// Executa o middleware
    async fn execute(&self, request: Request, next: Next) -> Result<Response, StatusCode>;

    /// Prioridade do middleware (menor = maior prioridade)
    fn priority(&self) -> i32 {
        100
    }

    /// Se o middleware deve ser executado para esta rota
    fn should_execute(&self, method: &Method, path: &str) -> bool {
        true
    }
}

// ============================================================================
// MIDDLEWARE DE LOGGING
// ============================================================================

pub struct LoggingMiddleware {
    pub include_body: bool,
    pub include_headers: bool,
}

impl LoggingMiddleware {
    pub fn new(include_body: bool, include_headers: bool) -> Self {
        Self {
            include_body,
            include_headers,
        }
    }
}

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    fn name(&self) -> &str {
        "logging"
    }

    fn priority(&self) -> i32 {
        10
    }

    async fn execute(&self, request: Request, next: Next) -> Result<Response, StatusCode> {
        let method = request.method().clone();
        let path = request.uri().path().to_string();
        let span = info_span!("http.request", "http.method"=%method, "http.route"=%path);
        let _enter = span.enter();

        // Log da requisição
        tracing::info!(
            method = %method,
            path = %path,
            "Processing request"
        );

        // Executa o próximo middleware/handler
        let response = next.run(request).await;

        // Log da resposta
        tracing::info!(
            method = %method,
            path = %path,
            status = %response.status(),
            "Request completed"
        );

        Ok(response)
    }
}

// ============================================================================
// MIDDLEWARE DE AUTENTICAÇÃO
// ============================================================================

pub struct AuthMiddleware {
    pub required_roles: Vec<String>,
    pub skip_paths: Vec<String>,
}

impl AuthMiddleware {
    pub fn new(required_roles: Vec<String>, skip_paths: Vec<String>) -> Self {
        Self {
            required_roles,
            skip_paths,
        }
    }
}

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    fn name(&self) -> &str {
        "auth"
    }

    fn priority(&self) -> i32 {
        20
    }

    fn should_execute(&self, _method: &Method, path: &str) -> bool {
        !self
            .skip_paths
            .iter()
            .any(|skip_path| path.starts_with(skip_path))
    }

    async fn execute(&self, request: Request, next: Next) -> Result<Response, StatusCode> {
        // Verificar token JWT
        let auth_header = request.headers().get("Authorization");

        match auth_header {
            Some(header) => {
                if let Ok(token) = header.to_str() {
                    if token.starts_with("Bearer ") {
                        // Validar token (implementação simplificada)
                        tracing::debug!("Token found, proceeding");
                        return Ok(next.run(request).await);
                    }
                }
            }
            None => {
                tracing::warn!("No authorization header found");
                return Err(StatusCode::UNAUTHORIZED);
            }
        }

        Err(StatusCode::UNAUTHORIZED)
    }
}

// ============================================================================
// MIDDLEWARE DE RATE LIMITING
// ============================================================================

pub struct RateLimitMiddleware {
    pub requests_per_minute: u32,
    pub requests: Arc<tokio::sync::Mutex<HashMap<String, (u32, std::time::Instant)>>>,
}

impl RateLimitMiddleware {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
            requests: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RateLimitMiddleware {
    fn name(&self) -> &str {
        "rate_limit"
    }

    fn priority(&self) -> i32 {
        30
    }

    async fn execute(&self, request: Request, next: Next) -> Result<Response, StatusCode> {
        let client_ip = request
            .headers()
            .get("x-forwarded-for")
            .or_else(|| request.headers().get("x-real-ip"))
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown");

        let mut requests = self.requests.lock().await;
        let now = std::time::Instant::now();

        // Limpar entradas antigas
        requests.retain(|_, (_, time)| now.duration_since(*time).as_secs() < 60);

        let (count, _) = requests.entry(client_ip.to_string()).or_insert((0, now));

        if *count >= self.requests_per_minute {
            tracing::warn!(client_ip = %client_ip, "Rate limit exceeded");
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

        *count += 1;

        Ok(next.run(request).await)
    }
}

// ============================================================================
// MIDDLEWARE DE CORS
// ============================================================================

pub struct CorsMiddleware {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<Method>,
    pub allowed_headers: Vec<HeaderName>,
}

impl CorsMiddleware {
    pub fn new(
        allowed_origins: Vec<String>,
        allowed_methods: Vec<Method>,
        allowed_headers: Vec<HeaderName>,
    ) -> Self {
        Self {
            allowed_origins,
            allowed_methods,
            allowed_headers,
        }
    }
}

#[async_trait::async_trait]
impl Middleware for CorsMiddleware {
    fn name(&self) -> &str {
        "cors"
    }

    fn priority(&self) -> i32 {
        5
    }

    async fn execute(&self, request: Request, next: Next) -> Result<Response, StatusCode> {
        let origin = request.headers().get("origin").cloned();
        let method = request.method().clone();

        // Verificar origem
        if let Some(origin_header) = &origin {
            if let Ok(origin_str) = origin_header.to_str() {
                if !self.allowed_origins.contains(&origin_str.to_string())
                    && !self.allowed_origins.contains(&"*".to_string())
                {
                    return Err(StatusCode::FORBIDDEN);
                }
            }
        }

        // Verificar método
        if !self.allowed_methods.contains(&method) {
            return Err(StatusCode::METHOD_NOT_ALLOWED);
        }

        let mut response = next.run(request).await;

        // Adicionar headers CORS
        let headers = response.headers_mut();

        if let Some(origin_header) = origin {
            headers.insert("Access-Control-Allow-Origin", origin_header);
        }

        headers.insert(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static("GET, POST, PUT, PATCH, DELETE, OPTIONS"),
        );

        headers.insert(
            "Access-Control-Allow-Headers",
            HeaderValue::from_static("Content-Type, Authorization"),
        );

        Ok(response)
    }
}

// ============================================================================
// SISTEMA DE MIDDLEWARE
// ============================================================================

pub struct MiddlewareSystem {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl MiddlewareSystem {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Box<dyn Middleware>) {
        self.middlewares.push(middleware);
        // Ordenar por prioridade
        self.middlewares.sort_by_key(|m| m.priority());
    }

    pub async fn execute(&self, request: Request, next: Next) -> Result<Response, StatusCode> {
        // Por simplicidade, executar apenas o primeiro middleware que se aplica
        // Em uma implementação mais robusta, seria necessário implementar uma cadeia completa
        for middleware in &self.middlewares {
            let method = request.method().clone();
            let path = request.uri().path().to_string();

            if middleware.should_execute(&method, &path) {
                return middleware.execute(request, next).await;
            }
        }

        Ok(next.run(request).await)
    }
}

impl Default for MiddlewareSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MACROS PARA MIDDLEWARE
// ============================================================================

/// Macro para criar middleware simples
#[macro_export]
macro_rules! create_middleware {
    ($name:ident, $priority:expr, $execute:expr) => {
        pub struct $name;

        #[async_trait::async_trait]
        impl $crate::shared::middleware_system::Middleware for $name {
            fn name(&self) -> &str {
                stringify!($name)
            }

            fn priority(&self) -> i32 {
                $priority
            }

            async fn execute(
                &self,
                request: &mut axum::extract::Request,
                next: axum::middleware::Next,
            ) -> Result<axum::response::Response, axum::http::StatusCode> {
                $execute(request, next).await
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request, response::Response};

    #[tokio::test]
    async fn test_middleware_system() {
        let mut system = MiddlewareSystem::new();

        let logging = LoggingMiddleware::new(false, false);
        system.add_middleware(Box::new(logging));

        // Teste básico - sistema deve compilar e executar
        assert_eq!(system.middlewares.len(), 1);
    }

    #[test]
    fn test_middleware_priority() {
        let mut system = MiddlewareSystem::new();

        let auth = AuthMiddleware::new(vec![], vec![]);
        let logging = LoggingMiddleware::new(false, false);

        system.add_middleware(Box::new(auth));
        system.add_middleware(Box::new(logging));

        // Logging deve ter prioridade menor (maior prioridade de execução)
        assert_eq!(system.middlewares[0].priority(), 10);
        assert_eq!(system.middlewares[1].priority(), 20);
    }
}
