#!/usr/bin/env sh
set -e
apk add --no-cache curl openssl jq || true
VAULT_ADDR=${VAULT_ADDR:-http://vault:8200}
VAULT_TOKEN=${VAULT_TOKEN:-root}
for i in $(seq 1 60); do curl -s "$VAULT_ADDR/v1/sys/health" | jq -e '.initialized==true' >/dev/null && break || sleep 1; done
export VAULT_ADDR VAULT_TOKEN
vault login -no-print $VAULT_TOKEN || true
vault secrets enable -path=database database || true
vault secrets enable -path=transit transit || true
vault secrets enable -path=kv kv-v2 || true
vault write -f transit/keys/pii-doc || true
vault kv put kv/webhook secret=$(openssl rand -hex 32)
echo "Vault setup done"
