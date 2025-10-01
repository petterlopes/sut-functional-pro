// ============================================================================
// ORG UNIT DTOs - DATA TRANSFER OBJECTS
// ============================================================================
// DTOs para operações de unidades organizacionais na camada de aplicação
// Seguem o padrão de separação entre camadas da Clean Architecture

use crate::domain::entities::OrgUnit;
use crate::domain::value_objects::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// REQUEST DTOs - DADOS DE ENTRADA
// ============================================================================

/// DTO para criação de novas unidades organizacionais
/// Contém dados necessários para criar uma unidade organizacional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrgUnitRequest {
    /// Nome da unidade organizacional
    pub name: String,
    /// ID da unidade pai (opcional, para hierarquia)
    pub parent_id: Option<Uuid>,
}

/// DTO para atualização de unidades organizacionais existentes
/// Todos os campos são opcionais para permitir atualização parcial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrgUnitRequest {
    /// ID da unidade organizacional (não serializado, vem da URL)
    #[serde(skip)]
    pub id: String,
    /// Nome da unidade organizacional (opcional)
    pub name: Option<String>,
    /// ID da unidade pai (opcional)
    pub parent_id: Option<Uuid>,
}

/// DTO para busca de unidades organizacionais com filtros
/// Suporta paginação e filtros diversos
#[derive(Debug, Clone, Deserialize)]
pub struct OrgUnitSearchRequest {
    /// Termo de busca geral (nome da unidade)
    pub search_term: Option<String>,
    /// Filtro por unidade pai
    pub parent_id: Option<Uuid>,
    /// Limite de resultados por página
    pub limit: Option<i64>,
    /// Offset para paginação
    pub offset: Option<i64>,
}

// ============================================================================
// RESPONSE DTOs - DADOS DE SAÍDA
// ============================================================================

/// DTO de resposta para unidades organizacionais individuais
/// Contém todos os dados de uma unidade organizacional para resposta da API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgUnitResponse {
    /// ID único da unidade organizacional
    pub id: Uuid,
    /// Nome da unidade organizacional
    pub name: String,
    /// ID da unidade pai (se houver)
    pub parent_id: Option<Uuid>,
    /// Data de criação
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Data da última atualização
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// DTO de resposta para busca de unidades organizacionais
/// Contém lista paginada de unidades e metadados
#[derive(Debug, Clone, Serialize)]
pub struct OrgUnitSearchResponse {
    /// Lista de unidades organizacionais encontradas
    pub items: Vec<OrgUnitResponse>,
    /// Total de registros disponíveis
    pub total: i64,
}

/// DTO de resposta para hierarquia de unidades organizacionais
/// Contém estrutura hierárquica com unidades e seus filhos
#[derive(Debug, Clone, Serialize)]
pub struct OrgUnitHierarchyResponse {
    /// Lista de unidades organizacionais
    pub items: Vec<OrgUnitResponse>,
    /// Mapa de filhos por unidade pai
    pub children: std::collections::HashMap<Uuid, Vec<OrgUnitResponse>>,
}

// ============================================================================
// CONVERSÕES - DOMAIN ENTITIES TO DTOs
// ============================================================================

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar OrgUnit (domínio) em OrgUnitResponse (aplicação)
impl From<OrgUnit> for OrgUnitResponse {
    fn from(org_unit: OrgUnit) -> Self {
        OrgUnitResponse {
            id: org_unit.id.0,
            name: org_unit.name.value,
            parent_id: org_unit.parent_id.map(|id| id.0),
            created_at: org_unit.created_at,
            updated_at: org_unit.updated_at,
        }
    }
}
