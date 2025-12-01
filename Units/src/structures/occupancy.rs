//! Occupation management for structures.
//!
//! This module provides utilities for managing unit occupation of structures,
//! including applying bonuses to occupying units.

use crate::combat::{CombatStats, Resistances};
use crate::unit_trait::UnitId;

/// Helper struct for managing structure occupation bonuses.
#[derive(Clone, Debug, Default)]
pub struct OccupancyBonus {
    pub defense: i32,
    pub attack: i32,
    pub range: i32,
    pub resistance_bonuses: Resistances,
    pub vision: i32,
    pub healing: u32,
}

impl OccupancyBonus {
    /// Creates a new occupancy bonus with the specified values.
    pub fn new(
        defense: i32,
        attack: i32,
        range: i32,
        resistance_bonuses: Resistances,
        vision: i32,
        healing: u32,
    ) -> Self {
        Self {
            defense,
            attack,
            range,
            resistance_bonuses,
            vision,
            healing,
        }
    }

    /// Applies this bonus to a unit's combat stats.
    ///
    /// This modifies the unit's stats to include structure bonuses.
    /// Note: This is a helper function - actual implementation will depend
    /// on how your unit system handles temporary bonuses.
    /// Attack bonuses are applied to attack_modifier field.
    pub fn apply_to_stats(&self, stats: &mut CombatStats) {
        // CombatStats doesn't have a defense field, but attack_modifier can be used
        stats.attack_modifier += self.attack;

        // Apply resistance bonuses
        stats.resistances.blunt = (stats.resistances.blunt as i32
            + self.resistance_bonuses.blunt as i32)
            .clamp(0, 100) as u8;
        stats.resistances.pierce = (stats.resistances.pierce as i32
            + self.resistance_bonuses.pierce as i32)
            .clamp(0, 100) as u8;
        stats.resistances.fire = (stats.resistances.fire as i32
            + self.resistance_bonuses.fire as i32)
            .clamp(0, 100) as u8;
        stats.resistances.dark = (stats.resistances.dark as i32
            + self.resistance_bonuses.dark as i32)
            .clamp(0, 100) as u8;
        stats.resistances.slash = (stats.resistances.slash as i32
            + self.resistance_bonuses.slash as i32)
            .clamp(0, 100) as u8;
        stats.resistances.crush = (stats.resistances.crush as i32
            + self.resistance_bonuses.crush as i32)
            .clamp(0, 100) as u8;
    }

    /// Removes this bonus from a unit's combat stats.
    pub fn remove_from_stats(&self, stats: &mut CombatStats) {
        stats.attack_modifier -= self.attack;

        // Remove resistance bonuses
        stats.resistances.blunt = (stats.resistances.blunt as i32
            - self.resistance_bonuses.blunt as i32)
            .clamp(0, 100) as u8;
        stats.resistances.pierce = (stats.resistances.pierce as i32
            - self.resistance_bonuses.pierce as i32)
            .clamp(0, 100) as u8;
        stats.resistances.fire = (stats.resistances.fire as i32
            - self.resistance_bonuses.fire as i32)
            .clamp(0, 100) as u8;
        stats.resistances.dark = (stats.resistances.dark as i32
            - self.resistance_bonuses.dark as i32)
            .clamp(0, 100) as u8;
        stats.resistances.slash = (stats.resistances.slash as i32
            - self.resistance_bonuses.slash as i32)
            .clamp(0, 100) as u8;
        stats.resistances.crush = (stats.resistances.crush as i32
            - self.resistance_bonuses.crush as i32)
            .clamp(0, 100) as u8;
    }
}

/// Checks if a unit can occupy a structure.
///
/// # Arguments
///
/// * `structure_max_occupants` - Maximum occupants allowed
/// * `current_occupants` - Current occupant count
///
/// # Returns
///
/// `true` if there is space for another occupant
pub fn can_occupy(structure_max_occupants: u32, current_occupants: usize) -> bool {
    (current_occupants as u32) < structure_max_occupants
}

/// Checks if a unit ID is in the occupants list.
pub fn is_occupying(unit_id: UnitId, occupants: &[UnitId]) -> bool {
    occupants.contains(&unit_id)
}
