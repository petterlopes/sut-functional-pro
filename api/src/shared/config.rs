// ============================================================================
// CONFIGURATION SYSTEM - SISTEMA DE CONFIGURAÇÃO CENTRALIZADO
// ============================================================================
// Sistema unificado para gerenciamento de configurações da aplicação
// Elimina redundância e centraliza todas as configurações

use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

// ============================================================================
// CONFIGURAÇÃO PRINCIPAL DA APLICAÇÃO
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Configurações do servidor
    pub server: ServerConfig,
    /// Configurações do banco de dados
    pub database: DatabaseConfig,
    /// Configurações de autenticação
    pub auth: AuthConfig,
    /// Configurações de logging
    pub logging: LoggingConfig,
    /// Configurações de métricas
    pub metrics: MetricsConfig,
    /// Configurações de cache
    pub cache: CacheConfig,
    /// Configurações de CORS
    pub cors: CorsConfig,
    /// Configurações de Vault (opcional)
    pub vault: Option<VaultConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Porta do servidor
    pub port: u16,
    /// Host do servidor
    pub host: String,
    /// Modo debug
    pub debug: bool,
    /// Timeout de requisição
    pub request_timeout: Duration,
    /// Tamanho máximo do body
    pub max_body_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// String de conexão do PostgreSQL
    pub connection_string: String,
    /// Tamanho do pool de conexões
    pub pool_size: u32,
    /// Timeout de conexão
    pub connection_timeout: Duration,
    /// Timeout de query
    pub query_timeout: Duration,
    /// Máximo de conexões ociosas
    pub max_idle_connections: u32,
    /// Máximo de tempo de vida da conexão
    pub max_connection_lifetime: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// URL do Keycloak
    pub keycloak_url: String,
    /// Realm do Keycloak
    pub realm: String,
    /// Client ID
    pub client_id: String,
    /// Client Secret
    pub client_secret: String,
    /// Tempo de vida do token
    pub token_lifetime: Duration,
    /// Tempo de refresh do JWKS
    pub jwks_refresh_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Nível de log
    pub level: String,
    /// Formato do log (json, pretty)
    pub format: String,
    /// Incluir spans
    pub include_spans: bool,
    /// Incluir eventos
    pub include_events: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Habilitar métricas
    pub enabled: bool,
    /// Porta das métricas
    pub port: u16,
    /// Token de autenticação das métricas
    pub token: Option<String>,
    /// Intervalo de coleta
    pub collection_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Habilitar cache
    pub enabled: bool,
    /// TTL padrão do cache
    pub default_ttl: Duration,
    /// Tamanho máximo do cache
    pub max_size: usize,
    /// Estratégia de eviction
    pub eviction_strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Origens permitidas
    pub allowed_origins: Vec<String>,
    /// Métodos permitidos
    pub allowed_methods: Vec<String>,
    /// Headers permitidos
    pub allowed_headers: Vec<String>,
    /// Expor headers
    pub exposed_headers: Vec<String>,
    /// Permitir credenciais
    pub allow_credentials: bool,
    /// Tempo de cache do preflight
    pub max_age: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    /// URL do Vault
    pub url: String,
    /// Token do Vault
    pub token: String,
    /// Path do secret
    pub secret_path: String,
    /// Timeout de conexão
    pub timeout: Duration,
}

// ============================================================================
// IMPLEMENTAÇÃO DA CONFIGURAÇÃO
// ============================================================================

impl AppConfig {
    /// Carrega configuração a partir de variáveis de ambiente
    pub fn from_env() -> Result<Self, ConfigError> {
        let server = ServerConfig {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidPort)?,
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            debug: env::var("DEBUG")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            request_timeout: Duration::from_secs(
                env::var("REQUEST_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            ),
            max_body_size: env::var("MAX_BODY_SIZE")
                .unwrap_or_else(|_| "1048576".to_string()) // 1MB
                .parse()
                .unwrap_or(1048576),
        };

        let database = DatabaseConfig {
            connection_string: env::var("DATABASE_URL")
                .map_err(|_| ConfigError::MissingDatabaseUrl)?,
            pool_size: env::var("DB_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            connection_timeout: Duration::from_secs(
                env::var("DB_CONNECTION_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            ),
            query_timeout: Duration::from_secs(
                env::var("DB_QUERY_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            ),
            max_idle_connections: env::var("DB_MAX_IDLE")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            max_connection_lifetime: Duration::from_secs(
                env::var("DB_MAX_LIFETIME")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap_or(3600),
            ),
        };

        let auth = AuthConfig {
            keycloak_url: env::var("KEYCLOAK_URL").map_err(|_| ConfigError::MissingKeycloakUrl)?,
            realm: env::var("KEYCLOAK_REALM").map_err(|_| ConfigError::MissingKeycloakRealm)?,
            client_id: env::var("KEYCLOAK_CLIENT_ID")
                .map_err(|_| ConfigError::MissingKeycloakClientId)?,
            client_secret: env::var("KEYCLOAK_CLIENT_SECRET")
                .map_err(|_| ConfigError::MissingKeycloakClientSecret)?,
            token_lifetime: Duration::from_secs(
                env::var("TOKEN_LIFETIME")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap_or(3600),
            ),
            jwks_refresh_interval: Duration::from_secs(
                env::var("JWKS_REFRESH_INTERVAL")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap_or(3600),
            ),
        };

        let logging = LoggingConfig {
            level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            format: env::var("LOG_FORMAT").unwrap_or_else(|_| "json".to_string()),
            include_spans: env::var("LOG_INCLUDE_SPANS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            include_events: env::var("LOG_INCLUDE_EVENTS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
        };

        let metrics = MetricsConfig {
            enabled: env::var("METRICS_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            port: env::var("METRICS_PORT")
                .unwrap_or_else(|_| "9090".to_string())
                .parse()
                .unwrap_or(9090),
            token: env::var("METRICS_TOKEN").ok(),
            collection_interval: Duration::from_secs(
                env::var("METRICS_COLLECTION_INTERVAL")
                    .unwrap_or_else(|_| "60".to_string())
                    .parse()
                    .unwrap_or(60),
            ),
        };

        let cache = CacheConfig {
            enabled: env::var("CACHE_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            default_ttl: Duration::from_secs(
                env::var("CACHE_DEFAULT_TTL")
                    .unwrap_or_else(|_| "300".to_string())
                    .parse()
                    .unwrap_or(300),
            ),
            max_size: env::var("CACHE_MAX_SIZE")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .unwrap_or(1000),
            eviction_strategy: env::var("CACHE_EVICTION_STRATEGY")
                .unwrap_or_else(|_| "lru".to_string()),
        };

        let cors = CorsConfig {
            allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "*".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            allowed_methods: env::var("CORS_ALLOWED_METHODS")
                .unwrap_or_else(|_| "GET,POST,PUT,PATCH,DELETE,OPTIONS".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            allowed_headers: env::var("CORS_ALLOWED_HEADERS")
                .unwrap_or_else(|_| "*".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            exposed_headers: env::var("CORS_EXPOSED_HEADERS")
                .unwrap_or_else(|_| "".to_string())
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
            allow_credentials: env::var("CORS_ALLOW_CREDENTIALS")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            max_age: Duration::from_secs(
                env::var("CORS_MAX_AGE")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()
                    .unwrap_or(86400),
            ),
        };

        let vault = if env::var("VAULT_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false)
        {
            Some(VaultConfig {
                url: env::var("VAULT_URL").map_err(|_| ConfigError::MissingVaultUrl)?,
                token: env::var("VAULT_TOKEN").map_err(|_| ConfigError::MissingVaultToken)?,
                secret_path: env::var("VAULT_SECRET_PATH")
                    .unwrap_or_else(|_| "secret/data/sut".to_string()),
                timeout: Duration::from_secs(
                    env::var("VAULT_TIMEOUT")
                        .unwrap_or_else(|_| "30".to_string())
                        .parse()
                        .unwrap_or(30),
                ),
            })
        } else {
            None
        };

        Ok(AppConfig {
            server,
            database,
            auth,
            logging,
            metrics,
            cache,
            cors,
            vault,
        })
    }

    /// Valida a configuração
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validar servidor
        if self.server.port == 0 {
            return Err(ConfigError::InvalidPort);
        }

        // Validar banco de dados
        if self.database.connection_string.is_empty() {
            return Err(ConfigError::MissingDatabaseUrl);
        }

        if self.database.pool_size == 0 {
            return Err(ConfigError::InvalidPoolSize);
        }

        // Validar autenticação
        if self.auth.keycloak_url.is_empty() {
            return Err(ConfigError::MissingKeycloakUrl);
        }

        if self.auth.realm.is_empty() {
            return Err(ConfigError::MissingKeycloakRealm);
        }

        if self.auth.client_id.is_empty() {
            return Err(ConfigError::MissingKeycloakClientId);
        }

        if self.auth.client_secret.is_empty() {
            return Err(ConfigError::MissingKeycloakClientSecret);
        }

        // Validar métricas
        if self.metrics.enabled && self.metrics.port == 0 {
            return Err(ConfigError::InvalidMetricsPort);
        }

        // Validar cache
        if self.cache.enabled && self.cache.max_size == 0 {
            return Err(ConfigError::InvalidCacheSize);
        }

        Ok(())
    }

    /// Retorna configuração padrão para desenvolvimento
    pub fn default_dev() -> Self {
        Self {
            server: ServerConfig {
                port: 3000,
                host: "0.0.0.0".to_string(),
                debug: true,
                request_timeout: Duration::from_secs(30),
                max_body_size: 1048576, // 1MB
            },
            database: DatabaseConfig {
                connection_string: "postgresql://user:password@localhost:5432/sut_db".to_string(),
                pool_size: 10,
                connection_timeout: Duration::from_secs(30),
                query_timeout: Duration::from_secs(30),
                max_idle_connections: 5,
                max_connection_lifetime: Duration::from_secs(3600),
            },
            auth: AuthConfig {
                keycloak_url: "http://localhost:8080".to_string(),
                realm: "sut".to_string(),
                client_id: "sut-api".to_string(),
                client_secret: "dev-secret".to_string(),
                token_lifetime: Duration::from_secs(3600),
                jwks_refresh_interval: Duration::from_secs(3600),
            },
            logging: LoggingConfig {
                level: "debug".to_string(),
                format: "pretty".to_string(),
                include_spans: true,
                include_events: true,
            },
            metrics: MetricsConfig {
                enabled: true,
                port: 9090,
                token: None,
                collection_interval: Duration::from_secs(60),
            },
            cache: CacheConfig {
                enabled: false,
                default_ttl: Duration::from_secs(300),
                max_size: 1000,
                eviction_strategy: "lru".to_string(),
            },
            cors: CorsConfig {
                allowed_origins: vec!["*".to_string()],
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "PATCH".to_string(),
                    "DELETE".to_string(),
                    "OPTIONS".to_string(),
                ],
                allowed_headers: vec!["*".to_string()],
                exposed_headers: vec![],
                allow_credentials: false,
                max_age: Duration::from_secs(86400),
            },
            vault: None,
        }
    }
}

// ============================================================================
// ERROS DE CONFIGURAÇÃO
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid port number")]
    InvalidPort,
    #[error("Missing DATABASE_URL environment variable")]
    MissingDatabaseUrl,
    #[error("Invalid database pool size")]
    InvalidPoolSize,
    #[error("Missing KEYCLOAK_URL environment variable")]
    MissingKeycloakUrl,
    #[error("Missing KEYCLOAK_REALM environment variable")]
    MissingKeycloakRealm,
    #[error("Missing KEYCLOAK_CLIENT_ID environment variable")]
    MissingKeycloakClientId,
    #[error("Missing KEYCLOAK_CLIENT_SECRET environment variable")]
    MissingKeycloakClientSecret,
    #[error("Invalid metrics port")]
    InvalidMetricsPort,
    #[error("Invalid cache size")]
    InvalidCacheSize,
    #[error("Missing VAULT_URL environment variable")]
    MissingVaultUrl,
    #[error("Missing VAULT_TOKEN environment variable")]
    MissingVaultToken,
    #[error("Configuration validation failed: {0}")]
    ValidationError(String),
}

// ============================================================================
// IMPLEMENTAÇÃO DOS TRAITS BASE
// ============================================================================

impl crate::shared::base_traits::AppConfig for AppConfig {
    fn port(&self) -> u16 {
        self.server.port
    }

    fn host(&self) -> &str {
        &self.server.host
    }

    fn is_debug(&self) -> bool {
        self.server.debug
    }

    fn log_level(&self) -> &str {
        &self.logging.level
    }
}

impl crate::shared::base_traits::RepositoryConfig for DatabaseConfig {
    fn connection_string(&self) -> &str {
        &self.connection_string
    }

    fn pool_size(&self) -> u32 {
        self.pool_size
    }

    fn connection_timeout(&self) -> Duration {
        self.connection_timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dev_config() {
        let config = AppConfig::default_dev();
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.server.debug, true);
        assert_eq!(config.database.pool_size, 10);
        assert_eq!(config.auth.realm, "sut");
        assert_eq!(config.logging.level, "debug");
        assert_eq!(config.metrics.enabled, true);
        assert_eq!(config.cache.enabled, false);
    }

    #[test]
    fn test_config_validation() {
        let config = AppConfig::default_dev();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config() {
        let mut config = AppConfig::default_dev();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
}
