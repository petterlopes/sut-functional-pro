use crate::application::dto::*;
use crate::application::use_cases::contact::*;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::ContactId;
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

pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        .route("/v1/contacts", get(get_contacts).post(create_contact))
        .route("/v1/contacts/:id", get(get_contact).patch(update_contact).delete(delete_contact))
        .route("/v1/contacts/statistics", get(get_contact_statistics))
}


// GET /v1/contacts-clean
async fn get_contacts(
    State(state): State<Arc<crate::AppState>>,
    Query(params): Query<ContactSearchRequest>,
) -> Result<Json<ContactSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetContactsUseCase::new(state.contact_repository.as_ref());
    
    match use_case.execute(params).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => {
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

// GET /v1/contacts-clean/:id
async fn get_contact(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetContactsUseCase::new(state.contact_repository.as_ref());
    
    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            let contact_id = ContactId(uuid);
            match use_case.execute_by_id(&contact_id).await {
                Ok(response) => Ok(Json(response)),
                Err(err) => {
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
        },
        Err(_) => Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid UUID format"})))),
    }
}

// POST /v1/contacts-clean
async fn create_contact(
    State(state): State<Arc<crate::AppState>>,
    Json(request): Json<CreateContactRequest>,
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = CreateContactUseCase::new(state.contact_repository.as_ref());
    
    match use_case.execute(request).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => {
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

// PATCH /v1/contacts-clean/:id
async fn update_contact(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
    Json(mut request): Json<UpdateContactRequest>,
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)> {
    request.id = id;
    
    let use_case = UpdateContactUseCase::new(state.contact_repository.as_ref());
    
    match use_case.execute(request).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => {
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

// DELETE /v1/contacts-clean/:id
async fn delete_contact(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let use_case = DeleteContactUseCase::new(state.contact_repository.as_ref());
    
    match use_case.execute(&id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
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

// GET /v1/contacts-clean/statistics
async fn get_contact_statistics(
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<ContactStatisticsResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetContactStatisticsUseCase::new(state.contact_repository.as_ref());
    
    match use_case.execute().await {
        Ok(response) => Ok(Json(response)),
        Err(err) => {
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