-- Initialize Keycloak Database and SUT User
-- This script creates the Keycloak database and user with proper security settings

-- Create SUT user with strong password
CREATE USER sut WITH PASSWORD 'Sut@Postgres2024!';

-- Create Keycloak database
CREATE DATABASE keycloak;

-- Create Keycloak user with strong password
CREATE USER keycloak WITH PASSWORD 'Keycloak@DB2024!';

-- Grant privileges to SUT user
GRANT ALL PRIVILEGES ON DATABASE sut TO sut;

-- Grant privileges to Keycloak user
GRANT ALL PRIVILEGES ON DATABASE keycloak TO keycloak;

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
