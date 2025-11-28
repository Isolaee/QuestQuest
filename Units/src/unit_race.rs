//! Character races and terrain system.
//!
//! This module defines the various playable and non-playable races in the game,
//! along with terrain types and their interactions. Each race has different
//! defensive bonuses based on the terrain they're standing on.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
    Dragonborn,
    Orc,
    Goblin,
    Hobgoblin,
    Kobold,
    Lizardfolk,
    Triton,
    Changeling,
    Skeleton,
    Zombie,
    Undead,
}

impl Race {
    /// Returns the racial base defense value based on terrain.
    ///
    /// **NOTE**: This method returns 99 (very poor defense) as a fallback value.
    /// Individual unit implementations should define their own terrain-specific
    /// defenses via the `terrain_defenses` HashMap in `BaseUnit`.
    ///
    /// Lower values are better (harder to hit). This represents how well
    /// a unit can utilize terrain for defensive purposes.
    ///
    /// # Arguments
    ///
    /// * `_terrain` - The terrain type to check (currently unused as all return 99)
    ///
    /// # Returns
    ///
    /// Always returns 99 to indicate that unit-specific terrain defenses should be used.
    pub fn get_base_defense(self, _terrain: Terrain) -> u8 {
        // All races return 99 as a fallback to encourage unit-specific terrain defense definitions
        // Individual units should define their terrain defenses in BaseUnit.terrain_defenses
        99
    }

    /// Returns the terrain-based hit chance for attacks.
    ///
    /// This is equivalent to `get_base_defense` but framed as an offensive value.
    /// Higher values mean the target is easier to hit.
    ///
    /// # Arguments
    ///
    /// * `terrain` - The terrain type where combat occurs
    ///
    /// # Returns
    ///
    /// A percentage (0-100) representing the base hit chance against this race
    /// on this terrain before other modifiers.
    pub fn get_terrain_hit_chance(self, terrain: Terrain) -> u8 {
        // The get_base_defense returns "how hard to hit" percentage
        // We use it directly as hit chance (lower defense = harder to hit = lower hit chance)
        self.get_base_defense(terrain)
    }

    /// Returns the display name of the race.
    ///
    /// # Returns
    ///
    /// A string slice with the race's human-readable name.
    pub fn get_name(self) -> &'static str {
        match self {
            Race::Human => "Human",
            Race::Elf => "Elf",
            Race::Dwarf => "Dwarf",
            Race::Halfling => "Halfling",
            Race::Gnome => "Gnome",
            Race::HalfElf => "Half-Elf",
            Race::HalfOrc => "Half-Orc",
            Race::Tiefling => "Tiefling",
            Race::Dragonborn => "Dragonborn",
            Race::Orc => "Orc",
            Race::Goblin => "Goblin",
            Race::Hobgoblin => "Hobgoblin",
            Race::Kobold => "Kobold",
            Race::Lizardfolk => "Lizardfolk",
            Race::Triton => "Triton",
            Race::Changeling => "Changeling",
            Race::Skeleton => "Skeleton",
            Race::Zombie => "Zombie",
            Race::Undead => "Undead",
        }
    }

    /// Returns the display color for rendering units of this race.
    ///
    /// Each race has a distinctive color for easy visual identification
    /// on the game map.
    ///
    /// # Returns
    ///
    /// An RGB color array with values in the range [0.0, 1.0].
    pub fn get_display_color(self) -> [f32; 3] {
        match self {
            Race::Human => [0.8, 0.7, 0.6],         // Flesh tone
            Race::Elf => [0.9, 0.9, 0.7],           // Pale
            Race::Dwarf => [0.7, 0.6, 0.5],         // Ruddy
            Race::Halfling => [0.8, 0.65, 0.55],    // Rosy
            Race::Gnome => [0.75, 0.7, 0.65],       // Tan
            Race::HalfElf => [0.85, 0.8, 0.65],     // Mixed tone
            Race::HalfOrc => [0.6, 0.65, 0.5],      // Gray-green
            Race::Tiefling => [0.7, 0.4, 0.4],      // Red skin
            Race::Dragonborn => [0.6, 0.5, 0.4],    // Scaled bronze
            Race::Orc => [0.4, 0.6, 0.3],           // Green
            Race::Goblin => [0.5, 0.7, 0.4],        // Light green
            Race::Hobgoblin => [0.6, 0.5, 0.3],     // Orange-brown
            Race::Kobold => [0.6, 0.4, 0.3],        // Brown-red
            Race::Lizardfolk => [0.4, 0.6, 0.5],    // Reptilian green
            Race::Triton => [0.5, 0.7, 0.8],        // Blue-green
            Race::Changeling => [0.85, 0.85, 0.85], // Pale gray
            Race::Skeleton => [0.6, 0.6, 0.7],      // Pale blue
            Race::Zombie => [0.7, 0.7, 0.7],        // Gray
            Race::Undead => [0.5, 0.5, 0.5],        // Dark gray
        }
    }

    /// Returns all available races in the game.
    ///
    /// # Returns
    ///
    /// An array containing all 19 race variants.
    pub fn all_races() -> [Race; 19] {
        [
            Race::Human,
            Race::Elf,
            Race::Dwarf,
            Race::Halfling,
            Race::Gnome,
            Race::HalfElf,
            Race::HalfOrc,
            Race::Tiefling,
            Race::Dragonborn,
            Race::Orc,
            Race::Goblin,
            Race::Hobgoblin,
            Race::Kobold,
            Race::Lizardfolk,
            Race::Triton,
            Race::Changeling,
            Race::Skeleton,
            Race::Zombie,
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
            Race::Halfling => write!(f, "Halfling"),
            Race::Gnome => write!(f, "Gnome"),
            Race::HalfElf => write!(f, "Half-Elf"),
            Race::HalfOrc => write!(f, "Half-Orc"),
            Race::Tiefling => write!(f, "Tiefling"),
            Race::Dragonborn => write!(f, "Dragonborn"),
            Race::Orc => write!(f, "Orc"),
            Race::Goblin => write!(f, "Goblin"),
            Race::Hobgoblin => write!(f, "Hobgoblin"),
            Race::Kobold => write!(f, "Kobold"),
            Race::Lizardfolk => write!(f, "Lizardfolk"),
            Race::Triton => write!(f, "Triton"),
            Race::Changeling => write!(f, "Changeling"),
            Race::Skeleton => write!(f, "Skeleton"),
            Race::Zombie => write!(f, "Zombie"),
            Race::Undead => write!(f, "Undead"),
        }
    }
}

/// Represents different terrain types in the game.
///
/// Terrain affects movement costs, defensive bonuses, and line of sight.
/// Different races perform better on certain terrains.
///
/// # Variants
///
/// - `Forest0`, `Forest1`: Dense woodland with cover
/// - `Grasslands`: Open plains (default terrain)
/// - `HauntedWoods`: Cursed forests with undead advantages
/// - `Hills`: Elevated terrain with defensive bonuses
/// - `Mountain`: High altitude, difficult terrain
/// - `Swamp`: Waterlogged terrain, slows most units
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Terrain {
    Forest0,
    Forest1,
    #[default]
    Grasslands,
    HauntedWoods,
    Hills,
    Mountain,
    Swamp,
}

impl Terrain {
    /// Get the display name of this terrain type
    pub fn name(self) -> &'static str {
        match self {
            Terrain::Forest0 | Terrain::Forest1 => "Forest",
            Terrain::Grasslands => "Grasslands",
            Terrain::HauntedWoods => "Haunted Woods",
            Terrain::Hills => "Hills",
            Terrain::Mountain => "Mountain",
            Terrain::Swamp => "Swamp",
        }
    }

    /// Get the hit chance for a race on this terrain (defense value)
    pub fn get_hit_chance(self, race: Race) -> u8 {
        race.get_base_defense(self)
    }
}
