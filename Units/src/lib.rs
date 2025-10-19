pub mod combat;
pub mod item; // Kept for backward compatibility
pub mod unit;
pub mod unit_class;
pub mod unit_race;

// Re-export commonly used types
pub use combat::CombatStats;
pub use items::{ConsumableEffect, Equipment, Item, ItemProperties, ItemType, RangeType};
pub use unit::{Unit, UnitId};
pub use unit_class::UnitClass;
pub use unit_race::{Race, Terrain};
