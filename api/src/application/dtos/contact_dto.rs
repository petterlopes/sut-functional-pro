// ============================================================================
// CONTACT DTOs - DATA TRANSFER OBJECTS
// ============================================================================
// DTOs para operações de contatos na camada de aplicação
// Seguem o padrão de separação entre camadas da Clean Architecture

use crate::domain::entities::Contact;
use crate::domain::value_objects::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// REQUEST DTOs - DADOS DE ENTRADA
// ============================================================================

/// DTO para criação de novos contatos
/// Contém todos os dados necessários para criar um contato
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactRequest {
    /// Nome completo do contato
    pub full_name: String,
    /// Tipo do contato (Person, Organization, Department)
    pub contact_type: String,
    /// Status do contato (Active, Inactive)
    pub status: String,
    /// Documento de identificação (CPF, CNPJ, etc.)
    pub document: Option<String>,
    /// ID da unidade organizacional (opcional)
    pub unit_id: Option<Uuid>,
    /// ID do departamento (opcional)
    pub department_id: Option<Uuid>,
    /// Lista de emails do contato
    pub emails: Vec<Email>,
    /// Lista de telefones do contato
    pub phones: Vec<Phone>,
}

/// DTO para atualização de contatos existentes
/// Todos os campos são opcionais para permitir atualização parcial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactRequest {
    /// ID do contato (não serializado, vem da URL)
    #[serde(skip)]
    pub id: String,
    /// Nome completo do contato (opcional)
    pub full_name: Option<String>,
    /// Tipo do contato (opcional)
    pub contact_type: Option<String>,
    /// Status do contato (opcional)
    pub status: Option<String>,
    /// Documento de identificação (opcional)
    pub document: Option<String>,
    /// ID da unidade organizacional (opcional)
    pub unit_id: Option<Uuid>,
    /// ID do departamento (opcional)
    pub department_id: Option<Uuid>,
    /// Lista de emails (opcional)
    pub emails: Option<Vec<Email>>,
    /// Lista de telefones (opcional)
    pub phones: Option<Vec<Phone>>,
    /// ETag para controle de concorrência otimista
    pub etag: String,
}

/// DTO para busca de contatos com filtros
/// Suporta paginação e filtros diversos
#[derive(Debug, Clone, Deserialize)]
pub struct ContactSearchRequest {
    /// Termo de busca geral (nome, documento, etc.)
    pub search_term: Option<String>,
    /// Filtro por tipo de contato
    pub contact_type: Option<String>,
    /// Filtro por status
    pub status: Option<String>,
    /// Filtro por unidade organizacional
    pub unit_id: Option<Uuid>,
    /// Filtro por departamento
    pub department_id: Option<Uuid>,
    /// Limite de resultados por página
    pub limit: Option<i64>,
    /// Offset para paginação
    pub offset: Option<i64>,
}

// ============================================================================
// RESPONSE DTOs - DADOS DE SAÍDA
// ============================================================================

/// DTO de resposta para contatos individuais
/// Contém todos os dados de um contato para resposta da API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactResponse {
    /// ID único do contato
    pub id: Uuid,
    /// Nome completo do contato
    pub full_name: String,
    /// Tipo do contato
    pub contact_type: String,
    /// Status do contato
    pub status: String,
    /// Documento de identificação
    pub document: Option<String>,
    /// ID da unidade organizacional
    pub unit_id: Option<Uuid>,
    /// ID do departamento
    pub department_id: Option<Uuid>,
    /// Lista de emails
    pub emails: Vec<Email>,
    /// Lista de telefones
    pub phones: Vec<Phone>,
    /// ETag para controle de concorrência
    pub etag: String,
    /// Data de criação
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Data da última atualização
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// DTO de resposta para busca de contatos
/// Contém lista paginada de contatos e metadados
#[derive(Debug, Clone, Serialize)]
pub struct ContactSearchResponse {
    /// Lista de contatos encontrados
    pub items: Vec<ContactResponse>,
    /// Total de registros disponíveis
    pub total: i64,
}

/// DTO de resposta para estatísticas de contatos
/// Contém dados agregados para dashboards e relatórios
#[derive(Debug, Clone, Serialize)]
pub struct ContactStatisticsResponse {
    /// Total de contatos
    pub total_contacts: i64,
    /// Contatos ativos
    pub active_contacts: i64,
    /// Contatos inativos
    pub inactive_contacts: i64,
    /// Pessoas físicas
    pub persons: i64,
    /// Organizações
    pub organizations: i64,
    /// Departamentos
    pub departments: i64,
}

// ============================================================================
// CONVERSÕES - DOMAIN ENTITIES TO DTOs
// ============================================================================

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar Contact (domínio) em ContactResponse (aplicação)
impl From<Contact> for ContactResponse {
    fn from(contact: Contact) -> Self {
        ContactResponse {
            id: contact.id.0,
            full_name: contact.full_name,
            contact_type: contact.contact_type.to_string(),
            status: contact.status.to_string(),
            document: contact.document,
            unit_id: contact.unit_id.map(|id| id.0),
            department_id: contact.department_id.map(|id| id.0),
            emails: contact.emails,
            phones: contact.phones,
            etag: contact.etag,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
        }
    }
}
