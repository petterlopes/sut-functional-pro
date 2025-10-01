use anyhow::Context;
use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use once_cell::sync::OnceCell;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

/// Claims JWT estruturadas
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub iss: String,         // Issuer
    pub sub: String,         // Subject (user ID)
    pub aud: Vec<String>,    // Audience
    pub exp: u64,            // Expiration time
    pub iat: u64,            // Issued at
    pub jti: Option<String>, // JWT ID
    pub typ: Option<String>, // Token type
    pub azp: Option<String>, // Authorized party
    pub session_state: Option<String>,
    pub realm_access: Option<RealmAccess>,
    pub resource_access: Option<serde_json::Value>,
    pub scope: Option<String>,
    pub sid: Option<String>, // Session ID
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub preferred_username: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

/// Configuração de segurança JWT
#[derive(Debug, Clone)]
pub struct JwtSecurityConfig {
    pub allowed_algorithms: Vec<Algorithm>,
    pub max_token_age: u64,
    pub clock_skew: u64,
    pub require_audience: bool,
    pub require_issuer: bool,
    pub validate_exp: bool,
    pub validate_nbf: bool,
}

impl Default for JwtSecurityConfig {
    fn default() -> Self {
        Self {
            allowed_algorithms: vec![Algorithm::RS256, Algorithm::RS384, Algorithm::RS512],
            max_token_age: 3600, // 1 hora
            clock_skew: 60,      // 1 minuto
            require_audience: true,
            require_issuer: true,
            validate_exp: true,
            validate_nbf: true,
        }
    }
}

/// Cache de tokens revogados
#[derive(Debug, Clone)]
pub struct TokenBlacklist {
    tokens: std::sync::Arc<parking_lot::RwLock<HashSet<String>>>,
}

impl TokenBlacklist {
    pub fn new() -> Self {
        Self {
            tokens: std::sync::Arc::new(parking_lot::RwLock::new(HashSet::new())),
        }
    }

    pub fn add(&self, jti: String) {
        self.tokens.write().insert(jti);
    }

    pub fn contains(&self, jti: &str) -> bool {
        self.tokens.read().contains(jti)
    }

    pub fn remove(&self, jti: &str) {
        self.tokens.write().remove(jti);
    }

    pub fn clear_expired(&self, current_time: u64) {
        // Implementar limpeza de tokens expirados se necessário
        // Por simplicidade, mantemos todos os tokens por enquanto
    }
}

#[derive(Clone)]
pub struct Jwks {
    pub uri: String,
    http: Client,
    keys: std::sync::Arc<parking_lot::RwLock<serde_json::Value>>,
}
impl Jwks {
    pub fn new(uri: String) -> Self {
        Self {
            uri,
            http: Client::new(),
            keys: std::sync::Arc::new(parking_lot::RwLock::new(serde_json::json!({}))),
        }
    }
    pub async fn refresh(&self) -> anyhow::Result<()> {
        // Retry the JWKS fetch a few times with exponential backoff to handle
        // short startup races where Keycloak may not yet be ready.
        let mut attempt: u32 = 0;
        let max_attempts: u32 = 5;
        let mut wait = Duration::from_millis(500);
        loop {
            attempt += 1;
            match self.http.get(&self.uri).send().await {
                Ok(resp) => {
                    let v: serde_json::Value = resp.json().await.context("parsing JWKS")?;
                    *self.keys.write() = v;
                    return Ok(());
                }
                Err(e) => {
                    if attempt >= max_attempts {
                        return Err(e).context(format!("fetching JWKS after {} attempts", attempt));
                    }
                    eprintln!(
                        "[auth] JWKS fetch attempt {} failed: {}. retrying in {:?}",
                        attempt, e, wait
                    );
                    sleep(wait).await;
                    wait = wait * 2;
                    continue;
                }
            }
        }
    }
    pub fn decoding_key(&self, kid: &str) -> Option<DecodingKey> {
        let keys = self.keys.read();
        for k in keys["keys"].as_array().unwrap_or(&vec![]) {
            if k["kid"].as_str() == Some(kid) && k["kty"] == "RSA" {
                if let (Some(n), Some(e)) = (k["n"].as_str(), k["e"].as_str()) {
                    return DecodingKey::from_rsa_components(n, e).ok();
                }
            }
        }
        None
    }
}

/// Return true if we have at least one key loaded in the JWKS store.
pub fn jwks_has_keys() -> bool {
    if let Some(auth_state) = AUTH.get() {
        let keys = auth_state.jwks.keys.read();
        return keys["keys"]
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false);
    }
    false
}

/// Refresh the JWKS from the configured URI. Returns an error if auth not initialized
/// or the refresh fails.
pub async fn refresh_jwks() -> anyhow::Result<()> {
    if let Some(auth_state) = AUTH.get() {
        auth_state.jwks.refresh().await
    } else {
        Err(anyhow::anyhow!("auth not initialized"))
    }
}

pub struct AuthConfig {
    pub jwks_uri: String,
    pub issuer: Option<String>,
    pub audiences: Vec<String>,
    pub leeway_secs: u64,
}

struct AuthState {
    jwks: Jwks,
    issuer: Option<String>,
    audiences: Vec<String>,
    leeway: u64,
}

static AUTH: OnceCell<AuthState> = OnceCell::new();

pub async fn init(config: AuthConfig) -> anyhow::Result<()> {
    let AuthConfig {
        jwks_uri,
        issuer,
        audiences,
        leeway_secs,
    } = config;
    let jwks = Jwks::new(jwks_uri);
    // Try an initial refresh but do not fail startup if Keycloak is still
    // warming up. We log a warning and continue — the main background
    // refresher will attempt to update keys periodically.
    if let Err(e) = jwks.refresh().await {
        tracing::warn!(error = ?e, "initial JWKS fetch failed; continuing without keys — will refresh in background");
    }
    let state = AuthState {
        jwks,
        issuer,
        audiences,
        leeway: leeway_secs,
    };
    AUTH.set(state)
        .map_err(|_| anyhow::anyhow!("auth already initialised"))?;
    Ok(())
}

pub async fn jwt_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    tracing::info!("Entering jwt_middleware");

    // Optional development bypass. Only active when DEV_AUTH_BYPASS=1 AND we are not in production.
    let dev_bypass_enabled = matches!(std::env::var("DEV_AUTH_BYPASS"), Ok(ref v) if v == "1");
    let is_production =
        matches!(std::env::var("RUST_ENV"), Ok(ref v) if v.eq_ignore_ascii_case("production"));

    if dev_bypass_enabled && is_production {
        tracing::warn!("DEV_AUTH_BYPASS=1 ignored because RUST_ENV=production");
    }

    if dev_bypass_enabled && !is_production {
        if let Some(dev_user) = req
            .headers()
            .get("x-dev-user")
            .and_then(|v| v.to_str().ok())
        {
            let roles: Vec<String> = req
                .headers()
                .get("x-dev-roles")
                .and_then(|v| v.to_str().ok())
                .map(|raw| {
                    raw.split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(str::to_string)
                        .collect()
                })
                .filter(|roles: &Vec<String>| !roles.is_empty())
                .unwrap_or_else(|| vec!["directory.read".to_string()]);

            tracing::info!(
                "DEV_AUTH_BYPASS active for synthetic user {} with roles {:?}",
                dev_user,
                roles
            );
            let scope = roles
                .iter()
                .map(|role| role.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            let claims = serde_json::json!({
                "sub": dev_user,
                "realm_access": { "roles": roles },
                "scope": scope,
            });
            req.extensions_mut().insert(claims);
            return Ok(next.run(req).await);
        } else {
            tracing::warn!("DEV_AUTH_BYPASS active but X-Dev-User header missing or invalid");
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = &auth_header[7..];
    let header = decode_header(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let kid = header.kid.ok_or(StatusCode::UNAUTHORIZED)?;
    let auth_state = AUTH.get().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    if auth_state.jwks.decoding_key(&kid).is_none() {
        auth_state
            .jwks
            .refresh()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
    }
    let Some(decoding_key) = auth_state.jwks.decoding_key(&kid) else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation.leeway = auth_state.leeway;
    validation.required_spec_claims.extend([
        "exp".to_string(),
        "iat".to_string(),
        "nbf".to_string(),
    ]);
    if let Some(issuer) = &auth_state.issuer {
        let mut issuers = HashSet::new();
        issuers.insert(issuer.clone());
        validation.iss = Some(issuers);
    }
    // Do NOT set audience in the library validator; we'll verify aud/azp manually below

    let data = decode::<serde_json::Value>(token, &decoding_key, &validation)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    // Manual audience check: accept if aud contains any configured audience OR if azp equals one
    if !auth_state.audiences.is_empty() {
        let mut aud_ok = false;
        if let Some(aud_val) = data.claims.get("aud") {
            match aud_val {
                serde_json::Value::String(aud_str) => {
                    if auth_state.audiences.iter().any(|a| a == aud_str) {
                        aud_ok = true;
                    }
                }
                serde_json::Value::Array(arr) => {
                    let set: std::collections::HashSet<&str> =
                        arr.iter().filter_map(|v| v.as_str()).collect();
                    if auth_state
                        .audiences
                        .iter()
                        .any(|a| set.contains(a.as_str()))
                    {
                        aud_ok = true;
                    }
                }
                _ => {}
            }
        }
        if !aud_ok {
            if let Some(azp) = data.claims.get("azp").and_then(|v| v.as_str()) {
                if auth_state.audiences.iter().any(|a| a == azp) {
                    aud_ok = true;
                }
            }
        }
        if !aud_ok {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    if let Some(exp) = data.claims.get("exp").and_then(|x| x.as_i64()) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .as_secs() as i64;
        if now - exp > auth_state.leeway as i64 {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    req.extensions_mut().insert(data.claims);
    Ok(next.run(req).await)
}

/// Middleware para verificar roles específicas
pub async fn require_role_middleware(
    required_roles: Vec<String>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extrair claims do request
    let claims = request
        .extensions()
        .get::<serde_json::Value>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extrair roles do realm_access
    let user_roles = claims
        .get("realm_access")
        .and_then(|ra| ra.get("roles"))
        .and_then(|roles| roles.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    // Verificar se o usuário tem pelo menos uma das roles necessárias
    let has_required_role = required_roles
        .iter()
        .any(|required_role| user_roles.contains(required_role));

    if !has_required_role {
        let user_id = claims
            .get("sub")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        tracing::warn!(
            user_id = %user_id,
            user_roles = ?user_roles,
            required_roles = ?required_roles,
            "Access denied: insufficient permissions"
        );
        return Err(StatusCode::FORBIDDEN);
    }

    let user_id = claims
        .get("sub")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    tracing::info!(
        user_id = %user_id,
        roles = ?user_roles,
        "Role-based access granted"
    );

    Ok(next.run(request).await)
}

/// Middleware para verificar se o usuário é admin
pub async fn require_admin_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    require_role_middleware(vec!["admin".to_string()], request, next).await
}

/// Middleware para verificar se o usuário pode ler dados
pub async fn require_read_permission_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    require_role_middleware(
        vec!["directory.read".to_string(), "admin".to_string()],
        request,
        next,
    )
    .await
}

/// Middleware para verificar se o usuário pode escrever dados
pub async fn require_write_permission_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    require_role_middleware(
        vec!["directory.write".to_string(), "admin".to_string()],
        request,
        next,
    )
    .await
}

/// Middleware para verificar se o usuário pode acessar dados PII
pub async fn require_pii_permission_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    require_role_middleware(
        vec!["directory.pii.read".to_string(), "admin".to_string()],
        request,
        next,
    )
    .await
}

/// Middleware para verificar se o usuário pode fazer merge de dados
pub async fn require_merge_permission_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    require_role_middleware(
        vec!["directory.merge".to_string(), "admin".to_string()],
        request,
        next,
    )
    .await
}

/// Função auxiliar para extrair user ID das claims
pub fn extract_user_id(claims: &serde_json::Value) -> Option<String> {
    claims
        .get("sub")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Função auxiliar para extrair username das claims
pub fn extract_username(claims: &serde_json::Value) -> Option<String> {
    claims
        .get("preferred_username")
        .or_else(|| claims.get("name"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Função auxiliar para extrair email das claims
pub fn extract_email(claims: &serde_json::Value) -> Option<String> {
    claims
        .get("email")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Função auxiliar para extrair roles das claims
pub fn extract_roles(claims: &serde_json::Value) -> Vec<String> {
    claims
        .get("realm_access")
        .and_then(|ra| ra.get("roles"))
        .and_then(|roles| roles.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Função auxiliar para verificar se o usuário tem uma role específica
pub fn has_role(claims: &serde_json::Value, role: &str) -> bool {
    extract_roles(claims).contains(&role.to_string())
}

/// Função auxiliar para verificar se o usuário é admin
pub fn is_admin(claims: &serde_json::Value) -> bool {
    has_role(claims, "admin")
}
