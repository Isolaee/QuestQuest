mod resolver;

// Re-export from combat crate
pub use combat::{CombatAction, CombatResult, CombatStats};

// Re-export unit-specific combat functions
pub use resolver::{resolve_combat, resolve_stack_combat};
