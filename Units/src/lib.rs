pub mod base_unit;
pub mod combat;
pub mod unit;
pub mod unit_class;
pub mod unit_factory;
pub mod unit_race;
pub mod unit_trait;

// Re-export commonly used types
pub use combat::{CombatAction, CombatResult, CombatStats};
pub use items::{ConsumableEffect, Equipment, Item, ItemProperties, ItemType, RangeType};
pub use unit::{Unit, UnitId}; // Keep old Unit struct for backward compatibility
pub use unit_class::UnitClass;
pub use unit_factory::UnitFactory;
pub use unit_race::{Race, Terrain};
pub use unit_trait::Unit as UnitTrait; // Export trait with different name to avoid conflict
