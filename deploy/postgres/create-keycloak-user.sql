-- Create Keycloak user and database
CREATE USER keycloak WITH PASSWORD 'Keycloak@DB2024!';
CREATE DATABASE keycloak OWNER keycloak;
GRANT ALL PRIVILEGES ON DATABASE keycloak TO keycloak;
