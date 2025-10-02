-- Install required PostgreSQL extensions for SUT
-- This script installs the necessary extensions for the application

-- Connect to SUT database
\c sut;

-- Extensions installed

-- Install uuid-ossp extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Grant usage on extensions to sut user
GRANT USAGE ON SCHEMA public TO sut;

-- Log completion
\echo 'PostgreSQL extensions installed successfully'
