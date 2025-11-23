//! # Units Crate
//!
//! This crate provides a comprehensive unit system for the QuestQuest game.
//! It includes character races, classes, combat stats, equipment management,
//! and a flexible trait-based unit system.
//!
//! ## Core Components
//!
//! - **[`Unit`]** trait: The main interface that all units must implement
//! - **[`BaseUnit`]**: Common data structure shared by all unit implementations
//! - **[`UnitFactory`]**: Factory for creating different unit types
//! - **[`Race`]**: Character races with terrain-specific bonuses
//! - **[`UnitClass`]**: Character classes with unique resistances and abilities
//! - **[`Attack`]**: Attack definitions with damage types and ranges
//!
//! ## Example
//!
//! ```rust,no_run
//! use units::{UnitFactory, Terrain};
//! use graphics::HexCoord;
//!
//! // Create a human warrior at position (0, 0)
//! let mut warrior = UnitFactory::create_human_warrior(
//!     "Aragorn".to_string(),
//!     HexCoord::new(0, 0),
//!     Terrain::Grasslands,
//! );
//!
//! // Access unit properties
//! println!("Unit: {}", warrior.name());
//! println!("Health: {}", warrior.combat_stats().health);
//! ```

pub mod attack;
pub mod base_unit;
pub mod combat;
pub mod unit_factory;
pub mod unit_macros; // Macro to reduce boilerplate
pub mod unit_race;
pub mod unit_registry; // Dynamic unit registry
pub mod unit_trait;
pub mod units; // Concrete unit implementations

// Re-export commonly used types
pub use attack::Attack;
pub use base_unit::BaseUnit;
pub use combat::{CombatAction, CombatResult, CombatStats};
pub use items::{ConsumableEffect, Equipment, Item, ItemProperties, ItemType, RangeType};
pub use unit_factory::UnitFactory;
pub use unit_race::{Race, Terrain};
pub use unit_registry::{UnitRegistry, UnitTypeInfo};
pub use unit_trait::UnitId;

// Export the trait-based Unit interface
pub use unit_trait::Unit;

// Export concrete unit types
pub use units::{
    DwarfVeteranWarrior, DwarfWarrior, DwarfYoungWarrior, ElfArcher, ElfMage, ElfWarrior,
    HumanArcher, HumanMage, HumanWarrior, OrcEliteSwordsman, OrcSwordsman, OrcYoungSwordsman,
};
