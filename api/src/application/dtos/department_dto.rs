// ============================================================================
// DEPARTMENT DTOs - DATA TRANSFER OBJECTS
// ============================================================================
// DTOs para operações de departamentos na camada de aplicação
// Seguem o padrão de separação entre camadas da Clean Architecture

use crate::domain::entities::Department;
use crate::domain::value_objects::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// REQUEST DTOs - DADOS DE ENTRADA
// ============================================================================

/// DTO para criação de novos departamentos
/// Contém dados necessários para criar um departamento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentRequest {
    /// ID da unidade organizacional à qual o departamento pertence
    pub unit_id: Uuid,
    /// Nome do departamento
    pub name: String,
}

/// DTO para atualização de departamentos existentes
/// Todos os campos são opcionais para permitir atualização parcial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentRequest {
    /// ID do departamento (não serializado, vem da URL)
    #[serde(skip)]
    pub id: String,
    /// ID da unidade organizacional (opcional)
    pub unit_id: Option<Uuid>,
    /// Nome do departamento (opcional)
    pub name: Option<String>,
}

/// DTO para busca de departamentos com filtros
/// Suporta paginação e filtros diversos
#[derive(Debug, Clone, Deserialize)]
pub struct DepartmentSearchRequest {
    /// Termo de busca geral (nome do departamento)
    pub search_term: Option<String>,
    /// Filtro por unidade organizacional
    pub unit_id: Option<Uuid>,
    /// Limite de resultados por página
    pub limit: Option<i64>,
    /// Offset para paginação
    pub offset: Option<i64>,
}

// ============================================================================
// RESPONSE DTOs - DADOS DE SAÍDA
// ============================================================================

/// DTO de resposta para departamentos individuais
/// Contém todos os dados de um departamento para resposta da API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentResponse {
    /// ID único do departamento
    pub id: Uuid,
    /// ID da unidade organizacional à qual pertence
    pub unit_id: Uuid,
    /// Nome do departamento
    pub name: String,
    /// Data de criação
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Data da última atualização
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// DTO de resposta para busca de departamentos
/// Contém lista paginada de departamentos e metadados
#[derive(Debug, Clone, Serialize)]
pub struct DepartmentSearchResponse {
    /// Lista de departamentos encontrados
    pub items: Vec<DepartmentResponse>,
    /// Total de registros disponíveis
    pub total: i64,
}

/// DTO de resposta para estatísticas de departamentos
/// Contém dados agregados para dashboards e relatórios
#[derive(Debug, Clone, Serialize)]
pub struct DepartmentStatisticsResponse {
    /// Total de departamentos
    pub total_departments: i64,
    /// Contagem de departamentos por unidade organizacional
    pub departments_by_unit: std::collections::HashMap<Uuid, i64>,
}

// ============================================================================
// CONVERSÕES - DOMAIN ENTITIES TO DTOs
// ============================================================================

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar Department (domínio) em DepartmentResponse (aplicação)
impl From<Department> for DepartmentResponse {
    fn from(department: Department) -> Self {
        DepartmentResponse {
            id: department.id.0,
            unit_id: department.unit_id.0,
            name: department.name.value,
            created_at: department.created_at,
            updated_at: department.updated_at,
        }
    }
}
