use crate::application::dto::*;
use crate::application::use_cases::org_unit::*;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::OrgUnitId;
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
        .route("/v1/org-units", get(get_org_units).post(create_org_unit))
        .route("/v1/org-units/:id", get(get_org_unit).patch(update_org_unit).delete(delete_org_unit))
        .route("/v1/org-units/:id/hierarchy", get(get_org_unit_hierarchy))
}

// GET /v1/org-units
async fn get_org_units(
    State(state): State<Arc<crate::AppState>>,
    Query(params): Query<OrgUnitSearchRequest>,
) -> Result<Json<OrgUnitSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetOrgUnitsUseCase::new(state.org_unit_repository.as_ref());
    
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

// GET /v1/org-units/:id
async fn get_org_unit(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<Json<OrgUnitResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetOrgUnitsUseCase::new(state.org_unit_repository.as_ref());
    
    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            let org_unit_id = OrgUnitId(uuid);
            match use_case.execute_by_id(&org_unit_id).await {
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

// POST /v1/org-units
async fn create_org_unit(
    State(state): State<Arc<crate::AppState>>,
    Json(request): Json<CreateOrgUnitRequest>,
) -> Result<Json<OrgUnitResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = CreateOrgUnitUseCase::new(state.org_unit_repository.as_ref());
    
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

// PATCH /v1/org-units/:id
async fn update_org_unit(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
    Json(mut request): Json<UpdateOrgUnitRequest>,
) -> Result<Json<OrgUnitResponse>, (StatusCode, Json<serde_json::Value>)> {
    request.id = id;
    
    let use_case = UpdateOrgUnitUseCase::new(state.org_unit_repository.as_ref());
    
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

// DELETE /v1/org-units/:id
async fn delete_org_unit(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let use_case = DeleteOrgUnitUseCase::new(state.org_unit_repository.as_ref());
    
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

// GET /v1/org-units/:id/hierarchy
async fn get_org_unit_hierarchy(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<Json<OrgUnitHierarchyResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetOrgUnitsUseCase::new(state.org_unit_repository.as_ref());
    
    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            let org_unit_id = OrgUnitId(uuid);
            match use_case.execute_hierarchy(&org_unit_id).await {
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
