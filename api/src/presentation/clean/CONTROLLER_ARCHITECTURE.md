# Arquitetura do Contact Controller - Clean Architecture

## Visão Geral

O `contact_controller.rs` implementa a **camada de apresentação** da Clean Architecture, responsável por:

- **Receber requisições HTTP** e extrair dados
- **Delegar lógica de negócio** para a camada de aplicação
- **Mapear erros de domínio** para códigos HTTP apropriados
- **Retornar respostas JSON** estruturadas

## Padrões Implementados

### 1. **Controller Pattern**
- **Separação de Responsabilidades**: Controller apenas coordena, não contém lógica de negócio
- **Thin Controllers**: Handlers são finos e delegam para casos de uso
- **Single Responsibility**: Cada handler tem uma responsabilidade específica

### 2. **Use Case Pattern**
- **Dependency Injection**: Casos de uso recebem dependências via construtor
- **Encapsulation**: Lógica de negócio encapsulada nos casos de uso
- **Testability**: Fácil de testar isoladamente

### 3. **Error Mapping Pattern**
- **Domain to HTTP**: Converte erros de domínio em códigos HTTP
- **Consistent Error Format**: Formato JSON consistente para erros
- **Proper Status Codes**: Códigos HTTP semanticamente corretos

## Estrutura dos Handlers

### **GET /v1/contacts** - Listar Contatos
```rust
async fn get_contacts(
    State(state): State<Arc<crate::AppState>>,
    Query(params): Query<ContactSearchRequest>,
) -> Result<Json<ContactSearchResponse>, (StatusCode, Json<serde_json::Value>)>
```

**Características:**
- **Query Parameters**: Suporte a filtros, paginação e ordenação
- **Search Use Case**: Delega para `GetContactsUseCase`
- **Response**: Lista paginada de contatos

### **GET /v1/contacts/:id** - Buscar Contato por ID
```rust
async fn get_contact(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)>
```

**Características:**
- **UUID Validation**: Valida formato UUID antes de processar
- **Value Object**: Converte string para `ContactId` value object
- **Single Resource**: Retorna um contato específico

### **POST /v1/contacts** - Criar Contato
```rust
async fn create_contact(
    State(state): State<Arc<crate::AppState>>,
    Json(request): Json<CreateContactRequest>,
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)>
```

**Características:**
- **JSON Input**: Recebe dados via JSON
- **Create Use Case**: Delega para `CreateContactUseCase`
- **Response**: Retorna contato criado (status 201)

### **PATCH /v1/contacts/:id** - Atualizar Contato
```rust
async fn update_contact(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
    Json(mut request): Json<UpdateContactRequest>,
) -> Result<Json<ContactResponse>, (StatusCode, Json<serde_json::Value>)>
```

**Características:**
- **Partial Update**: Atualização parcial (PATCH)
- **ID Injection**: Injeta ID da URL no request
- **Update Use Case**: Delega para `UpdateContactUseCase`

### **DELETE /v1/contacts/:id** - Deletar Contato
```rust
async fn delete_contact(
    State(state): State<Arc<crate::AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)>
```

**Características:**
- **No Content**: Retorna 204 No Content (padrão RESTful)
- **Delete Use Case**: Delega para `DeleteContactUseCase`
- **Idempotent**: Operação idempotente

### **GET /v1/contacts/statistics** - Estatísticas
```rust
async fn get_contact_statistics(
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<ContactStatisticsResponse>, (StatusCode, Json<serde_json::Value>)>
```

**Características:**
- **Aggregated Data**: Dados agregados para dashboards
- **Statistics Use Case**: Delega para `GetContactStatisticsUseCase`
- **No Parameters**: Não requer parâmetros

## Mapeamento de Erros

### **DomainError → HTTP Status Code**

| DomainError | HTTP Status | Descrição |
|-------------|-------------|-----------|
| `NotFound` | 404 | Recurso não encontrado |
| `ValidationError` | 400 | Dados inválidos |
| `Unauthorized` | 401 | Não autenticado |
| `Forbidden` | 403 | Sem permissão |
| `Conflict` | 409 | Conflito de estado |
| `InternalError` | 500 | Erro interno |
| `DatabaseError` | 500 | Erro de banco de dados |
| `ExternalServiceError` | 502 | Erro de serviço externo |
| `BusinessRuleViolation` | 422 | Violação de regra de negócio |

### **Formato de Erro**
```json
{
  "error": "Mensagem de erro descritiva"
}
```

## Fluxo de Dados

```
HTTP Request
     ↓
Controller Handler
     ↓
Extract Parameters (Path, Query, JSON)
     ↓
Initialize Use Case
     ↓
Execute Use Case
     ↓
Domain Logic Processing
     ↓
Repository Access
     ↓
Return Result
     ↓
Map Domain Errors to HTTP
     ↓
HTTP Response (JSON)
```

## Aspectos Técnicos

### **1. Async/Await**
- **Non-blocking**: Todos os handlers são assíncronos
- **Concurrency**: Suporte a múltiplas requisições simultâneas
- **Error Propagation**: Uso de `?` para propagação de erros

### **2. Type Safety**
- **Strong Typing**: Uso extensivo do sistema de tipos do Rust
- **Value Objects**: Conversão de strings para value objects tipados
- **DTOs**: DTOs tipados para entrada e saída

### **3. Error Handling**
- **Explicit Errors**: Tratamento explícito de todos os erros
- **Result Type**: Uso de `Result<T, E>` para operações que podem falhar
- **Error Mapping**: Mapeamento consistente de erros

### **4. Dependency Injection**
- **State Extraction**: Estado injetado via `State<Arc<AppState>>`
- **Repository Access**: Acesso a repositórios via estado compartilhado
- **Use Case Creation**: Criação de casos de uso com dependências injetadas

## Padrões RESTful

### **URLs**
- `/v1/contacts` - Coleção de contatos
- `/v1/contacts/:id` - Recurso específico
- `/v1/contacts/statistics` - Sub-recurso de estatísticas

### **HTTP Methods**
- `GET` - Leitura (listar, buscar)
- `POST` - Criação
- `PATCH` - Atualização parcial
- `DELETE` - Remoção

### **Status Codes**
- `200` - Sucesso (GET, PATCH)
- `201` - Criado (POST)
- `204` - No Content (DELETE)
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `409` - Conflict
- `422` - Unprocessable Entity
- `500` - Internal Server Error
- `502` - Bad Gateway

## Benefícios da Implementação

### **1. Clean Architecture**
- **Separation of Concerns**: Responsabilidades bem separadas
- **Dependency Inversion**: Dependência de abstrações
- **Testability**: Fácil de testar isoladamente

### **2. Maintainability**
- **Single Responsibility**: Cada handler tem uma responsabilidade
- **Consistent Patterns**: Padrões consistentes em todos os handlers
- **Clear Error Handling**: Tratamento de erro claro e consistente

### **3. Scalability**
- **Async Operations**: Operações assíncronas para alta concorrência
- **Stateless**: Handlers stateless para escalabilidade horizontal
- **Resource Efficient**: Uso eficiente de recursos

### **4. Developer Experience**
- **Type Safety**: Compile-time safety
- **Clear APIs**: APIs RESTful claras e consistentes
- **Good Error Messages**: Mensagens de erro descritivas

## Oportunidades de Melhoria

### **1. Validation**
- **Input Validation**: Validação mais robusta de entrada
- **Custom Validators**: Validadores customizados para regras específicas

### **2. Caching**
- **Response Caching**: Cache de respostas para melhor performance
- **Statistics Caching**: Cache de estatísticas que mudam pouco

### **3. Rate Limiting**
- **Per-endpoint Limits**: Limites específicos por endpoint
- **User-based Limits**: Limites baseados em usuário

### **4. Monitoring**
- **Metrics**: Métricas específicas por endpoint
- **Tracing**: Tracing distribuído para debugging

## Conclusão

O `contact_controller.rs` implementa uma **camada de apresentação robusta** que segue os princípios da Clean Architecture. A implementação demonstra:

- **Separação clara de responsabilidades**
- **Padrões consistentes** em todos os handlers
- **Tratamento robusto de erros**
- **APIs RESTful bem definidas**
- **Código limpo e manutenível**

Esta implementação serve como um **excelente exemplo** de como implementar controllers em Rust seguindo as melhores práticas de engenharia de software.
