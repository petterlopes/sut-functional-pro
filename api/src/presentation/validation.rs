// ============================================================================
// VALIDATION UTILITIES - UTILITÁRIOS DE VALIDAÇÃO
// ============================================================================
// Módulo que centraliza validações comuns usadas nos controllers
// Elimina redundância e garante consistência

use crate::presentation::error_mapper::{invalid_uuid_error, ErrorResponse};
use uuid::Uuid;

/// Resultado de validação de UUID
pub type ValidationResult<T> = Result<T, ErrorResponse>;

/// Valida e converte uma string para UUID
///
/// # Argumentos
/// * `uuid_str` - String contendo o UUID
///
/// # Retorna
/// * `Ok(Uuid)` - UUID válido
/// * `Err(ErrorResponse)` - Erro de validação
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::validation::validate_uuid;
///
/// let result = validate_uuid("550e8400-e29b-41d4-a716-446655440000");
/// match result {
///     Ok(uuid) => println!("Valid UUID: {}", uuid),
///     Err((status, response)) => println!("Invalid UUID: {}", response.0["error"]),
/// }
/// ```
pub fn validate_uuid(uuid_str: &str) -> ValidationResult<Uuid> {
    Uuid::parse_str(uuid_str).map_err(|_| invalid_uuid_error())
}

/// Valida múltiplos UUIDs de uma vez
///
/// # Argumentos
/// * `uuid_strings` - Slice de strings contendo UUIDs
///
/// # Retorna
/// * `Ok(Vec<Uuid>)` - Lista de UUIDs válidos
/// * `Err(ErrorResponse)` - Erro de validação (primeiro UUID inválido encontrado)
pub fn validate_uuids(uuid_strings: &[&str]) -> ValidationResult<Vec<Uuid>> {
    let mut uuids = Vec::with_capacity(uuid_strings.len());

    for uuid_str in uuid_strings {
        let uuid = validate_uuid(uuid_str)?;
        uuids.push(uuid);
    }

    Ok(uuids)
}

/// Valida se uma string não está vazia
///
/// # Argumentos
/// * `value` - String a ser validada
/// * `field_name` - Nome do campo para mensagem de erro
///
/// # Retorna
/// * `Ok(())` - String válida
/// * `Err(ErrorResponse)` - String vazia
pub fn validate_not_empty(value: &str, field_name: &str) -> ValidationResult<()> {
    if value.trim().is_empty() {
        Err((
            axum::http::StatusCode::BAD_REQUEST,
            axum::response::Json(serde_json::json!({
                "error": format!("{} cannot be empty", field_name)
            })),
        ))
    } else {
        Ok(())
    }
}

/// Valida se um email tem formato válido (básico)
///
/// # Argumentos
/// * `email` - Email a ser validado
///
/// # Retorna
/// * `Ok(())` - Email válido
/// * `Err(ErrorResponse)` - Email inválido
pub fn validate_email_format(email: &str) -> ValidationResult<()> {
    if email.contains('@') && email.contains('.') && email.len() > 5 {
        Ok(())
    } else {
        Err((
            axum::http::StatusCode::BAD_REQUEST,
            axum::response::Json(serde_json::json!({
                "error": "Invalid email format"
            })),
        ))
    }
}

/// Valida se um valor está dentro de um range
///
/// # Argumentos
/// * `value` - Valor a ser validado
/// * `min` - Valor mínimo (inclusivo)
/// * `max` - Valor máximo (inclusivo)
/// * `field_name` - Nome do campo para mensagem de erro
///
/// # Retorna
/// * `Ok(())` - Valor válido
/// * `Err(ErrorResponse)` - Valor fora do range
pub fn validate_range<T>(value: T, min: T, max: T, field_name: &str) -> ValidationResult<()>
where
    T: PartialOrd + std::fmt::Display,
{
    if value >= min && value <= max {
        Ok(())
    } else {
        Err((
            axum::http::StatusCode::BAD_REQUEST,
            axum::response::Json(serde_json::json!({
                "error": format!("{} must be between {} and {}", field_name, min, max)
            })),
        ))
    }
}

/// Macro para simplificar validação de UUID em handlers
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::validation::validate_uuid_param;
///
/// let uuid = validate_uuid_param!(id)?;
/// ```
#[macro_export]
macro_rules! validate_uuid_param {
    ($param:expr) => {
        $crate::presentation::validation::validate_uuid($param)?
    };
}

/// Macro para validação de múltiplos parâmetros
///
/// # Exemplos
///
/// ```rust
/// use crate::presentation::validation::validate_params;
///
/// let (uuid1, uuid2) = validate_params!(id1, id2)?;
/// ```
#[macro_export]
macro_rules! validate_params {
    ($($param:expr),+) => {
        {
            $(
                let $param = $crate::presentation::validation::validate_uuid($param)?;
            )+
            ($($param),+)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_uuid_valid() {
        let valid_uuid = "550e8400-e29b-41d4-a716-446655440000";
        let result = validate_uuid(valid_uuid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_uuid_invalid() {
        let invalid_uuid = "not-a-uuid";
        let result = validate_uuid(invalid_uuid);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_uuids_mixed() {
        let uuids = vec![
            "550e8400-e29b-41d4-a716-446655440000",
            "not-a-uuid",
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
        ];
        let result = validate_uuids(&uuids);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_not_empty_valid() {
        let result = validate_not_empty("valid string", "field");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_not_empty_invalid() {
        let result = validate_not_empty("", "field");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_email_format_valid() {
        let result = validate_email_format("test@example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_email_format_invalid() {
        let result = validate_email_format("invalid-email");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_range_valid() {
        let result = validate_range(5, 1, 10, "value");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_range_invalid() {
        let result = validate_range(15, 1, 10, "value");
        assert!(result.is_err());
    }
}
