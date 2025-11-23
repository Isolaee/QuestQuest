//! Structure type definitions and categories.
//!
//! This module defines the various structure types available in the game,
//! organized by category (Fortifications, Buildings, Defensive).

use serde::{Deserialize, Serialize};

/// Categories of structures for organizational purposes.
///
/// Structures are grouped by their primary function to help with
/// filtering, UI organization, and gameplay balance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructureCategory {
    /// Fortifications like walls, towers, and gates
    Fortification,
    /// Buildings like houses, barracks, and arsenals
    Building,
    /// Defensive structures like barricades and trenches
    Defensive,
    /// Resource structures like mines and farms
    Resource,
}

/// Specific types of structures that can be built.
///
/// Each structure type has unique properties, bonuses, and tactical uses.
/// Structures can be occupied by units to gain benefits, and some can be
/// destroyed (especially by siege units).
///
/// # Examples
///
/// ```
/// use units::structures::{StructureType, StructureCategory};
///
/// let wall = StructureType::StoneWall;
/// assert_eq!(wall.category(), StructureCategory::Fortification);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructureType {
    // Fortifications
    /// High defense stone wall, blocks movement unless destroyed
    StoneWall,
    /// Medium defense wooden wall, vulnerable to fire
    WoodenWall,
    /// Tower that grants vision and range bonuses
    Watchtower,
    /// Gate that allows friendly passage, blocks enemies
    Gate,

    // Buildings
    /// House that provides healing and rest
    House,
    /// Barracks for recruitment and defense
    Barracks,
    /// Arsenal that grants attack bonuses
    Arsenal,

    // Defensive
    /// Quick-build barricade with medium defense
    Barricade,
    /// Trench that provides defense bonus
    Trench,
    /// Spike trap that damages melee attackers
    Spikes,
}

impl StructureType {
    /// Returns the category this structure type belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use units::structures::{StructureType, StructureCategory};
    ///
    /// assert_eq!(StructureType::StoneWall.category(), StructureCategory::Fortification);
    /// assert_eq!(StructureType::House.category(), StructureCategory::Building);
    /// ```
    pub fn category(&self) -> StructureCategory {
        match self {
            StructureType::StoneWall
            | StructureType::WoodenWall
            | StructureType::Watchtower
            | StructureType::Gate => StructureCategory::Fortification,

            StructureType::House | StructureType::Barracks | StructureType::Arsenal => {
                StructureCategory::Building
            }

            StructureType::Barricade | StructureType::Trench | StructureType::Spikes => {
                StructureCategory::Defensive
            }
        }
    }

    /// Returns the display name of this structure type.
    pub fn name(&self) -> &'static str {
        match self {
            StructureType::StoneWall => "Stone Wall",
            StructureType::WoodenWall => "Wooden Wall",
            StructureType::Watchtower => "Watchtower",
            StructureType::Gate => "Gate",
            StructureType::House => "House",
            StructureType::Barracks => "Barracks",
            StructureType::Arsenal => "Arsenal",
            StructureType::Barricade => "Barricade",
            StructureType::Trench => "Trench",
            StructureType::Spikes => "Spikes",
        }
    }

    /// Returns a brief description of this structure type.
    pub fn description(&self) -> &'static str {
        match self {
            StructureType::StoneWall => {
                "Heavy fortification that blocks movement and provides excellent defense"
            }
            StructureType::WoodenWall => {
                "Moderate fortification, vulnerable to fire but easier to build"
            }
            StructureType::Watchtower => "Elevated position that increases vision and attack range",
            StructureType::Gate => {
                "Controlled passage that allows allies through but blocks enemies"
            }
            StructureType::House => {
                "Shelter that provides healing and protection from the elements"
            }
            StructureType::Barracks => {
                "Military structure that enhances defense and serves as recruitment point"
            }
            StructureType::Arsenal => {
                "Weapon storage that grants attack bonuses to occupying units"
            }
            StructureType::Barricade => "Quick defensive position with moderate protection",
            StructureType::Trench => "Dug defensive position that provides cover bonus",
            StructureType::Spikes => "Trap that damages enemies attempting melee attacks",
        }
    }
}
