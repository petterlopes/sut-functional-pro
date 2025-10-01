//! =============================================================================
//! INFRASTRUCTURE MODULE
//! =============================================================================
//! Módulo de infraestrutura para integração com serviços externos
//! Inclui integração com Vault, PostgreSQL, auditoria e outros serviços

pub mod audit;
pub mod pg;
pub mod vault;

pub use audit::*;
pub use pg::*;
pub use vault::*;
