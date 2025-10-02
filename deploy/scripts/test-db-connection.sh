#!/bin/bash
# =============================================================================
# Database Connection Test Script
# =============================================================================
# This script tests the database connectivity for both SUT and Keycloak

set -e

echo "============================================================================="
echo "Testing Database Connectivity"
echo "============================================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Database connection parameters
DB_HOST="${DB_HOST:-postgres}"
DB_PORT="${DB_PORT:-5432}"
POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-Sut@Postgres2024!}"
SUT_USER="${SUT_USER:-sut}"
SUT_PASSWORD="${SUT_PASSWORD:-Sut@Postgres2024!}"
KEYCLOAK_USER="${KEYCLOAK_USER:-keycloak}"
KEYCLOAK_PASSWORD="${KEYCLOAK_PASSWORD:-Keycloak@DB2024!}"

# Function to test database connection
test_connection() {
    local user=$1
    local password=$2
    local database=$3
    local description=$4
    
    echo -e "${YELLOW}Testing $description...${NC}"
    
    if PGPASSWORD="$password" psql -h "$DB_HOST" -p "$DB_PORT" -U "$user" -d "$database" -c "SELECT 1;" >/dev/null 2>&1; then
        echo -e "${GREEN}✓ $description connection successful${NC}"
        return 0
    else
        echo -e "${RED}✗ $description connection failed${NC}"
        return 1
    fi
}

# Function to check if database exists
check_database() {
    local database=$1
    local description=$2
    
    echo -e "${YELLOW}Checking if $description exists...${NC}"
    
    if PGPASSWORD="$POSTGRES_PASSWORD" psql -h "$DB_HOST" -p "$DB_PORT" -U "$POSTGRES_USER" -d "postgres" -c "SELECT 1 FROM pg_database WHERE datname='$database';" | grep -q "1 row"; then
        echo -e "${GREEN}✓ $description exists${NC}"
        return 0
    else
        echo -e "${RED}✗ $description does not exist${NC}"
        return 1
    fi
}

# Function to check if user exists
check_user() {
    local user=$1
    local description=$2
    
    echo -e "${YELLOW}Checking if $description user exists...${NC}"
    
    if PGPASSWORD="$POSTGRES_PASSWORD" psql -h "$DB_HOST" -p "$DB_PORT" -U "$POSTGRES_USER" -d "postgres" -c "SELECT 1 FROM pg_roles WHERE rolname='$user';" | grep -q "1 row"; then
        echo -e "${GREEN}✓ $description user exists${NC}"
        return 0
    else
        echo -e "${RED}✗ $description user does not exist${NC}"
        return 1
    fi
}

echo "Testing PostgreSQL server connectivity..."
if ! test_connection "$POSTGRES_USER" "$POSTGRES_PASSWORD" "postgres" "PostgreSQL server"; then
    echo -e "${RED}PostgreSQL server is not accessible. Please check if the container is running.${NC}"
    exit 1
fi

echo ""
echo "Checking databases..."
check_database "sut" "SUT database"
check_database "keycloak" "Keycloak database"

echo ""
echo "Checking users..."
check_user "$SUT_USER" "SUT"
check_user "$KEYCLOAK_USER" "Keycloak"

echo ""
echo "Testing application connections..."
test_connection "$SUT_USER" "$SUT_PASSWORD" "sut" "SUT application"
test_connection "$KEYCLOAK_USER" "$KEYCLOAK_PASSWORD" "keycloak" "Keycloak application"

echo ""
echo "============================================================================="
echo -e "${GREEN}Database connectivity test completed successfully!${NC}"
echo "============================================================================="
