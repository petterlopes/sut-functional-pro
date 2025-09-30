#!/bin/sh
# Utility: aplica schema, seeds e views no container Postgres do docker-compose
# Uso (na raiz do repo):
#   docker compose -f deploy/docker-compose.dev.yml up -d postgres keycloak
#   ./api/seeds/apply_to_postgres.sh deploy postgres postgres 5432
# Parâmetros:
# 1) arquivo docker-compose (ex: deploy/docker-compose.dev.yml)
# 2) serviço do postgres conforme compose (ex: postgres)
# 3) usuário Postgres (ex: postgres)
# 4) porta (opcional)

COMPOSE_FILE=${1:-deploy/docker-compose.dev.yml}
PG_SERVICE=${2:-postgres}
PG_USER=${3:-postgres}

echo "Using compose file: $COMPOSE_FILE"
echo "Postgres service: $PG_SERVICE"

echo "Waiting a few seconds for Postgres to accept connections..."
sleep 5

echo "Creating database 'sut_db' (if not exists)"
docker compose -f "$COMPOSE_FILE" exec -T "$PG_SERVICE" sh -c "psql -U $PG_USER -tc \"SELECT 1 FROM pg_database WHERE datname='sut_db'\" | grep -q 1 || psql -U $PG_USER -c \"CREATE DATABASE sut_db;\""

echo "Copying SQL files into container"
docker cp api/seeds/schema.sql $(docker compose -f "$COMPOSE_FILE" ps -q $PG_SERVICE):/tmp/schema.sql
docker cp api/seeds/seeds.sql $(docker compose -f "$COMPOSE_FILE" ps -q $PG_SERVICE):/tmp/seeds.sql
docker cp api/seeds/views.sql $(docker compose -f "$COMPOSE_FILE" ps -q $PG_SERVICE):/tmp/views.sql

echo "Applying schema to sut_db"
docker compose -f "$COMPOSE_FILE" exec -T "$PG_SERVICE" sh -c "psql -U $PG_USER -d sut_db -f /tmp/schema.sql"

echo "Applying seeds to sut_db"
docker compose -f "$COMPOSE_FILE" exec -T "$PG_SERVICE" sh -c "psql -U $PG_USER -d sut_db -f /tmp/seeds.sql"

echo "Applying views to sut_db"
docker compose -f "$COMPOSE_FILE" exec -T "$PG_SERVICE" sh -c "psql -U $PG_USER -d sut_db -f /tmp/views.sql"

echo "Done. You can connect with: docker compose -f $COMPOSE_FILE exec -T $PG_SERVICE psql -U $PG_USER -d sut_db"
