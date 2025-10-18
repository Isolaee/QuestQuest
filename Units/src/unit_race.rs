use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents different races available in the game
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
    /// Get racial base defense (hit chance %) based on terrain - lower is better
    /// This represents how hard the unit is to hit based on racial traits and terrain
    pub fn get_base_defense(self, terrain: Terrain) -> u8 {
        match (self, terrain) {
            // Human - adaptable, decent everywhere
            (Race::Human, Terrain::Forest0) => 50,
            (Race::Human, Terrain::Forest1) => 50,
            (Race::Human, Terrain::Grasslands) => 48,
            (Race::Human, Terrain::HauntedWoods) => 52,
            (Race::Human, Terrain::Hills) => 50,
            (Race::Human, Terrain::Mountain) => 53,
            (Race::Human, Terrain::Swamp) => 54,

            // Elf - excellent in forests, good in nature
            (Race::Elf, Terrain::Forest0) => 42,
            (Race::Elf, Terrain::Forest1) => 42,
            (Race::Elf, Terrain::Grasslands) => 45,
            (Race::Elf, Terrain::HauntedWoods) => 44,
            (Race::Elf, Terrain::Hills) => 47,
            (Race::Elf, Terrain::Mountain) => 50,
            (Race::Elf, Terrain::Swamp) => 52,

            // Dwarf - excellent in mountains/hills, poor in swamps
            (Race::Dwarf, Terrain::Forest0) => 55,
            (Race::Dwarf, Terrain::Forest1) => 55,
            (Race::Dwarf, Terrain::Grasslands) => 54,
            (Race::Dwarf, Terrain::HauntedWoods) => 57,
            (Race::Dwarf, Terrain::Hills) => 50,
            (Race::Dwarf, Terrain::Mountain) => 48,
            (Race::Dwarf, Terrain::Swamp) => 60,

            // Halfling - good in forests and grasslands
            (Race::Halfling, Terrain::Forest0) => 40,
            (Race::Halfling, Terrain::Forest1) => 40,
            (Race::Halfling, Terrain::Grasslands) => 38,
            (Race::Halfling, Terrain::HauntedWoods) => 43,
            (Race::Halfling, Terrain::Hills) => 42,
            (Race::Halfling, Terrain::Mountain) => 46,
            (Race::Halfling, Terrain::Swamp) => 48,

            // Gnome - good in forests, decent in hills
            (Race::Gnome, Terrain::Forest0) => 41,
            (Race::Gnome, Terrain::Forest1) => 41,
            (Race::Gnome, Terrain::Grasslands) => 43,
            (Race::Gnome, Terrain::HauntedWoods) => 42,
            (Race::Gnome, Terrain::Hills) => 43,
            (Race::Gnome, Terrain::Mountain) => 47,
            (Race::Gnome, Terrain::Swamp) => 49,

            // HalfElf - versatile like elves but less specialized
            (Race::HalfElf, Terrain::Forest0) => 45,
            (Race::HalfElf, Terrain::Forest1) => 45,
            (Race::HalfElf, Terrain::Grasslands) => 46,
            (Race::HalfElf, Terrain::HauntedWoods) => 47,
            (Race::HalfElf, Terrain::Hills) => 48,
            (Race::HalfElf, Terrain::Mountain) => 50,
            (Race::HalfElf, Terrain::Swamp) => 51,

            // HalfOrc - tough, decent in most terrains
            (Race::HalfOrc, Terrain::Forest0) => 52,
            (Race::HalfOrc, Terrain::Forest1) => 52,
            (Race::HalfOrc, Terrain::Grasslands) => 50,
            (Race::HalfOrc, Terrain::HauntedWoods) => 53,
            (Race::HalfOrc, Terrain::Hills) => 51,
            (Race::HalfOrc, Terrain::Mountain) => 52,
            (Race::HalfOrc, Terrain::Swamp) => 54,

            // Tiefling - decent everywhere, slightly better in haunted areas
            (Race::Tiefling, Terrain::Forest0) => 48,
            (Race::Tiefling, Terrain::Forest1) => 48,
            (Race::Tiefling, Terrain::Grasslands) => 48,
            (Race::Tiefling, Terrain::HauntedWoods) => 45,
            (Race::Tiefling, Terrain::Hills) => 49,
            (Race::Tiefling, Terrain::Mountain) => 51,
            (Race::Tiefling, Terrain::Swamp) => 50,

            // Dragonborn - large, easier to hit
            (Race::Dragonborn, Terrain::Forest0) => 55,
            (Race::Dragonborn, Terrain::Forest1) => 55,
            (Race::Dragonborn, Terrain::Grasslands) => 52,
            (Race::Dragonborn, Terrain::HauntedWoods) => 56,
            (Race::Dragonborn, Terrain::Hills) => 53,
            (Race::Dragonborn, Terrain::Mountain) => 51,
            (Race::Dragonborn, Terrain::Swamp) => 57,

            // Orc - large and brutal
            (Race::Orc, Terrain::Forest0) => 54,
            (Race::Orc, Terrain::Forest1) => 54,
            (Race::Orc, Terrain::Grasslands) => 52,
            (Race::Orc, Terrain::HauntedWoods) => 55,
            (Race::Orc, Terrain::Hills) => 53,
            (Race::Orc, Terrain::Mountain) => 54,
            (Race::Orc, Terrain::Swamp) => 56,

            // Goblin - small, evasive, good in forests/swamps
            (Race::Goblin, Terrain::Forest0) => 38,
            (Race::Goblin, Terrain::Forest1) => 38,
            (Race::Goblin, Terrain::Grasslands) => 42,
            (Race::Goblin, Terrain::HauntedWoods) => 37,
            (Race::Goblin, Terrain::Hills) => 40,
            (Race::Goblin, Terrain::Mountain) => 43,
            (Race::Goblin, Terrain::Swamp) => 36,

            // Hobgoblin - disciplined warriors
            (Race::Hobgoblin, Terrain::Forest0) => 51,
            (Race::Hobgoblin, Terrain::Forest1) => 51,
            (Race::Hobgoblin, Terrain::Grasslands) => 49,
            (Race::Hobgoblin, Terrain::HauntedWoods) => 52,
            (Race::Hobgoblin, Terrain::Hills) => 50,
            (Race::Hobgoblin, Terrain::Mountain) => 52,
            (Race::Hobgoblin, Terrain::Swamp) => 53,

            // Kobold - very small, excellent in caves/mountains
            (Race::Kobold, Terrain::Forest0) => 40,
            (Race::Kobold, Terrain::Forest1) => 40,
            (Race::Kobold, Terrain::Grasslands) => 42,
            (Race::Kobold, Terrain::HauntedWoods) => 39,
            (Race::Kobold, Terrain::Hills) => 36,
            (Race::Kobold, Terrain::Mountain) => 34,
            (Race::Kobold, Terrain::Swamp) => 38,

            // Lizardfolk - excellent in swamps
            (Race::Lizardfolk, Terrain::Forest0) => 56,
            (Race::Lizardfolk, Terrain::Forest1) => 56,
            (Race::Lizardfolk, Terrain::Grasslands) => 55,
            (Race::Lizardfolk, Terrain::HauntedWoods) => 57,
            (Race::Lizardfolk, Terrain::Hills) => 58,
            (Race::Lizardfolk, Terrain::Mountain) => 60,
            (Race::Lizardfolk, Terrain::Swamp) => 48,

            // Triton - poor on land, terrible in mountains
            (Race::Triton, Terrain::Forest0) => 52,
            (Race::Triton, Terrain::Forest1) => 52,
            (Race::Triton, Terrain::Grasslands) => 50,
            (Race::Triton, Terrain::HauntedWoods) => 53,
            (Race::Triton, Terrain::Hills) => 54,
            (Race::Triton, Terrain::Mountain) => 58,
            (Race::Triton, Terrain::Swamp) => 46,

            // Changeling - adaptable, deceptive
            (Race::Changeling, Terrain::Forest0) => 46,
            (Race::Changeling, Terrain::Forest1) => 46,
            (Race::Changeling, Terrain::Grasslands) => 45,
            (Race::Changeling, Terrain::HauntedWoods) => 45,
            (Race::Changeling, Terrain::Hills) => 47,
            (Race::Changeling, Terrain::Mountain) => 49,
            (Race::Changeling, Terrain::Swamp) => 48,

            // Skeleton - undead, good in haunted woods
            (Race::Skeleton, Terrain::Forest0) => 58,
            (Race::Skeleton, Terrain::Forest1) => 58,
            (Race::Skeleton, Terrain::Grasslands) => 60,
            (Race::Skeleton, Terrain::HauntedWoods) => 54,
            (Race::Skeleton, Terrain::Hills) => 59,
            (Race::Skeleton, Terrain::Mountain) => 60,
            (Race::Skeleton, Terrain::Swamp) => 56,

            // Zombie - slow, shambling
            (Race::Zombie, Terrain::Forest0) => 60,
            (Race::Zombie, Terrain::Forest1) => 60,
            (Race::Zombie, Terrain::Grasslands) => 61,
            (Race::Zombie, Terrain::HauntedWoods) => 56,
            (Race::Zombie, Terrain::Hills) => 62,
            (Race::Zombie, Terrain::Mountain) => 64,
            (Race::Zombie, Terrain::Swamp) => 58,

            // Undead - generic undead
            (Race::Undead, Terrain::Forest0) => 61,
            (Race::Undead, Terrain::Forest1) => 61,
            (Race::Undead, Terrain::Grasslands) => 62,
            (Race::Undead, Terrain::HauntedWoods) => 55,
            (Race::Undead, Terrain::Hills) => 63,
            (Race::Undead, Terrain::Mountain) => 65,
            (Race::Undead, Terrain::Swamp) => 59,
        }
    }

    /// Get racial attack bonus modifier
    pub fn get_attack_bonus(self) -> i32 {
        match self {
            Race::Human => 0,      // Balanced
            Race::Elf => 1,        // Precise and skilled
            Race::Dwarf => 0,      // Steady fighters
            Race::Halfling => 0,   // Surprising fighters
            Race::Gnome => -1,     // Weak but clever
            Race::HalfElf => 1,    // Versatile
            Race::HalfOrc => 2,    // Savage attacks
            Race::Tiefling => 1,   // Infernal cunning
            Race::Dragonborn => 1, // Draconic strength
            Race::Orc => 2,        // Brutal attackers
            Race::Goblin => -1,    // Weak but numerous
            Race::Hobgoblin => 1,  // Disciplined warriors
            Race::Kobold => -2,    // Very weak
            Race::Lizardfolk => 1, // Natural weapons
            Race::Triton => 0,     // Standard combat
            Race::Changeling => 0, // Deceptive
            Race::Skeleton => 0,   // Relentless but not skilled
            Race::Zombie => 0,     // Mindless but relentless
            Race::Undead => 0,     // Slow and shambling
        }
    }

    /// Get racial movement speed modifier
    pub fn get_movement_bonus(self) -> i32 {
        match self {
            Race::Human => 0,      // Standard speed
            Race::Elf => 1,        // Swift and graceful
            Race::Dwarf => -1,     // Short legs, slower
            Race::Halfling => 0,   // Quick despite size
            Race::Gnome => -1,     // Small stride
            Race::HalfElf => 1,    // Elven grace
            Race::HalfOrc => 0,    // Standard speed
            Race::Tiefling => 0,   // Standard speed
            Race::Dragonborn => 0, // Standard speed
            Race::Orc => 0,        // Standard speed
            Race::Goblin => 1,     // Small and quick
            Race::Hobgoblin => 0,  // Disciplined march
            Race::Kobold => 1,     // Fast and skittish
            Race::Lizardfolk => 0, // Standard speed
            Race::Triton => -1,    // Better in water
            Race::Changeling => 0, // Human-like
            Race::Skeleton => -1,  // Shambling gait
            Race::Zombie => 0,     // Mindless but relentless
            Race::Undead => -1,    // Slow and shambling
        }
    }

    /// Get the display name of the race
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

    /// Get the display color for rendering units of this race
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

    /// Get all available races
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

/// Represents different terrain types in the game
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
