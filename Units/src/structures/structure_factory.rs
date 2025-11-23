//! Factory for creating various structure types.
//!
//! This module provides convenient constructors for all structure types
//! with predefined, balanced statistics.

use super::structure_units::stone_wall::StoneWall;
use crate::team::Team;
use graphics::HexCoord;

/// Factory for creating structure instances.
pub struct StructureFactory;

impl StructureFactory {
    /// Creates a stone wall structure.
    ///
    /// Stone walls are heavy fortifications with:
    /// - High durability (500 HP)
    /// - Excellent resistances to physical damage
    /// - Vulnerable to siege weapons (2.5x damage)
    /// - Blocks all movement
    /// - Can hold 2 occupants
    /// - Grants +15 defense to occupants
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate for the wall
    /// * `team` - Which team controls this wall
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use units::structures::StructureFactory;
    /// use graphics::HexCoord;
    /// use units::Team;
    ///
    /// let wall = StructureFactory::create_stone_wall(
    ///     HexCoord::new(10, 10),
    ///     Team::Player,
    /// );
    /// ```
    pub fn create_stone_wall(position: HexCoord, team: Team) -> Box<StoneWall> {
        Box::new(StoneWall::new(position, team))
    }

    // TODO: Add factory methods for other structure types:
    // - create_wooden_wall
    // - create_watchtower
    // - create_gate
    // - create_house
    // - create_barracks
    // - create_arsenal
    // - create_barricade
    // - create_trench
    // - create_spikes
}
