//! # Structures Module
//!
//! This module provides a comprehensive structure system for the QuestQuest game.
//! Structures can be built, occupied by units, destroyed (especially by siege units),
//! and provide various tactical bonuses to occupying units.
//!
//! ## Core Components
//!
//! - **[`Structure`]** trait: Main interface that all structures implement
//! - **[`StructureStats`]**: Statistics and properties for structures
//! - **[`StructureType`]**: Enum of all available structure types
//! - **[`StructureFactory`]**: Factory for creating structure instances
//!
//! ## Structure Categories
//!
//! - **Fortifications**: Walls, towers, gates - defensive strongpoints
//! - **Buildings**: Houses, barracks, arsenals - provide various bonuses
//! - **Defensive**: Barricades, trenches, spikes - tactical positioning
//!
//! ## Key Features
//!
//! ### Occupation
//! Units can move onto structure hexes and occupy them to gain bonuses:
//! - Defense bonuses (cover)
//! - Attack bonuses (elevated positions, weapon caches)
//! - Range bonuses (towers)
//! - Healing over time (rest in buildings)
//!
//! ### Durability & Destruction
//! All structures have durability that can be depleted through combat:
//! - Regular units deal reduced damage to structures
//! - Siege units deal bonus damage (2-3x multiplier)
//! - Occupied structures can be repaired over time
//! - Destroyed structures are removed from the map
//!
//! ### Movement Blocking
//! Some structures block movement:
//! - Walls block all movement until destroyed
//! - Gates allow friendly units to pass
//! - Buildings typically allow free entry
//!
//! ## Example Usage
//!
//! See `units/examples/structure_demo.rs` for a complete example.

pub mod occupancy;
pub mod structure_factory;
pub mod structure_stats;
pub mod structure_trait;
pub mod structure_type;
pub mod structure_units;

// Re-export commonly used types
pub use occupancy::{can_occupy, is_occupying, OccupancyBonus};
pub use structure_factory::StructureFactory;
pub use structure_stats::StructureStats;
pub use structure_trait::{Structure, StructureId};
pub use structure_type::{StructureCategory, StructureType};
pub use structure_units::{House, StoneWall};
