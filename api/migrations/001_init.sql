CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Simple text normalization function
CREATE OR REPLACE FUNCTION normalize_text(input_text text) RETURNS text AS $$
  SELECT LOWER(TRIM(input_text));
$$ LANGUAGE sql IMMUTABLE;

CREATE TABLE IF NOT EXISTS org_units(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  parent_id UUID NULL REFERENCES org_units(id)
);
CREATE INDEX IF NOT EXISTS org_units_name_idx ON org_units (normalize_text(name));

CREATE TABLE IF NOT EXISTS departments(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  unit_id UUID NOT NULL REFERENCES org_units(id),
  name TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS departments_ux ON departments(unit_id, LOWER(name));

CREATE TABLE IF NOT EXISTS contacts(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  type TEXT NOT NULL DEFAULT 'PERSON',
  full_name TEXT NOT NULL,
  full_name_norm TEXT NOT NULL DEFAULT '',
  document TEXT NULL UNIQUE,
  unit_id UUID NULL REFERENCES org_units(id),
  department_id UUID NULL REFERENCES departments(id),
  status TEXT NOT NULL DEFAULT 'ACTIVE',
  source_of_truth TEXT NULL,
  duplicate_of UUID NULL REFERENCES contacts(id),
  etag TEXT NOT NULL DEFAULT '',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  search_vector tsvector,
  email TEXT NOT NULL DEFAULT 'unknown@example.com'
);
CREATE INDEX IF NOT EXISTS contacts_trgm ON contacts USING gin (full_name_norm gin_trgm_ops);
CREATE INDEX IF NOT EXISTS contacts_fts  ON contacts USING gin (search_vector);

CREATE TABLE IF NOT EXISTS phones(
  contact_id UUID REFERENCES contacts(id) ON DELETE CASCADE,
  e164 TEXT NOT NULL,
  extension TEXT NOT NULL DEFAULT '',
  type TEXT,
  is_primary BOOLEAN DEFAULT FALSE,
  PRIMARY KEY(contact_id, e164, extension)
);
CREATE INDEX IF NOT EXISTS phones_e164_idx ON phones(e164);

CREATE TABLE IF NOT EXISTS emails(
  contact_id UUID REFERENCES contacts(id) ON DELETE CASCADE,
  address TEXT NOT NULL,
  is_primary BOOLEAN DEFAULT FALSE,
  PRIMARY KEY(contact_id, address)
);
CREATE UNIQUE INDEX IF NOT EXISTS emails_addr_ci_ux ON emails(contact_id, LOWER(address));

CREATE TABLE IF NOT EXISTS source_records(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  source TEXT NOT NULL,
  source_key TEXT NOT NULL,
  hash TEXT NOT NULL,
  payload JSONB NOT NULL,
  fetched_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX IF NOT EXISTS source_records_ux ON source_records(source, source_key);

CREATE TABLE IF NOT EXISTS contact_sources(
  contact_id UUID REFERENCES contacts(id) ON DELETE CASCADE,
  source_record_id UUID REFERENCES source_records(id) ON DELETE CASCADE,
  confidence NUMERIC NOT NULL DEFAULT 1.0,
  PRIMARY KEY(contact_id, source_record_id)
);

CREATE TABLE IF NOT EXISTS merge_candidates(
  contact_a UUID REFERENCES contacts(id) ON DELETE CASCADE,
  contact_b UUID REFERENCES contacts(id) ON DELETE CASCADE,
  score NUMERIC NOT NULL,
  features JSONB NOT NULL,
  PRIMARY KEY(contact_a, contact_b)
);
CREATE INDEX IF NOT EXISTS merge_candidates_score_idx ON merge_candidates(score DESC);

CREATE TABLE IF NOT EXISTS merge_decisions(
  primary_contact UUID REFERENCES contacts(id),
  duplicate_contact UUID REFERENCES contacts(id),
  decision TEXT NOT NULL CHECK (decision IN ('MERGE','REJECT')),
  chosen_fields JSONB,
  decided_by UUID,
  decided_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY(primary_contact, duplicate_contact)
);

CREATE TABLE IF NOT EXISTS audit_events(
  id BIGSERIAL PRIMARY KEY,
  actor_sub TEXT,
  action TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  before JSONB,
  after JSONB,
  at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS audit_idx ON audit_events(entity_type, entity_id, at DESC);

-- FTS trigger with simple text normalization
CREATE OR REPLACE FUNCTION update_search_vector() RETURNS trigger AS $$
BEGIN
  NEW.full_name_norm := normalize_text(coalesce(NEW.full_name,''));
  NEW.search_vector := setweight(to_tsvector('simple', NEW.full_name_norm), 'A');
  RETURN NEW;
END
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS contacts_search_vector_tgr ON contacts;
CREATE TRIGGER contacts_search_vector_tgr
BEFORE INSERT OR UPDATE OF full_name ON contacts
FOR EACH ROW EXECUTE FUNCTION update_search_vector();

-- SEEDS
INSERT INTO org_units (id, name) VALUES
  ('00000000-0000-0000-0000-000000000001','Headquarters')
ON CONFLICT DO NOTHING;

INSERT INTO departments (id, unit_id, name) VALUES
  ('00000000-0000-0000-0000-000000000101','00000000-0000-0000-0000-000000000001','Engineering'),
  ('00000000-0000-0000-0000-000000000102','00000000-0000-0000-0000-000000000001','HR')
ON CONFLICT DO NOTHING;

INSERT INTO contacts (id, full_name, unit_id, department_id, status, document, email, created_at, updated_at)
VALUES
  ('10000000-0000-0000-0000-000000000001','Alice Silva','00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000101','ACTIVE', NULL, 'alice.silva@example.com', now(), now()),
  ('10000000-0000-0000-0000-000000000002','Bruno Souza','00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000102','ACTIVE', NULL, 'bruno.souza@example.com', now(), now())
ON CONFLICT DO NOTHING;

-- Strong ETag computed from content
CREATE OR REPLACE FUNCTION compute_contact_etag(c contacts) RETURNS text AS $$
BEGIN
  RETURN encode(digest(
    coalesce(c.full_name,'') || '|' ||
    coalesce(c.status,'') || '|' ||
    coalesce(c.unit_id::text,'') || '|' ||
    coalesce(c.department_id::text,'') || '|' ||
    coalesce(c.document,'')
  ,'sha256'),'hex');
END
$$ LANGUAGE plpgsql IMMUTABLE;

DROP TRIGGER IF EXISTS contacts_etag_tgr ON contacts;
CREATE OR REPLACE FUNCTION contacts_set_etag() RETURNS trigger AS $$
BEGIN
  NEW.etag := compute_contact_etag(NEW);
  NEW.updated_at := now();
  RETURN NEW;
END$$ LANGUAGE plpgsql;

CREATE TRIGGER contacts_etag_tgr
BEFORE INSERT OR UPDATE ON contacts
FOR EACH ROW EXECUTE FUNCTION contacts_set_etag();

-- Replay protection for webhooks
CREATE TABLE IF NOT EXISTS webhook_receipts(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  source TEXT NOT NULL,
  nonce TEXT NOT NULL,
  received_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX IF NOT EXISTS webhook_receipts_ux ON webhook_receipts(source, nonce);
