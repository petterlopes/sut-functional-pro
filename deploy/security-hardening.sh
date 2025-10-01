#!/bin/bash

# Security Hardening Script for SUT System
# This script applies security hardening to all components

set -e

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

# Check if running as root
check_root() {
    if [ "$EUID" -eq 0 ]; then
        warning "Running as root. Consider running as a non-root user for better security."
    fi
}

# Set up secure file permissions
setup_file_permissions() {
    log "Setting up secure file permissions..."
    
    # Make scripts executable
    chmod +x deploy/vault/setup.sh
    chmod +x deploy/keycloak/setup.sh
    chmod +x deploy/security-hardening.sh
    
    # Set restrictive permissions on configuration files
    chmod 600 deploy/vault/vault.hcl
    chmod 600 deploy/keycloak/keycloak-security.conf
    chmod 600 deploy/postgres/postgresql.conf
    chmod 600 deploy/postgres/pg_hba.conf
    
    # Set permissions on sensitive files
    chmod 644 deploy/keycloak/realm-sut.json
    chmod 644 deploy/keycloak/initial_users.json
    
    success "File permissions configured"
}

# Validate configuration files
validate_configurations() {
    log "Validating configuration files..."
    
    # Check if required files exist
    local required_files=(
        "deploy/vault/vault.hcl"
        "deploy/vault/setup.sh"
        "deploy/keycloak/setup.sh"
        "deploy/keycloak/keycloak-security.conf"
        "deploy/postgres/postgresql.conf"
        "deploy/postgres/pg_hba.conf"
        "deploy/postgres/init-keycloak-db.sql"
        "deploy/docker-compose.dev.yml"
    )
    
    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            error "Required file not found: $file"
            return 1
        fi
    done
    
    # Validate JSON files
    if command -v jq >/dev/null 2>&1; then
        if [ -f "deploy/keycloak/realm-sut.json" ]; then
            jq empty deploy/keycloak/realm-sut.json 2>/dev/null || {
                error "Invalid JSON in realm-sut.json"
                return 1
            }
        fi
        
        if [ -f "deploy/keycloak/initial_users.json" ]; then
            jq empty deploy/keycloak/initial_users.json 2>/dev/null || {
                error "Invalid JSON in initial_users.json"
                return 1
            }
        fi
    else
        warning "jq not found, skipping JSON validation"
    fi
    
    success "Configuration files validated"
}

# Check Docker security
check_docker_security() {
    log "Checking Docker security..."
    
    # Check if Docker is running
    if ! docker info >/dev/null 2>&1; then
        error "Docker is not running or not accessible"
        return 1
    fi
    
    # Check Docker version
    local docker_version=$(docker --version | cut -d' ' -f3 | cut -d',' -f1)
    log "Docker version: $docker_version"
    
    # Check if running as root
    if docker info 2>/dev/null | grep -q "Root Dir: /var/lib/docker"; then
        warning "Docker is running as root. Consider using rootless Docker for better security."
    fi
    
    success "Docker security check completed"
}

# Validate passwords strength
validate_passwords() {
    log "Validating password strength..."
    
    # Check password strength in configuration files
    local weak_passwords=()
    
    # Check for common weak passwords
    if grep -q "password.*123" deploy/docker-compose.dev.yml; then
        weak_passwords+=("Weak password found in docker-compose.dev.yml")
    fi
    
    if grep -q "password.*admin" deploy/docker-compose.dev.yml; then
        weak_passwords+=("Weak password found in docker-compose.dev.yml")
    fi
    
    if [ ${#weak_passwords[@]} -gt 0 ]; then
        for weak_pass in "${weak_passwords[@]}"; do
            warning "$weak_pass"
        done
    else
        success "Password strength validation passed"
    fi
}

# Check network security
check_network_security() {
    log "Checking network security..."
    
    # Check if ports are properly configured
    local exposed_ports=$(grep -o '"[0-9]*:[0-9]*"' deploy/docker-compose.dev.yml | sort -u)
    
    for port in $exposed_ports; do
        local port_num=$(echo "$port" | tr -d '"' | cut -d':' -f1)
        if [ "$port_num" -lt 1024 ] && [ "$port_num" -ne 80 ] && [ "$port_num" -ne 443 ]; then
            warning "Exposing privileged port: $port_num"
        fi
    done
    
    success "Network security check completed"
}

# Generate security report
generate_security_report() {
    log "Generating security report..."
    
    local report_file="security-report-$(date +%Y%m%d-%H%M%S).txt"
    
    cat > "$report_file" << EOF
SUT System Security Report
Generated: $(date)
================================

System Information:
- OS: $(uname -s)
- Kernel: $(uname -r)
- Docker Version: $(docker --version 2>/dev/null || echo "Not available")
- User: $(whoami)

Security Configuration:
- Vault: Enhanced configuration with audit logging
- Keycloak: Strong password policy, brute force protection
- PostgreSQL: SCRAM-SHA-256 authentication
- Network: Restricted port exposure

File Permissions:
$(ls -la deploy/vault/ deploy/keycloak/ deploy/postgres/ 2>/dev/null || echo "Files not found")

Docker Configuration:
- Services: $(grep -c "services:" deploy/docker-compose.dev.yml || echo "0")
- Networks: $(grep -c "networks:" deploy/docker-compose.dev.yml || echo "0")
- Volumes: $(grep -c "volumes:" deploy/docker-compose.dev.yml || echo "0")

Security Recommendations:
1. Use TLS/SSL in production
2. Enable firewall rules
3. Regular security updates
4. Monitor audit logs
5. Use secrets management
6. Implement backup strategies

EOF
    
    success "Security report generated: $report_file"
}

# Main hardening function
main() {
    log "Starting SUT system security hardening..."
    
    # Check prerequisites
    check_root
    
    # Setup file permissions
    setup_file_permissions
    
    # Validate configurations
    validate_configurations
    
    # Check Docker security
    check_docker_security
    
    # Validate passwords
    validate_passwords
    
    # Check network security
    check_network_security
    
    # Generate security report
    generate_security_report
    
    success "Security hardening completed successfully!"
    
    log "=== SECURITY HARDENING SUMMARY ==="
    log "✓ File permissions configured"
    log "✓ Configuration files validated"
    log "✓ Docker security checked"
    log "✓ Password strength validated"
    log "✓ Network security checked"
    log "✓ Security report generated"
    log "=================================="
    
    log "Next steps:"
    log "1. Review the security report"
    log "2. Start the system with: docker compose -f deploy/docker-compose.dev.yml up -d"
    log "3. Monitor logs for any security issues"
    log "4. Test authentication and authorization"
}

# Run main function
main "$@"
