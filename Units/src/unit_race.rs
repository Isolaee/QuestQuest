use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents different races available in the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Orc,
    Goblin,
    Undead,
}

impl Race {
    /// Get racial defense bonus modifier
    pub fn get_defense_bonus(self) -> i32 {
        match self {
            Race::Human => 0,   // Balanced
            Race::Elf => -1,    // Agile but fragile
            Race::Dwarf => 2,   // Tough and resilient
            Race::Orc => 1,     // Hardy warriors
            Race::Goblin => -2, // Small and weak
            Race::Undead => 1,  // Resistant to damage
        }
    }

    /// Get racial attack bonus modifier
    pub fn get_attack_bonus(self) -> i32 {
        match self {
            Race::Human => 0,   // Balanced
            Race::Elf => 1,     // Precise and skilled
            Race::Dwarf => 0,   // Steady fighters
            Race::Orc => 2,     // Brutal attackers
            Race::Goblin => -1, // Weak but numerous
            Race::Undead => 0,  // Relentless but not skilled
        }
    }

    /// Get racial movement speed modifier
    pub fn get_movement_bonus(self) -> i32 {
        match self {
            Race::Human => 0,   // Standard speed
            Race::Elf => 1,     // Swift and graceful
            Race::Dwarf => -1,  // Short legs, slower
            Race::Orc => 0,     // Standard speed
            Race::Goblin => 1,  // Small and quick
            Race::Undead => -1, // Shambling gait
        }
    }

    /// Get the display name of the race
    pub fn get_name(self) -> &'static str {
        match self {
            Race::Human => "Human",
            Race::Elf => "Elf",
            Race::Dwarf => "Dwarf",
            Race::Orc => "Orc",
            Race::Goblin => "Goblin",
            Race::Undead => "Undead",
        }
    }

    /// Get the display color for rendering units of this race
    pub fn get_display_color(self) -> [f32; 3] {
        match self {
            Race::Human => [0.8, 0.7, 0.6],  // Flesh tone
            Race::Elf => [0.9, 0.9, 0.7],    // Pale
            Race::Dwarf => [0.7, 0.6, 0.5],  // Ruddy
            Race::Orc => [0.4, 0.6, 0.3],    // Green
            Race::Goblin => [0.5, 0.7, 0.4], // Light green
            Race::Undead => [0.6, 0.6, 0.7], // Pale blue
        }
    }

    /// Get all available races
    pub fn all_races() -> [Race; 6] {
        [
            Race::Human,
            Race::Elf,
            Race::Dwarf,
            Race::Orc,
            Race::Goblin,
            Race::Undead,
        ]
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Race::Human => write!(f, "Human"),
            Race::Elf => write!(f, "Elf"),
            Race::Dwarf => write!(f, "Dwarf"),
            Race::Orc => write!(f, "Orc"),
            Race::Goblin => write!(f, "Goblin"),
            Race::Undead => write!(f, "Undead"),
        }
    }
}
