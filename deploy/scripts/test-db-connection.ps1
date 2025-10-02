# =============================================================================
# Database Connection Test Script (PowerShell)
# =============================================================================
# This script tests the database connectivity for both SUT and Keycloak

param(
    [string]$DB_HOST = "localhost",
    [int]$DB_PORT = 5432,
    [string]$POSTGRES_USER = "postgres",
    [string]$POSTGRES_PASSWORD = "Sut@Postgres2024!",
    [string]$SUT_USER = "sut",
    [string]$SUT_PASSWORD = "Sut@Postgres2024!",
    [string]$KEYCLOAK_USER = "keycloak",
    [string]$KEYCLOAK_PASSWORD = "Keycloak@DB2024!"
)

Write-Host "=============================================================================" -ForegroundColor Cyan
Write-Host "Testing Database Connectivity" -ForegroundColor Cyan
Write-Host "=============================================================================" -ForegroundColor Cyan

# Function to test database connection
function Test-DatabaseConnection {
    param(
        [string]$User,
        [string]$Password,
        [string]$Database,
        [string]$Description
    )
    
    Write-Host "Testing $Description..." -ForegroundColor Yellow
    
    try {
        $env:PGPASSWORD = $Password
        $result = psql -h $DB_HOST -p $DB_PORT -U $User -d $Database -c "SELECT 1;" 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✓ $Description connection successful" -ForegroundColor Green
            return $true
        } else {
            Write-Host "✗ $Description connection failed" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "✗ $Description connection failed: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    } finally {
        Remove-Item Env:PGPASSWORD -ErrorAction SilentlyContinue
    }
}

# Function to check if database exists
function Test-DatabaseExists {
    param(
        [string]$Database,
        [string]$Description
    )
    
    Write-Host "Checking if $Description exists..." -ForegroundColor Yellow
    
    try {
        $env:PGPASSWORD = $POSTGRES_PASSWORD
        $result = psql -h $DB_HOST -p $DB_PORT -U $POSTGRES_USER -d "postgres" -c "SELECT 1 FROM pg_database WHERE datname='$Database';" 2>$null
        if ($LASTEXITCODE -eq 0 -and $result -match "1 row") {
            Write-Host "✓ $Description exists" -ForegroundColor Green
            return $true
        } else {
            Write-Host "✗ $Description does not exist" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "✗ $Description does not exist: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    } finally {
        Remove-Item Env:PGPASSWORD -ErrorAction SilentlyContinue
    }
}

# Function to check if user exists
function Test-UserExists {
    param(
        [string]$User,
        [string]$Description
    )
    
    Write-Host "Checking if $Description user exists..." -ForegroundColor Yellow
    
    try {
        $env:PGPASSWORD = $POSTGRES_PASSWORD
        $result = psql -h $DB_HOST -p $DB_PORT -U $POSTGRES_USER -d "postgres" -c "SELECT 1 FROM pg_roles WHERE rolname='$User';" 2>$null
        if ($LASTEXITCODE -eq 0 -and $result -match "1 row") {
            Write-Host "✓ $Description user exists" -ForegroundColor Green
            return $true
        } else {
            Write-Host "✗ $Description user does not exist" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "✗ $Description user does not exist: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    } finally {
        Remove-Item Env:PGPASSWORD -ErrorAction SilentlyContinue
    }
}

Write-Host "Testing PostgreSQL server connectivity..." -ForegroundColor Cyan
if (-not (Test-DatabaseConnection -User $POSTGRES_USER -Password $POSTGRES_PASSWORD -Database "postgres" -Description "PostgreSQL server")) {
    Write-Host "PostgreSQL server is not accessible. Please check if the container is running." -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Checking databases..." -ForegroundColor Cyan
Test-DatabaseExists -Database "sut" -Description "SUT database"
Test-DatabaseExists -Database "keycloak" -Description "Keycloak database"

Write-Host ""
Write-Host "Checking users..." -ForegroundColor Cyan
Test-UserExists -User $SUT_USER -Description "SUT"
Test-UserExists -User $KEYCLOAK_USER -Description "Keycloak"

Write-Host ""
Write-Host "Testing application connections..." -ForegroundColor Cyan
Test-DatabaseConnection -User $SUT_USER -Password $SUT_PASSWORD -Database "sut" -Description "SUT application"
Test-DatabaseConnection -User $KEYCLOAK_USER -Password $KEYCLOAK_PASSWORD -Database "keycloak" -Description "Keycloak application"

Write-Host ""
Write-Host "=============================================================================" -ForegroundColor Cyan
Write-Host "Database connectivity test completed successfully!" -ForegroundColor Green
Write-Host "=============================================================================" -ForegroundColor Cyan
