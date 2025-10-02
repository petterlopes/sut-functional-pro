# SUT - Sistema Único de Telefonia

[![Rust](https://img.shields.io/badge/Rust-1.84-orange?logo=rust)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue?logo=typescript)](https://www.typescriptlang.org/)
[![React](https://img.shields.io/badge/React-18.x-blue?logo=react)](https://reactjs.org/)
[![Vite](https://img.shields.io/badge/Vite-5.x-purple?logo=vite)](https://vitejs.dev/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-blue?logo=postgresql)](https://www.postgresql.org/)
[![Docker](https://img.shields.io/badge/Docker-Compose-blue?logo=docker)](https://www.docker.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## 🏗️ Arquitetura Clean Architecture

Este projeto implementa uma solução completa de diretório corporativo seguindo os princípios de **Clean Architecture**, **Domain-Driven Design (DDD)** e **SOLID**, tanto no backend (Rust) quanto no frontend (React/TypeScript).

### 🎯 Visão Geral

O SUT é um sistema full-stack moderno que combina:
- **Backend**: API REST em Rust com Axum, SQLx e PostgreSQL
- **Frontend**: SPA React/TypeScript com Vite e Material-UI
- **Infraestrutura**: PostgreSQL, Keycloak, Vault, Prometheus/Grafana

### 🏛️ Arquitetura do Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                    PRESENTATION LAYER                       │
├─────────────────────────────────────────────────────────────┤
│  Frontend (React/TS)    │    Backend (Rust/Axum)           │
│  - Components           │    - Controllers                 │
│  - Hooks                │    - Routes                      │
│  - Pages                │    - Middleware                  │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER                        │
├─────────────────────────────────────────────────────────────┤
│  Frontend (React/TS)    │    Backend (Rust)                │
│  - Use Cases            │    - Use Cases                   │
│  - Services             │    - DTOs                        │
│  - Dependency Injection │    - Application Services        │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                      DOMAIN LAYER                           │
├─────────────────────────────────────────────────────────────┤
│  Frontend (React/TS)    │    Backend (Rust)                │
│  - Entities             │    - Entities                    │
│  - Value Objects        │    - Value Objects               │
│  - Repository Interfaces│    - Repository Interfaces       │
│  - Domain Services      │    - Domain Services             │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                  INFRASTRUCTURE LAYER                       │
├─────────────────────────────────────────────────────────────┤
│  Frontend (React/TS)    │    Backend (Rust)                │
│  - API Clients          │    - Repository Implementations  │
│  - HTTP Services        │    - Database Access             │
│  - External Services    │    - External Integrations       │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Início Rápido

### Pré-requisitos
- Docker & Docker Compose v2
- Node.js 18+ (para desenvolvimento local)
- Rust (cargo) para desenvolvimento do backend

### Executando o Sistema

1. **Subir toda a infraestrutura:**
```bash
cd deploy
docker compose -f docker-compose.dev.yml up --build
```

2. **Acessos principais:**
- 🌐 **Frontend**: http://localhost:5173
- 🔧 **API**: http://localhost:8080
- 🔐 **Keycloak**: http://localhost:8081 (admin/admin)
- 📊 **Grafana**: http://localhost:3000 (admin/admin)
- 📈 **Prometheus**: http://localhost:9090
- 🔒 **Vault**: http://localhost:8200 (token: root)

3. **Usuários de desenvolvimento:**
- `admin/admin` - Administrador do sistema
- `dev/dev` - Usuário de desenvolvimento

### ⚠️ Notas Importantes

- **Função `unaccent` removida**: O projeto foi refatorado para não depender mais da função PostgreSQL `unaccent`. Agora usa uma função `normalize_text` customizada.
- **Autenticação de métricas**: O endpoint `/metrics` requer autenticação Basic Auth com credenciais `metrics:dev-metrics-token`.
- **Versões corrigidas**: Todas as dependências do frontend foram atualizadas para versões compatíveis.

## 📁 Estrutura do Projeto

```
sut-functional-pro/
├── api/                          # Backend Rust - Clean Architecture
│   ├── src/
│   │   ├── domain/               # Camada de Domínio
│   │   │   ├── entities.rs       # Entidades de negócio
│   │   │   ├── value_objects.rs  # Objetos de valor
│   │   │   ├── repositories.rs   # Interfaces de repositório
│   │   │   └── errors.rs         # Erros de domínio
│   │   ├── application/          # Camada de Aplicação
│   │   │   ├── use_cases/        # Casos de uso
│   │   │   └── dto.rs            # Data Transfer Objects
│   │   ├── infrastructure/       # Camada de Infraestrutura
│   │   │   ├── repositories/     # Implementações de repositório
│   │   │   └── mappers.rs        # Mapeadores de dados
│   │   ├── presentation/         # Camada de Apresentação
│   │   │   ├── clean/            # Controladores Clean Architecture
│   │   │   └── auth.rs           # Middleware de autenticação
│   │   └── main.rs               # Ponto de entrada
│   ├── migrations/               # Migrações do banco de dados
│   └── Cargo.toml                # Dependências Rust
├── frontend/                     # Frontend React - Clean Architecture
│   ├── src/
│   │   ├── domain/               # Camada de Domínio
│   │   │   ├── entities/         # Entidades TypeScript
│   │   │   └── repositories/     # Interfaces de repositório
│   │   ├── application/          # Camada de Aplicação
│   │   │   ├── use-cases/        # Casos de uso
│   │   │   └── services/         # Serviços de aplicação
│   │   ├── infrastructure/       # Camada de Infraestrutura
│   │   │   ├── api/              # Clientes HTTP
│   │   │   └── di/               # Injeção de dependência
│   │   ├── presentation/         # Camada de Apresentação
│   │   │   ├── components/       # Componentes React
│   │   │   ├── pages/            # Páginas
│   │   │   └── hooks/            # Hooks customizados
│   │   └── main.tsx              # Ponto de entrada
│   └── package.json              # Dependências Node.js
├── deploy/                       # Configurações de deployment
│   ├── docker-compose.dev.yml    # Compose para desenvolvimento
│   ├── keycloak/                 # Configurações do Keycloak
│   └── grafana/                  # Dashboards do Grafana
└── openapi.yaml                  # Especificação OpenAPI
```

## 🔧 Desenvolvimento

### Backend (Rust)

#### Estrutura Clean Architecture

**Domain Layer** (`api/src/domain/`):
- **Entities**: `Contact`, `OrgUnit`, `Department`, `User`
- **Value Objects**: `ContactId`, `Email`, `Phone`, `OrgUnitId`
- **Repository Interfaces**: Abstrações para acesso a dados
- **Domain Errors**: Erros específicos do domínio

**Application Layer** (`api/src/application/`):
- **Use Cases**: Lógica de negócio isolada
- **DTOs**: Objetos de transferência de dados
- **Services**: Serviços de aplicação

**Infrastructure Layer** (`api/src/infrastructure/`):
- **Repository Implementations**: Implementações concretas
- **Mappers**: Conversão entre domínio e persistência
- **External Services**: Integrações externas

**Presentation Layer** (`api/src/presentation/`):
- **Controllers**: Endpoints REST
- **Routes**: Definição de rotas
- **Middleware**: Autenticação, CORS, etc.

#### Comandos Úteis

```bash
# Executar testes
cargo test

# Compilar
cargo build

# Executar localmente
cargo run

# Verificar código
cargo clippy
cargo fmt
```

### Frontend (React/TypeScript)

#### Estrutura Clean Architecture

**Domain Layer** (`frontend/src/domain/`):
- **Entities**: Interfaces TypeScript para entidades
- **Repository Interfaces**: Contratos para acesso a dados

**Application Layer** (`frontend/src/application/`):
- **Use Cases**: Lógica de aplicação
- **Services**: Serviços de aplicação
- **Dependency Injection**: Container de dependências

**Infrastructure Layer** (`frontend/src/infrastructure/`):
- **API Clients**: Clientes HTTP (Axios)
- **Repository Implementations**: Implementações concretas

**Presentation Layer** (`frontend/src/presentation/`):
- **Components**: Componentes React
- **Pages**: Páginas da aplicação
- **Hooks**: Hooks customizados

#### Comandos Úteis

```bash
# Instalar dependências
npm install

# Executar em desenvolvimento
npm run dev

# Compilar para produção
npm run build

# Gerar SDK TypeScript
npm run gen:sdk
```

## 🔐 Segurança

### Autenticação e Autorização
- **Keycloak**: Provedor de identidade OIDC
- **JWT**: Tokens RS256 com validação de issuer/audience
- **RBAC**: Controle de acesso baseado em roles
- **Middleware**: Proteção automática de rotas

### Variáveis de Ambiente

#### Backend
| Vari?vel | Obrigat?rio em produ??o | Descri??o |
| --- | --- | --- |
| `PG_DSN` | Sim | String de conex?o PostgreSQL |
| `CORS_ALLOWED_ORIGINS` | Sim | Lista de origens permitidas (separadas por v?rgula) |
| `KEYCLOAK_ISSUER` / `KEYCLOAK_JWKS` / `KEYCLOAK_AUDIENCE` | Sim | Configura??o OIDC para valida??o de JWT |
| `VAULT_ADDR` | Sim | URL do HashiCorp Vault |
| `VAULT_TOKEN` | Sim | Token emitido pelo Vault (nunca use o token dev em produ??o) |
| `METRICS_TOKEN` | Sim | Token compartilhado exigido pelo endpoint `/metrics` |
| `WEBHOOK_SHARED_SECRET` | Sim | Segredo compartilhado exigido por todos os webhooks |
| `DEV_AUTH_BYPASS` | N?o | Ative (`1`) apenas em desenvolvimento com `RUST_ENV != production` |

> **Dica**: mantenha os segredos fora do `.env`. Use um gerenciador seguro (Vault, AWS Secrets Manager, etc.) e injete no runtime.

#### Frontend
```bash
VITE_KC_URL=http://localhost:8081
VITE_KC_REALM=sut
VITE_KC_CLIENT=sut-frontend
VITE_API_BASE=http://localhost:8080
```

Observa??es importantes:
- As senhas de usu?rios de demonstra??o no Keycloak s?o tempor?rias; o primeiro login exige troca imediata.
- O seed SQL `003_seed_users.sql` s? insere usu?rios de exemplo quando o GUC `app.enable_demo_users` estiver `on`. Produ??es n?o devem habilitar esse flag.
- `DEV_AUTH_BYPASS=1` apenas injeta claims sint?ticas quando o cabe?alho `X-Dev-User` est? presente e `RUST_ENV` n?o ? `production`.

## 📊 Observabilidade

### Métricas
- **Prometheus**: Coleta de métricas com autenticação Basic Auth
- **Grafana**: Dashboards e visualizações
- **Endpoint**: `/metrics` (requer Basic Auth: `metrics:dev-metrics-token` ou header `X-Metrics-Token`)
- **Configuração**: Prometheus configurado para usar Basic Auth automaticamente

### Logs
- **Structured Logging**: Logs estruturados em JSON
- **Correlation IDs**: Rastreamento de requisições
- **Audit Trail**: Registro de ações críticas

## 🧪 Testes

### Backend
```bash
# Executar todos os testes
cargo test

# Testes com cobertura
cargo tarpaulin

# Testes de integração
cargo test --test integration
```

### Frontend
```bash
# Executar testes
npm test

# Testes com cobertura
npm run test:coverage

# Testes E2E
npm run test:e2e
```

## 🚀 Deployment

### Desenvolvimento
```bash
cd deploy
docker compose -f docker-compose.dev.yml up --build
```

### Produção
```bash
cd deploy
docker compose -f docker-compose.prod.yml up -d
```

## 📚 Documentação da API

A API segue a especificação OpenAPI 3.1.0. A documentação está disponível em:
- **Swagger UI**: http://localhost:8080/docs
- **OpenAPI Spec**: `openapi.yaml`

### Endpoints Principais

#### Contatos
- `GET /v1/contacts` - Listar contatos
- `POST /v1/contacts` - Criar contato
- `GET /v1/contacts/:id` - Buscar contato
- `PATCH /v1/contacts/:id` - Atualizar contato
- `DELETE /v1/contacts/:id` - Remover contato
- `GET /v1/contacts/statistics` - Estatísticas

#### Unidades Organizacionais
- `GET /v1/org-units` - Listar unidades
- `POST /v1/org-units` - Criar unidade
- `GET /v1/org-units/:id` - Buscar unidade
- `PATCH /v1/org-units/:id` - Atualizar unidade
- `DELETE /v1/org-units/:id` - Remover unidade

#### Departamentos
- `GET /v1/departments` - Listar departamentos
- `POST /v1/departments` - Criar departamento
- `GET /v1/departments/:id` - Buscar departamento
- `PATCH /v1/departments/:id` - Atualizar departamento
- `DELETE /v1/departments/:id` - Remover departamento

#### Usuários
- `GET /v1/users` - Listar usuários
- `POST /v1/users` - Criar usuário
- `GET /v1/users/:id` - Buscar usuário
- `PATCH /v1/users/:id` - Atualizar usuário
- `DELETE /v1/users/:id` - Remover usuário

## 🔧 Troubleshooting

### Problemas Comuns

#### API não inicia
- Verifique se o PostgreSQL está rodando
- Confirme as variáveis de ambiente
- Verifique os logs: `docker logs deploy-api-1`
- **Erro de migração**: Se houver erro com função `unaccent`, o projeto foi refatorado para usar `normalize_text`

#### Frontend não carrega
- Verifique se o Keycloak está acessível
- Confirme as variáveis `VITE_*`
- Verifique o console do navegador
- **Erro de dependências**: Todas as versões foram corrigidas para compatibilidade

#### Prometheus não coleta métricas
- Verifique se a API está rodando
- Confirme que o endpoint `/metrics` está acessível
- Teste com: `curl -u metrics:dev-metrics-token http://localhost:8080/metrics`
- Verifique a configuração do Prometheus em `deploy/prometheus.yml`

#### Autenticação falha
- Verifique a configuração do Keycloak
- Confirme os tokens JWT
- Verifique os logs de autenticação

### Logs Úteis
```bash
# Logs da API
docker logs deploy-api-1 -f

# Logs do Frontend
docker logs deploy-frontend-1 -f

# Logs do Keycloak
docker logs deploy-keycloak-1 -f
```

## 🤝 Contribuição

### Padrões de Código

#### Rust
- Siga as convenções do `rustfmt`
- Use `clippy` para linting
- Escreva testes para novas funcionalidades
- Documente APIs públicas

#### TypeScript/React
- Use ESLint e Prettier
- Siga as convenções do projeto
- Escreva testes unitários
- Use TypeScript strict mode

### Fluxo de Desenvolvimento
1. Fork do repositório
2. Crie uma branch para sua feature
3. Implemente seguindo Clean Architecture
4. Adicione testes
5. Submeta um Pull Request

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo `LICENSE` para mais detalhes.

## 🆘 Suporte

Para suporte e dúvidas:
- Abra uma issue no GitHub
- Consulte a documentação da API
- Verifique os logs do sistema

---

**Desenvolvido com ❤️ seguindo os princípios de Clean Architecture e Domain-Driven Design**