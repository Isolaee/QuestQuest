use items::RangeType;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents different unit classes/professions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitClass {
    Warrior,
    Archer,
    Mage,
    Rogue,
    Cleric,
    Paladin,
    Ranger,
}

impl UnitClass {
    /// Get the display name of the class
    pub fn get_name(self) -> &'static str {
        match self {
            UnitClass::Warrior => "Warrior",
            UnitClass::Archer => "Archer",
            UnitClass::Mage => "Mage",
            UnitClass::Rogue => "Rogue",
            UnitClass::Cleric => "Cleric",
            UnitClass::Paladin => "Paladin",
            UnitClass::Ranger => "Ranger",
        }
    }

    /// Get default range type for this class
    pub fn get_default_range(self) -> RangeType {
        match self {
            UnitClass::Warrior => RangeType::Melee,
            UnitClass::Archer => RangeType::Ranged,
            UnitClass::Mage => RangeType::Ranged,
            UnitClass::Rogue => RangeType::Melee,
            UnitClass::Cleric => RangeType::Melee,
            UnitClass::Paladin => RangeType::Melee,
            UnitClass::Ranger => RangeType::Ranged,
        }
    }

    /// Get class defense bonus
    pub fn get_defense_bonus(self) -> i32 {
        match self {
            UnitClass::Warrior => 3, // Heavy armor
            UnitClass::Archer => 0,  // Medium armor
            UnitClass::Mage => -2,   // Robes only
            UnitClass::Rogue => 1,   // Light armor + agility
            UnitClass::Cleric => 2,  // Medium armor + divine protection
            UnitClass::Paladin => 4, // Heavy armor + divine protection
            UnitClass::Ranger => 1,  // Light armor + nature knowledge
        }
    }

    /// Get class attack bonus
    pub fn get_attack_bonus(self) -> i32 {
        match self {
            UnitClass::Warrior => 2, // Weapon mastery
            UnitClass::Archer => 1,  // Ranged precision
            UnitClass::Mage => 0,    // Magic damage (separate system)
            UnitClass::Rogue => 3,   // Sneak attack
            UnitClass::Cleric => 0,  // Divine support
            UnitClass::Paladin => 2, // Divine might
            UnitClass::Ranger => 1,  // Nature bond
        }
    }

    /// Get base health for this class
    pub fn get_base_health(self) -> i32 {
        match self {
            UnitClass::Warrior => 120, // Tank
            UnitClass::Archer => 80,   // Medium health
            UnitClass::Mage => 60,     // Fragile
            UnitClass::Rogue => 90,    // Moderate health
            UnitClass::Cleric => 100,  // Supportive role
            UnitClass::Paladin => 110, // Divine resilience
            UnitClass::Ranger => 85,   // Hardy outdoorsman
        }
    }

    /// Get movement speed for this class
    pub fn get_movement_speed(self) -> i32 {
        match self {
            UnitClass::Warrior => 3, // Heavy armor slows down
            UnitClass::Archer => 4,  // Standard speed
            UnitClass::Mage => 3,    // Not built for speed
            UnitClass::Rogue => 5,   // Fast and stealthy
            UnitClass::Cleric => 4,  // Standard speed
            UnitClass::Paladin => 3, // Heavy armor
            UnitClass::Ranger => 5,  // Swift in nature
        }
    }

    /// Get all available classes
    pub fn all_classes() -> [UnitClass; 7] {
        [
            UnitClass::Warrior,
            UnitClass::Archer,
            UnitClass::Mage,
            UnitClass::Rogue,
            UnitClass::Cleric,
            UnitClass::Paladin,
            UnitClass::Ranger,
        ]
    }
}

impl fmt::Display for UnitClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
