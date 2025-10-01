// ============================================================================
// APPLICATION LAYER MODULE
// ============================================================================
// Módulo principal da camada de aplicação da Clean Architecture
// Organiza casos de uso e DTOs

pub mod dto; // DTOs principais (re-exporta dtos separados)
pub mod dtos;
pub mod use_cases; // Casos de uso da aplicação // DTOs organizados por entidade

pub use dto::*;
pub use use_cases::*; // Re-exporta todos os casos de uso // Re-exporta todos os DTOs
