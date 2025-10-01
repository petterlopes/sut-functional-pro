// ============================================================================
// SECURITY HEADERS MIDDLEWARE
// ============================================================================
// Middleware para adicionar headers de segurança HTTP
// Implementa as melhores práticas de segurança web

use axum::{
    extract::Request,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;

/// Headers de segurança configuráveis
#[derive(Debug, Clone)]
pub struct SecurityHeaders {
    pub content_security_policy: String,
    pub x_frame_options: String,
    pub x_content_type_options: String,
    pub x_xss_protection: String,
    pub referrer_policy: String,
    pub strict_transport_security: String,
    pub permissions_policy: String,
    pub cross_origin_embedder_policy: String,
    pub cross_origin_opener_policy: String,
    pub cross_origin_resource_policy: String,
}

impl Default for SecurityHeaders {
    fn default() -> Self {
        Self {
            content_security_policy: "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self' https:; frame-ancestors 'none'; base-uri 'self'; form-action 'self'".to_string(),
            x_frame_options: "DENY".to_string(),
            x_content_type_options: "nosniff".to_string(),
            x_xss_protection: "1; mode=block".to_string(),
            referrer_policy: "strict-origin-when-cross-origin".to_string(),
            strict_transport_security: "max-age=31536000; includeSubDomains; preload".to_string(),
            permissions_policy: "camera=(), microphone=(), geolocation=(), payment=(), usb=(), magnetometer=(), gyroscope=(), accelerometer=()".to_string(),
            cross_origin_embedder_policy: "require-corp".to_string(),
            cross_origin_opener_policy: "same-origin".to_string(),
            cross_origin_resource_policy: "same-origin".to_string(),
        }
    }
}

impl SecurityHeaders {
    /// Cria configuração de segurança para desenvolvimento
    pub fn development() -> Self {
        Self {
            content_security_policy: "default-src 'self' 'unsafe-inline' 'unsafe-eval' data: blob:; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: https:; font-src 'self' data:; connect-src 'self' http: https: ws: wss:; frame-ancestors 'none'; base-uri 'self'; form-action 'self'".to_string(),
            x_frame_options: "SAMEORIGIN".to_string(),
            strict_transport_security: "max-age=86400".to_string(),
            ..Default::default()
        }
    }

    /// Cria configuração de segurança para produção
    pub fn production() -> Self {
        Self::default()
    }

    /// Adiciona todos os headers de segurança à resposta
    pub fn add_headers(&self, headers: &mut HeaderMap) {
        let security_headers = [
            ("content-security-policy", &self.content_security_policy),
            ("x-frame-options", &self.x_frame_options),
            ("x-content-type-options", &self.x_content_type_options),
            ("x-xss-protection", &self.x_xss_protection),
            ("referrer-policy", &self.referrer_policy),
            ("strict-transport-security", &self.strict_transport_security),
            ("permissions-policy", &self.permissions_policy),
            (
                "cross-origin-embedder-policy",
                &self.cross_origin_embedder_policy,
            ),
            (
                "cross-origin-opener-policy",
                &self.cross_origin_opener_policy,
            ),
            (
                "cross-origin-resource-policy",
                &self.cross_origin_resource_policy,
            ),
        ];

        for (name, value) in security_headers {
            if let (Ok(name), Ok(value)) = (
                HeaderName::try_from(name),
                HeaderValue::try_from(value.as_str()),
            ) {
                headers.insert(name, value);
            }
        }
    }
}

/// Middleware para adicionar headers de segurança
pub async fn security_headers_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Determinar ambiente baseado na variável de ambiente
    let security_headers = if std::env::var("RUST_ENV").unwrap_or_default() == "production" {
        SecurityHeaders::production()
    } else {
        SecurityHeaders::development()
    };

    // Adicionar configuração de segurança ao request
    request.extensions_mut().insert(security_headers.clone());

    // Processar request
    let mut response = next.run(request).await;

    // Adicionar headers de segurança à resposta
    let headers = response.headers_mut();
    security_headers.add_headers(headers);

    Ok(response)
}

/// Headers de segurança específicos para APIs
pub fn add_api_security_headers(headers: &mut HeaderMap) {
    let api_headers = [
        ("x-api-version", "1.0"),
        ("x-response-time", "0ms"),
        ("x-request-id", "unknown"),
        ("cache-control", "no-cache, no-store, must-revalidate"),
        ("pragma", "no-cache"),
        ("expires", "0"),
    ];

    for (name, value) in api_headers {
        if let (Ok(name), Ok(value)) = (HeaderName::try_from(name), HeaderValue::try_from(value)) {
            headers.insert(name, value);
        }
    }
}

/// Headers de segurança para CORS
pub fn add_cors_security_headers(headers: &mut HeaderMap) {
    let cors_headers = [
        ("access-control-allow-origin", "http://localhost:5173"),
        (
            "access-control-allow-methods",
            "GET, POST, PUT, PATCH, DELETE, OPTIONS",
        ),
        (
            "access-control-allow-headers",
            "Content-Type, Authorization, X-Requested-With",
        ),
        ("access-control-allow-credentials", "true"),
        ("access-control-max-age", "86400"),
    ];

    for (name, value) in cors_headers {
        if let (Ok(name), Ok(value)) = (HeaderName::try_from(name), HeaderValue::try_from(value)) {
            headers.insert(name, value);
        }
    }
}

/// Validação de headers de segurança
pub fn validate_security_headers(headers: &HeaderMap) -> Result<(), String> {
    let required_headers = [
        "content-security-policy",
        "x-frame-options",
        "x-content-type-options",
        "strict-transport-security",
    ];

    for header_name in required_headers {
        if !headers.contains_key(header_name) {
            return Err(format!("Missing required security header: {}", header_name));
        }
    }

    Ok(())
}

/// Configuração de rate limiting
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub window_size_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 100,
            burst_size: 10,
            window_size_seconds: 60,
        }
    }
}

/// Headers de rate limiting
pub fn add_rate_limit_headers(
    headers: &mut HeaderMap,
    config: &RateLimitConfig,
    remaining: u32,
    reset_time: u64,
) {
    let rate_limit_headers = [
        ("x-ratelimit-limit", config.requests_per_minute.to_string()),
        ("x-ratelimit-remaining", remaining.to_string()),
        ("x-ratelimit-reset", reset_time.to_string()),
        ("x-ratelimit-burst", config.burst_size.to_string()),
    ];

    for (name, value) in rate_limit_headers {
        if let (Ok(name), Ok(value)) = (
            HeaderName::try_from(name),
            HeaderValue::try_from(value.as_str()),
        ) {
            headers.insert(name, value);
        }
    }
}
