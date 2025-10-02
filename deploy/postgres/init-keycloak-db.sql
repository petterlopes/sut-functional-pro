-- Initialize Keycloak Database and SUT User
-- This script creates the Keycloak database and user with proper security settings

-- Create SUT user with strong password
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'sut') THEN
        CREATE USER sut WITH PASSWORD 'Sut@Postgres2024!';
    END IF;
END
$$;

-- Create Keycloak user with strong password
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'keycloak') THEN
        CREATE USER keycloak WITH PASSWORD 'Keycloak@DB2024!';
    END IF;
END
$$;

-- Create Keycloak database
SELECT 'CREATE DATABASE keycloak OWNER keycloak'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'keycloak')\gexec

-- Grant privileges to SUT user
GRANT ALL PRIVILEGES ON DATABASE sut TO sut;

-- Grant privileges to Keycloak user
GRANT ALL PRIVILEGES ON DATABASE keycloak TO keycloak;

-- Connect to SUT database and grant schema privileges
\c sut;

-- Install required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Grant schema privileges to SUT user
GRANT ALL ON SCHEMA public TO sut;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO sut;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO sut;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public TO sut;

-- Set default privileges for future tables
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO sut;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO sut;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON FUNCTIONS TO sut;

ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO keycloak;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO keycloak;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON FUNCTIONS TO keycloak;

-- Connect to Keycloak database and set up schema
\c keycloak;

-- Create schema for Keycloak
CREATE SCHEMA IF NOT EXISTS keycloak;

-- Grant schema privileges
GRANT ALL ON SCHEMA keycloak TO keycloak;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA keycloak TO keycloak;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA keycloak TO keycloak;

-- Set search path
ALTER DATABASE keycloak SET search_path TO keycloak, public;

-- Log the completion
\echo 'Keycloak database and users created successfully'
