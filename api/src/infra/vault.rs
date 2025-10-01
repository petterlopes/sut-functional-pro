//! =============================================================================
//! VAULT INTEGRATION MODULE
//! =============================================================================
//! Módulo para integração segura com HashiCorp Vault
//! Implementa gerenciamento de secrets, criptografia e autenticação

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Configuração do cliente Vault
#[derive(Debug, Clone)]
pub struct VaultConfig {
    /// URL do servidor Vault
    pub addr: String,
    /// Token de autenticação
    pub token: String,
    /// Timeout para requisições
    pub timeout: Duration,
    /// Máximo de tentativas de retry
    pub max_retries: u32,
    /// Intervalo entre tentativas
    pub retry_delay: Duration,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            addr: std::env::var("VAULT_ADDR").unwrap_or_else(|_| "http://vault:8200".to_string()),
            token: std::env::var("VAULT_TOKEN").unwrap_or_default(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
        }
    }
}

/// Resposta da API do Vault
#[derive(Debug, Deserialize)]
pub struct VaultResponse<T> {
    pub data: T,
    pub lease_duration: Option<u64>,
    pub renewable: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

/// Dados de um secret KV
#[derive(Debug, Deserialize)]
pub struct KvData {
    pub data: HashMap<String, Value>,
    pub metadata: Option<KvMetadata>,
}

/// Metadados de um secret KV
#[derive(Debug, Deserialize)]
pub struct KvMetadata {
    pub created_time: Option<String>,
    pub current_version: Option<u32>,
    pub delete_version_after: Option<String>,
    pub max_versions: Option<u32>,
    pub oldest_version: Option<u32>,
    pub updated_time: Option<String>,
    pub versions: Option<HashMap<String, Value>>,
}

/// Dados de credenciais do database
#[derive(Debug, Deserialize)]
pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    pub lease_duration: u64,
    pub renewable: bool,
}

/// Dados de criptografia do Transit
#[derive(Debug, Serialize, Deserialize)]
pub struct TransitEncryptRequest {
    pub plaintext: String,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransitEncryptResponse {
    pub ciphertext: String,
    pub key_version: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitDecryptRequest {
    pub ciphertext: String,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransitDecryptResponse {
    pub plaintext: String,
}

/// Cliente Vault com cache e retry automático
#[derive(Clone)]
pub struct VaultClient {
    config: VaultConfig,
    client: Client,
    cache: Arc<RwLock<HashMap<String, (Value, std::time::Instant)>>>,
    cache_ttl: Duration,
}

impl VaultClient {
    /// Criar novo cliente Vault
    pub fn new(config: VaultConfig) -> Result<Self> {
        if config.token.trim().is_empty() {
            return Err(anyhow::anyhow!(
                "VAULT_TOKEN must be set before initializing VaultClient"
            ));
        }
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .context("Falha ao criar cliente HTTP para Vault")?;

        Ok(Self {
            config,
            client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::from_secs(300), // 5 minutos
        })
    }

    /// Criar cliente com configuração padrão
    pub fn default() -> Result<Self> {
        Self::new(VaultConfig::default())
    }

    /// Fazer requisição autenticada para o Vault
    async fn request<T>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<VaultResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/v1/{}", self.config.addr, path.trim_start_matches('/'));

        let mut request = self
            .client
            .request(method, &url)
            .header("X-Vault-Token", &self.config.token)
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            request = request.json(&body);
        }

        let mut retries = 0;
        loop {
            match request.try_clone().unwrap().send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let vault_response: VaultResponse<T> = response
                            .json()
                            .await
                            .context("Falha ao deserializar resposta do Vault")?;

                        debug!("Requisição Vault bem-sucedida: {}", path);
                        return Ok(vault_response);
                    } else {
                        let status = response.status();
                        let error_text = response.text().await.unwrap_or_default();
                        error!("Erro na requisição Vault: {} - {}", status, error_text);
                        return Err(anyhow::anyhow!("Vault error: {} - {}", status, error_text));
                    }
                }
                Err(e) => {
                    retries += 1;
                    if retries >= self.config.max_retries {
                        error!(
                            "Máximo de tentativas excedido para requisição Vault: {}",
                            path
                        );
                        return Err(e.into());
                    }

                    warn!("Tentativa {} falhou para {}: {}", retries, path, e);
                    tokio::time::sleep(self.config.retry_delay).await;
                }
            }
        }
    }

    /// Obter secret do KV store
    pub async fn get_secret(&self, path: &str) -> Result<HashMap<String, Value>> {
        // Verificar cache primeiro
        {
            let cache = self.cache.read().await;
            if let Some((value, timestamp)) = cache.get(path) {
                if timestamp.elapsed() < self.cache_ttl {
                    debug!("Cache hit para secret: {}", path);
                    return Ok(serde_json::from_value(value.clone())?);
                }
            }
        }

        // Buscar no Vault
        let response: VaultResponse<KvData> = self
            .request(reqwest::Method::GET, &format!("kv/data/{}", path), None)
            .await?;

        let data = response.data.data;

        // Atualizar cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(
                path.to_string(),
                (serde_json::to_value(&data)?, std::time::Instant::now()),
            );
        }

        info!("Secret obtido do Vault: {}", path);
        Ok(data)
    }

    /// Obter secret específico por chave
    pub async fn get_secret_value(&self, path: &str, key: &str) -> Result<String> {
        let secrets = self.get_secret(path).await?;
        secrets
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Secret não encontrado: {}/{}", path, key))
    }

    /// Armazenar secret no KV store
    pub async fn put_secret(&self, path: &str, data: HashMap<String, Value>) -> Result<()> {
        let body = serde_json::json!({ "data": data });

        self.request::<Value>(
            reqwest::Method::POST,
            &format!("kv/data/{}", path),
            Some(body),
        )
        .await?;

        // Invalidar cache
        {
            let mut cache = self.cache.write().await;
            cache.remove(path);
        }

        info!("Secret armazenado no Vault: {}", path);
        Ok(())
    }

    /// Obter credenciais do database
    pub async fn get_database_credentials(&self, role: &str) -> Result<DatabaseCredentials> {
        let response: VaultResponse<DatabaseCredentials> = self
            .request(
                reqwest::Method::GET,
                &format!("database/creds/{}", role),
                None,
            )
            .await?;

        info!("Credenciais do database obtidas para role: {}", role);
        Ok(response.data)
    }

    /// Criptografar dados usando Transit
    pub async fn encrypt(&self, key_name: &str, plaintext: &str) -> Result<String> {
        let request = TransitEncryptRequest {
            plaintext: base64::engine::general_purpose::STANDARD.encode(plaintext),
            context: None,
        };

        let response: VaultResponse<TransitEncryptResponse> = self
            .request(
                reqwest::Method::POST,
                &format!("transit/encrypt/{}", key_name),
                Some(serde_json::to_value(request)?),
            )
            .await?;

        info!("Dados criptografados com chave: {}", key_name);
        Ok(response.data.ciphertext)
    }

    /// Descriptografar dados usando Transit
    pub async fn decrypt(&self, key_name: &str, ciphertext: &str) -> Result<String> {
        let request = TransitDecryptRequest {
            ciphertext: ciphertext.to_string(),
            context: None,
        };

        let response: VaultResponse<TransitDecryptResponse> = self
            .request(
                reqwest::Method::POST,
                &format!("transit/decrypt/{}", key_name),
                Some(serde_json::to_value(request)?),
            )
            .await?;

        let plaintext = base64::engine::general_purpose::STANDARD
            .decode(&response.data.plaintext)
            .context("Falha ao decodificar dados descriptografados")?;

        info!("Dados descriptografados com chave: {}", key_name);
        Ok(String::from_utf8(plaintext)?)
    }

    /// Gerar certificado usando PKI
    pub async fn generate_certificate(
        &self,
        role: &str,
        common_name: &str,
        ttl: Option<&str>,
    ) -> Result<HashMap<String, Value>> {
        let mut body = serde_json::json!({
            "common_name": common_name,
            "format": "pem"
        });

        if let Some(ttl) = ttl {
            body["ttl"] = serde_json::Value::String(ttl.to_string());
        }

        let response: VaultResponse<HashMap<String, Value>> = self
            .request(
                reqwest::Method::POST,
                &format!("pki/issue/{}", role),
                Some(body),
            )
            .await?;

        info!("Certificado gerado para: {}", common_name);
        Ok(response.data)
    }

    /// Verificar saúde do Vault
    pub async fn health_check(&self) -> Result<bool> {
        match self
            .client
            .get(&format!("{}/v1/sys/health", self.config.addr))
            .send()
            .await
        {
            Ok(response) => {
                let is_healthy = response.status().is_success();
                if is_healthy {
                    debug!("Vault health check: OK");
                } else {
                    warn!("Vault health check: FAILED - {}", response.status());
                }
                Ok(is_healthy)
            }
            Err(e) => {
                error!("Vault health check error: {}", e);
                Ok(false)
            }
        }
    }

    /// Limpar cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        info!("Cache do Vault limpo");
    }

    /// Obter estatísticas do cache
    pub async fn cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.read().await;
        let total = cache.len();
        let expired = cache
            .values()
            .filter(|(_, timestamp)| timestamp.elapsed() >= self.cache_ttl)
            .count();
        (total, expired)
    }
}

/// Trait para serviços que precisam de acesso ao Vault
#[async_trait::async_trait]
pub trait VaultService {
    /// Obter cliente Vault
    fn vault_client(&self) -> &VaultClient;

    /// Obter configuração do database do Vault
    async fn get_database_config(&self) -> Result<DatabaseConfig> {
        let secrets = self.vault_client().get_secret("sut/database").await?;

        Ok(DatabaseConfig {
            host: secrets
                .get("host")
                .and_then(|v| v.as_str())
                .unwrap_or("postgres")
                .to_string(),
            port: secrets
                .get("port")
                .and_then(|v| v.as_str())
                .unwrap_or("5432")
                .to_string(),
            database: secrets
                .get("database")
                .and_then(|v| v.as_str())
                .unwrap_or("sut")
                .to_string(),
            username: secrets
                .get("username")
                .and_then(|v| v.as_str())
                .unwrap_or("sut")
                .to_string(),
            password: secrets
                .get("password")
                .and_then(|v| v.as_str())
                .unwrap_or("sut")
                .to_string(),
            ssl_mode: secrets
                .get("ssl_mode")
                .and_then(|v| v.as_str())
                .unwrap_or("require")
                .to_string(),
        })
    }

    /// Obter configuração da API do Vault
    async fn get_api_config(&self) -> Result<ApiConfig> {
        let secrets = self.vault_client().get_secret("sut/api").await?;

        Ok(ApiConfig {
            jwt_secret: secrets
                .get("jwt_secret")
                .and_then(|v| v.as_str())
                .unwrap_or("default-secret")
                .to_string(),
            encryption_key: secrets
                .get("encryption_key")
                .and_then(|v| v.as_str())
                .unwrap_or("default-key")
                .to_string(),
            webhook_secret: secrets
                .get("webhook_secret")
                .and_then(|v| v.as_str())
                .unwrap_or("default-webhook")
                .to_string(),
            api_key: secrets
                .get("api_key")
                .and_then(|v| v.as_str())
                .unwrap_or("default-api-key")
                .to_string(),
        })
    }

    /// Obter configuração do Keycloak do Vault
    async fn get_keycloak_config(&self) -> Result<KeycloakConfig> {
        let secrets = self.vault_client().get_secret("sut/keycloak").await?;

        Ok(KeycloakConfig {
            admin_user: secrets
                .get("admin_user")
                .and_then(|v| v.as_str())
                .unwrap_or("admin")
                .to_string(),
            admin_password: secrets
                .get("admin_password")
                .and_then(|v| v.as_str())
                .unwrap_or("admin")
                .to_string(),
            client_secret: secrets
                .get("client_secret")
                .and_then(|v| v.as_str())
                .unwrap_or("default-client-secret")
                .to_string(),
            jwt_secret: secrets
                .get("jwt_secret")
                .and_then(|v| v.as_str())
                .unwrap_or("default-jwt-secret")
                .to_string(),
        })
    }
}

/// Configuração do database obtida do Vault
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: String,
}

/// Configuração da API obtida do Vault
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub jwt_secret: String,
    pub encryption_key: String,
    pub webhook_secret: String,
    pub api_key: String,
}

/// Configuração do Keycloak obtida do Vault
#[derive(Debug, Clone)]
pub struct KeycloakConfig {
    pub admin_user: String,
    pub admin_password: String,
    pub client_secret: String,
    pub jwt_secret: String,
}

/// Serviço de gerenciamento de secrets
pub struct SecretsManager {
    vault_client: VaultClient,
}

impl SecretsManager {
    /// Criar novo gerenciador de secrets
    pub fn new(vault_client: VaultClient) -> Self {
        Self { vault_client }
    }

    /// Obter secret com fallback para variáveis de ambiente
    pub async fn get_secret_or_env(
        &self,
        vault_path: &str,
        key: &str,
        env_var: &str,
    ) -> Result<String> {
        match self.vault_client.get_secret_value(vault_path, key).await {
            Ok(value) => {
                debug!("Secret obtido do Vault: {}/{}", vault_path, key);
                Ok(value)
            }
            Err(_) => {
                warn!(
                    "Secret não encontrado no Vault, usando variável de ambiente: {}",
                    env_var
                );
                std::env::var(env_var)
                    .with_context(|| format!("Variável de ambiente não encontrada: {}", env_var))
            }
        }
    }

    /// Rotacionar secret
    pub async fn rotate_secret(&self, path: &str, key: &str) -> Result<String> {
        // Gerar novo secret
        let new_secret = self.generate_secure_secret(32);

        // Obter secrets existentes
        let mut secrets = self.vault_client.get_secret(path).await?;

        // Atualizar secret
        secrets.insert(
            key.to_string(),
            serde_json::Value::String(new_secret.clone()),
        );

        // Salvar no Vault
        self.vault_client.put_secret(path, secrets).await?;

        info!("Secret rotacionado: {}/{}", path, key);
        Ok(new_secret)
    }

    /// Gerar secret seguro
    fn generate_secure_secret(&self, length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
        let mut rng = rand::thread_rng();

        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

#[async_trait::async_trait]
impl VaultService for SecretsManager {
    fn vault_client(&self) -> &VaultClient {
        &self.vault_client
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_vault_client_creation() {
        let config = VaultConfig {
            addr: "http://localhost:8200".to_string(),
            token: "test-token".to_string(),
            timeout: Duration::from_secs(5),
            max_retries: 1,
            retry_delay: Duration::from_millis(100),
        };

        let client = VaultClient::new(config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_health_check() {
        let _m = mock("GET", "/v1/sys/health")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"initialized": true, "sealed": false}"#)
            .create();

        let config = VaultConfig {
            addr: server_url(),
            token: "test-token".to_string(),
            timeout: Duration::from_secs(5),
            max_retries: 1,
            retry_delay: Duration::from_millis(100),
        };

        let client = VaultClient::new(config).unwrap();
        let is_healthy = client.health_check().await.unwrap();
        assert!(is_healthy);
    }
}
