# Clean Architecture Implementation

## 🏗️ Arquitetura Implementada

Este projeto implementa **Clean Architecture** com padrões **SOLID** e **DDD** (Domain-Driven Design).

### 📁 Estrutura de Pastas

```
src/
├── domain/                    # Camada de Domínio
│   ├── entities/             # Entidades de Negócio
│   │   ├── Contact.ts
│   │   ├── Localidade.ts
│   │   └── Departamento.ts
│   └── repositories/         # Interfaces de Repositório
│       ├── IContactRepository.ts
│       ├── ILocalidadeRepository.ts
│       └── IDepartamentoRepository.ts
├── application/              # Camada de Aplicação
│   └── use-cases/           # Casos de Uso
│       └── contact/
│           ├── CreateContactUseCase.ts
│           ├── UpdateContactUseCase.ts
│           ├── DeleteContactUseCase.ts
│           ├── GetContactsUseCase.ts
│           └── GetContactStatisticsUseCase.ts
├── infrastructure/           # Camada de Infraestrutura
│   ├── api/                 # Cliente HTTP
│   │   ├── IApiClient.ts
│   │   └── AxiosApiClient.ts
│   ├── repositories/        # Implementações de Repositório
│   │   └── ContactRepository.ts
│   └── di/                  # Injeção de Dependência
│       ├── Container.ts
│       └── ServiceRegistry.ts
└── presentation/            # Camada de Apresentação
    └── hooks/               # Hooks Customizados
        └── useContactUseCases.ts
```

## 🎯 Princípios SOLID Aplicados

### 1. **Single Responsibility Principle (SRP)**
- Cada classe tem uma única responsabilidade
- `Contact` entity: apenas lógica de domínio
- `CreateContactUseCase`: apenas criação de contatos
- `ContactRepository`: apenas persistência

### 2. **Open/Closed Principle (OCP)**
- Interfaces permitem extensão sem modificação
- Novos repositórios podem ser criados implementando `IContactRepository`
- Novos casos de uso podem ser adicionados sem alterar existentes

### 3. **Liskov Substitution Principle (LSP)**
- Implementações podem ser substituídas por suas interfaces
- `ContactRepository` pode ser substituído por qualquer implementação de `IContactRepository`

### 4. **Interface Segregation Principle (ISP)**
- Interfaces específicas e coesas
- `IContactRepository` tem métodos específicos para contatos
- Não há dependência de métodos não utilizados

### 5. **Dependency Inversion Principle (DIP)**
- Dependências apontam para abstrações
- Use cases dependem de interfaces, não de implementações
- Container de DI gerencia as dependências

## 🏛️ Clean Architecture

### Camadas e Dependências

```
┌─────────────────────────────────────┐
│           Presentation              │ ← React Components, Hooks
├─────────────────────────────────────┤
│           Application               │ ← Use Cases, DTOs
├─────────────────────────────────────┤
│             Domain                  │ ← Entities, Value Objects, Interfaces
├─────────────────────────────────────┤
│          Infrastructure             │ ← Repositories, API Clients, DI
└─────────────────────────────────────┘
```

### Regras de Dependência
- ✅ **Presentation** → **Application** → **Domain**
- ✅ **Infrastructure** → **Domain**
- ❌ **Domain** não depende de nenhuma camada externa
- ❌ **Application** não depende de **Infrastructure**

## 🎭 Domain-Driven Design (DDD)

### Entidades de Domínio
- **Contact**: Representa um contato no sistema
- **Localidade**: Representa uma localização geográfica
- **Departamento**: Representa um departamento organizacional

### Value Objects
- **ContactId**: Identificador único do contato
- **Email**: Email com validação
- **Phone**: Telefone com formatação
- **Document**: Documento com validação

### Repositórios
- Abstrações para persistência
- Interfaces definidas no domínio
- Implementações na infraestrutura

## 🔧 Injeção de Dependência

### Container Personalizado
```typescript
const container = new Container()

// Registrar serviços
container.registerClass('contactRepository', ContactRepository)
container.registerClass('createContactUseCase', CreateContactUseCase)

// Resolver dependências
const useCase = container.resolve<CreateContactUseCase>('createContactUseCase')
```

### Service Registry
- Centraliza o registro de todos os serviços
- Configura dependências automaticamente
- Gerencia tokens de autenticação

## 🚀 Como Usar

### 1. Inicialização
```typescript
// No App.tsx
AppInitializer.initialize(apiBase, token)
```

### 2. Hooks Customizados
```typescript
// Usar casos de uso através de hooks
const { data: contacts } = useContacts()
const createMutation = useCreateContact()
```

### 3. Adicionar Novo CRUD

1. **Criar Entity** em `domain/entities/`
2. **Criar Repository Interface** em `domain/repositories/`
3. **Criar Use Cases** em `application/use-cases/`
4. **Implementar Repository** em `infrastructure/repositories/`
5. **Registrar no ServiceRegistry**
6. **Criar Hooks** em `presentation/hooks/`

## 📋 Benefícios

- ✅ **Testabilidade**: Fácil mock de dependências
- ✅ **Manutenibilidade**: Código organizado e coeso
- ✅ **Escalabilidade**: Fácil adição de novas funcionalidades
- ✅ **Flexibilidade**: Troca de implementações sem impacto
- ✅ **Separação de Responsabilidades**: Cada camada tem seu propósito
- ✅ **Independência de Frameworks**: Lógica de negócio isolada

## 🔄 Próximos Passos

1. **Implementar API Backend** seguindo a mesma arquitetura
2. **Adicionar Testes Unitários** para cada camada
3. **Implementar Cache** na camada de infraestrutura
4. **Adicionar Validações** mais robustas
5. **Implementar Logging** e monitoramento
