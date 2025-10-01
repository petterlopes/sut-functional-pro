#!/bin/sh
set -eu

KEYCLOAK_URL="${KEYCLOAK_URL:-http://keycloak:8080}"
KEYCLOAK_USER="${KEYCLOAK_USER:-admin}"
KEYCLOAK_PASSWORD="${KEYCLOAK_PASSWORD:-admin}"
KCADM=/opt/keycloak/bin/kcadm.sh

retry() {
  n=0
  until [ $n -ge 30 ]; do
    "$@" && break
    n=$((n+1))
    sleep 2
  done
}

echo "[kc-setup] trying kcadm login until ready"
retry $KCADM config credentials --server "$KEYCLOAK_URL" --realm master --user "$KEYCLOAK_USER" --password "$KEYCLOAK_PASSWORD"

if $KCADM get realms/sut >/dev/null 2>&1; then
  echo "[kc-setup] realm sut exists; enabling"
  $KCADM update realms/sut -s enabled=true
else
  echo "[kc-setup] create realm sut"
  $KCADM create realms -s realm=sut -s enabled=true
fi

for role in directory.read directory.write directory.merge directory.pii.read; do
  if $KCADM get "realms/sut/roles/$role" >/dev/null 2>&1; then
    echo "[kc-setup] role $role ok"
  else
    echo "[kc-setup] create role $role"
    $KCADM create "realms/sut/roles" -s name="$role"
  fi
done

echo "[kc-setup] ensure client sut-frontend"
CLIENT_ID=""
OUT=$($KCADM get clients -r sut -q clientId=sut-frontend)
echo "$OUT" | grep -q 'clientId" : "sut-frontend"' && CLIENT_ID=$(echo "$OUT" | sed -n 's/.*"id" : "\([^"]*\)".*/\1/p') || true
if [ -z "$CLIENT_ID" ]; then
  CLIENT_ID=$($KCADM create clients -r sut -s clientId=sut-frontend -s publicClient=true -s protocol=openid-connect -s standardFlowEnabled=true -s enabled=true -i)
fi
$KCADM update clients/$CLIENT_ID -r sut -s publicClient=true -s enabled=true -s standardFlowEnabled=true -s directAccessGrantsEnabled=false || true
$KCADM update clients/$CLIENT_ID -r sut -s redirectUris='["http://localhost:5173/*"]' || true
$KCADM update clients/$CLIENT_ID -r sut -s webOrigins='["*"]' || true
$KCADM update clients/$CLIENT_ID -r sut -s attributes.'"pkce.code.challenge.method"'=S256 || true

ensure_user() {
  USERNAME="$1"; PASSWORD="$2"; shift 2; ROLES="$*"
  USER_JSON=$($KCADM get users -r sut -q username="$USERNAME")
  USER_ID=$(echo "$USER_JSON" | sed -n 's/.*"id" : "\([^"]*\)".*/\1/p')
  if [ -z "$USER_ID" ]; then
    echo "[kc-setup] create user $USERNAME"
    USER_ID=$($KCADM create users -r sut -s username="$USERNAME" -s enabled=true -i)
  else
    $KCADM update users/$USER_ID -r sut -s enabled=true || true
  fi
  $KCADM set-password -r sut --username "$USERNAME" --new-password "$PASSWORD" --temporary=false || true
  # assign roles
  for r in $ROLES; do
    $KCADM add-roles -r sut --uusername "$USERNAME" --rolename "$r" >/dev/null 2>&1 || true
  done
}

ensure_user dev dev directory.read directory.write directory.merge directory.pii.read
ensure_user admin admin123 directory.read directory.write directory.pii.read
ensure_user manager manager123 directory.read directory.write
ensure_user analyst analyst123 directory.read

echo "[kc-setup] done"

