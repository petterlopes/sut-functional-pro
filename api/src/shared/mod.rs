pub fn has_scope(claims: &serde_json::Value, scope: &str) -> bool {
    if let Some(s) = claims.get("scope").and_then(|x| x.as_str()) {
        if s.split_whitespace().any(|t| t == scope) {
            return true;
        }
    }
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
