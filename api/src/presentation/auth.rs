use anyhow::Context;
use axum::{extract::Request, http::StatusCode};
use axum::{middleware::Next, response::Response};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use once_cell::sync::OnceCell;
use reqwest::Client;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

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
                    let v: serde_json::Value = resp
                        .json()
                        .await
                        .context("parsing JWKS")?;
                    *self.keys.write() = v;
                    return Ok(());
                }
                Err(e) => {
                    if attempt >= max_attempts {
                        return Err(e).context(format!("fetching JWKS after {} attempts", attempt));
                    }
                    eprintln!("[auth] JWKS fetch attempt {} failed: {}. retrying in {:?}", attempt, e, wait);
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
        return keys["keys"].as_array().map(|a| !a.is_empty()).unwrap_or(false);
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
    // Development bypass: if DEV_AUTH_BYPASS=1 is set in the environment and the
    // request provides an X-Dev-User header, inject a synthetic claims object
    // so local development can call protected endpoints without a real JWT.
    if std::env::var("DEV_AUTH_BYPASS").ok().as_deref() == Some("1") {
        if let Some(dev_user) = req.headers().get("x-dev-user").and_then(|v| v.to_str().ok()) {
            tracing::info!("DEV_AUTH_BYPASS active, X-Dev-User: {}", dev_user);
            let claims = serde_json::json!({
                "sub": dev_user,
                // Provide realm roles that the code checks for (directory.read/write/pii.read)
                "realm_access": { "roles": ["directory.read", "directory.write", "directory.pii.read"] },
                "scope": "directory.read directory.write"
            });
            req.extensions_mut().insert(claims);
            return Ok(next.run(req).await);
        } else {
            tracing::warn!("DEV_AUTH_BYPASS active but X-Dev-User header missing or invalid");
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
                    let set: std::collections::HashSet<&str> = arr
                        .iter()
                        .filter_map(|v| v.as_str())
                        .collect();
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
