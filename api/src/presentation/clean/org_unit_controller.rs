// ============================================================================
// IMPORTS E DEPENDÊNCIAS - ORG UNIT CONTROLLER
// ============================================================================
// Controller que implementa a camada de apresentação para operações de unidades organizacionais
// Segue os princípios da Clean Architecture com separação clara de responsabilidades

// ===== CLEAN ARCHITECTURE IMPORTS =====
use crate::application::dto::*; // DTOs (Data Transfer Objects) para comunicação entre camadas
use crate::application::use_cases::org_unit::*; // Casos de uso da camada de aplicação
use crate::domain::{value_objects::OrgUnitId, DomainError}; // Value objects do domínio

// ===== PRESENTATION UTILITIES =====
use crate::presentation::{
    error_mapper::map_domain_error, // Mapeamento centralizado de erros
                                    // validation::validate_uuid, // Validação de UUID
};

// ===== AXUM FRAMEWORK IMPORTS =====
use axum::{
    extract::{Path, Query, State}, // Extractors para parâmetros de rota, query e estado
    http::StatusCode,              // Códigos de status HTTP
    response::Json,                // Resposta JSON
    routing::get,                  // Macros de roteamento HTTP
    Router,                        // Roteador principal do Axum
};

// ===== UTILITY IMPORTS =====
use serde_json::json; // Para criação de JSON dinâmico
use std::sync::Arc; // Para compartilhamento thread-safe do estado
use uuid::Uuid; // Para validação de UUIDs

// ============================================================================
// CONFIGURAÇÃO DE ROTAS - REST API ENDPOINTS
// ============================================================================
// Define todas as rotas REST para operações de unidades organizacionais
// Implementa o padrão RESTful com operações CRUD completas e funcionalidades hierárquicas

pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        // ===== ROTAS DE COLECAO =====
        .route(
            "/v1/org-units",
            get(get_org_units) // GET /v1/org-units - Listar unidades organizacionais com filtros
                .post(create_org_unit), // POST /v1/org-units - Criar nova unidade organizacional
        )
        // ===== ROTAS DE RECURSO INDIVIDUAL =====
        .route(
            "/v1/org-units/:id",
            get(get_org_unit) // GET /v1/org-units/:id - Buscar unidade organizacional por ID
                .patch(update_org_unit) // PATCH /v1/org-units/:id - Atualizar unidade organizacional
                .delete(delete_org_unit), // DELETE /v1/org-units/:id - Deletar unidade organizacional
        )
        // ===== ROTAS DE HIERARQUIA =====
        .route(
            "/v1/org-units/:id/hierarchy",
            get(get_org_unit_hierarchy), // GET /v1/org-units/:id/hierarchy - Hierarquia da unidade
        )
}

// ============================================================================
// HANDLER: GET /v1/org-units - LISTAR UNIDADES ORGANIZACIONAIS COM FILTROS
// ============================================================================
// Endpoint para buscar unidades organizacionais com suporte a filtros, paginação e ordenação
// Implementa o padrão de busca RESTful com parâmetros de query

async fn get_org_units(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Query(params): Query<OrgUnitSearchRequest>, // Parâmetros de query (filtros, paginação)
) -> Result<Json<OrgUnitSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    // Cria instância do caso de uso injetando a dependência do repositório
    let use_case = GetOrgUnitsUseCase::new(state.org_unit_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(params).await {
        Ok(response) => {
            // Sucesso: retorna resposta JSON com as unidades organizacionais encontradas
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS USANDO UTILITÁRIO CENTRALIZADO =====
            // Usa função centralizada para mapear erros de domínio para HTTP
            Err(map_domain_error(&err))
        }
    }
}

// ============================================================================
// HANDLER: GET /v1/org-units/:id - BUSCAR UNIDADE ORGANIZACIONAL POR ID
// ============================================================================
// Endpoint para buscar uma unidade organizacional específica pelo seu ID
// Inclui validação de formato UUID e tratamento de erros

async fn get_org_unit(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID da unidade organizacional extraído da URL
) -> Result<Json<OrgUnitResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetOrgUnitsUseCase::new(state.org_unit_repository.as_ref());

    // ===== VALIDAÇÃO DE UUID =====
    // Valida se o ID fornecido é um UUID válido antes de processar
    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            // UUID válido: converte para value object do domínio
            let org_unit_id = OrgUnitId(uuid);

            // ===== EXECUÇÃO DO CASO DE USO =====
            match use_case.execute_by_id(&org_unit_id).await {
                Ok(response) => {
                    // Sucesso: retorna a unidade organizacional encontrada
                    Ok(Json(response))
                }
                Err(err) => {
                    // ===== MAPEAMENTO DE ERROS =====
                    let status = match err {
                        DomainError::NotFound(_) => StatusCode::NOT_FOUND,
                        DomainError::ValidationError(_) => StatusCode::BAD_REQUEST,
                        DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                        DomainError::Forbidden(_) => StatusCode::FORBIDDEN,
                        DomainError::Conflict(_) => StatusCode::CONFLICT,
                        DomainError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                        DomainError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                        DomainError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
                        DomainError::BusinessRuleViolation(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    };
                    Err((status, Json(json!({"error": err.to_string()}))))
                }
            }
        }
        Err(_) => {
            // UUID inválido: retorna erro 400 Bad Request
            Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid UUID format"})),
            ))
        }
    }
}

// ============================================================================
// HANDLER: POST /v1/org-units - CRIAR NOVA UNIDADE ORGANIZACIONAL
// ============================================================================
// Endpoint para criação de novas unidades organizacionais
// Recebe dados via JSON e valida através do caso de uso

async fn create_org_unit(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Json(request): Json<CreateOrgUnitRequest>, // Dados da unidade organizacional em formato JSON
) -> Result<Json<OrgUnitResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    // Cria instância do caso de uso para criação de unidades organizacionais
    let use_case = CreateOrgUnitUseCase::new(state.org_unit_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => {
            // Sucesso: retorna a unidade organizacional criada com status 201 (será definido pelo Axum)
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND,
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // Dados inválidos
                DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                DomainError::Forbidden(_) => StatusCode::FORBIDDEN,
                DomainError::Conflict(_) => StatusCode::CONFLICT, // Conflito (ex: nome duplicado)
                DomainError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DomainError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DomainError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
                DomainError::BusinessRuleViolation(_) => StatusCode::UNPROCESSABLE_ENTITY, // Regra de negócio
            };
            Err((status, Json(json!({"error": err.to_string()}))))
        }
    }
}

// ============================================================================
// HANDLER: PATCH /v1/org-units/:id - ATUALIZAR UNIDADE ORGANIZACIONAL
// ============================================================================
// Endpoint para atualização parcial de unidades organizacionais existentes
// Combina ID da URL com dados do JSON para atualização

async fn update_org_unit(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID da unidade organizacional extraído da URL
    Json(mut request): Json<UpdateOrgUnitRequest>, // Dados de atualização em JSON
) -> Result<Json<OrgUnitResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== PREPARAÇÃO DOS DADOS =====
    // Adiciona o ID da URL ao request para que o caso de uso tenha o ID completo
    request.id = id;

    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = UpdateOrgUnitUseCase::new(state.org_unit_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => {
            // Sucesso: retorna a unidade organizacional atualizada
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Unidade não encontrada
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // Dados inválidos
                DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                DomainError::Forbidden(_) => StatusCode::FORBIDDEN,
                DomainError::Conflict(_) => StatusCode::CONFLICT, // Conflito de versão (ETag)
                DomainError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DomainError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DomainError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
                DomainError::BusinessRuleViolation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            };
            Err((status, Json(json!({"error": err.to_string()}))))
        }
    }
}

// ============================================================================
// HANDLER: DELETE /v1/org-units/:id - DELETAR UNIDADE ORGANIZACIONAL
// ============================================================================
// Endpoint para remoção de unidades organizacionais existentes
// Retorna 204 No Content em caso de sucesso (padrão RESTful)

async fn delete_org_unit(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID da unidade organizacional a ser deletada
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = DeleteOrgUnitUseCase::new(state.org_unit_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(&id).await {
        Ok(_) => {
            // Sucesso: retorna 204 No Content (padrão RESTful para DELETE)
            Ok(StatusCode::NO_CONTENT)
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Unidade não encontrada
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // ID inválido
                DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                DomainError::Forbidden(_) => StatusCode::FORBIDDEN, // Sem permissão para deletar
                DomainError::Conflict(_) => StatusCode::CONFLICT,   // Conflito (ex: unidade em uso)
                DomainError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DomainError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DomainError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
                DomainError::BusinessRuleViolation(_) => StatusCode::UNPROCESSABLE_ENTITY, // Regra de negócio
            };
            Err((status, Json(json!({"error": err.to_string()}))))
        }
    }
}

// ============================================================================
// HANDLER: GET /v1/org-units/:id/hierarchy - HIERARQUIA DA UNIDADE ORGANIZACIONAL
// ============================================================================
// Endpoint para obter a hierarquia completa de uma unidade organizacional
// Retorna a unidade e todas as suas unidades filhas organizadas hierarquicamente

async fn get_org_unit_hierarchy(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID da unidade organizacional extraído da URL
) -> Result<Json<OrgUnitHierarchyResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetOrgUnitsUseCase::new(state.org_unit_repository.as_ref());

    // ===== VALIDAÇÃO DE UUID =====
    // Valida se o ID da unidade é um UUID válido antes de processar
    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            // UUID válido: converte para value object do domínio
            let org_unit_id = OrgUnitId(uuid);

            // ===== EXECUÇÃO DO CASO DE USO =====
            match use_case.execute_hierarchy(&org_unit_id).await {
                Ok(response) => {
                    // Sucesso: retorna hierarquia da unidade organizacional
                    Ok(Json(response))
                }
                Err(err) => {
                    // ===== MAPEAMENTO DE ERROS =====
                    let status = match err {
                        DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Unidade não encontrada
                        DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // ID inválido
                        DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                        DomainError::Forbidden(_) => StatusCode::FORBIDDEN,
                        DomainError::Conflict(_) => StatusCode::CONFLICT,
                        DomainError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                        DomainError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                        DomainError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
                        DomainError::BusinessRuleViolation(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    };
                    Err((status, Json(json!({"error": err.to_string()}))))
                }
            }
        }
        Err(_) => {
            // UUID inválido: retorna erro 400 Bad Request
            Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid UUID format"})),
            ))
        }
    }
}
