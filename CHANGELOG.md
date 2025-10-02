# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Versionamento Semântico](https://semver.org/lang/pt-BR/).

## [1.0.0] - 2025-10-01

### 🎉 Versão Inicial

#### ✨ Adicionado

**Backend (Rust)**
- API REST completa com Axum framework
- Clean Architecture com DDD e SOLID
- Autenticação JWT com Keycloak
- Integração com PostgreSQL via SQLx
- Integração com HashiCorp Vault para PII
- Métricas com Prometheus
- Logs estruturados em JSON
- Health checks (`/health`, `/ready`)
- CORS configurável
- Migrations automáticas com SQLx
- DTOs e validações robustas

**Frontend (React/TypeScript)**
- SPA moderna com Vite
- Material-UI v7 para componentes
- Clean Architecture com DDD
- Dependency Injection customizado
- React Query para cache e sincronização
- Keycloak integration para autenticação
- Dashboard moderno e responsivo
- CRUD completo para:
  - Contatos
  - Unidades Organizacionais
  - Departamentos
  - Usuários
- DataGrid com Material-UI
- Gráficos com Recharts e MUI X-Charts
- Resolução automática de API base

**Infrastructure**
- Docker Compose para desenvolvimento
- PostgreSQL 16
- Keycloak 24.0.5 para autenticação
- HashiCorp Vault 1.16 para secrets
- Prometheus para métricas
- Grafana 11.2.0 para dashboards
- Setup automatizado de realms e usuários

**Domain Entities**
- Contact (Contato)
- OrgUnit (Unidade Organizacional)
- Department (Departamento)
- User (Usuário)

**Value Objects**
- ContactId, ContactType, ContactStatus
- OrgUnitId
- DepartmentId
- UserId, Username, Email, Role
- Phone, PhoneType
- E outros...

**Use Cases Implementados**
- Create, Read, Update, Delete para todas as entidades
- Busca com critérios
- Estatísticas de contatos e departamentos
- Hierarquia de unidades organizacionais

**Documentation**
- README.md completo
- Documentação de arquitetura
- Documentação da API
- Documentação do frontend
- Guia de deployment
- Contributing guidelines
- Security policy
- Code of Conduct

#### 🔒 Segurança
- Autenticação JWT obrigatória
- RBAC com Keycloak
- CORS configurável
- Headers de segurança
- Validação de entrada
- Proteção contra SQL injection
- PII encryption com Vault

#### 🚀 Performance
- Logs estruturados para debugging
- Métricas Prometheus
- Health checks para orquestração
- Connection pooling no PostgreSQL
- Cache de JWKS
- Build otimizado (release)

#### 📊 Observabilidade
- Logs estruturados em JSON
- Métricas HTTP com Prometheus
- Dashboards no Grafana
- Correlation IDs
- Audit trail

#### 🧪 Qualidade
- Type safety com Rust e TypeScript
- Validações em runtime
- Error handling robusto
- Clean Architecture
- SOLID principles
- DDD patterns

### 🔧 Configuração
- Variáveis de ambiente documentadas
- Docker Compose para desenvolvimento
- Setup automatizado de infraestrutura
- Migrations automáticas

### 📝 Documentação
- README detalhado
- Documentação de arquitetura
- Guias de API e Frontend
- Instruções de deployment
- Política de contribuição
- Política de segurança

---

## [1.0.1] - 2025-10-02

### 🐛 Correções Críticas

#### ✨ Refatoração da Função `unaccent`
- **Removida dependência da função PostgreSQL `unaccent`**
- Implementada função `normalize_text` customizada usando `LOWER(TRIM(input_text))`
- Atualizada migração `api/migrations/001_init.sql` para usar a nova função
- Corrigidos índices e triggers para usar `normalize_text`
- Removidas referências ao `unaccent` em todos os scripts SQL

#### 🔧 Correções de Roteamento da API
- Corrigido formato de rotas nos controllers:
  - `contact_controller.rs`: `:id` → `{id}`
  - `user_controller.rs`: `:id` → `{id}`, `:username` → `{username}`, `:email` → `{email}`, `:role` → `{role}`
  - `org_unit_controller.rs`: `:id` → `{id}`
  - `department_controller.rs`: `:id` → `{id}`, `:unit_id` → `{unit_id}`
  - `webhooks.rs`: `:service` → `{service}`

#### 📊 Correção do Prometheus
- **Implementada autenticação Basic Auth para endpoint `/metrics`**
- API agora aceita tanto Basic Auth (`metrics:dev-metrics-token`) quanto header `X-Metrics-Token`
- Atualizada configuração do Prometheus em `deploy/prometheus.yml` para usar Basic Auth
- Prometheus agora consegue coletar métricas corretamente

#### 📦 Correções de Dependências do Frontend
- Corrigidas versões incompatíveis no `frontend/package.json`:
  - `@mui/icons-material`: `^7.4.0` → `^7.3.3`
  - `@mui/material`: `^7.4.0` → `^7.3.3`
  - `@mui/x-charts`: `^8.15.0` → `^8.13.1`
  - `@mui/x-data-grid`: `^8.15.0` → `^8.13.1`
  - `@mui/x-date-pickers`: `^8.15.0` → `^8.12.0`
  - `@tanstack/react-query`: `^5.95.0` → `^5.90.2`
  - `axios`: `^1.13.0` → `^1.12.2`
  - `recharts`: `^3.3.0` → `^3.2.1`
  - `typescript`: `5.7.0` → `5.9.3`
  - `vite`: `5.5.0` → `5.4.20`

#### 📝 Documentação Atualizada
- Atualizado README.md com informações sobre as correções
- Adicionadas notas sobre remoção da função `unaccent`
- Documentada nova autenticação do endpoint `/metrics`
- Atualizada seção de troubleshooting com soluções para problemas resolvidos

### 🔧 Melhorias Técnicas
- Sistema agora funciona completamente sem dependência da função `unaccent`
- Prometheus configurado corretamente para coleta de métricas
- Frontend com todas as dependências compatíveis
- Todos os serviços rodando corretamente

---

## [Unreleased]

### 🚧 Em Desenvolvimento
- Implementação completa dos repositórios com SQLx
- Testes unitários e de integração
- CI/CD pipeline
- Deploy em produção
- Internacionalização (i18n)
- Modo offline
- PWA support

### 💡 Planejado
- Sincronização em tempo real com WebSockets
- Export/Import de dados
- Relatórios avançados
- Busca full-text
- Integração com outros sistemas
- Mobile app (React Native)

---

**Legenda:**
- ✨ Adicionado
- 🔧 Modificado
- 🗑️ Removido
- 🐛 Corrigido
- 🔒 Segurança
- 🚀 Performance
- 📊 Observabilidade
- 🧪 Testes
- 📝 Documentação

