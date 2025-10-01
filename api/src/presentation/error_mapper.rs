// ============================================================================
// ERROR MAPPER - MAPEAMENTO DE ERROS DE DOMÍNIO PARA HTTP
// ============================================================================
// Módulo que centraliza o mapeamento de erros de domínio para códigos HTTP
// Elimina redundância e garante consistência em todos os controllers

use crate::domain::errors::DomainError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Tipo de resposta de erro padronizada
pub type ErrorResponse = (StatusCode, Json<serde_json::Value>);

/// Mapeia erros de domínio para códigos de status HTTP apropriados
///
/// # Argumentos
/// * `error` - Erro de domínio a ser mapeado
///
/// # Retorna
/// Tupla contendo o código de status HTTP e a resposta JSON de erro
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::error_mapper::map_domain_error;
///
/// let domain_error = DomainError::NotFound("Resource not found".to_string());
/// let (status, response) = map_domain_error(&domain_error);
/// // status = StatusCode::NOT_FOUND
/// // response = Json({"error": "Resource not found"})
/// ```
pub fn map_domain_error(error: &DomainError) -> ErrorResponse {
    let status = match error {
        DomainError::NotFound(_) => StatusCode::NOT_FOUND, // 404 - Recurso não encontrado
        DomainError::ValidationError(_) => StatusCode::BAD_REQUEST, // 400 - Dados inválidos
        DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED, // 401 - Não autenticado
        DomainError::Forbidden(_) => StatusCode::FORBIDDEN, // 403 - Sem permissão
        DomainError::Conflict(_) => StatusCode::CONFLICT,  // 409 - Conflito de estado
        DomainError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500 - Erro interno
        DomainError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500 - Erro de BD
        DomainError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY, // 502 - Erro de serviço externo
        DomainError::BusinessRuleViolation(_) => StatusCode::UNPROCESSABLE_ENTITY, // 422 - Regra de negócio
    };

    (status, Json(json!({"error": error.to_string()})))
}

/// Cria uma resposta de erro para UUID inválido
///
/// # Retorna
/// Resposta padronizada para erro de UUID inválido
pub fn invalid_uuid_error() -> ErrorResponse {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({"error": "Invalid UUID format"})),
    )
}

/// Cria uma resposta de erro customizada
///
/// # Argumentos
/// * `status` - Código de status HTTP
/// * `message` - Mensagem de erro
///
/// # Retorna
/// Resposta de erro customizada
pub fn custom_error(status: StatusCode, message: &str) -> ErrorResponse {
    (status, Json(json!({"error": message})))
}

/// Trait para facilitar o mapeamento de erros em handlers
pub trait IntoErrorResponse {
    fn into_error_response(self) -> ErrorResponse;
}

impl IntoErrorResponse for DomainError {
    fn into_error_response(self) -> ErrorResponse {
        map_domain_error(&self)
    }
}

/// Macro para simplificar o tratamento de erros em handlers
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::error_mapper::handle_domain_result;
///
/// let result: Result<Json<Response>, DomainError> = use_case.execute().await;
/// handle_domain_result!(result)
/// ```
#[macro_export]
macro_rules! handle_domain_result {
    ($result:expr) => {
        match $result {
            Ok(response) => Ok(response),
            Err(err) => Err($crate::presentation::error_mapper::map_domain_error(&err)),
        }
    };
}

/// Macro para simplificar o tratamento de resultados com mapeamento automático
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::error_mapper::handle_result;
///
/// let result: Result<Response, DomainError> = use_case.execute().await;
/// handle_result!(result, |response| Json(response))
/// ```
#[macro_export]
macro_rules! handle_result {
    ($result:expr, $mapper:expr) => {
        match $result {
            Ok(data) => Ok($mapper(data)),
            Err(err) => Err($crate::presentation::error_mapper::map_domain_error(&err)),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_domain_error_not_found() {
        let error = DomainError::NotFound("Resource not found".to_string());
        let (status, response) = map_domain_error(&error);

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(response.0["error"], "Resource not found");
    }

    #[test]
    fn test_map_domain_error_validation() {
        let error = DomainError::ValidationError("Invalid input".to_string());
        let (status, response) = map_domain_error(&error);

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.0["error"], "Invalid input");
    }

    #[test]
    fn test_invalid_uuid_error() {
        let (status, response) = invalid_uuid_error();

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.0["error"], "Invalid UUID format");
    }

    #[test]
    fn test_custom_error() {
        let (status, response) = custom_error(StatusCode::IM_A_TEAPOT, "Custom error");

        assert_eq!(status, StatusCode::IM_A_TEAPOT);
        assert_eq!(response.0["error"], "Custom error");
    }
}
