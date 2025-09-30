-- Seed initial users idempotently. Requires pgcrypto for gen_salt/crypt; ensured in 001_init.sql
-- Use ON CONFLICT DO NOTHING to avoid duplicate inserts across environments.

INSERT INTO users (id, username, email, password, roles)
VALUES
  (gen_random_uuid(), 'admin',   'admin@example.com',   crypt('admin123', gen_salt('bf')), ARRAY['directory.read','directory.write','directory.pii.read']),
  (gen_random_uuid(), 'manager', 'manager@example.com', crypt('manager123', gen_salt('bf')), ARRAY['directory.read','directory.write']),
  (gen_random_uuid(), 'analyst', 'analyst@example.com', crypt('analyst123', gen_salt('bf')), ARRAY['directory.read'])
ON CONFLICT (username) DO NOTHING;

