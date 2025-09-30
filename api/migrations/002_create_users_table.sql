-- Idempotent users table migration
-- Note: The canonical users table is defined in 001_init.sql with UUID ids and roles as TEXT[]
-- This migration is made a no-op if the table already exists, to avoid conflicts when
-- applying migrations in different environments.

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name = 'users'
    ) THEN
        CREATE TABLE users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            roles TEXT[] DEFAULT '{}'
        );
        CREATE INDEX IF NOT EXISTS users_email_idx ON users (LOWER(email));
    END IF;
END $$;