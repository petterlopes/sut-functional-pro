# Comentários Arquiteturais - main.rs

## Visão Geral

O arquivo `main.rs` implementa uma API REST em Rust seguindo os princípios da **Clean Architecture**. A aplicação demonstra excelentes práticas de engenharia de software, incluindo:

- **Separação de Responsabilidades**: Cada camada tem uma responsabilidade específica
- **Dependency Injection**: Facilita testes e manutenção
- **Error Handling**: Tratamento robusto de erros
- **Observability**: Logging estruturado e métricas
- **Security**: Autenticação JWT e configuração CORS

## Estrutura da Aplicação

```
┌─────────────────────────────────────────────────────────────┐
│                    PRESENTATION LAYER                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │ Controllers │ │ Middleware  │ │     Routes          │   │
│  │             │ │ (Auth/CORS) │ │                     │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   APPLICATION LAYER                        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │ Use Cases   │ │    DTOs     │ │   Application       │   │
│  │             │ │             │ │   Services          │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     DOMAIN LAYER                           │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │  Entities   │ │ Repositories│ │  Value Objects      │   │
│  │             │ │ (Interfaces)│ │                     │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                 INFRASTRUCTURE LAYER                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │ PostgreSQL  │ │   Vault     │ │   External APIs     │   │
│  │ Repositories│ │   Client    │ │                     │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Componentes Principais

### 1. AppState - Dependency Injection Container

```rust
pub struct AppState {
    // Infraestrutura básica
    pub pg: sqlx::Pool<sqlx::Postgres>,
    pub vault: Option<infra::vault::VaultClient>,
    pub metrics_token: Option<String>,
    
    // Repositórios - Clean Architecture
    pub contact_repository: Arc<infrastructure::repositories::PostgresContactRepository>,
    pub org_unit_repository: Arc<infrastructure::repositories::PostgresOrgUnitRepository>,
    pub department_repository: Arc<infrastructure::repositories::PostgresDepartmentRepository>,
    pub user_repository: Arc<infrastructure::repositories::PostgresUserRepository>,
}
```

**Características:**
- **Thread Safety**: Uso de `Arc<T>` para compartilhamento seguro
- **Optional Dependencies**: Vault e metrics_token são opcionais
- **Clean Architecture**: Separação entre interfaces e implementações

### 2. Configuração de Segurança

#### Autenticação JWT
- **Keycloak Integration**: Validação de tokens JWT
- **JWKS Refresh**: Atualização periódica das chaves públicas
- **Audience Validation**: Verificação de audiences permitidos
- **Leeway Configuration**: Tolerância de tempo para validação

#### CORS Configuration
- **Multiple Origins**: Suporte a múltiplas origens
- **Method Restrictions**: Limitação a métodos HTTP específicos
- **Header Control**: Controle granular de headers
- **Development Defaults**: Fallback para localhost

### 3. Observability

#### Logging Estruturado
```rust
let fmt_layer = tracing_subscriber::fmt::layer().json();
let filter = tracing_subscriber::EnvFilter::from_default_env();
```

#### Métricas Prometheus
- **HTTP Metrics**: Coleta automática de métricas HTTP
- **Authentication**: Proteção do endpoint de métricas
- **Custom Metrics**: Suporte a métricas customizadas

### 4. Health Checks

#### `/health` - Liveness Probe
- Verificação simples de que a aplicação está rodando
- Usado por Kubernetes para verificar se o container está vivo

#### `/ready` - Readiness Probe
- Verificação de dependências críticas:
  - Conectividade com PostgreSQL
  - Chaves JWKS carregadas
- Usado por Kubernetes para verificar se a aplicação está pronta

## Padrões de Design Implementados

### 1. Repository Pattern
- **Interface Segregation**: Cada repositório tem uma interface específica
- **Dependency Inversion**: Dependência de abstrações, não implementações
- **Testability**: Facilita criação de mocks para testes

### 2. Dependency Injection
- **Constructor Injection**: Dependências injetadas via construtor
- **Service Locator**: AppState atua como container de dependências
- **Lifetime Management**: Arc<T> para gerenciamento de lifetime

### 3. Middleware Pattern
- **Chain of Responsibility**: Middlewares aplicados em sequência
- **Cross-cutting Concerns**: CORS, logging, métricas
- **Order Matters**: Ordem de aplicação dos middlewares

## Aspectos Técnicos Importantes

### 1. Async/Await
- **Non-blocking I/O**: Operações de rede não bloqueiam
- **Concurrency**: Múltiplas requisições processadas simultaneamente
- **Error Propagation**: Uso de `?` para propagação de erros

### 2. Error Handling
- **anyhow**: Para erros de aplicação
- **thiserror**: Para tipos de erro customizados
- **Result<T, E>**: Tratamento explícito de erros

### 3. Type Safety
- **SQLx Macros**: Verificação de queries em tempo de compilação
- **Strong Typing**: Uso extensivo do sistema de tipos do Rust
- **Zero-cost Abstractions**: Abstrações sem overhead de performance

## Configuração e Deploy

### Variáveis de Ambiente
- **PG_DSN**: String de conexão PostgreSQL
- **KEYCLOAK_JWKS**: URL do JWKS do Keycloak
- **CORS_ALLOWED_ORIGINS**: Origens permitidas para CORS
- **METRICS_TOKEN**: Token para autenticação de métricas
- **BIND**: Endereço e porta para bind do servidor

### Docker e Kubernetes
- **Health Checks**: Compatível com probes do Kubernetes
- **Graceful Shutdown**: Suporte a shutdown graceful
- **Resource Management**: Pool de conexões otimizado

## Pontos Fortes da Implementação

1. **Clean Architecture**: Separação clara de responsabilidades
2. **Type Safety**: Uso extensivo do sistema de tipos do Rust
3. **Error Handling**: Tratamento robusto e explícito de erros
4. **Observability**: Logging estruturado e métricas
5. **Security**: Autenticação JWT e configuração CORS adequada
6. **Performance**: Operações assíncronas e pool de conexões
7. **Testability**: Injeção de dependências facilita testes
8. **Maintainability**: Código limpo e bem documentado

## Oportunidades de Melhoria

1. **Configuration Management**: Usar biblioteca como `config-rs`
2. **Error Types**: Implementar tipos de erro mais específicos
3. **Caching**: Adicionar cache para operações frequentes
4. **Rate Limiting**: Implementar rate limiting
5. **Database Migrations**: Automatizar migrações em produção
6. **Circuit Breaker**: Implementar circuit breaker para dependências externas

## Conclusão

Esta implementação demonstra excelentes práticas de engenharia de software em Rust. O código é **limpo**, **testável**, **manutenível** e **performático**, servindo como um excelente exemplo de como implementar uma API robusta seguindo princípios de Clean Architecture. A separação de responsabilidades, o tratamento de erros e a configuração flexível tornam este código adequado tanto para produção quanto para fins educacionais.
