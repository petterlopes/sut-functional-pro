// ============================================================================
// IMPORTS E DEPENDÊNCIAS - CONTACT CONTROLLER
// ============================================================================
// Controller que implementa a camada de apresentação para operações de contatos
// Segue os princípios da Clean Architecture com separação clara de responsabilidades

// ===== CLEAN ARCHITECTURE IMPORTS =====
use crate::application::dto::*; // DTOs (Data Transfer Objects) para comunicação entre camadas
use crate::application::use_cases::contact::*; // Casos de uso da camada de aplicação
use crate::domain::value_objects::ContactId; // Value objects do domínio

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
// use serde_json::json; // Para criação de JSON dinâmico
use std::sync::Arc; // Para compartilhamento thread-safe do estado
                    // use uuid::Uuid; // Para validação de UUIDs

// ============================================================================
// CONFIGURAÇÃO DE ROTAS - REST API ENDPOINTS
// ============================================================================
// Define todas as rotas REST para operações de contatos
// Implementa o padrão RESTful com operações CRUD completas

pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        // ===== ROTAS DE COLECAO =====
        .route(
            "/v1/contacts",
            get(get_contacts) // GET /v1/contacts - Listar contatos com filtros
                .post(create_contact), // POST /v1/contacts - Criar novo contato
        )
        // ===== ROTAS DE RECURSO INDIVIDUAL =====
        .route(
            "/v1/contacts/{id}",
            get(get_contact) // GET /v1/contacts/{id} - Buscar contato por ID
                .patch(update_contact) // PATCH /v1/contacts/{id} - Atualizar contato
                .delete(delete_contact), // DELETE /v1/contacts/{id} - Deletar contato
        )
        // ===== ROTAS DE ESTATÍSTICAS =====
        .route(
            "/v1/contacts/statistics",
            get(get_contact_statistics), // GET /v1/contacts/statistics - Estatísticas de contatos
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
    // Cria instância do caso de uso injetando a dependência do repositório
    let use_case = GetContactsUseCase::new(state.contact_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(params).await {
        Ok(response) => {
            // Sucesso: retorna resposta JSON com os contatos encontrados
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
// HANDLER: GET /v1/contacts/:id - BUSCAR CONTATO POR ID
// ============================================================================
// Endpoint para buscar um contato específico pelo seu ID
// Inclui validação de formato UUID e tratamento de erros

async fn get_contact(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID do contato extraído da URL
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = GetContactsUseCase::new(state.contact_repository.as_ref());

    // ===== VALIDAÇÃO DE UUID USANDO UTILITÁRIO CENTRALIZADO =====
    // Usa função centralizada para validação de UUID
    let uuid = validate_uuid(&id)?;
    let contact_id = ContactId(uuid);

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute_by_id(&contact_id).await {
        Ok(response) => {
            // Sucesso: retorna o contato encontrado
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS USANDO UTILITÁRIO CENTRALIZADO =====
            Err(map_domain_error(&err))
        }
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
    // Cria instância do caso de uso para criação de contatos
    let use_case = CreateContactUseCase::new(state.contact_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => {
            // Sucesso: retorna o contato criado com status 201 (será definido pelo Axum)
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS USANDO UTILITÁRIO CENTRALIZADO =====
            Err(map_domain_error(&err))
        }
    }
}

// ============================================================================
// HANDLER: PATCH /v1/contacts/:id - ATUALIZAR CONTATO
// ============================================================================
// Endpoint para atualização parcial de contatos existentes
// Combina ID da URL com dados do JSON para atualização

async fn update_contact(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID do contato extraído da URL
    Json(mut request): Json<UpdateContactRequest>, // Dados de atualização em JSON
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)> {
    // ===== PREPARAÇÃO DOS DADOS =====
    // Adiciona o ID da URL ao request para que o caso de uso tenha o ID completo
    request.id = id;

    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = UpdateContactUseCase::new(state.contact_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(request).await {
        Ok(response) => {
            // Sucesso: retorna o contato atualizado
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS USANDO UTILITÁRIO CENTRALIZADO =====
            Err(map_domain_error(&err))
        }
    }
}

// ============================================================================
// HANDLER: DELETE /v1/contacts/:id - DELETAR CONTATO
// ============================================================================
// Endpoint para remoção de contatos existentes
// Retorna 204 No Content em caso de sucesso (padrão RESTful)

async fn delete_contact(
    State(state): State<Arc<crate::AppState>>, // Estado compartilhado da aplicação
    Path(id): Path<String>,                    // ID do contato a ser deletado
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // ===== INICIALIZAÇÃO DO CASO DE USO =====
    let use_case = DeleteContactUseCase::new(state.contact_repository.as_ref());

    // ===== EXECUÇÃO DO CASO DE USO =====
    match use_case.execute(&id).await {
        Ok(_) => {
            // Sucesso: retorna 204 No Content (padrão RESTful para DELETE)
            Ok(StatusCode::NO_CONTENT)
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS USANDO UTILITÁRIO CENTRALIZADO =====
            Err(map_domain_error(&err))
        }
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
        Ok(response) => {
            // Sucesso: retorna estatísticas agregadas
            Ok(Json(response))
        }
        Err(err) => {
            // ===== MAPEAMENTO DE ERROS USANDO UTILITÁRIO CENTRALIZADO =====
            Err(map_domain_error(&err))
        }
    }
}
