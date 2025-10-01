-- Seed initial users only when explicit demo flag enabled. Requires pgcrypto for gen_salt/crypt.
-- Enable via: SET app.enable_demo_users = 'on'; prior to running migration in dev.

DO $$
BEGIN
    IF coalesce(current_setting('app.enable_demo_users', true), 'off') = 'on' THEN
        INSERT INTO users (id, username, email, password, roles)
        VALUES
          (gen_random_uuid(), 'admin',   'admin@example.com',   crypt('admin123', gen_salt('bf')), ARRAY['directory.read','directory.write','directory.pii.read']),
          (gen_random_uuid(), 'manager', 'manager@example.com', crypt('manager123', gen_salt('bf')), ARRAY['directory.read','directory.write']),
          (gen_random_uuid(), 'analyst', 'analyst@example.com', crypt('analyst123', gen_salt('bf')), ARRAY['directory.read'])
        ON CONFLICT (username) DO NOTHING;
    ELSE
        RAISE NOTICE 'Skipping demo user seed because app.enable_demo_users GUC is not set to on';
    END IF;
END;
$$;
