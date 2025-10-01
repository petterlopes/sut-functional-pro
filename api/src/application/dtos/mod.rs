// ============================================================================
// DTOs MODULE - DATA TRANSFER OBJECTS
// ============================================================================
// Módulo que organiza todos os DTOs da camada de aplicação
// Segue o padrão de separação por entidade da Clean Architecture

// ===== CONTACT DTOs =====
pub mod contact_dto;
pub use contact_dto::*;

// ===== ORG UNIT DTOs =====
pub mod org_unit_dto;
pub use org_unit_dto::*;

// ===== DEPARTMENT DTOs =====
pub mod department_dto;
pub use department_dto::*;

// ===== USER DTOs =====
pub mod user_dto;
pub use user_dto::*;

// ===== AUDIT EVENT DTOs =====
pub mod audit_event_dto;
pub use audit_event_dto::*;

// ===== MERGE DTOs =====
pub mod merge_dto;
pub use merge_dto::*;
