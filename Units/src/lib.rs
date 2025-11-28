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

#![allow(dead_code)]

pub mod ability; // Ability system (passive, active, aura)
pub mod attack;
pub mod base_unit;
pub mod combat;
pub mod structures; // Structure system (walls, towers, buildings, etc.)
pub mod team; // Team affiliation for units and structures
pub mod unit_factory;
pub mod unit_race;
pub mod unit_registry; // Dynamic unit registry
pub mod unit_trait;
pub mod unit_type; // Type-safe unit identifiers
pub mod units; // Concrete unit implementations

// Re-export commonly used types
pub use ability::{
    Ability, AbilityId, AbilityState, ActiveAbility, ActiveEffect, ActiveEffectInstance,
    AuraAbility, AuraEffect, AuraTarget, BuffStat, PassiveAbility, PassiveEffect, PassiveTrigger,
    TargetType,
};
pub use attack::Attack;
pub use base_unit::BaseUnit;
pub use combat::{CombatAction, CombatResult, CombatStats};
pub use items::{ConsumableEffect, Equipment, Item, ItemProperties, ItemType, RangeType};
pub use team::Team;
pub use unit_factory::UnitFactory;
pub use unit_race::{Race, Terrain};
pub use unit_registry::{UnitRegistry, UnitTypeInfo};
pub use unit_trait::UnitId;
pub use unit_type::UnitType;

// Export the trait-based Unit interface
pub use unit_trait::Unit;

// Export concrete unit types
pub use units::{
    DwarfVeteranWarrior, DwarfWarrior, DwarfYoungWarrior, ElfArcher, ElfMage, ElfWarrior,
    OrcEliteSwordsman, OrcSwordsman, OrcYoungSwordsman,
};

// Export structure types and factory
pub use structures::{
    StoneWall, Structure, StructureCategory, StructureFactory, StructureId, StructureStats,
    StructureType,
};
