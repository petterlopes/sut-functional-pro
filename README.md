```markdown
# SUT - Sistema Único de Telefonia (Modo de desenvolvimento)

Este repositório contém um protótipo full-stack (API Rust + Frontend React) e infraestrutura de apoio (Postgres, Keycloak, Vault, Prometheus/Grafana, OTEL Collector, Jaeger/Tempo) para uma solução de diretório corporativo com ingestão assinada e observabilidade.

## Sumário rápido
- Local dev: `docker compose -f deploy/docker-compose.dev.yml up --build`
- Frontend dev: `pnpm --prefix frontend dev` (ou `npm run dev --prefix frontend`)
- API: `cd api && cargo run` (ou via Docker Compose)

## Estrutura
- `api/` – backend em Rust (Axum, SQLx). Código, migrations e infra.
- `frontend/` – SPA React + Vite; usa Keycloak JS Adapter para SSO.
- `deploy/` – compose e configs (Keycloak realm, OTEL, Grafana dashboards).
- `openapi.yaml` – contrato OpenAPI; gerador de SDK TypeScript usado pelo frontend.

## Requisitos
- Docker & Docker Compose v2
- Node 18+ (para dev local sem container) e pnpm/npm
- Rust (cargo) para desenvolvimento do backend

## Executando localmente (modo recomendado para desenvolvimento)
1. Subir todo o stack:

```powershell
cd deploy
docker compose -f docker-compose.dev.yml up --build
```

2. Acessos relevantes (ports mapeadas para o host):
- API: http://localhost:8080
- Frontend: http://localhost:5173 (Vite) — ver terminal do frontend se a porta estiver em uso (pode cair em 5174).
- Keycloak (admin UI): http://localhost:8081 (admin/admin)
- Vault: http://localhost:8200 (token: root)
- Grafana: http://localhost:3000 (admin/admin)

3. Usuários úteis no realm `sut`:
- admin/admin (Keycloak admin)
- dev/dev (usuário de desenvolvimento)

## Variáveis de ambiente importantes
- Backend (`api` container):
  - `PG_DSN` – string de conexão com o Postgres
  - `KEYCLOAK_ISSUER`, `KEYCLOAK_JWKS`, `KEYCLOAK_AUDIENCE` – configurações de validação JWT
  - `VAULT_ADDR`, `VAULT_TOKEN` – integração com Vault
  - `METRICS_TOKEN` – cabeçalho exigido em `/metrics` (opcional)
- Frontend (vite):
  - `VITE_KC_URL` – URL do Keycloak; para desenvolvimento no host usar `http://localhost:8081`; dentro do Compose usar `http://keycloak:8080`.
  - `VITE_KC_REALM`, `VITE_KC_CLIENT`, `VITE_API_BASE`

Observação: o frontend detecta se `VITE_KC_URL` está presente; caso contrário assume `http://localhost:8081` para evitar problemas de DNS quando o navegador roda no host.

## Segurança e boas práticas (resumo operativo)
- Nunca comite segredos (tokens, senhas, chaves privadas) no repositório.
- Use o Vault para armazenar segredos de produção e leads; em dev o Vault roda em modo dev com token `root` apenas para conveniência.
- Tokens JWT devem ser validados pelo backend: issuer, audience, exp/nbf/iat e leeway. Há proteção para atualização de JWKS com retry/backoff.
- Endpoints sensíveis devem registrar auditoria com o actor (`sub` do token).

## Troubleshooting rápido
- Se o frontend ficar com mensagem de autenticação travada: verifique `VITE_KC_URL` e se o Keycloak está acessível host:8081
- Se o API falhar ao iniciar por falha ao obter JWKS: o serviço agora faz retries; verifique logs de startup e conectividade com Keycloak (porta 8081 no host ou `http://keycloak:8080` em Compose).

## Testes e CI
- Backend: `cargo test --locked`
- Frontend: `pnpm --prefix frontend test` (se houver; no protótipo, use `pnpm --prefix frontend build`)

## Próximos passos sugeridos (engineer + security)
1. Adicionar integração contínua (GitHub Actions) com checagens: fmt, clippy, cargo audit, pnpm audit, build verification.
2. Automatizar provisionamento de Keycloak/Vault em scripts idempotentes. Não confiar em import manual em produção.
3. Implementar secret scanning (git secrets / pre-commit) e dependabot para dependências.

---

Consulte `SECURITY.md` para políticas de divulgação e manuseio de segredos.

```
# SUT - Sistema Funcional Completo

## Visao Geral
A stack combina backend em Rust (Axum + SQLx), frontend React/Vite e infraestrutura de apoio (PostgreSQL, Keycloak, Vault, Prometheus/Grafana, OTEL Collector, Jaeger, Tempo). O objetivo e servir um diretorio corporativo com ingestao assinada de eventos e observabilidade completa.

## Arquitetura
- **API (`api/`)**: servico Axum com rastreamento OTLP/Prometheus, pool SQLx, migracoes SQL puras, auditoria e integracao com Vault.
- **Frontend (`frontend/`)**: SPA Vite/React que consome a API via client gerado automaticamente a partir do `openapi.yaml`.
- **Seguranca**: Keycloak fornece tokens JWT; a API valida issuer/audience/leeway e aplica autorizacao por escopos. Webhooks usam HMAC armazenado no Vault.
- **Observabilidade**: metricas expostas em `/metrics`, tracing via OTLP -> Collector -> Jaeger/Tempo, dashboards Grafana.

## Requisitos
- Docker / Docker Compose v2
- Portas 5432 (Postgres), 8080, 5173, 8081, 3000, 9090, 16686, 3200 livres

## Subindo o ambiente
```bash
cd deploy
docker compose -f docker-compose.dev.yml up --build
```
Servicos expostos:
- API: http://localhost:8080 (`/metrics` opcionalmente protegido por `METRICS_TOKEN`)
- Frontend: http://localhost:5173
- Keycloak: http://localhost:8081 (admin/admin, realm `sut`, usuario `dev/dev`)
- Vault: http://localhost:8200 (token `root`)
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3000 (admin/admin)
- Jaeger: http://localhost:16686
- Tempo: http://localhost:3200

## Backend
### Configuracao (variaveis principais)
- `PG_DSN`: string de conexao Postgres.
- `CORS_ALLOWED_ORIGINS`: lista separada por virgulas.
- `KEYCLOAK_ISSUER` / `KEYCLOAK_JWKS` / `KEYCLOAK_AUDIENCE`: controle de tokens.
- `JWT_LEEWAY_SECS`: tolerancia em segundos.
- `METRICS_TOKEN`: exige header `X-Metrics-Token` em `/metrics`.
- `VAULT_ADDR` / `VAULT_TOKEN`: habilitam acesso a segredos e Transit.
- `PG_CONNECT_ATTEMPTS` / `PG_CONNECT_BACKOFF_MS`: controle de retry na conexao ao Postgres (default 10 tentativas, 1s).

### Migracoes
- Arquivo unico `api/migrations/001_init.sql` cria extensoes (`pg_trgm`, `unaccent`, `pgcrypto`), tabelas de diretorio, indices GIN/Trigram, seeds iniciais e estruturas de auditoria/replay.
- Executadas em runtime pelo `infra::pg::migrate`, garantindo idempotencia.

### Autenticacao e Autorizacao
1. `presentation::auth::init` carrega JWKS, issuer, audiences e leeway.
2. `jwt_middleware` protege rotas (`/v1/org`, `/v1/search`, `/v1/contacts`), exige Bearer token RS256 e verifica claims obrigatorias (`exp`, `nbf`, `iat`).
3. Escopos sao checados por `shared::has_scope`, controlando leitura/escrita e acesso a PII.
4. O claim `sub` e propagado ao tracing para auditoria.

### Webhooks de Ingestao
- Endpoint `/v1/ingestion/events` valida HMAC `sha256=` usando segredo hex armazenado no Vault (`kv/webhook`).
- Janela de 5 minutos (`ts`) e replay protection via tabela `webhook_receipts (source, nonce)`.
- Respostas: `accepted` ou `duplicate`.

### Auditoria
- `infra::audit::log_audit` grava acoes com actor (`sub`), antes/depois e timestamp.
- Operacoes criticas (PATCH de documento) registram eventos.

### Observabilidade
- `axum_prometheus` serve metricas; TraceLayer e OTLP exportam spans.
- Dashboards pre-configurados em `deploy/grafana/provisioning`.

## Frontend
- Scripts principais: `pnpm dev`, `pnpm build`, `pnpm run gen:sdk` (gera client TypeScript via `openapi-typescript`).
- Ambiente injeta `VITE_KC_URL`, `VITE_KC_REALM`, `VITE_KC_CLIENT`, `VITE_API_BASE`.
- Componentes requisitam tokens via Keycloak JS Adapter e chamam API com escopos apropriados.

## Documentacao da API
- Spec OpenAPI em `openapi.yaml` (3.1.0) com rotas de contatos, org, search, ingestao.
- Frontend gera `src/api/schema.d.ts` automaticamente durante o compose.

## Testes e Builds
- Backend: `cargo test --locked`, `cargo build --release --locked`.
- Frontend: `pnpm install && pnpm build` (requer Node/PNPM locais ou container).
- Docker: `docker build -f api/Dockerfile .` para imagem isolada.

## Fluxos Comuns
### Obter Token
1. Acesse Keycloak (`http://localhost:8081`), use client `sut-frontend`.
2. Realize login via fluxos OAuth suportados e copie o JWT.
3. Consuma a API: `curl http://localhost:8080/v1/contacts -H "Authorization: Bearer <token>"`.

### Assinar Webhook
```bash
body='{"source":"crm-x","sourceKey":"contact:123","payload":{"event":"upsert"},"nonce":"<uuid>","ts":'$(date +%s)'}'
secret_hex=$(curl -s http://localhost:8200/v1/kv/data/webhook -H 'X-Vault-Token: root' | jq -r '.data.data.secret')
sig=$(node scripts/sign_webhook.js "$body" "$secret_hex")

curl -i http://localhost:8080/v1/ingestion/events \
  -H 'Content-Type: application/json' \
  -H "X-Signature: $sig" \
  -d "$body"
```

## Troubleshooting
- **API nao sobe**: verifique logs; erros comuns sao migracoes especificas do Postgres (necessario `pg_trgm`, `unaccent`, `pgcrypto`).
- **Prometheus mostra target DOWN**: se a API demorar a conectar em Postgres, ajuste `PG_CONNECT_*`.
- **Frontend falha em `gen:sdk`**: confirme que o volume `../openapi.yaml:/openapi.yaml:ro` esta montado.
- **Tokens invalidos**: cheque issuer/audience configurados no Keycloak e variaveis de ambiente do container `api`.

### Runtime overrides (dev)

Quando estiver desenvolvendo com o frontend rodando no seu browser e os outros serviços no Docker, use o painel de configurações (ícone de engrenagem no canto superior direito da UI) para definir overrides em tempo de execução:

- `API base` (localStorage key `sut_api_base`), por exemplo `http://localhost:8080`
- `Keycloak base` (localStorage key `sut_kc_base`), por exemplo `http://localhost:8081`

Use o botão "Probe" no diálogo para verificar `/health` do backend e a descoberta OIDC do Keycloak. Salvar grava os valores em localStorage e recarrega a página.

Alternativamente, defina `VITE_API_BASE` e `VITE_KC_URL` antes de rodar o dev server (`npm run dev` / `pnpm dev`) para aplicar os valores em tempo de build.

## Proximos Passos
- Adicionar testes de integracao para contatos/webhooks.
- Expandir o OpenAPI com schemas de resposta detalhados.
- Automatizar provisionamento de dados de Keycloak/Vault via scripts idempotentes.
