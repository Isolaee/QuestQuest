//! Core structure trait and related types.
//!
//! This module defines the main `Structure` trait that all structures
//! in the game must implement, providing a unified interface for
//! structure management, occupation, and combat.

use crate::attack::Attack;
use crate::combat::Resistances;
use crate::team::Team;
use crate::unit_race::Terrain;
use crate::unit_trait::UnitId;
use graphics::HexCoord;
use uuid::Uuid;

use super::structure_type::StructureType;

/// Unique identifier for structures.
pub type StructureId = Uuid;

/// Core trait that all structures must implement.
///
/// The `Structure` trait provides a unified interface for all structure types,
/// handling identity, position, stats, occupation, and combat interactions.
///
/// # Examples
///
/// ```rust,no_run
/// use units::structures::{Structure, StructureFactory};
/// use graphics::HexCoord;
/// use units::Team;
///
/// let wall = StructureFactory::create_stone_wall(
///     HexCoord::new(5, 5),
///     Team::Player,
/// );
///
/// println!("Structure: {}", wall.name());
/// println!("Durability: {}/{}", wall.current_durability(), wall.max_durability());
/// ```
pub trait Structure {
    // ===== Identity =====

    /// Returns the structure's unique identifier.
    fn id(&self) -> StructureId;

    /// Returns the structure's display name.
    fn name(&self) -> &str;

    /// Returns the structure type.
    fn structure_type(&self) -> StructureType;

    /// Returns the current position on the hex grid.
    fn position(&self) -> HexCoord;

    /// Sets the position on the hex grid.
    fn set_position(&mut self, position: HexCoord);

    /// Returns the team that controls this structure.
    fn team(&self) -> Team;

    /// Sets the controlling team.
    fn set_team(&mut self, team: Team);

    // ===== Durability =====

    /// Returns maximum durability.
    fn max_durability(&self) -> u32;

    /// Returns current durability.
    fn current_durability(&self) -> u32;

    /// Checks if the structure is destroyed.
    fn is_destroyed(&self) -> bool;

    /// Applies damage to the structure.
    ///
    /// # Arguments
    ///
    /// * `damage` - Base damage value
    /// * `is_siege` - Whether this damage is from a siege unit
    ///
    /// # Returns
    ///
    /// Actual damage dealt after resistances
    fn take_damage(&mut self, damage: u32, is_siege: bool) -> u32;

    /// Repairs the structure.
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount of durability to restore
    ///
    /// # Returns
    ///
    /// Actual amount repaired
    fn repair(&mut self, amount: u32) -> u32;

    /// Performs automatic repair (called each turn).
    fn auto_repair(&mut self) -> u32;

    // ===== Occupation =====

    /// Returns maximum number of occupants.
    fn max_occupants(&self) -> u32;

    /// Returns current occupant IDs.
    fn occupants(&self) -> &[UnitId];

    /// Checks if structure has space for more occupants.
    fn has_space(&self) -> bool;

    /// Checks if a specific unit is occupying.
    fn is_occupied_by(&self, unit_id: UnitId) -> bool;

    /// Adds an occupant.
    fn add_occupant(&mut self, unit_id: UnitId) -> Result<(), String>;

    /// Removes an occupant.
    fn remove_occupant(&mut self, unit_id: UnitId) -> bool;

    // ===== Bonuses =====

    /// Returns defense bonus granted to occupants.
    fn defense_bonus(&self) -> i32;

    /// Returns attack bonus granted to occupants.
    fn attack_bonus(&self) -> i32;

    /// Returns range bonus granted to occupants.
    fn range_bonus(&self) -> i32;

    /// Returns resistance bonuses granted to occupants.
    fn resistance_bonuses(&self) -> &Resistances;

    /// Returns vision bonus granted to occupants.
    fn vision_bonus(&self) -> i32;

    /// Returns healing per turn for occupants.
    fn healing_per_turn(&self) -> u32;

    // ===== Movement & Blocking =====

    /// Returns whether this structure blocks movement.
    fn blocks_movement(&self) -> bool;

    /// Returns which team (if any) can pass through.
    fn allows_passage_team(&self) -> Option<Team>;

    /// Returns extra movement cost to enter.
    fn movement_cost_modifier(&self) -> i32;

    /// Checks if a team can pass through this structure.
    fn can_pass_through(&self, team: Team) -> bool;

    // ===== Combat =====

    /// Returns thorns damage dealt to melee attackers.
    fn thorns_damage(&self) -> u32;

    /// Returns whether this structure can initiate attacks.
    fn can_attack(&self) -> bool;

    /// Returns available attacks.
    fn attacks(&self) -> &[Attack];

    // ===== Terrain =====

    /// Returns terrains this structure can be built on.
    fn buildable_on(&self) -> &[Terrain];

    /// Checks if structure can be built on specific terrain.
    fn can_build_on(&self, terrain: Terrain) -> bool;

    /// Returns terrain type this structure simulates for bonuses.
    fn provides_terrain_bonus(&self) -> Option<Terrain>;
}
