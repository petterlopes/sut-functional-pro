# Documentação da API SUT

## 🌐 Visão Geral

A API do Sistema Único de Telefonia (SUT) é uma API REST moderna construída em Rust com Axum, seguindo os princípios de Clean Architecture. A API fornece endpoints para gerenciamento de contatos, unidades organizacionais, departamentos e usuários.

## 🔗 Base URL

```
http://localhost:8080
```

## 🔐 Autenticação

A API utiliza autenticação JWT com Keycloak. Todas as rotas protegidas requerem um token Bearer válido.

### Headers de Autenticação

```http
Authorization: Bearer <jwt_token>
```

### Desenvolvimento (Fallback)

Para desenvolvimento, você pode usar o header `X-Dev-User`:

```http
X-Dev-User: admin
```

## 📊 Endpoints

### 🏥 Health Check

#### GET /health
Verifica se a API está funcionando.

**Resposta:**
```http
HTTP/1.1 200 OK
Content-Type: text/plain

ok
```

#### GET /ready
Verifica se a API está pronta para receber requisições (banco de dados e JWKS carregados).

**Resposta:**
```http
HTTP/1.1 200 OK
Content-Type: text/plain

ok
```

**Erro:**
```http
HTTP/1.1 503 Service Unavailable
Content-Type: text/plain

db_down;jwks_missing;
```

### 📇 Contatos

#### GET /v1/contacts
Lista contatos com filtros opcionais.

**Query Parameters:**
- `full_name` (string, opcional): Filtro por nome completo
- `contact_type` (string, opcional): Tipo de contato (PERSON, COMPANY)
- `status` (string, opcional): Status do contato (ACTIVE, INACTIVE)
- `unit_id` (string, opcional): ID da unidade organizacional
- `department_id` (string, opcional): ID do departamento
- `limit` (integer, opcional): Limite de resultados (padrão: 50)
- `offset` (integer, opcional): Offset para paginação (padrão: 0)

**Exemplo de Requisição:**
```http
GET /v1/contacts?status=ACTIVE&limit=10
Authorization: Bearer <token>
```

**Resposta:**
```json
{
  "items": [
    {
      "id": "10000000-0000-0000-0000-000000000001",
      "fullName": "Alice Silva",
      "contactType": "PERSON",
      "status": "ACTIVE",
      "document": "12345678901",
      "unitId": "00000000-0000-0000-0000-000000000001",
      "departmentId": "00000000-0000-0000-0000-000000000101",
      "etag": "etag-value",
      "createdAt": "2025-01-01T00:00:00Z",
      "updatedAt": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1
}
```

#### POST /v1/contacts
Cria um novo contato.

**Body:**
```json
{
  "fullName": "João Silva",
  "contactType": "PERSON",
  "status": "ACTIVE",
  "document": "12345678901",
  "unitId": "00000000-0000-0000-0000-000000000001",
  "departmentId": "00000000-0000-0000-0000-000000000101"
}
```

**Resposta:**
```json
{
  "id": "10000000-0000-0000-0000-000000000002",
  "fullName": "João Silva",
  "contactType": "PERSON",
  "status": "ACTIVE",
  "document": "12345678901",
  "unitId": "00000000-0000-0000-0000-000000000001",
  "departmentId": "00000000-0000-0000-0000-000000000101",
  "etag": "new-etag-value",
  "createdAt": "2025-01-01T00:00:00Z",
  "updatedAt": "2025-01-01T00:00:00Z"
}
```

#### GET /v1/contacts/{id}
Busca um contato específico por ID.

**Path Parameters:**
- `id` (string): ID do contato

**Exemplo de Requisição:**
```http
GET /v1/contacts/10000000-0000-0000-0000-000000000001
Authorization: Bearer <token>
```

**Resposta:**
```json
{
  "id": "10000000-0000-0000-0000-000000000001",
  "fullName": "Alice Silva",
  "contactType": "PERSON",
  "status": "ACTIVE",
  "document": "12345678901",
  "unitId": "00000000-0000-0000-0000-000000000001",
  "departmentId": "00000000-0000-0000-0000-000000000101",
  "etag": "etag-value",
  "createdAt": "2025-01-01T00:00:00Z",
  "updatedAt": "2025-01-01T00:00:00Z"
}
```

#### PATCH /v1/contacts/{id}
Atualiza um contato existente.

**Path Parameters:**
- `id` (string): ID do contato

**Headers:**
```http
If-Match: <etag_value>
```

**Body:**
```json
{
  "fullName": "Alice Silva Santos",
  "status": "INACTIVE"
}
```

**Resposta:**
```json
{
  "id": "10000000-0000-0000-0000-000000000001",
  "fullName": "Alice Silva Santos",
  "contactType": "PERSON",
  "status": "INACTIVE",
  "document": "12345678901",
  "unitId": "00000000-0000-0000-0000-000000000001",
  "departmentId": "00000000-0000-0000-0000-000000000101",
  "etag": "new-etag-value",
  "createdAt": "2025-01-01T00:00:00Z",
  "updatedAt": "2025-01-01T12:00:00Z"
}
```

#### DELETE /v1/contacts/{id}
Remove um contato.

**Path Parameters:**
- `id` (string): ID do contato

**Resposta:**
```http
HTTP/1.1 204 No Content
```

#### GET /v1/contacts/statistics
Retorna estatísticas dos contatos.

**Resposta:**
```json
{
  "totalContacts": 150,
  "contactsByType": {
    "PERSON": 120,
    "COMPANY": 30
  },
  "contactsByStatus": {
    "ACTIVE": 140,
    "INACTIVE": 10
  }
}
```

### 🏢 Unidades Organizacionais

#### GET /v1/org-units
Lista unidades organizacionais.

**Query Parameters:**
- `name` (string, opcional): Filtro por nome
- `parent_id` (string, opcional): ID da unidade pai
- `limit` (integer, opcional): Limite de resultados
- `offset` (integer, opcional): Offset para paginação

**Resposta:**
```json
{
  "items": [
    {
      "id": "00000000-0000-0000-0000-000000000001",
      "name": "Diretoria",
      "parentId": null,
      "createdAt": "2025-01-01T00:00:00Z",
      "updatedAt": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1
}
```

#### POST /v1/org-units
Cria uma nova unidade organizacional.

**Body:**
```json
{
  "name": "Nova Unidade",
  "parentId": "00000000-0000-0000-0000-000000000001"
}
```

#### GET /v1/org-units/{id}
Busca uma unidade organizacional específica.

#### PATCH /v1/org-units/{id}
Atualiza uma unidade organizacional.

#### DELETE /v1/org-units/{id}
Remove uma unidade organizacional.

#### GET /v1/org-units/{id}/hierarchy
Retorna a hierarquia de uma unidade organizacional.

**Resposta:**
```json
[
  {
    "id": "00000000-0000-0000-0000-000000000001",
    "name": "Diretoria",
    "parentId": null,
    "level": 0
  },
  {
    "id": "00000000-0000-0000-0000-000000000002",
    "name": "Departamento de TI",
    "parentId": "00000000-0000-0000-0000-000000000001",
    "level": 1
  }
]
```

### 🏬 Departamentos

#### GET /v1/departments
Lista departamentos.

**Query Parameters:**
- `name` (string, opcional): Filtro por nome
- `unit_id` (string, opcional): ID da unidade organizacional
- `limit` (integer, opcional): Limite de resultados
- `offset` (integer, opcional): Offset para paginação

**Resposta:**
```json
{
  "items": [
    {
      "id": "00000000-0000-0000-0000-000000000101",
      "name": "Desenvolvimento",
      "unitId": "00000000-0000-0000-0000-000000000001",
      "createdAt": "2025-01-01T00:00:00Z",
      "updatedAt": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1
}
```

#### POST /v1/departments
Cria um novo departamento.

**Body:**
```json
{
  "name": "Novo Departamento",
  "unitId": "00000000-0000-0000-0000-000000000001"
}
```

#### GET /v1/departments/{id}
Busca um departamento específico.

#### PATCH /v1/departments/{id}
Atualiza um departamento.

#### DELETE /v1/departments/{id}
Remove um departamento.

#### GET /v1/departments/statistics
Retorna estatísticas dos departamentos.

**Resposta:**
```json
{
  "totalDepartments": 25,
  "departmentsByUnit": {
    "00000000-0000-0000-0000-000000000001": 10,
    "00000000-0000-0000-0000-000000000002": 15
  }
}
```

#### GET /v1/departments/by-unit/{unit_id}
Lista departamentos por unidade organizacional.

### 👥 Usuários

#### GET /v1/users
Lista usuários.

**Query Parameters:**
- `username` (string, opcional): Filtro por nome de usuário
- `email` (string, opcional): Filtro por email
- `role` (string, opcional): Filtro por role
- `limit` (integer, opcional): Limite de resultados
- `offset` (integer, opcional): Offset para paginação

**Resposta:**
```json
{
  "items": [
    {
      "id": "20000000-0000-0000-0000-000000000001",
      "username": "admin",
      "email": "admin@example.com",
      "roles": ["ADMIN", "USER"],
      "createdAt": "2025-01-01T00:00:00Z",
      "updatedAt": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1
}
```

#### POST /v1/users
Cria um novo usuário.

**Body:**
```json
{
  "username": "novo_usuario",
  "email": "usuario@example.com",
  "roles": ["USER"]
}
```

#### GET /v1/users/{id}
Busca um usuário específico.

#### PATCH /v1/users/{id}
Atualiza um usuário.

#### DELETE /v1/users/{id}
Remove um usuário.

#### GET /v1/users/by-username/{username}
Busca usuário por nome de usuário.

#### GET /v1/users/by-email/{email}
Busca usuário por email.

#### GET /v1/users/by-role/{role}
Lista usuários por role.

## 📊 Métricas

### GET /metrics
Retorna métricas do Prometheus (protegido por token).

**Headers:**
```http
X-Metrics-Token: <metrics_token>
```

**Resposta:**
```
# HELP http_requests_total Total number of HTTP requests
# TYPE http_requests_total counter
http_requests_total{method="GET",status="200"} 150
http_requests_total{method="POST",status="201"} 25
```

## 📚 Documentação OpenAPI

### GET /docs
Interface Swagger UI para documentação interativa da API.

### GET /openapi.json
Especificação OpenAPI 3.1.0 em formato JSON.

## 🚨 Códigos de Status HTTP

| Código | Descrição |
|--------|-----------|
| 200 | OK - Requisição bem-sucedida |
| 201 | Created - Recurso criado com sucesso |
| 204 | No Content - Recurso removido com sucesso |
| 400 | Bad Request - Dados inválidos |
| 401 | Unauthorized - Token inválido ou ausente |
| 403 | Forbidden - Sem permissão para acessar o recurso |
| 404 | Not Found - Recurso não encontrado |
| 409 | Conflict - Conflito (ex: ETag inválido) |
| 422 | Unprocessable Entity - Erro de validação |
| 500 | Internal Server Error - Erro interno do servidor |
| 503 | Service Unavailable - Serviço indisponível |

## 🔍 Filtros e Paginação

### Paginação
Todos os endpoints de listagem suportam paginação através dos parâmetros:
- `limit`: Número máximo de itens por página (padrão: 50, máximo: 100)
- `offset`: Número de itens a pular (padrão: 0)

### Ordenação
Alguns endpoints suportam ordenação através do parâmetro `sort`:
- `sort=field` - Ordenação ascendente
- `sort=-field` - Ordenação descendente

### Filtros
Cada endpoint suporta filtros específicos através de query parameters.

## 🔄 Controle de Concorrência

A API utiliza ETags para controle de concorrência otimista:

1. **GET**: Retorna o ETag no header `ETag`
2. **PATCH/PUT**: Requer o header `If-Match` com o ETag atual
3. **409 Conflict**: Retornado quando o ETag não confere

## 📝 Exemplos de Uso

### Criar um Contato
```bash
curl -X POST http://localhost:8080/v1/contacts \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "fullName": "João Silva",
    "contactType": "PERSON",
    "status": "ACTIVE",
    "document": "12345678901"
  }'
```

### Atualizar um Contato
```bash
curl -X PATCH http://localhost:8080/v1/contacts/10000000-0000-0000-0000-000000000001 \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -H "If-Match: <etag_value>" \
  -d '{
    "fullName": "João Silva Santos"
  }'
```

### Listar Contatos com Filtros
```bash
curl -X GET "http://localhost:8080/v1/contacts?status=ACTIVE&limit=10&offset=0" \
  -H "Authorization: Bearer <token>"
```

### Buscar Contato por ID
```bash
curl -X GET http://localhost:8080/v1/contacts/10000000-0000-0000-0000-000000000001 \
  -H "Authorization: Bearer <token>"
```

### Remover Contato
```bash
curl -X DELETE http://localhost:8080/v1/contacts/10000000-0000-0000-0000-000000000001 \
  -H "Authorization: Bearer <token>"
```

## 🧪 Testes

### Testes de Integração
```bash
# Executar todos os testes
cargo test

# Testes específicos
cargo test contact_controller

# Testes com logs
RUST_LOG=debug cargo test
```

### Testes de Carga
```bash
# Usando wrk
wrk -t12 -c400 -d30s http://localhost:8080/v1/contacts

# Usando ab
ab -n 1000 -c 10 http://localhost:8080/v1/contacts
```

## 🔧 Desenvolvimento

### Executar Localmente
```bash
# Configurar variáveis de ambiente
export PG_DSN="postgresql://user:pass@localhost:5432/sut"
export KEYCLOAK_ISSUER="http://localhost:8081/realms/sut"
export KEYCLOAK_JWKS="http://localhost:8081/realms/sut/protocol/openid-connect/certs"
export KEYCLOAK_AUDIENCE="sut-api"

# Executar a API
cargo run
```

### Debug
```bash
# Executar com logs detalhados
RUST_LOG=debug cargo run

# Executar com logs de uma biblioteca específica
RUST_LOG=sut_api=debug,sqlx=warn cargo run
```

## 📈 Performance

### Benchmarks
- **Throughput**: ~10,000 req/s (GET /v1/contacts)
- **Latência P50**: ~1ms
- **Latência P95**: ~5ms
- **Latência P99**: ~10ms

### Otimizações
- Pool de conexões PostgreSQL otimizado
- Cache de JWKS do Keycloak
- Compressão gzip habilitada
- Logs estruturados para observabilidade

## 🔒 Segurança

### Autenticação
- JWT RS256 com validação de assinatura
- Validação de issuer, audience e expiração
- Refresh automático de JWKS

### Autorização
- Controle de acesso baseado em roles
- Middleware de autenticação em todas as rotas protegidas
- Auditoria de ações críticas

### Validação
- Validação de entrada em todos os endpoints
- Sanitização de dados
- Proteção contra SQL injection

## 🚀 Deploy

### Docker
```bash
# Build da imagem
docker build -f api/Dockerfile -t sut-api .

# Executar container
docker run -p 8080:8080 sut-api
```

### Docker Compose
```bash
# Desenvolvimento
docker compose -f deploy/docker-compose.dev.yml up

# Produção
docker compose -f deploy/docker-compose.prod.yml up -d
```

---

Esta documentação cobre todos os aspectos da API SUT. Para mais detalhes, consulte a especificação OpenAPI em `/docs` ou `/openapi.json`.
