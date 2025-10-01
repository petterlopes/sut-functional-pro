#!/bin/bash

# Vault Setup Script with Enhanced Security
# This script configures Vault with proper security policies and secrets engines

set -e

# Configuration
VAULT_ADDR=${VAULT_ADDR:-"http://vault:8200"}
VAULT_TOKEN=${VAULT_TOKEN:-"root"}
MAX_RETRIES=30
RETRY_INTERVAL=2

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Wait for Vault to be ready
wait_for_vault() {
    log "Waiting for Vault to be ready..."
    local retries=0
    
    while [ $retries -lt $MAX_RETRIES ]; do
        if vault status >/dev/null 2>&1; then
            success "Vault is ready!"
            return 0
        fi
        
        log "Vault not ready yet, waiting ${RETRY_INTERVAL}s... (attempt $((retries + 1))/$MAX_RETRIES)"
        sleep $RETRY_INTERVAL
        retries=$((retries + 1))
    done
    
    error "Vault failed to become ready after $MAX_RETRIES attempts"
    return 1
}

# Enable audit logging
enable_audit_logging() {
    log "Enabling audit logging..."
    
    if ! vault audit list | grep -q "file/"; then
        vault audit enable file file_path=/vault/audit/vault-audit.log
        success "Audit logging enabled"
    else
        warning "Audit logging already enabled"
    fi
}

# Create security policies
create_security_policies() {
    log "Creating security policies..."
    
    # Policy for SUT API
    vault policy write sut-api-policy - <<EOF
# SUT API Policy - Limited access to specific secrets
path "secret/data/sut/*" {
  capabilities = ["read"]
}

path "secret/data/keycloak/*" {
  capabilities = ["read"]
}

path "secret/data/database/*" {
  capabilities = ["read"]
}

path "secret/data/frontend/*" {
  capabilities = ["read"]
}

# Deny access to other secrets
path "secret/data/*" {
  capabilities = ["deny"]
}

# Allow token renewal
path "auth/token/renew-self" {
  capabilities = ["update"]
}

path "auth/token/lookup-self" {
  capabilities = ["read"]
}
EOF

    # Policy for development tools
    vault policy write dev-tools-policy - <<EOF
# Development tools policy - Read-only access to development secrets
path "secret/data/dev/*" {
  capabilities = ["read"]
}

path "secret/data/sut/*" {
  capabilities = ["read"]
}

# Allow token renewal
path "auth/token/renew-self" {
  capabilities = ["update"]
}

path "auth/token/lookup-self" {
  capabilities = ["read"]
}
EOF

    success "Security policies created"
}

# Enable and configure KV v2 secrets engine
setup_kv_secrets() {
    log "Setting up KV v2 secrets engine..."
    
    # Enable KV v2 if not already enabled
    if ! vault secrets list | grep -q "secret/"; then
        vault secrets enable -version=2 -path=secret kv
        success "KV v2 secrets engine enabled"
    else
        warning "KV v2 secrets engine already enabled"
    fi
    
    # Configure KV v2 with proper settings
    vault kv metadata put -max-versions=10 secret/
    vault kv metadata put -delete-version-after=30d secret/
    
    success "KV v2 secrets engine configured"
}

# Enable and configure database secrets engine
setup_database_secrets() {
    log "Setting up database secrets engine..."
    
    # Enable database secrets engine if not already enabled
    if ! vault secrets list | grep -q "database/"; then
        vault secrets enable database
        success "Database secrets engine enabled"
    else
        warning "Database secrets engine already enabled"
    fi
    
    # Configure PostgreSQL connection
    vault write database/config/postgresql \
        plugin_name=postgresql-database-plugin \
        connection_url="postgresql://{{username}}:{{password}}@postgres:5432/sut?sslmode=disable" \
        allowed_roles="sut-api-role" \
        username="sut" \
        password="Sut@Postgres2024!"
    
    # Create database role
    vault write database/roles/sut-api-role \
        db_name=postgresql \
        creation_statements="CREATE ROLE \"{{name}}\" WITH LOGIN PASSWORD '{{password}}' VALID UNTIL '{{expiration}}'; GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO \"{{name}}\";" \
        default_ttl="1h" \
        max_ttl="24h"
    
    success "Database secrets engine configured"
}

# Enable and configure transit secrets engine
setup_transit_secrets() {
    log "Setting up transit secrets engine..."
    
    # Enable transit secrets engine if not already enabled
    if ! vault secrets list | grep -q "transit/"; then
        vault secrets enable transit
        success "Transit secrets engine enabled"
    else
        warning "Transit secrets engine already enabled"
    fi
    
    # Create encryption key for SUT
    vault write -f transit/keys/sut-encryption-key \
        type="aes256-gcm96" \
        exportable=false \
        allow_plaintext_backup=false
    
    success "Transit secrets engine configured"
}

# Store application secrets
store_application_secrets() {
    log "Storing application secrets..."
    
    # Database secrets
    vault kv put secret/database/sut \
        host="postgres" \
        port="5432" \
        database="sut" \
        username="sut" \
        password="Sut@Postgres2024!" \
        ssl_mode="disable"
    
    vault kv put secret/database/keycloak \
        host="postgres" \
        port="5432" \
        database="keycloak" \
        username="keycloak" \
        password="Keycloak@DB2024!" \
        ssl_mode="disable"
    
    # Keycloak secrets
    vault kv put secret/keycloak/admin \
        username="admin" \
        password="Admin@Keycloak2024!"
    
    vault kv put secret/keycloak/realm \
        realm="sut" \
        client_id="sut-frontend" \
        client_secret="" \
        issuer="http://keycloak:8080/realms/sut"
    
    # SUT API secrets
    vault kv put secret/sut/api \
        jwt_secret="Sut@JWT2024!SecretKey" \
        encryption_key="Sut@Encryption2024!Key" \
        api_key="Sut@API2024!Key"
    
    # Frontend secrets
    vault kv put secret/frontend/config \
        api_base_url="http://localhost:8080" \
        keycloak_url="http://localhost:8081" \
        keycloak_realm="sut" \
        keycloak_client_id="sut-frontend"
    
    # Development secrets
    vault kv put secret/dev/credentials \
        admin_username="admin" \
        admin_password="Admin@SUT2024!" \
        test_user="testuser" \
        test_password="Test@SUT2024!"
    
    success "Application secrets stored"
}

# Enable AppRole authentication
setup_approle_auth() {
    log "Setting up AppRole authentication..."
    
    # Enable AppRole auth method if not already enabled
    if ! vault auth list | grep -q "approle/"; then
        vault auth enable approle
        success "AppRole authentication enabled"
    else
        warning "AppRole authentication already enabled"
    fi
    
    # Create AppRole for SUT API
    vault write auth/approle/role/sut-api \
        token_policies="sut-api-policy" \
        token_ttl=1h \
        token_max_ttl=4h \
        bind_secret_id=true \
        secret_id_ttl=24h
    
    # Get Role ID
    ROLE_ID=$(vault read -field=role_id auth/approle/role/sut-api/role-id)
    log "SUT API Role ID: $ROLE_ID"
    
    # Generate Secret ID
    SECRET_ID=$(vault write -field=secret_id -f auth/approle/role/sut-api/secret-id)
    log "SUT API Secret ID: $SECRET_ID"
    
    # Store credentials for API
    vault kv put secret/sut/api/credentials \
        role_id="$ROLE_ID" \
        secret_id="$SECRET_ID"
    
    success "AppRole authentication configured"
}

# Enable Token authentication with proper settings
setup_token_auth() {
    log "Configuring token authentication..."
    
    # Set default token TTL
    vault auth tune -default-lease-ttl=1h -max-lease-ttl=24h token/
    
    success "Token authentication configured"
}

# Setup monitoring and health checks
setup_monitoring() {
    log "Setting up monitoring..."
    
    # Enable sys audit device for monitoring
    if ! vault audit list | grep -q "syslog/"; then
        vault audit enable syslog tag="vault" facility="LOCAL0"
        success "Syslog audit device enabled"
    else
        warning "Syslog audit device already enabled"
    fi
    
    success "Monitoring configured"
}

# Main setup function
main() {
    log "Starting Vault setup with enhanced security..."
    
    # Wait for Vault to be ready
    wait_for_vault
    
    # Enable audit logging
    enable_audit_logging
    
    # Create security policies
    create_security_policies
    
    # Setup secrets engines
    setup_kv_secrets
    setup_database_secrets
    setup_transit_secrets
    
    # Store application secrets
    store_application_secrets
    
    # Setup authentication methods
    setup_approle_auth
    setup_token_auth
    
    # Setup monitoring
    setup_monitoring
    
    success "Vault setup completed successfully!"
    
    # Display summary
    log "=== VAULT SETUP SUMMARY ==="
    log "Vault Address: $VAULT_ADDR"
    log "Root Token: $VAULT_TOKEN"
    log "Secrets Engines: KV v2, Database, Transit"
    log "Auth Methods: AppRole, Token"
    log "Policies: sut-api-policy, dev-tools-policy"
    log "Audit Logging: Enabled (file + syslog)"
    log "=========================="
}

# Run main function
main "$@"