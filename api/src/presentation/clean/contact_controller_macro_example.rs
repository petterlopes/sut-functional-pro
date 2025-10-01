// ============================================================================
// CONTACT CONTROLLER COM MACROS - EXEMPLO DE USO DAS MACROS
// ============================================================================
// Exemplo de como usar as macros para criar handlers extremamente concisos
// Demonstra o poder das macros para eliminar redundância

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
use std::sync::Arc;

// ============================================================================
// CONFIGURAÇÃO DE ROTAS - REST API ENDPOINTS
// ============================================================================

pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        .route("/v1/contacts", 
            get(get_contacts)
            .post(create_contact)
        )
        .route("/v1/contacts/:id", 
            get(get_contact)
            .patch(update_contact)
            .delete(delete_contact)
        )
        .route("/v1/contacts/statistics", 
            get(get_contact_statistics)
        )
}

// ============================================================================
// HANDLERS USANDO MACROS - VERSÃO ULTRA CONCISA
// ============================================================================

// Handler de listagem usando macro
list_handler!(
    GetContactsUseCase,
    ContactSearchRequest,
    ContactSearchResponse,
    contact_repository
);

// Handler de busca por ID usando macro
get_by_id_handler!(
    GetContactsUseCase,
    ContactResponse,
    contact_repository,
    ContactId
);

// Handler de criação usando macro
create_handler!(
    CreateContactUseCase,
    CreateContactRequest,
    ContactResponse,
    contact_repository
);

// Handler de atualização usando macro
update_handler!(
    UpdateContactUseCase,
    UpdateContactRequest,
    ContactResponse,
    contact_repository
);

// Handler de exclusão usando macro
delete_handler!(
    DeleteContactUseCase,
    contact_repository
);

// Handler de estatísticas usando macro
statistics_handler!(
    GetContactStatisticsUseCase,
    ContactStatisticsResponse,
    contact_repository
);
