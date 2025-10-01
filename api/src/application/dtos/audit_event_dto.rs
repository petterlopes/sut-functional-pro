// ============================================================================
// AUDIT EVENT DTOs - DATA TRANSFER OBJECTS
// ============================================================================
// DTOs para operações de eventos de auditoria na camada de aplicação
// Seguem o padrão de separação entre camadas da Clean Architecture

use crate::domain::entities::AuditEvent;
use crate::domain::value_objects::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// REQUEST DTOs - DADOS DE ENTRADA
// ============================================================================

/// DTO para busca de eventos de auditoria com filtros
/// Suporta paginação e filtros diversos para auditoria
#[derive(Debug, Clone, Deserialize)]
pub struct AuditEventSearchRequest {
    /// Filtro por tipo de entidade auditada
    pub entity_type: Option<String>,
    /// Filtro por ID da entidade auditada
    pub entity_id: Option<String>,
    /// Filtro por usuário que executou a ação
    pub actor_sub: Option<String>,
    /// Limite de resultados por página
    pub limit: Option<i64>,
    /// Offset para paginação
    pub offset: Option<i64>,
}

// ============================================================================
// RESPONSE DTOs - DADOS DE SAÍDA
// ============================================================================

/// DTO de resposta para eventos de auditoria individuais
/// Contém todos os dados de um evento de auditoria para resposta da API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventResponse {
    /// ID único do evento de auditoria
    pub id: i64,
    /// Identificador do usuário que executou a ação (subject do JWT)
    pub actor_sub: Option<String>,
    /// Ação executada (CREATE, UPDATE, DELETE, etc.)
    pub action: String,
    /// Tipo da entidade que foi modificada
    pub entity_type: String,
    /// ID da entidade que foi modificada
    pub entity_id: String,
    /// Estado anterior da entidade (JSON)
    pub before: Option<serde_json::Value>,
    /// Estado posterior da entidade (JSON)
    pub after: Option<serde_json::Value>,
    /// Timestamp do evento
    pub at: chrono::DateTime<chrono::Utc>,
}

/// DTO de resposta para busca de eventos de auditoria
/// Contém lista paginada de eventos e metadados
#[derive(Debug, Clone, Serialize)]
pub struct AuditEventSearchResponse {
    /// Lista de eventos de auditoria encontrados
    pub items: Vec<AuditEventResponse>,
    /// Total de registros disponíveis
    pub total: i64,
}

// ============================================================================
// CONVERSÕES - DOMAIN ENTITIES TO DTOs
// ============================================================================

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar AuditEvent (domínio) em AuditEventResponse (aplicação)
impl From<AuditEvent> for AuditEventResponse {
    fn from(event: AuditEvent) -> Self {
        AuditEventResponse {
            id: event.id.0,
            actor_sub: event.actor_sub,
            action: event.action.value,
            entity_type: event.entity_type.value,
            entity_id: event.entity_id,
            before: event.before,
            after: event.after,
            at: event.at,
        }
    }
}
