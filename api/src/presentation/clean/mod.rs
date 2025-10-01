// ============================================================================
// CLEAN ARCHITECTURE CONTROLLERS MODULE
// ============================================================================
// Módulo que organiza todos os controllers da camada de apresentação
// Segue os princípios da Clean Architecture com separação clara de responsabilidades

// ===== CONTROLLER MODULES =====
pub mod contact_controller; // Controller para operações de contatos
pub mod department_controller; // Controller para operações de departamentos
pub mod org_unit_controller; // Controller para operações de unidades organizacionais
pub mod user_controller; // Controller para operações de usuários

// ===== RE-EXPORTS =====
// Re-exporta todas as funções de rotas dos controllers para facilitar o uso
pub use contact_controller::*;
pub use department_controller::*;
pub use org_unit_controller::*;
pub use user_controller::*;
