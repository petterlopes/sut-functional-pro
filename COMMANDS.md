# 🛠️ Comandos Úteis - SUT

Este documento contém comandos úteis para desenvolvimento e manutenção do projeto SUT.

## 📦 Docker & Docker Compose

### Iniciar todos os serviços
```bash
cd deploy
docker compose -f docker-compose.dev.yml up -d
```

### Iniciar com rebuild forçado
```bash
docker compose -f docker-compose.dev.yml up -d --build
```

### Parar todos os serviços
```bash
docker compose -f docker-compose.dev.yml down
```

### Parar e remover volumes
```bash
docker compose -f docker-compose.dev.yml down -v
```

### Ver logs de todos os serviços
```bash
docker compose -f docker-compose.dev.yml logs -f
```

### Ver logs de um serviço específico
```bash
docker compose -f docker-compose.dev.yml logs -f api
docker compose -f docker-compose.dev.yml logs -f frontend
docker compose -f docker-compose.dev.yml logs -f postgres
```

### Reiniciar um serviço específico
```bash
docker compose -f docker-compose.dev.yml restart api
```

### Acessar shell de um container
```bash
docker compose -f docker-compose.dev.yml exec api sh
docker compose -f docker-compose.dev.yml exec postgres psql -U sut
```

### Ver status dos serviços
```bash
docker compose -f docker-compose.dev.yml ps
```

### Limpar recursos não utilizados
```bash
docker system prune -af
docker volume prune -f
```

## 🦀 Rust API

### Compilar em modo desenvolvimento
```bash
cd api
cargo build
```

### Compilar em modo release
```bash
cargo build --release
```

### Executar a API
```bash
cargo run
```

### Executar com logs detalhados
```bash
RUST_LOG=debug cargo run
```

### Executar testes
```bash
cargo test
```

### Verificar código (sem compilar)
```bash
cargo check
```

### Formatar código
```bash
cargo fmt
```

### Lint (clippy)
```bash
cargo clippy
```

### Executar migrations
```bash
sqlx migrate run
```

### Reverter última migration
```bash
sqlx migrate revert
```

### Criar nova migration
```bash
sqlx migrate add nome_da_migration
```

### Preparar queries SQLx (para build offline)
```bash
cargo sqlx prepare
```

## ⚛️ Frontend React

### Instalar dependências
```bash
cd frontend
npm install
# ou
pnpm install
```

### Executar servidor de desenvolvimento
```bash
npm run dev
```

### Build para produção
```bash
npm run build
```

### Preview da build de produção
```bash
npm run preview
```

### Gerar SDK da API (OpenAPI)
```bash
npm run gen:sdk
```

### Lint
```bash
npm run lint
```

### Type check
```bash
npm run type-check
```

## 🗄️ PostgreSQL

### Conectar ao banco
```bash
docker compose -f docker-compose.dev.yml exec postgres psql -U sut
```

### Executar query direta
```bash
docker compose -f docker-compose.dev.yml exec postgres psql -U sut -c "SELECT * FROM contacts LIMIT 10;"
```

### Backup do banco
```bash
docker compose -f docker-compose.dev.yml exec postgres pg_dump -U sut sut > backup.sql
```

### Restaurar backup
```bash
cat backup.sql | docker compose -f docker-compose.dev.yml exec -T postgres psql -U sut sut
```

### Ver tamanho do banco
```bash
docker compose -f docker-compose.dev.yml exec postgres psql -U sut -c "SELECT pg_size_pretty(pg_database_size('sut'));"
```

## 🔐 Keycloak

### Acessar console admin
```bash
# Abra http://localhost:8081
# Login: admin / admin
```

### Exportar realm
```bash
docker compose -f docker-compose.dev.yml exec keycloak /opt/keycloak/bin/kc.sh export --file /tmp/realm-export.json
docker compose -f docker-compose.dev.yml cp keycloak:/tmp/realm-export.json ./realm-export.json
```

### Criar novo usuário via CLI
```bash
docker compose -f docker-compose.dev.yml exec keycloak /opt/keycloak/bin/kcadm.sh create users \
  -r sut \
  -s username=novo_usuario \
  -s enabled=true
```

## 🔒 Vault

### Acessar CLI do Vault
```bash
docker compose -f docker-compose.dev.yml exec vault vault login root
```

### Listar secrets
```bash
docker compose -f docker-compose.dev.yml exec vault vault kv list secret/
```

### Ler um secret
```bash
docker compose -f docker-compose.dev.yml exec vault vault kv get secret/pii
```

### Criar um secret
```bash
docker compose -f docker-compose.dev.yml exec vault vault kv put secret/test value=teste
```

## 📊 Prometheus & Grafana

### Verificar targets do Prometheus
```bash
# Abra http://localhost:9090/targets
```

### Executar query no Prometheus
```bash
# Abra http://localhost:9090/graph
# Exemplo de query: http_requests_total
```

### Acessar Grafana
```bash
# Abra http://localhost:3000
# Login: admin / admin
```

## 🧹 Limpeza

### Limpar artefatos de build Rust
```bash
cd api
cargo clean
```

### Limpar node_modules
```bash
cd frontend
rm -rf node_modules pnpm-lock.yaml
```

### Limpar tudo (Docker + builds)
```bash
# Para containers
docker compose -f deploy/docker-compose.dev.yml down -v

# Limpa builds
cd api && cargo clean && cd ..
cd frontend && rm -rf node_modules dist .vite && cd ..

# Limpa Docker
docker system prune -af
docker volume prune -f
```

## 🔍 Debugging

### Ver logs da API em tempo real
```bash
docker compose -f deploy/docker-compose.dev.yml logs -f api
```

### Ver métricas da API
```bash
curl http://localhost:8080/metrics
```

### Health check da API
```bash
curl http://localhost:8080/health
curl http://localhost:8080/ready
```

### Testar endpoint com autenticação
```bash
# Com X-Dev-User header (desenvolvimento)
curl -H "X-Dev-User: admin" http://localhost:8080/v1/contacts

# Com JWT
TOKEN="seu_token_jwt"
curl -H "Authorization: Bearer $TOKEN" http://localhost:8080/v1/contacts
```

### Inspecionar container
```bash
docker compose -f deploy/docker-compose.dev.yml exec api sh
```

### Ver variáveis de ambiente
```bash
docker compose -f deploy/docker-compose.dev.yml exec api env
```

## 📝 Git

### Verificar status
```bash
git status
```

### Criar branch
```bash
git checkout -b feature/nova-funcionalidade
```

### Commit
```bash
git add .
git commit -m "feat: adiciona nova funcionalidade"
```

### Push
```bash
git push origin feature/nova-funcionalidade
```

### Atualizar branch
```bash
git pull origin main
```

## 🚀 Deploy

### Build da API para produção
```bash
cd api
cargo build --release
```

### Build do frontend para produção
```bash
cd frontend
npm run build
```

### Criar imagens Docker
```bash
docker build -f api/Dockerfile -t sut-api:latest .
docker build -f frontend/Dockerfile -t sut-frontend:latest .
```

---

**Dica:** Adicione alias no seu shell para comandos mais usados!

```bash
# Exemplo de alias para .bashrc ou .zshrc
alias sut-up='cd deploy && docker compose -f docker-compose.dev.yml up -d'
alias sut-down='cd deploy && docker compose -f docker-compose.dev.yml down'
alias sut-logs='cd deploy && docker compose -f docker-compose.dev.yml logs -f'
alias sut-api='cd deploy && docker compose -f docker-compose.dev.yml logs -f api'
```

