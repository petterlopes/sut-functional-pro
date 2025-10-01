// ============================================================================
// RESPONSE HELPERS - HELPERS PARA RESPOSTAS HTTP
// ============================================================================
// Módulo que centraliza helpers para criação de respostas HTTP padronizadas
// Elimina redundância e garante consistência

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Trait para facilitar a criação de respostas JSON
pub trait IntoJsonResponse {
    fn into_json_response(self) -> Json<Self>
    where
        Self: Serialize + Sized;
}

impl<T> IntoJsonResponse for T
where
    T: Serialize,
{
    fn into_json_response(self) -> Json<Self> {
        Json(self)
    }
}

/// Cria uma resposta de sucesso com status 200
///
/// # Argumentos
/// * `data` - Dados a serem serializados
///
/// # Retorna
/// Resposta JSON com status 200
pub fn ok_response<T>(data: T) -> (StatusCode, Json<T>)
where
    T: Serialize,
{
    (StatusCode::OK, Json(data))
}

/// Cria uma resposta de sucesso com status 201 (Created)
///
/// # Argumentos
/// * `data` - Dados a serem serializados
///
/// # Retorna
/// Resposta JSON com status 201
pub fn created_response<T>(data: T) -> (StatusCode, Json<T>)
where
    T: Serialize,
{
    (StatusCode::CREATED, Json(data))
}

/// Cria uma resposta de sucesso com status 204 (No Content)
///
/// # Retorna
/// Resposta vazia com status 204
pub fn no_content_response() -> StatusCode {
    StatusCode::NO_CONTENT
}

/// Cria uma resposta de erro com status 400 (Bad Request)
///
/// # Argumentos
/// * `message` - Mensagem de erro
///
/// # Retorna
/// Resposta de erro com status 400
pub fn bad_request_response(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"error": message})),
    )
}

/// Cria uma resposta de erro com status 404 (Not Found)
///
/// # Argumentos
/// * `message` - Mensagem de erro
///
/// # Retorna
/// Resposta de erro com status 404
pub fn not_found_response(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": message})),
    )
}

/// Cria uma resposta de erro com status 409 (Conflict)
///
/// # Argumentos
/// * `message` - Mensagem de erro
///
/// # Retorna
/// Resposta de erro com status 409
pub fn conflict_response(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::CONFLICT,
        Json(serde_json::json!({"error": message})),
    )
}

/// Cria uma resposta de erro com status 422 (Unprocessable Entity)
///
/// # Argumentos
/// * `message` - Mensagem de erro
///
/// # Retorna
/// Resposta de erro com status 422
pub fn unprocessable_entity_response(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(serde_json::json!({"error": message})),
    )
}

/// Cria uma resposta de erro com status 500 (Internal Server Error)
///
/// # Argumentos
/// * `message` - Mensagem de erro
///
/// # Retorna
/// Resposta de erro com status 500
pub fn internal_server_error_response(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": message})),
    )
}

/// Macro para simplificar a criação de respostas de sucesso
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::response_helpers::success_response;
///
/// let response = success_response!(data, 201); // Created
/// let response = success_response!(data); // Default 200
/// ```
#[macro_export]
macro_rules! success_response {
    ($data:expr, $status:expr) => {
        ($status, $crate::presentation::response_helpers::Json($data))
    };
    ($data:expr) => {
        $crate::presentation::response_helpers::ok_response($data)
    };
}

/// Macro para simplificar a criação de respostas de erro
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::response_helpers::error_response;
///
/// let response = error_response!(400, "Bad request");
/// let response = error_response!(404, "Not found");
/// ```
#[macro_export]
macro_rules! error_response {
    ($status:expr, $message:expr) => {
        (
            $status,
            $crate::presentation::response_helpers::Json(serde_json::json!({"error": $message}))
        )
    };
}

/// Trait para facilitar a conversão de Result em respostas HTTP
pub trait IntoHttpResponse<T> {
    fn into_http_response(
        self,
    ) -> Result<(StatusCode, Json<T>), (StatusCode, Json<serde_json::Value>)>
    where
        T: Serialize;
}

impl<T, E> IntoHttpResponse<T> for Result<T, E>
where
    T: Serialize,
    E: std::fmt::Display,
{
    fn into_http_response(
        self,
    ) -> Result<(StatusCode, Json<T>), (StatusCode, Json<serde_json::Value>)> {
        match self {
            Ok(data) => Ok(ok_response(data)),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": err.to_string()})),
            )),
        }
    }
}

/// Helper para criar respostas paginadas
///
/// # Argumentos
/// * `items` - Lista de itens
/// * `total` - Total de itens disponíveis
/// * `page` - Página atual
/// * `per_page` - Itens por página
///
/// # Retorna
/// Resposta JSON com metadados de paginação
pub fn paginated_response<T>(
    items: Vec<T>,
    total: i64,
    page: i64,
    per_page: i64,
) -> (StatusCode, Json<serde_json::Value>)
where
    T: Serialize,
{
    let total_pages = (total as f64 / per_page as f64).ceil() as i64;

    let response = serde_json::json!({
        "data": items,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": total_pages,
            "has_next": page < total_pages,
            "has_prev": page > 1
        }
    });

    ok_response(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_response() {
        let data = serde_json::json!({"message": "success"});
        let (status, response) = ok_response(data);

        assert_eq!(status, StatusCode::OK);
        assert_eq!(response.0["message"], "success");
    }

    #[test]
    fn test_created_response() {
        let data = serde_json::json!({"id": 1});
        let (status, response) = created_response(data);

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(response.0["id"], 1);
    }

    #[test]
    fn test_no_content_response() {
        let status = no_content_response();
        assert_eq!(status, StatusCode::NO_CONTENT);
    }

    #[test]
    fn test_bad_request_response() {
        let (status, response) = bad_request_response("Invalid input");

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.0["error"], "Invalid input");
    }

    #[test]
    fn test_not_found_response() {
        let (status, response) = not_found_response("Resource not found");

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(response.0["error"], "Resource not found");
    }

    #[test]
    fn test_paginated_response() {
        let items = vec![1, 2, 3];
        let (status, response) = paginated_response(items, 10, 1, 3);

        assert_eq!(status, StatusCode::OK);
        assert_eq!(response.0["data"], vec![1, 2, 3]);
        assert_eq!(response.0["pagination"]["total"], 10);
        assert_eq!(response.0["pagination"]["page"], 1);
        assert_eq!(response.0["pagination"]["per_page"], 3);
    }
}
