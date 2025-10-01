use crate::application::dto::*;
use crate::application::use_cases::department::*;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::DepartmentId;
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
        .route("/v1/departments", get(get_departments).post(create_department))
        .route("/v1/departments/:id", get(get_department).patch(update_department).delete(delete_department))
        .route("/v1/departments/statistics", get(get_department_statistics))
        .route("/v1/departments/by-unit/:unit_id", get(get_departments_by_unit))
}

// GET /v1/departments
async fn get_departments(
    State(state): State<Arc<crate::AppState>>,
    Query(params): Query<DepartmentSearchRequest>,
) -> Result<Json<DepartmentSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetDepartmentsUseCase::new(state.department_repository.as_ref());
    
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

// GET /v1/departments/:id
async fn get_department(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DepartmentResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetDepartmentsUseCase::new(state.department_repository.as_ref());
    
    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            let department_id = DepartmentId(uuid);
            match use_case.execute_by_id(&department_id).await {
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

// POST /v1/departments
async fn create_department(
    State(state): State<Arc<crate::AppState>>,
    Json(request): Json<CreateDepartmentRequest>,
) -> Result<Json<DepartmentResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = CreateDepartmentUseCase::new(state.department_repository.as_ref());
    
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

// PATCH /v1/departments/:id
async fn update_department(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
    Json(mut request): Json<UpdateDepartmentRequest>,
) -> Result<Json<DepartmentResponse>, (StatusCode, Json<serde_json::Value>)> {
    request.id = id;
    
    let use_case = UpdateDepartmentUseCase::new(state.department_repository.as_ref());
    
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

// DELETE /v1/departments/:id
async fn delete_department(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let use_case = DeleteDepartmentUseCase::new(state.department_repository.as_ref());
    
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

// GET /v1/departments/statistics
async fn get_department_statistics(
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<DepartmentStatisticsResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetDepartmentStatisticsUseCase::new(state.department_repository.as_ref());
    
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

// GET /v1/departments/by-unit/:unit_id
async fn get_departments_by_unit(
    State(state): State<Arc<crate::AppState>>,
    Path(unit_id): Path<String>,
) -> Result<Json<DepartmentSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetDepartmentsUseCase::new(state.department_repository.as_ref());
    
    match Uuid::parse_str(&unit_id) {
        Ok(uuid) => {
            let org_unit_id = crate::domain::value_objects::OrgUnitId(uuid);
            match use_case.execute_by_unit(&org_unit_id).await {
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
