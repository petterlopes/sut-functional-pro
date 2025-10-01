use crate::application::dto::*;
use crate::application::use_cases::user::*;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::UserId;
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
        .route("/v1/users", get(get_users).post(create_user))
        .route("/v1/users/:id", get(get_user).patch(update_user).delete(delete_user))
        .route("/v1/users/by-username/:username", get(get_user_by_username))
        .route("/v1/users/by-email/:email", get(get_user_by_email))
        .route("/v1/users/by-role/:role", get(get_users_by_role))
}

// GET /v1/users
async fn get_users(
    State(state): State<Arc<crate::AppState>>,
    Query(params): Query<UserSearchRequest>,
) -> Result<Json<UserSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());
    
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

// GET /v1/users/:id
async fn get_user(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());
    
    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            let user_id = UserId(uuid);
            match use_case.execute_by_id(&user_id).await {
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

// POST /v1/users
async fn create_user(
    State(state): State<Arc<crate::AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = CreateUserUseCase::new(state.user_repository.as_ref());
    
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

// PATCH /v1/users/:id
async fn update_user(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
    Json(mut request): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    request.id = id;
    
    let use_case = UpdateUserUseCase::new(state.user_repository.as_ref());
    
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

// DELETE /v1/users/:id
async fn delete_user(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let use_case = DeleteUserUseCase::new(state.user_repository.as_ref());
    
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

// GET /v1/users/by-username/:username
async fn get_user_by_username(
    State(state): State<Arc<crate::AppState>>,
    Path(username): Path<String>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());
    
    match use_case.execute_by_username(&username).await {
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

// GET /v1/users/by-email/:email
async fn get_user_by_email(
    State(state): State<Arc<crate::AppState>>,
    Path(email): Path<String>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());
    
    match use_case.execute_by_email(&email).await {
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

// GET /v1/users/by-role/:role
async fn get_users_by_role(
    State(state): State<Arc<crate::AppState>>,
    Path(role): Path<String>,
) -> Result<Json<UserSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetUsersUseCase::new(state.user_repository.as_ref());
    
    match use_case.execute_by_role(&role).await {
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
