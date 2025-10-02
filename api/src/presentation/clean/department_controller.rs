// ============================================================================
// IMPORTS E DEPENDÊNCIAS - DEPARTMENT CONTROLLER
// ============================================================================
// Controller que implementa a camada de apresentação para operações de departamentos
// Segue os princípios da Clean Architecture com separação clara de responsabilidades

// ===== CLEAN ARCHITECTURE IMPORTS =====
use crate::application::dto::*; // DTOs (Data Transfer Objects) para comunicação entre camadas
use crate::application::use_cases::department::*; // Casos de uso da camada de aplicação
use crate::domain::{value_objects::DepartmentId, DomainError}; // Value objects do domínio

// ===== PRESENTATION UTILITIES =====
use crate::presentation::{
    error_mapper::map_domain_error, // Mapeamento centralizado de erros
    validation::validate_uuid,      // Validação de UUID
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
// Define todas as rotas REST para operações de departamentos
// Implementa o padrão RESTful com operações CRUD completas

pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        // ===== ROTAS DE COLECAO =====
        .route(
            "/v1/departments",
            get(get_departments) // GET /v1/departments - Listar departamentos com filtros
                .post(create_department), // POST /v1/departments - Criar novo departamento
        )
        // ===== ROTAS DE RECURSO INDIVIDUAL =====
        .route(
            "/v1/departments/{id}",
            get(get_department) // GET /v1/departments/{id} - Buscar departamento por ID
                .patch(update_department) // PATCH /v1/departments/{id} - Atualizar departamento
                .delete(delete_department), // DELETE /v1/departments/{id} - Deletar departamento
        )
        // ===== ROTAS DE ESTATÍSTICAS =====
        .route(
            "/v1/departments/statistics",
            get(get_department_statistics), // GET /v1/departments/statistics - Estatísticas de departamentos
        )
        // ===== ROTAS DE RELACIONAMENTO =====
        .route(
            "/v1/departments/by-unit/{unit_id}",
            get(get_departments_by_unit), // GET /v1/departments/by-unit/{unit_id} - Departamentos por unidade
        )
}

// ============================================================================
// HANDLER: GET /v1/departments - LISTAR DEPARTAMENTOS COM FILTROS
// ============================================================================
// Endpoint para buscar departamentos com suporte a filtros, paginação e ordenação
// Implementa o padrão de busca RESTful com parâmetros de query

async fn get_departments(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Query(params): Query<DepartmentSearchRequest>, // Parâmetros de query (filtros, paginação)
) -> Result<Json<DepartmentSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    // Cria instância do caso de uso injetando a dependência do repositório
    let use_case = GetDepartmentsUseCase::new(state.department_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(params).await {
        Ok(response) => {
            // Sucesso: retorna resposta JSON com os departamentos encontrados
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
// HANDLER: GET /v1/departments/:id - BUSCAR DEPARTAMENTO POR ID
// ============================================================================
// Endpoint para buscar um departamento específico pelo seu ID
// Inclui validação de formato UUID e tratamento de erros

async fn get_department(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID do departamento extraído da URL
) -> Result<Json<DepartmentResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetDepartmentsUseCase::new(state.department_repository.as_ref());

    // ===== VALIDAÇÃO DE UUID USANDO UTILITÁRIO CENTRALIZADO =====
    // Usa função centralizada para validação de UUID
    let uuid = validate_uuid(&id)?;
    let department_id = DepartmentId(uuid);

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute_by_id(&department_id).await {
        Ok(response) => {
            // Sucesso: retorna o departamento encontrado
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS USANDO UTILITÁRIO CENTRALIZADO =====
            Err(map_domain_error(&err))
        }
    }
}

// ============================================================================
// HANDLER: POST /v1/departments - CRIAR NOVO DEPARTAMENTO
// ============================================================================
// Endpoint para criação de novos departamentos
// Recebe dados via JSON e valida através do caso de uso

async fn create_department(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Json(request): Json<CreateDepartmentRequest>, // Dados do departamento em formato JSON
) -> Result<Json<DepartmentResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    // Cria instância do caso de uso para criação de departamentos
    let use_case = CreateDepartmentUseCase::new(state.department_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => {
            // Sucesso: retorna o departamento criado com status 201 (será definido pelo Axum)
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
// HANDLER: PATCH /v1/departments/:id - ATUALIZAR DEPARTAMENTO
// ============================================================================
// Endpoint para atualização parcial de departamentos existentes
// Combina ID da URL com dados do JSON para atualização

async fn update_department(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID do departamento extraído da URL
    Json(mut request): Json<UpdateDepartmentRequest>, // Dados de atualização em JSON
) -> Result<Json<DepartmentResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== PREPARAÇÃO DOS DADOS =====
    // Adiciona o ID da URL ao request para que o caso de uso tenha o ID completo
    request.id = id;

    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = UpdateDepartmentUseCase::new(state.department_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => {
            // Sucesso: retorna o departamento atualizado
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Departamento não encontrado
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
// HANDLER: DELETE /v1/departments/:id - DELETAR DEPARTAMENTO
// ============================================================================
// Endpoint para remoção de departamentos existentes
// Retorna 204 No Content em caso de sucesso (padrão RESTful)

async fn delete_department(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID do departamento a ser deletado
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = DeleteDepartmentUseCase::new(state.department_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(&id).await {
        Ok(_) => {
            // Sucesso: retorna 204 No Content (padrão RESTful para DELETE)
            Ok(StatusCode::NO_CONTENT)
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Departamento não encontrado
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // ID inválido
                DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                DomainError::Forbidden(_) => StatusCode::FORBIDDEN, // Sem permissão para deletar
                DomainError::Conflict(_) => StatusCode::CONFLICT, // Conflito (ex: departamento em uso)
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
// HANDLER: GET /v1/departments/statistics - ESTATÍSTICAS DE DEPARTAMENTOS
// ============================================================================
// Endpoint para obter estatísticas agregadas dos departamentos
// Útil para dashboards e relatórios

async fn get_department_statistics(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
) -> Result<Json<DepartmentStatisticsResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetDepartmentStatisticsUseCase::new(state.department_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute().await {
        Ok(response) => {
            // Sucesso: retorna estatísticas agregadas
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
                DomainError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR, // Erro ao calcular estatísticas
                DomainError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
                DomainError::BusinessRuleViolation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            };
            Err((status, Json(json!({"error": err.to_string()}))))
        }
    }
}

// ============================================================================
// HANDLER: GET /v1/departments/by-unit/:unit_id - DEPARTAMENTOS POR UNIDADE
// ============================================================================
// Endpoint para buscar departamentos de uma unidade organizacional específica
// Útil para navegação hierárquica e relatórios por unidade

async fn get_departments_by_unit(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(unit_id): Path<String>,               // ID da unidade organizacional extraído da URL
) -> Result<Json<DepartmentSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetDepartmentsUseCase::new(state.department_repository.as_ref());

    // ===== VALIDAÇÃO DE UUID =====
    // Valida se o ID da unidade é um UUID válido antes de processar
    match Uuid::parse_str(&unit_id) {
        Ok(uuid) => {
            // UUID válido: converte para value object do domínio
            let org_unit_id = crate::domain::value_objects::OrgUnitId(uuid);

            // ===== EXECUÇÃO DO CASO DE USO =====
            match use_case.execute_by_unit(&org_unit_id).await {
                Ok(response) => {
                    // Sucesso: retorna lista de departamentos da unidade
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
