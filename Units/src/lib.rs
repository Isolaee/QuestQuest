pub mod base_unit;
pub mod combat;
pub mod unit; // Old Unit struct - kept for backward compatibility with tests
pub mod unit_class;
pub mod unit_factory;
pub mod unit_race;
pub mod unit_trait;

// Re-export commonly used types
pub use base_unit::BaseUnit;
pub use combat::{CombatAction, CombatResult, CombatStats};
pub use items::{ConsumableEffect, Equipment, Item, ItemProperties, ItemType, RangeType};
pub use unit::UnitId;
pub use unit_class::UnitClass;
pub use unit_factory::UnitFactory;
pub use unit_race::{Race, Terrain};

// Export the NEW trait-based Unit
pub use unit_trait::Unit;

// Keep the old struct Unit available as LegacyUnit for tests/examples
#[allow(dead_code)]
pub use unit::Unit as LegacyUnit;
