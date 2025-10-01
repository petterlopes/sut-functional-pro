// ============================================================================
// BASE TRAITS - TRAITS FUNDAMENTAIS PARA ELIMINAR REDUNDÂNCIA
// ============================================================================
// Traits base que definem contratos comuns para repositórios e casos de uso
// Seguem os princípios SOLID e DRY para máxima reutilização

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

// ============================================================================
// TRAIT BASE PARA ENTIDADES
// ============================================================================

/// Trait base para todas as entidades do domínio
/// Define operações comuns que todas as entidades devem implementar
pub trait Entity: Clone + Debug + Send + Sync {
    /// Tipo do ID da entidade
    type Id: Clone + Debug + Send + Sync + PartialEq;

    /// Retorna o ID da entidade
    fn id(&self) -> &Self::Id;

    /// Valida se a entidade está em estado válido
    fn is_valid(&self) -> bool;

    /// Retorna a versão da entidade (para controle de concorrência)
    fn version(&self) -> Option<i32>;
}

// ============================================================================
// TRAIT BASE PARA REPOSITÓRIOS
// ============================================================================

/// Trait base para todos os repositórios
/// Define operações CRUD comuns que todos os repositórios devem implementar
#[async_trait]
pub trait BaseRepository<T: Entity>: Send + Sync {
    /// Tipo do critério de busca
    type SearchCriteria: Send + Sync;
    /// Tipo do resultado de busca
    type SearchResult: Send + Sync;
    /// Tipo das estatísticas
    type Statistics: Send + Sync;

    /// Busca uma entidade por ID
    async fn find_by_id(&self, id: &T::Id)
        -> Result<Option<T>, crate::domain::errors::DomainError>;

    /// Busca entidades com critérios
    async fn search(
        &self,
        criteria: &Self::SearchCriteria,
    ) -> Result<Self::SearchResult, crate::domain::errors::DomainError>;

    /// Salva uma entidade (cria ou atualiza)
    async fn save(&self, entity: &T) -> Result<T, crate::domain::errors::DomainError>;

    /// Remove uma entidade por ID
    async fn delete(&self, id: &T::Id) -> Result<(), crate::domain::errors::DomainError>;

    /// Retorna estatísticas da entidade
    async fn get_statistics(&self) -> Result<Self::Statistics, crate::domain::errors::DomainError>;

    /// Verifica se uma entidade existe
    async fn exists(&self, id: &T::Id) -> Result<bool, crate::domain::errors::DomainError>;

    /// Conta entidades com critérios
    async fn count(
        &self,
        criteria: &Self::SearchCriteria,
    ) -> Result<i64, crate::domain::errors::DomainError>;
}

// ============================================================================
// TRAIT BASE PARA CASOS DE USO
// ============================================================================

/// Trait base para casos de uso de criação
#[async_trait]
pub trait CreateUseCase<TRequest, TResponse>: Send + Sync {
    async fn execute(
        &self,
        request: TRequest,
    ) -> Result<TResponse, crate::domain::errors::DomainError>;
}

/// Trait base para casos de uso de atualização
#[async_trait]
pub trait UpdateUseCase<TRequest, TResponse>: Send + Sync {
    async fn execute(
        &self,
        request: TRequest,
    ) -> Result<TResponse, crate::domain::errors::DomainError>;
}

/// Trait base para casos de uso de busca
#[async_trait]
pub trait GetUseCase<TRequest, TResponse>: Send + Sync {
    async fn execute(
        &self,
        request: TRequest,
    ) -> Result<TResponse, crate::domain::errors::DomainError>;
}

/// Trait base para casos de uso de listagem
#[async_trait]
pub trait ListUseCase<TRequest, TResponse>: Send + Sync {
    async fn execute(
        &self,
        request: TRequest,
    ) -> Result<TResponse, crate::domain::errors::DomainError>;
}

/// Trait base para casos de uso de exclusão
#[async_trait]
pub trait DeleteUseCase<TRequest>: Send + Sync {
    async fn execute(&self, request: TRequest) -> Result<(), crate::domain::errors::DomainError>;
}

/// Trait base para casos de uso de estatísticas
#[async_trait]
pub trait StatisticsUseCase<TResponse>: Send + Sync {
    async fn execute(&self) -> Result<TResponse, crate::domain::errors::DomainError>;
}

// ============================================================================
// TRAIT BASE PARA DTOs
// ============================================================================

/// Trait base para DTOs de requisição
pub trait RequestDto: Debug + Clone + Send + Sync {
    /// Valida os dados da requisição
    fn validate(&self) -> Result<(), String>;

    /// Converte para entidade do domínio
    fn to_entity(&self) -> Result<Box<Self>, String>;
}

/// Trait base para DTOs de resposta
pub trait ResponseDto: Debug + Clone + Send + Sync + Serialize {
    /// Cria a partir de uma entidade do domínio
    fn from_entity(entity: &Self) -> Self;
}

/// Trait base para DTOs de busca
pub trait SearchDto: Debug + Clone + Send + Sync + for<'de> Deserialize<'de> {
    /// Converte para critério de busca do repositório
    fn to_search_criteria(&self) -> Box<dyn Send + Sync>;
}

// ============================================================================
// TRAIT BASE PARA VALIDAÇÃO
// ============================================================================

/// Trait para validação de dados
pub trait Validator<T>: Send + Sync {
    /// Valida um valor
    fn validate(&self, value: &T) -> Result<(), String>;

    /// Retorna mensagem de erro personalizada
    fn error_message(&self) -> &str;
}

/// Trait para validação de UUID
pub trait UuidValidator: Send + Sync {
    fn validate_uuid(&self, uuid_str: &str) -> Result<Uuid, String>;
}

/// Trait para validação de email
pub trait EmailValidator: Send + Sync {
    fn validate_email(&self, email: &str) -> Result<(), String>;
}

/// Trait para validação de string não vazia
pub trait NonEmptyValidator: Send + Sync {
    fn validate_non_empty(&self, value: &str, field_name: &str) -> Result<(), String>;
}

// ============================================================================
// TRAIT BASE PARA MAPEAMENTO
// ============================================================================

/// Trait para mapeamento entre DTOs e entidades
pub trait Mapper<TDto, TEntity>: Send + Sync {
    /// Converte DTO para entidade
    fn to_entity(&self, dto: &TDto) -> Result<TEntity, String>;

    /// Converte entidade para DTO
    fn to_dto(&self, entity: &TEntity) -> TDto;

    /// Converte lista de entidades para DTOs
    fn to_dto_list(&self, entities: &[TEntity]) -> Vec<TDto> {
        entities.iter().map(|entity| self.to_dto(entity)).collect()
    }
}

// ============================================================================
// TRAIT BASE PARA CONFIGURAÇÃO
// ============================================================================

/// Trait para configuração de repositórios
pub trait RepositoryConfig: Send + Sync {
    /// Retorna a configuração de conexão
    fn connection_string(&self) -> &str;

    /// Retorna o pool size
    fn pool_size(&self) -> u32;

    /// Retorna o timeout de conexão
    fn connection_timeout(&self) -> std::time::Duration;
}

/// Trait para configuração de aplicação
pub trait AppConfig: Send + Sync {
    /// Retorna a porta da aplicação
    fn port(&self) -> u16;

    /// Retorna o host da aplicação
    fn host(&self) -> &str;

    /// Retorna se está em modo debug
    fn is_debug(&self) -> bool;

    /// Retorna a configuração de logging
    fn log_level(&self) -> &str;
}

// ============================================================================
// TRAIT BASE PARA MIDDLEWARE
// ============================================================================

/// Trait base para middleware
#[async_trait]
pub trait Middleware: Send + Sync {
    /// Nome do middleware
    fn name(&self) -> &str;

    /// Executa o middleware
    async fn execute(&self, request: &mut axum::extract::Request) -> Result<(), String>;

    /// Prioridade do middleware (menor = maior prioridade)
    fn priority(&self) -> i32 {
        100
    }
}

// ============================================================================
// TRAIT BASE PARA CACHE
// ============================================================================

/// Trait base para cache
#[async_trait]
pub trait Cache<T>: Send + Sync {
    /// Obtém valor do cache
    async fn get(&self, key: &str) -> Result<Option<T>, String>;

    /// Define valor no cache
    async fn set(
        &self,
        key: &str,
        value: &T,
        ttl: Option<std::time::Duration>,
    ) -> Result<(), String>;

    /// Remove valor do cache
    async fn delete(&self, key: &str) -> Result<(), String>;

    /// Limpa todo o cache
    async fn clear(&self) -> Result<(), String>;
}

// ============================================================================
// TRAIT BASE PARA LOGGING
// ============================================================================

/// Trait base para logging
pub trait Logger: Send + Sync {
    /// Log de debug
    fn debug(&self, message: &str, context: Option<&serde_json::Value>);

    /// Log de informação
    fn info(&self, message: &str, context: Option<&serde_json::Value>);

    /// Log de warning
    fn warn(&self, message: &str, context: Option<&serde_json::Value>);

    /// Log de erro
    fn error(&self, message: &str, context: Option<&serde_json::Value>);
}

// ============================================================================
// TRAIT BASE PARA MÉTRICAS
// ============================================================================

/// Trait base para métricas
pub trait Metrics: Send + Sync {
    /// Incrementa contador
    fn increment_counter(&self, name: &str, labels: Option<&[(&str, &str)]>);

    /// Define gauge
    fn set_gauge(&self, name: &str, value: f64, labels: Option<&[(&str, &str)]>);

    /// Registra histogram
    fn record_histogram(&self, name: &str, value: f64, labels: Option<&[(&str, &str)]>);

    /// Registra tempo de execução
    fn record_duration(
        &self,
        name: &str,
        duration: std::time::Duration,
        labels: Option<&[(&str, &str)]>,
    );
}

// ============================================================================
// MACROS PARA IMPLEMENTAÇÃO AUTOMÁTICA
// ============================================================================

/// Macro para implementar trait Entity automaticamente
#[macro_export]
macro_rules! impl_entity {
    ($entity_type:ty, $id_type:ty) => {
        impl $crate::shared::base_traits::Entity for $entity_type {
            type Id = $id_type;

            fn id(&self) -> &Self::Id {
                &self.id
            }

            fn is_valid(&self) -> bool {
                // Implementação padrão - pode ser sobrescrita
                true
            }

            fn version(&self) -> Option<i32> {
                // Implementação padrão - pode ser sobrescrita
                None
            }
        }
    };
}

/// Macro para implementar validação básica
#[macro_export]
macro_rules! impl_basic_validation {
    ($dto_type:ty) => {
        impl $crate::shared::base_traits::RequestDto for $dto_type {
            fn validate(&self) -> Result<(), String> {
                // Implementação básica - pode ser estendida
                Ok(())
            }

            fn to_entity(&self) -> Result<Box<dyn $crate::shared::base_traits::Entity>, String> {
                // Implementação específica deve ser fornecida
                Err("Not implemented".to_string())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    struct TestEntity {
        id: Uuid,
        name: String,
    }

    impl_entity!(TestEntity, Uuid);

    #[test]
    fn test_entity_trait() {
        let entity = TestEntity {
            id: Uuid::new_v4(),
            name: "Test".to_string(),
        };

        assert!(entity.is_valid());
        assert_eq!(entity.version(), None);
    }

    #[test]
    fn test_entity_id() {
        let id = Uuid::new_v4();
        let entity = TestEntity {
            id: id.clone(),
            name: "Test".to_string(),
        };

        assert_eq!(entity.id(), &id);
    }
}
