// ============================================================================
// HANDLER MACROS - MACROS PARA HANDLERS CRUD GENÉRICOS
// ============================================================================
// Módulo que define macros para eliminar redundância em handlers CRUD
// Segue o padrão DRY (Don't Repeat Yourself) e melhora a manutenibilidade

use crate::presentation::error_mapper::map_domain_error;
use crate::presentation::validation::validate_uuid;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Macro para criar handler de listagem com filtros
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$request_type` - Tipo da requisição de busca
/// * `$response_type` - Tipo da resposta
/// * `$repository_field` - Campo do repositório no AppState
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::list_handler;
///
/// list_handler!(
///     GetContactsUseCase,
///     ContactSearchRequest,
///     ContactSearchResponse,
///     contact_repository
/// );
/// ```
#[macro_export]
macro_rules! list_handler {
    ($use_case_type:ty, $request_type:ty, $response_type:ty, $repository_field:ident) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
            Query(params): Query<$request_type>,
        ) -> Result<Json<$response_type>, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            match use_case.execute(params).await {
                Ok(response) => Ok(Json(response)),
                Err(err) => Err(map_domain_error(&err)),
            }
        }
    };
}

/// Macro para criar handler de busca por ID
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$response_type` - Tipo da resposta
/// * `$repository_field` - Campo do repositório no AppState
/// * `$value_object_type` - Tipo do value object (ex: ContactId)
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::get_by_id_handler;
///
/// get_by_id_handler!(
///     GetContactsUseCase,
///     ContactResponse,
///     contact_repository,
///     ContactId
/// );
/// ```
#[macro_export]
macro_rules! get_by_id_handler {
    ($use_case_type:ty, $response_type:ty, $repository_field:ident, $value_object_type:ty) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
            Path(id): Path<String>,
        ) -> Result<Json<$response_type>, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            let uuid = validate_uuid(&id)?;
            let value_object = <$value_object_type>(uuid);

            match use_case.execute_by_id(&value_object).await {
                Ok(response) => Ok(Json(response)),
                Err(err) => Err(map_domain_error(&err))
            }
        }
    };
}

/// Macro para criar handler de criação
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$request_type` - Tipo da requisição
/// * `$response_type` - Tipo da resposta
/// * `$repository_field` - Campo do repositório no AppState
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::create_handler;
///
/// create_handler!(
///     CreateContactUseCase,
///     CreateContactRequest,
///     ContactResponse,
///     contact_repository
/// );
/// ```
#[macro_export]
macro_rules! create_handler {
    ($use_case_type:ty, $request_type:ty, $response_type:ty, $repository_field:ident) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
            Json(request): Json<$request_type>,
        ) -> Result<Json<$response_type>, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            match use_case.execute(request).await {
                Ok(response) => Ok(Json(response)),
                Err(err) => Err(map_domain_error(&err)),
            }
        }
    };
}

/// Macro para criar handler de atualização
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$request_type` - Tipo da requisição
/// * `$response_type` - Tipo da resposta
/// * `$repository_field` - Campo do repositório no AppState
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::update_handler;
///
/// update_handler!(
///     UpdateContactUseCase,
///     UpdateContactRequest,
///     ContactResponse,
///     contact_repository
/// );
/// ```
#[macro_export]
macro_rules! update_handler {
    ($use_case_type:ty, $request_type:ty, $response_type:ty, $repository_field:ident) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
            Path(id): Path<String>,
            Json(mut request): Json<$request_type>,
        ) -> Result<Json<$response_type>, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            request.id = id;

            match use_case.execute(request).await {
                Ok(response) => Ok(Json(response)),
                Err(err) => Err(map_domain_error(&err)),
            }
        }
    };
}

/// Macro para criar handler de exclusão
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$repository_field` - Campo do repositório no AppState
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::delete_handler;
///
/// delete_handler!(
///     DeleteContactUseCase,
///     contact_repository
/// );
/// ```
#[macro_export]
macro_rules! delete_handler {
    ($use_case_type:ty, $repository_field:ident) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
            Path(id): Path<String>,
        ) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            match use_case.execute(&id).await {
                Ok(_) => Ok(StatusCode::NO_CONTENT),
                Err(err) => Err(map_domain_error(&err)),
            }
        }
    };
}

/// Macro para criar handler de estatísticas
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$response_type` - Tipo da resposta
/// * `$repository_field` - Campo do repositório no AppState
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::statistics_handler;
///
/// statistics_handler!(
///     GetContactStatisticsUseCase,
///     ContactStatisticsResponse,
///     contact_repository
/// );
/// ```
#[macro_export]
macro_rules! statistics_handler {
    ($use_case_type:ty, $response_type:ty, $repository_field:ident) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
        ) -> Result<Json<$response_type>, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            match use_case.execute().await {
                Ok(response) => Ok(Json(response)),
                Err(err) => Err(map_domain_error(&err)),
            }
        }
    };
}

/// Macro para criar handler de busca por parâmetro específico
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$response_type` - Tipo da resposta
/// * `$repository_field` - Campo do repositório no AppState
/// * `$method_name` - Nome do método no caso de uso
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::get_by_param_handler;
///
/// get_by_param_handler!(
///     GetUsersUseCase,
///     UserResponse,
///     user_repository,
///     execute_by_username
/// );
/// ```
#[macro_export]
macro_rules! get_by_param_handler {
    ($use_case_type:ty, $response_type:ty, $repository_field:ident, $method_name:ident) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
            Path(param): Path<String>,
        ) -> Result<Json<$response_type>, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            match use_case.$method_name(&param).await {
                Ok(response) => Ok(Json(response)),
                Err(err) => Err(map_domain_error(&err)),
            }
        }
    };
}

/// Macro para criar handler de busca por parâmetro com validação UUID
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$response_type` - Tipo da resposta
/// * `$repository_field` - Campo do repositório no AppState
/// * `$method_name` - Nome do método no caso de uso
/// * `$value_object_type` - Tipo do value object
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::get_by_uuid_param_handler;
///
/// get_by_uuid_param_handler!(
///     GetDepartmentsUseCase,
///     DepartmentSearchResponse,
///     department_repository,
///     execute_by_unit,
///     OrgUnitId
/// );
/// ```
#[macro_export]
macro_rules! get_by_uuid_param_handler {
    ($use_case_type:ty, $response_type:ty, $repository_field:ident, $method_name:ident, $value_object_type:ty) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
            Path(param): Path<String>,
        ) -> Result<Json<$response_type>, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            let uuid = validate_uuid(&param)?;
            let value_object = <$value_object_type>(uuid);

            match use_case.$method_name(&value_object).await {
                Ok(response) => Ok(Json(response)),
                Err(err) => Err(map_domain_error(&err))
            }
        }
    };
}

/// Macro para criar handler de busca por parâmetro que retorna lista
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$response_type` - Tipo da resposta
/// * `$repository_field` - Campo do repositório no AppState
/// * `$method_name` - Nome do método no caso de uso
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::get_list_by_param_handler;
///
/// get_list_by_param_handler!(
///     GetUsersUseCase,
///     UserSearchResponse,
///     user_repository,
///     execute_by_role
/// );
/// ```
#[macro_export]
macro_rules! get_list_by_param_handler {
    ($use_case_type:ty, $response_type:ty, $repository_field:ident, $method_name:ident) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
            Path(param): Path<String>,
        ) -> Result<Json<$response_type>, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            match use_case.$method_name(&param).await {
                Ok(response) => Ok(Json(response)),
                Err(err) => Err(map_domain_error(&err)),
            }
        }
    };
}

/// Macro para criar handler de busca por parâmetro UUID que retorna lista
///
/// # Argumentos
/// * `$use_case_type` - Tipo do caso de uso
/// * `$response_type` - Tipo da resposta
/// * `$repository_field` - Campo do repositório no AppState
/// * `$method_name` - Nome do método no caso de uso
/// * `$value_object_type` - Tipo do value object
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::handler_macros::get_list_by_uuid_param_handler;
///
/// get_list_by_uuid_param_handler!(
///     GetDepartmentsUseCase,
///     DepartmentSearchResponse,
///     department_repository,
///     execute_by_unit,
///     OrgUnitId
/// );
/// ```
#[macro_export]
macro_rules! get_list_by_uuid_param_handler {
    ($use_case_type:ty, $response_type:ty, $repository_field:ident, $method_name:ident, $value_object_type:ty) => {
        async fn handler(
            State(state): State<Arc<crate::AppState>>,
            Path(param): Path<String>,
        ) -> Result<Json<$response_type>, (StatusCode, Json<serde_json::Value>)> {
            let use_case = <$use_case_type>::new(state.$repository_field.as_ref());

            let uuid = validate_uuid(&param)?;
            let value_object = <$value_object_type>(uuid);

            match use_case.$method_name(&value_object).await {
                Ok(response) => Ok(Json(response)),
                Err(err) => Err(map_domain_error(&err))
            }
        }
    };
}
