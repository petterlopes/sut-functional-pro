// ============================================================================
// IMPORTS E DEPENDÊNCIAS - USER CONTROLLER
// ============================================================================
// Controller que implementa a camada de apresentação para operações de usuários
// Segue os princípios da Clean Architecture com separação clara de responsabilidades

// ===== CLEAN ARCHITECTURE IMPORTS =====
use crate::application::dto::*; // DTOs (Data Transfer Objects) para comunicação entre camadas
use crate::application::use_cases::user::*; // Casos de uso da camada de aplicação
use crate::domain::{value_objects::UserId, DomainError}; // Value objects do domínio

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
// Define todas as rotas REST para operações de usuários
// Implementa o padrão RESTful com operações CRUD completas e buscas específicas

pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        // ===== ROTAS DE COLECAO =====
        .route(
            "/v1/users",
            get(get_users) // GET /v1/users - Listar usuários com filtros
                .post(create_user), // POST /v1/users - Criar novo usuário
        )
        // ===== ROTAS DE RECURSO INDIVIDUAL =====
        .route(
            "/v1/users/:id",
            get(get_user) // GET /v1/users/:id - Buscar usuário por ID
                .patch(update_user) // PATCH /v1/users/:id - Atualizar usuário
                .delete(delete_user), // DELETE /v1/users/:id - Deletar usuário
        )
        // ===== ROTAS DE BUSCA ESPECÍFICA =====
        .route(
            "/v1/users/by-username/:username",
            get(get_user_by_username), // GET /v1/users/by-username/:username - Buscar por username
        )
        .route(
            "/v1/users/by-email/:email",
            get(get_user_by_email), // GET /v1/users/by-email/:email - Buscar por email
        )
        .route(
            "/v1/users/by-role/:role",
            get(get_users_by_role), // GET /v1/users/by-role/:role - Buscar por role
        )
}

// ============================================================================
// HANDLER: GET /v1/users - LISTAR USUÁRIOS COM FILTROS
// ============================================================================
// Endpoint para buscar usuários com suporte a filtros, paginação e ordenação
// Implementa o padrão de busca RESTful com parâmetros de query

async fn get_users(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Query(params): Query<UserSearchRequest>,   // Parâmetros de query (filtros, paginação)
) -> Result<Json<UserSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    // Cria instância do caso de uso injetando a dependência do repositório
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(params).await {
        Ok(response) => {
            // Sucesso: retorna resposta JSON com os usuários encontrados
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
// HANDLER: GET /v1/users/:id - BUSCAR USUÁRIO POR ID
// ============================================================================
// Endpoint para buscar um usuário específico pelo seu ID
// Inclui validação de formato UUID e tratamento de erros

async fn get_user(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID do usuário extraído da URL
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());

    // ===== VALIDAÇÃO DE UUID =====
    // Valida se o ID fornecido é um UUID válido antes de processar
    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            // UUID válido: converte para value object do domínio
            let user_id = UserId(uuid);

            // ===== EXECUÇÃO DO CASO DE USO =====
            match use_case.execute_by_id(&user_id).await {
                Ok(response) => {
                    // Sucesso: retorna o usuário encontrado (sem senha)
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
// HANDLER: POST /v1/users - CRIAR NOVO USUÁRIO
// ============================================================================
// Endpoint para criação de novos usuários
// Recebe dados via JSON e valida através do caso de uso

async fn create_user(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Json(request): Json<CreateUserRequest>,    // Dados do usuário em formato JSON
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    // Cria instância do caso de uso para criação de usuários
    let use_case = CreateUserUseCase::new(state.user_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => {
            // Sucesso: retorna o usuário criado com status 201 (será definido pelo Axum)
            // Nota: A senha não é incluída na resposta por segurança
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND,
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // Dados inválidos
                DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                DomainError::Forbidden(_) => StatusCode::FORBIDDEN,
                DomainError::Conflict(_) => StatusCode::CONFLICT, // Conflito (ex: username/email duplicado)
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
// HANDLER: PATCH /v1/users/:id - ATUALIZAR USUÁRIO
// ============================================================================
// Endpoint para atualização parcial de usuários existentes
// Combina ID da URL com dados do JSON para atualização

async fn update_user(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID do usuário extraído da URL
    Json(mut request): Json<UpdateUserRequest>, // Dados de atualização em JSON
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== PREPARAÇÃO DOS DADOS =====
    // Adiciona o ID da URL ao request para que o caso de uso tenha o ID completo
    request.id = id;

    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = UpdateUserUseCase::new(state.user_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => {
            // Sucesso: retorna o usuário atualizado (sem senha)
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Usuário não encontrado
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // Dados inválidos
                DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                DomainError::Forbidden(_) => StatusCode::FORBIDDEN,
                DomainError::Conflict(_) => StatusCode::CONFLICT, // Conflito (ex: username/email duplicado)
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
// HANDLER: DELETE /v1/users/:id - DELETAR USUÁRIO
// ============================================================================
// Endpoint para remoção de usuários existentes
// Retorna 204 No Content em caso de sucesso (padrão RESTful)

async fn delete_user(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID do usuário a ser deletado
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = DeleteUserUseCase::new(state.user_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(&id).await {
        Ok(_) => {
            // Sucesso: retorna 204 No Content (padrão RESTful para DELETE)
            Ok(StatusCode::NO_CONTENT)
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Usuário não encontrado
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // ID inválido
                DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                DomainError::Forbidden(_) => StatusCode::FORBIDDEN, // Sem permissão para deletar
                DomainError::Conflict(_) => StatusCode::CONFLICT,   // Conflito (ex: usuário em uso)
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
// HANDLER: GET /v1/users/by-username/:username - BUSCAR USUÁRIO POR USERNAME
// ============================================================================
// Endpoint para buscar um usuário específico pelo seu username
// Útil para autenticação e validação de usernames únicos

async fn get_user_by_username(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(username): Path<String>,              // Username do usuário extraído da URL
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute_by_username(&username).await {
        Ok(response) => {
            // Sucesso: retorna o usuário encontrado (sem senha)
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Username não encontrado
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // Username inválido
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

// ============================================================================
// HANDLER: GET /v1/users/by-email/:email - BUSCAR USUÁRIO POR EMAIL
// ============================================================================
// Endpoint para buscar um usuário específico pelo seu email
// Útil para autenticação e validação de emails únicos

async fn get_user_by_email(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(email): Path<String>,                 // Email do usuário extraído da URL
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute_by_email(&email).await {
        Ok(response) => {
            // Sucesso: retorna o usuário encontrado (sem senha)
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Email não encontrado
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // Email inválido
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

// ============================================================================
// HANDLER: GET /v1/users/by-role/:role - BUSCAR USUÁRIOS POR ROLE
// ============================================================================
// Endpoint para buscar usuários que possuem uma role específica
// Útil para gerenciamento de permissões e relatórios por role

async fn get_users_by_role(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(role): Path<String>,                  // Role dos usuários extraída da URL
) -> Result<Json<UserSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute_by_role(&role).await {
        Ok(response) => {
            // Sucesso: retorna lista de usuários com a role especificada
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS =====
            let status = match err {
                DomainError::NotFound(_) => StatusCode::NOT_FOUND, // Role não encontrada
                DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // Role inválida
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
