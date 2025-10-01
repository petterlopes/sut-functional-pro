// ============================================================================
// USER DTOs - DATA TRANSFER OBJECTS
// ============================================================================
// DTOs para operações de usuários na camada de aplicação
// Seguem o padrão de separação entre camadas da Clean Architecture

use crate::domain::entities::User;
use crate::domain::value_objects::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// REQUEST DTOs - DADOS DE ENTRADA
// ============================================================================

/// DTO para criação de novos usuários
/// Contém dados necessários para criar um usuário
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// Nome de usuário único
    pub username: String,
    /// Email do usuário
    pub email: String,
    /// Senha do usuário (será hasheada)
    pub password: String,
    /// Lista de roles/permissões do usuário
    pub roles: Vec<String>,
}

/// DTO para atualização de usuários existentes
/// Todos os campos são opcionais para permitir atualização parcial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    /// ID do usuário (não serializado, vem da URL)
    #[serde(skip)]
    pub id: String,
    /// Nome de usuário (opcional)
    pub username: Option<String>,
    /// Email do usuário (opcional)
    pub email: Option<String>,
    /// Nova senha (opcional, será hasheada)
    pub password: Option<String>,
    /// Lista de roles/permissões (opcional)
    pub roles: Option<Vec<String>>,
}

/// DTO para busca de usuários com filtros
/// Suporta paginação e filtros diversos
#[derive(Debug, Clone, Deserialize)]
pub struct UserSearchRequest {
    /// Termo de busca geral (username, email)
    pub search_term: Option<String>,
    /// Filtro por role/permissão
    pub role: Option<String>,
    /// Limite de resultados por página
    pub limit: Option<i64>,
    /// Offset para paginação
    pub offset: Option<i64>,
}

// ============================================================================
// RESPONSE DTOs - DADOS DE SAÍDA
// ============================================================================

/// DTO de resposta para usuários individuais
/// Contém dados de um usuário para resposta da API (sem senha)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    /// ID único do usuário
    pub id: Uuid,
    /// Nome de usuário
    pub username: String,
    /// Email do usuário
    pub email: String,
    /// Lista de roles/permissões
    pub roles: Vec<String>,
    /// Data de criação
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Data da última atualização
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// DTO de resposta para busca de usuários
/// Contém lista paginada de usuários e metadados
#[derive(Debug, Clone, Serialize)]
pub struct UserSearchResponse {
    /// Lista de usuários encontrados
    pub items: Vec<UserResponse>,
    /// Total de registros disponíveis
    pub total: i64,
}

// ============================================================================
// CONVERSÕES - DOMAIN ENTITIES TO DTOs
// ============================================================================

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar User (domínio) em UserResponse (aplicação)
/// Nota: A senha não é incluída na resposta por segurança
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.0,
            username: user.username.value,
            email: user.email.value,
            roles: user.roles.into_iter().map(|r| r.value).collect(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
