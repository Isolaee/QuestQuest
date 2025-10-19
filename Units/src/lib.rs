pub mod attack;
pub mod base_unit;
pub mod combat;
pub mod unit_class;
pub mod unit_factory;
pub mod unit_macros; // Macro to reduce boilerplate
pub mod unit_race;
pub mod unit_trait;
pub mod units; // Concrete unit implementations

// Re-export commonly used types
pub use attack::Attack;
pub use base_unit::BaseUnit;
pub use combat::{CombatAction, CombatResult, CombatStats};
pub use items::{ConsumableEffect, Equipment, Item, ItemProperties, ItemType, RangeType};
pub use unit_class::UnitClass;
pub use unit_factory::UnitFactory;
pub use unit_race::{Race, Terrain};
pub use unit_trait::UnitId;

// Export the trait-based Unit interface
pub use unit_trait::Unit;

// Export concrete unit types
pub use units::{
    DwarfArcher, DwarfMage, DwarfWarrior, ElfArcher, ElfMage, ElfWarrior, HumanArcher, HumanMage,
    HumanWarrior,
};
