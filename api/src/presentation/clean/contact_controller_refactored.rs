// ============================================================================
// CONTACT CONTROLLER REFATORADO - VERSÃO OTIMIZADA
// ============================================================================
// Controller refatorado que usa utilitários centralizados para eliminar redundância
// Demonstra como aplicar DRY (Don't Repeat Yourself) e melhorar manutenibilidade

use crate::application::dto::*;
use crate::application::use_cases::contact::*;
use crate::domain::value_objects::ContactId;
use crate::presentation::{
    error_mapper::map_domain_error,
    validation::validate_uuid,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, patch, delete},
    Router,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

// ============================================================================
// CONFIGURAÇÃO DE ROTAS - REST API ENDPOINTS
// ============================================================================
// Define todas as rotas REST para operações de contatos
// Implementa o padrão RESTful com operações CRUD completas

pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        // ===== ROTAS DE COLECAO =====
        .route("/v1/contacts", 
            get(get_contacts)    // GET /v1/contacts - Listar contatos com filtros
            .post(create_contact) // POST /v1/contacts - Criar novo contato
        )
        // ===== ROTAS DE RECURSO INDIVIDUAL =====
        .route("/v1/contacts/:id", 
            get(get_contact)     // GET /v1/contacts/:id - Buscar contato por ID
            .patch(update_contact) // PATCH /v1/contacts/:id - Atualizar contato
            .delete(delete_contact) // DELETE /v1/contacts/:id - Deletar contato
        )
        // ===== ROTAS DE ESTATÍSTICAS =====
        .route("/v1/contacts/statistics", 
            get(get_contact_statistics) // GET /v1/contacts/statistics - Estatísticas de contatos
        )
}

// ============================================================================
// HANDLER: GET /v1/contacts - LISTAR CONTATOS COM FILTROS
// ============================================================================
// Endpoint para buscar contatos com suporte a filtros, paginação e ordenação
// Implementa o padrão de busca RESTful com parâmetros de query

async fn get_contacts(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Query(params): Query<ContactSearchRequest>, // Parâmetros de query (filtros, paginação)
) -> Result<Json<ContactSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetContactsUseCase::new(state.contact_repository.as_ref());
    
    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(params).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(map_domain_error(&err))
    }
}

// ============================================================================
// HANDLER: GET /v1/contacts/:id - BUSCAR CONTATO POR ID
// ============================================================================
// Endpoint para buscar um contato específico pelo seu ID
// Inclui validação de formato UUID e tratamento de erros

async fn get_contact(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>, // ID do contato extraído da URL
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)> {
    
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetContactsUseCase::new(state.contact_repository.as_ref());
    
    // ===== VALIDAÇÃO DE UUID =====
    let uuid = validate_uuid(&id)?;
    let contact_id = ContactId(uuid);
    
    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute_by_id(&contact_id).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(map_domain_error(&err))
    }
}

// ============================================================================
// HANDLER: POST /v1/contacts - CRIAR NOVO CONTATO
// ============================================================================
// Endpoint para criação de novos contatos
// Recebe dados via JSON e valida através do caso de uso

async fn create_contact(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Json(request): Json<CreateContactRequest>, // Dados do contato em formato JSON
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)> {
    
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = CreateContactUseCase::new(state.contact_repository.as_ref());
    
    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(map_domain_error(&err))
    }
}

// ============================================================================
// HANDLER: PATCH /v1/contacts/:id - ATUALIZAR CONTATO
// ============================================================================
// Endpoint para atualização parcial de contatos existentes
// Combina ID da URL com dados do JSON para atualização

async fn update_contact(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>, // ID do contato extraído da URL
    Json(mut request): Json<UpdateContactRequest>, // Dados de atualização em JSON
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)> {
    
    // ===== PREPARAÇÃO DOS DADOS =====
    request.id = id;
    
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = UpdateContactUseCase::new(state.contact_repository.as_ref());
    
    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(map_domain_error(&err))
    }
}

// ============================================================================
// HANDLER: DELETE /v1/contacts/:id - DELETAR CONTATO
// ============================================================================
// Endpoint para remoção de contatos existentes
// Retorna 204 No Content em caso de sucesso (padrão RESTful)

async fn delete_contact(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>, // ID do contato a ser deletado
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = DeleteContactUseCase::new(state.contact_repository.as_ref());
    
    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(&id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(map_domain_error(&err))
    }
}

// ============================================================================
// HANDLER: GET /v1/contacts/statistics - ESTATÍSTICAS DE CONTATOS
// ============================================================================
// Endpoint para obter estatísticas agregadas dos contatos
// Útil para dashboards e relatórios

async fn get_contact_statistics(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
) -> Result<Json<ContactStatisticsResponse>, (StatusCode, Json<serde_json::Value>)> {
    
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetContactStatisticsUseCase::new(state.contact_repository.as_ref());
    
    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute().await {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(map_domain_error(&err))
    }
}
