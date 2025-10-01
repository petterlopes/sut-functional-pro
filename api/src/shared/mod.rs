// ============================================================================
// SHARED MODULE - MÓDULO COMPARTILHADO
// ============================================================================
// Módulo que contém utilitários e código compartilhado entre camadas
// Centraliza funcionalidades comuns para eliminar redundância

// ===== CORE UTILITIES =====
pub mod base_traits; // Traits base para eliminar redundância
pub mod config; // Sistema de configuração centralizado
pub mod middleware_system; // Sistema de middleware centralizado

// ===== UTILITY FUNCTIONS =====

/// Verifica se o usuário tem o escopo/permissão necessário
///
/// # Argumentos
/// * `claims` - Claims JWT do usuário
/// * `scope` - Escopo/permissão a ser verificado
///
/// # Retorna
/// `true` se o usuário tem o escopo, `false` caso contrário
pub fn has_scope(claims: &serde_json::Value, scope: &str) -> bool {
    // Verificar scope direto
    if let Some(s) = claims.get("scope").and_then(|x| x.as_str()) {
        if s.split_whitespace().any(|t| t == scope) {
            return true;
        }
    }

    // Verificar roles do realm
    if let Some(roles) = claims
        .get("realm_access")
        .and_then(|r| r.get("roles"))
        .and_then(|r| r.as_array())
    {
        if roles.iter().any(|r| r.as_str() == Some(scope)) {
            return true;
        }
    }

    false
}

/// Extrai o ID do usuário dos claims JWT
///
/// # Argumentos
/// * `claims` - Claims JWT do usuário
///
/// # Retorna
/// ID do usuário se encontrado, `None` caso contrário
pub fn extract_user_id(claims: &serde_json::Value) -> Option<String> {
    claims
        .get("sub")
        .and_then(|s| s.as_str())
        .map(|s| s.to_string())
}

/// Extrai o nome do usuário dos claims JWT
///
/// # Argumentos
/// * `claims` - Claims JWT do usuário
///
/// # Retorna
/// Nome do usuário se encontrado, `None` caso contrário
pub fn extract_username(claims: &serde_json::Value) -> Option<String> {
    claims
        .get("preferred_username")
        .and_then(|s| s.as_str())
        .map(|s| s.to_string())
}

/// Extrai o email do usuário dos claims JWT
///
/// # Argumentos
/// * `claims` - Claims JWT do usuário
///
/// # Retorna
/// Email do usuário se encontrado, `None` caso contrário
pub fn extract_email(claims: &serde_json::Value) -> Option<String> {
    claims
        .get("email")
        .and_then(|s| s.as_str())
        .map(|s| s.to_string())
}

/// Extrai as roles do usuário dos claims JWT
///
/// # Argumentos
/// * `claims` - Claims JWT do usuário
///
/// # Retorna
/// Lista de roles do usuário
pub fn extract_roles(claims: &serde_json::Value) -> Vec<String> {
    claims
        .get("realm_access")
        .and_then(|r| r.get("roles"))
        .and_then(|r| r.as_array())
        .map(|roles| {
            roles
                .iter()
                .filter_map(|r| r.as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Valida se um token JWT não expirou
///
/// # Argumentos
/// * `claims` - Claims JWT do usuário
///
/// # Retorna
/// `true` se o token não expirou, `false` caso contrário
pub fn is_token_valid(claims: &serde_json::Value) -> bool {
    if let Some(exp) = claims.get("exp").and_then(|e| e.as_i64()) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        return exp > now;
    }
    false
}

/// Gera um ID único para rastreamento de requisições
///
/// # Retorna
/// ID único para rastreamento
pub fn generate_trace_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Formata uma duração em formato legível
///
/// # Argumentos
/// * `duration` - Duração a ser formatada
///
/// # Retorna
/// String formatada da duração
pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();

    if secs > 0 {
        format!("{}.{:03}s", secs, millis)
    } else {
        format!("{}ms", millis)
    }
}

/// Converte bytes para formato legível
///
/// # Argumentos
/// * `bytes` - Número de bytes
///
/// # Retorna
/// String formatada dos bytes
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;

    if bytes < THRESHOLD {
        format!("{} {}", bytes, UNITS[0])
    } else {
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
            size /= THRESHOLD as f64;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_has_scope() {
        let claims = json!({
            "scope": "read write",
            "realm_access": {
                "roles": ["user", "admin"]
            }
        });

        assert!(has_scope(&claims, "read"));
        assert!(has_scope(&claims, "write"));
        assert!(has_scope(&claims, "user"));
        assert!(has_scope(&claims, "admin"));
        assert!(!has_scope(&claims, "delete"));
    }

    #[test]
    fn test_extract_user_id() {
        let claims = json!({
            "sub": "user123",
            "preferred_username": "john.doe"
        });

        assert_eq!(extract_user_id(&claims), Some("user123".to_string()));
    }

    #[test]
    fn test_extract_username() {
        let claims = json!({
            "sub": "user123",
            "preferred_username": "john.doe"
        });

        assert_eq!(extract_username(&claims), Some("john.doe".to_string()));
    }

    #[test]
    fn test_extract_roles() {
        let claims = json!({
            "realm_access": {
                "roles": ["user", "admin"]
            }
        });

        let roles = extract_roles(&claims);
        assert_eq!(roles, vec!["user", "admin"]);
    }

    #[test]
    fn test_is_token_valid() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let valid_claims = json!({
            "exp": now + 3600 // 1 hora no futuro
        });

        let expired_claims = json!({
            "exp": now - 3600 // 1 hora no passado
        });

        assert!(is_token_valid(&valid_claims));
        assert!(!is_token_valid(&expired_claims));
    }

    #[test]
    fn test_format_duration() {
        let duration = std::time::Duration::from_millis(1500);
        assert_eq!(format_duration(duration), "1.500s");

        let duration = std::time::Duration::from_millis(500);
        assert_eq!(format_duration(duration), "500ms");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1023), "1023 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
    }
}
