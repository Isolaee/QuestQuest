mod combat_action;
mod combat_resolver;
mod combat_result;
mod combat_stats;

pub use combat_action::CombatAction;
pub use combat_resolver::resolve_combat;
pub use combat_result::CombatResult;
pub use combat_stats::{CombatStats, DamageType, RangeCategory, Resistances};
