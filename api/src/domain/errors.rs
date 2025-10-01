use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum DomainError {
    #[error("Entity not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    ValidationError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),
}

impl From<sqlx::Error> for DomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DomainError::NotFound("Entity not found in database".to_string()),
            sqlx::Error::Database(db_err) => {
                if db_err.constraint().is_some() {
                    DomainError::Conflict(format!("Database constraint violation: {}", db_err.message()))
                } else {
                    DomainError::DatabaseError(db_err.message().to_string())
                }
            }
            _ => DomainError::DatabaseError(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for DomainError {
    fn from(err: serde_json::Error) -> Self {
        DomainError::ValidationError(format!("JSON serialization error: {}", err))
    }
}

impl From<uuid::Error> for DomainError {
    fn from(err: uuid::Error) -> Self {
        DomainError::ValidationError(format!("Invalid UUID: {}", err))
    }
}