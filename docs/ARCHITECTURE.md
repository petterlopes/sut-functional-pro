# Arquitetura do Sistema SUT

## 🏗️ Visão Geral da Arquitetura

O Sistema Único de Telefonia (SUT) implementa uma arquitetura limpa e moderna seguindo os princípios de **Clean Architecture**, **Domain-Driven Design (DDD)** e **SOLID**. A arquitetura é aplicada tanto no backend (Rust) quanto no frontend (React/TypeScript).

## 🎯 Princípios Arquiteturais

### Clean Architecture
- **Independência de Frameworks**: O sistema não depende de frameworks externos
- **Testabilidade**: Lógica de negócio pode ser testada independentemente
- **Independência de UI**: A interface pode mudar sem afetar o sistema
- **Independência de Banco de Dados**: Pode trocar Oracle por SQL Server
- **Independência de Agentes Externos**: Regras de negócio não sabem do mundo exterior

### Domain-Driven Design (DDD)
- **Entidades**: Objetos com identidade única
- **Objetos de Valor**: Objetos imutáveis sem identidade
- **Repositórios**: Abstração para acesso a dados
- **Serviços de Domínio**: Lógica que não pertence a uma entidade específica
- **Agregados**: Conjuntos de entidades relacionadas

### SOLID Principles
- **S** - Single Responsibility Principle
- **O** - Open/Closed Principle
- **L** - Liskov Substitution Principle
- **I** - Interface Segregation Principle
- **D** - Dependency Inversion Principle

## 🏛️ Estrutura de Camadas

### Backend (Rust)

```
api/src/
├── domain/                    # Camada de Domínio
│   ├── entities.rs           # Entidades de negócio
│   ├── value_objects.rs      # Objetos de valor
│   ├── repositories.rs       # Interfaces de repositório
│   └── errors.rs             # Erros de domínio
├── application/              # Camada de Aplicação
│   ├── use_cases/           # Casos de uso
│   │   ├── contact.rs       # Casos de uso de contatos
│   │   ├── org_unit.rs      # Casos de uso de unidades
│   │   ├── department.rs    # Casos de uso de departamentos
│   │   └── user.rs          # Casos de uso de usuários
│   └── dto.rs               # Data Transfer Objects
├── infrastructure/           # Camada de Infraestrutura
│   ├── repositories/        # Implementações de repositório
│   │   ├── contact_repository_simple.rs
│   │   ├── org_unit_repository_simple.rs
│   │   ├── department_repository_simple.rs
│   │   └── user_repository_simple.rs
│   └── mappers.rs           # Mapeadores de dados
└── presentation/            # Camada de Apresentação
    ├── clean/               # Controladores Clean Architecture
    │   ├── contact_controller.rs
    │   ├── org_unit_controller.rs
    │   ├── department_controller.rs
    │   └── user_controller.rs
    └── auth.rs              # Middleware de autenticação
```

### Frontend (React/TypeScript)

```
frontend/src/
├── domain/                   # Camada de Domínio
│   ├── entities/            # Entidades TypeScript
│   │   └── Contact.ts       # Interface de contato
│   └── repositories/        # Interfaces de repositório
│       └── IContactRepository.ts
├── application/             # Camada de Aplicação
│   ├── use-cases/          # Casos de uso
│   │   └── contact/        # Casos de uso de contatos
│   │       ├── CreateContactUseCase.ts
│   │       ├── UpdateContactUseCase.ts
│   │       ├── DeleteContactUseCase.ts
│   │       ├── GetContactsUseCase.ts
│   │       └── GetContactStatisticsUseCase.ts
│   └── services/           # Serviços de aplicação
├── infrastructure/          # Camada de Infraestrutura
│   ├── api/                # Clientes HTTP
│   │   ├── AxiosApiClient.ts
│   │   └── IApiClient.ts
│   ├── repositories/       # Implementações de repositório
│   │   └── ContactRepository.ts
│   └── di/                 # Injeção de dependência
│       └── ServiceRegistry.ts
└── presentation/           # Camada de Apresentação
    ├── components/         # Componentes React
    ├── pages/              # Páginas
    └── hooks/              # Hooks customizados
        └── useContactUseCasesClean.ts
```

## 🔄 Fluxo de Dados

### 1. Requisição HTTP
```
Cliente → Frontend → API Client → Backend Controller
```

### 2. Processamento
```
Controller → Use Case → Repository → Database
```

### 3. Resposta
```
Database → Repository → Use Case → Controller → API Client → Frontend → Cliente
```

## 🏢 Entidades de Domínio

### Contact (Contato)
```rust
pub struct Contact {
    pub id: ContactId,
    pub full_name: String,
    pub contact_type: ContactType,
    pub status: ContactStatus,
    pub document: Option<String>,
    pub unit_id: Option<OrgUnitId>,
    pub department_id: Option<DepartmentId>,
    pub emails: Vec<Email>,
    pub phones: Vec<Phone>,
    pub etag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### OrgUnit (Unidade Organizacional)
```rust
pub struct OrgUnit {
    pub id: OrgUnitId,
    pub name: String,
    pub parent_id: Option<OrgUnitId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Department (Departamento)
```rust
pub struct Department {
    pub id: DepartmentId,
    pub name: String,
    pub unit_id: OrgUnitId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### User (Usuário)
```rust
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
    pub roles: Vec<Role>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## 🎯 Objetos de Valor

### ContactId
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContactId(pub Uuid);

impl ContactId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}
```

### Email
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email {
    pub address: String,
    pub is_primary: bool,
}

impl Email {
    pub fn new(address: String, is_primary: bool) -> Result<Self, String> {
        if address.is_empty() {
            return Err("Email address cannot be empty".to_string());
        }
        if !address.contains('@') {
            return Err("Invalid email format".to_string());
        }
        Ok(Self { address, is_primary })
    }
}
```

## 🔧 Casos de Uso

### CreateContactUseCase
```rust
pub struct CreateContactUseCase {
    contact_repository: Arc<dyn ContactRepository>,
}

impl CreateContactUseCase {
    pub async fn execute(&self, request: CreateContactRequest) -> Result<ContactResponse, DomainError> {
        // Validação de entrada
        let contact = Contact::new(
            ContactId::new(),
            request.full_name,
            ContactType::from_str(&request.contact_type)?,
            ContactStatus::from_str(&request.status)?,
            request.document,
            request.unit_id,
            request.department_id,
        )?;

        // Persistência
        let saved_contact = self.contact_repository.save(&contact).await?;
        
        // Resposta
        Ok(saved_contact.into())
    }
}
```

## 🗄️ Repositórios

### Interface
```rust
#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn find_by_id(&self, id: &ContactId) -> Result<Option<Contact>, DomainError>;
    async fn find_all(&self, criteria: &ContactSearchCriteria) -> Result<ContactSearchResult, DomainError>;
    async fn save(&self, contact: &Contact) -> Result<Contact, DomainError>;
    async fn update(&self, contact: &Contact) -> Result<Contact, DomainError>;
    async fn delete(&self, id: &ContactId) -> Result<(), DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Contact>, DomainError>;
    async fn find_by_document(&self, document: &str) -> Result<Option<Contact>, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<Contact>, DomainError>;
    async fn find_by_unit(&self, unit_id: &OrgUnitId) -> Result<Vec<Contact>, DomainError>;
    async fn find_by_department(&self, department_id: &DepartmentId) -> Result<Vec<Contact>, DomainError>;
    async fn count_by_status(&self, status: &ContactStatus) -> Result<i64, DomainError>;
    async fn count_by_type(&self, contact_type: &ContactType) -> Result<i64, DomainError>;
    async fn get_statistics(&self) -> Result<ContactStatistics, DomainError>;
}
```

### Implementação
```rust
pub struct PostgresContactRepository {
    pool: PgPool,
}

#[async_trait]
impl ContactRepository for PostgresContactRepository {
    async fn find_by_id(&self, id: &ContactId) -> Result<Option<Contact>, DomainError> {
        // Implementação com SQLx
        let row = sqlx::query_as!(
            ContactRow,
            "SELECT * FROM contacts WHERE id = $1",
            id.0
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(contact_row) => Ok(Some(build_contact_from_row(contact_row)?)),
            None => Ok(None),
        }
    }
    
    // ... outras implementações
}
```

## 🎮 Controladores

### ContactController
```rust
pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        .route("/v1/contacts", get(get_contacts).post(create_contact))
        .route("/v1/contacts/:id", get(get_contact).patch(update_contact).delete(delete_contact))
        .route("/v1/contacts/statistics", get(get_contact_statistics))
}

async fn get_contacts(
    State(state): State<Arc<crate::AppState>>,
    Query(params): Query<ContactSearchRequest>,
) -> Result<Json<ContactSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let use_case = GetContactsUseCase::new(state.contact_repository.clone());
    let criteria = params.into();
    
    match use_case.execute(&criteria).await {
        Ok(response) => Ok(Json(response)),
        Err(error) => {
            let error_response = serde_json::json!({
                "error": error.to_string()
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}
```

## 🔄 Injeção de Dependência

### Backend (Rust)
```rust
#[derive(Clone)]
pub struct AppState {
    pub pg: sqlx::Pool<sqlx::Postgres>,
    pub vault: Option<infra::vault::VaultClient>,
    pub metrics_token: Option<String>,
    // Clean Architecture repositories
    pub contact_repository: Arc<infrastructure::repositories::PostgresContactRepository>,
    pub org_unit_repository: Arc<infrastructure::repositories::PostgresOrgUnitRepository>,
    pub department_repository: Arc<infrastructure::repositories::PostgresDepartmentRepository>,
    pub user_repository: Arc<infrastructure::repositories::PostgresUserRepository>,
}
```

### Frontend (TypeScript)
```typescript
export class ServiceRegistry {
  private static instance: ServiceRegistry;
  private services: Map<string, any> = new Map();

  public static getInstance(): ServiceRegistry {
    if (!ServiceRegistry.instance) {
      ServiceRegistry.instance = new ServiceRegistry();
    }
    return ServiceRegistry.instance;
  }

  public register<T>(key: string, service: T): void {
    this.services.set(key, service);
  }

  public get<T>(key: string): T {
    const service = this.services.get(key);
    if (!service) {
      throw new Error(`Service ${key} not found`);
    }
    return service as T;
  }
}

// Registro de serviços
const apiClient = new AxiosApiClient(apiBaseUrl, token);
const contactRepository = new ContactRepository(apiClient);
const createContactUseCase = new CreateContactUseCase(contactRepository);

ServiceRegistry.getInstance().register('contactRepository', contactRepository);
ServiceRegistry.getInstance().register('createContactUseCase', createContactUseCase);
```

## 🧪 Testes

### Testes de Unidade
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        ContactRepository {}
        
        #[async_trait]
        impl ContactRepository for ContactRepository {
            async fn find_by_id(&self, id: &ContactId) -> Result<Option<Contact>, DomainError>;
            async fn save(&self, contact: &Contact) -> Result<Contact, DomainError>;
        }
    }

    #[tokio::test]
    async fn test_create_contact_success() {
        let mut mock_repo = MockContactRepository::new();
        let contact = Contact::new(/* ... */).unwrap();
        
        mock_repo
            .expect_save()
            .with(eq(contact.clone()))
            .times(1)
            .returning(move |_| Ok(contact.clone()));

        let use_case = CreateContactUseCase::new(Arc::new(mock_repo));
        let request = CreateContactRequest { /* ... */ };
        
        let result = use_case.execute(request).await;
        assert!(result.is_ok());
    }
}
```

### Testes de Integração
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_contact_crud_operations(pool: PgPool) {
        let repository = PostgresContactRepository::new(pool);
        let contact = Contact::new(/* ... */).unwrap();
        
        // Create
        let saved = repository.save(&contact).await.unwrap();
        assert_eq!(saved.id, contact.id);
        
        // Read
        let found = repository.find_by_id(&contact.id).await.unwrap();
        assert!(found.is_some());
        
        // Update
        let mut updated = saved.clone();
        updated.full_name = "Updated Name".to_string();
        let updated_contact = repository.update(&updated).await.unwrap();
        assert_eq!(updated_contact.full_name, "Updated Name");
        
        // Delete
        repository.delete(&contact.id).await.unwrap();
        let deleted = repository.find_by_id(&contact.id).await.unwrap();
        assert!(deleted.is_none());
    }
}
```

## 📊 Observabilidade

### Métricas
```rust
use axum_prometheus::PrometheusMetricLayer;

let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
```

### Logs Estruturados
```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "sut_api=debug,tower_http=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer().json())
    .init();
```

### Exemplo de Logs
```rust
tracing::info!(
    contact_id = %contact.id,
    action = "contact_created",
    "Contact created successfully"
);
```

## 🔒 Segurança

### Autenticação JWT
```rust
pub async fn jwt_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get("Authorization");
    
    match auth_header {
        Some(header) => {
            let token = extract_token(header)?;
            let claims = validate_jwt(&token).await?;
            req.extensions_mut().insert(claims);
        }
        None => {
            // Fallback para desenvolvimento
            if let Some(dev_user) = req.headers().get("X-Dev-User") {
                let claims = create_dev_claims(dev_user);
                req.extensions_mut().insert(claims);
            } else {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }
    
    Ok(next.run(req).await)
}
```

### Autorização por Roles
```rust
pub fn has_scope(claims: &serde_json::Value, scope: &str) -> bool {
    if let Some(scopes) = claims.get("scope").and_then(|s| s.as_str()) {
        scopes.split(' ').any(|s| s == scope)
    } else {
        false
    }
}
```

## 🚀 Performance

### Pool de Conexões
```rust
let pool = sqlx::PgPool::builder()
    .max_connections(20)
    .min_connections(5)
    .acquire_timeout(Duration::from_secs(30))
    .build(&dsn)
    .await?;
```

### Cache
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

pub struct CacheService {
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl CacheService {
    pub async fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }
    
    pub async fn set(&self, key: String, value: String) {
        let mut cache = self.cache.write().await;
        cache.insert(key, value);
    }
}
```

## 📈 Escalabilidade

### Horizontal Scaling
- **Stateless**: Aplicação não mantém estado
- **Load Balancer**: Distribuição de carga
- **Database Sharding**: Particionamento de dados
- **Microservices**: Decomposição em serviços menores

### Vertical Scaling
- **Connection Pooling**: Pool de conexões otimizado
- **Caching**: Cache em memória e Redis
- **Async/Await**: Programação assíncrona
- **Resource Optimization**: Otimização de recursos

## 🔄 CI/CD

### Pipeline de Build
```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --locked
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Run fmt check
        run: cargo fmt -- --check
```

### Pipeline de Deploy
```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build Docker image
        run: docker build -t sut-api .
      - name: Deploy to production
        run: docker-compose -f docker-compose.prod.yml up -d
```

## 📚 Referências

- [Clean Architecture - Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Domain-Driven Design - Eric Evans](https://domainlanguage.com/ddd/)
- [SOLID Principles](https://en.wikipedia.org/wiki/SOLID)
- [Rust Book](https://doc.rust-lang.org/book/)
- [React Documentation](https://react.dev/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)

---

Esta arquitetura garante que o sistema seja **maintível**, **testável**, **escalável** e **flexível**, seguindo as melhores práticas da indústria de software.
