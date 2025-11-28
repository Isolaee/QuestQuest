//! Unit type enumeration for type-safe unit identification.
//!
//! This module defines the `UnitType` enum which provides compile-time type safety
//! for unit evolution chains and factory creation, replacing string-based identifiers.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Enumeration of all unit types in the game.
///
/// This enum provides type-safe unit identification and is used for:
/// - Evolution chains (previous/next unit types)
/// - Unit factory creation
/// - Type checking at compile time
///
/// # Examples
///
/// ```rust,no_run
/// use units::UnitType;
///
/// let young = UnitType::OrcYoungSwordsman;
/// let next = UnitType::OrcSwordsman;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitType {
    // === Dwarf Units ===
    /// Level 1 Dwarf warrior
    DwarfYoungWarrior,
    /// Level 2 Dwarf warrior
    DwarfWarrior,
    /// Level 3 Dwarf warrior (max level)
    DwarfVeteranWarrior,

    // === Elf Units ===
    /// Melee elf warrior
    ElfWarrior,
    /// Ranged elf archer
    ElfArcher,
    /// Magic-wielding elf mage
    ElfMage,

    // === Goblin Units ===
    /// Basic goblin unit
    GoblinGrunt,
    /// Leader goblin unit
    GoblinChief,

    // === Human Noble Line ===
    /// Level 1 Human noble
    HumanNoble,
    /// Level 2 Human noble
    HumanPrince,
    /// Level 3 Human noble (max level)
    HumanKing,

    // === Human Knight Line ===
    /// Level 1 Human knight
    HumanSquire,
    /// Level 2 Human knight
    HumanKnight,
    /// Level 3 Human knight
    HumanGrandKnight,
    /// Level 4 Human knight (max level)
    HumanKnightCommander,

    // === Orc Units ===
    /// Level 1 Orc swordsman
    OrcYoungSwordsman,
    /// Level 2 Orc swordsman
    OrcSwordsman,
    /// Level 3 Orc swordsman (max level)
    OrcEliteSwordsman,
}

impl UnitType {
    /// Returns the string identifier used in the unit registry.
    ///
    /// This maps the enum variant to the string used by the factory system
    /// for dynamic unit creation.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use units::UnitType;
    ///
    /// assert_eq!(UnitType::OrcSwordsman.as_str(), "Orc Swordsman");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            // Dwarf
            UnitType::DwarfYoungWarrior => "Dwarf Young Warrior",
            UnitType::DwarfWarrior => "Dwarf Warrior",
            UnitType::DwarfVeteranWarrior => "Dwarf Veteran Warrior",

            // Elf
            UnitType::ElfWarrior => "Elf Warrior",
            UnitType::ElfArcher => "Elf Archer",
            UnitType::ElfMage => "Elf Mage",

            // Goblin
            UnitType::GoblinGrunt => "Goblin Grunt",
            UnitType::GoblinChief => "Goblin Chief",

            // Human Noble
            UnitType::HumanNoble => "Human Noble",
            UnitType::HumanPrince => "Human Prince",
            UnitType::HumanKing => "Human King",

            // Human Knight
            UnitType::HumanSquire => "Human Squire",
            UnitType::HumanKnight => "Human Knight",
            UnitType::HumanGrandKnight => "Human Grand Knight",
            UnitType::HumanKnightCommander => "Human Knight Commander",

            // Orc
            UnitType::OrcYoungSwordsman => "Orc Young Swordsman",
            UnitType::OrcSwordsman => "Orc Swordsman",
            UnitType::OrcEliteSwordsman => "Orc Elite Swordsman",
        }
    }

    /// Returns all available unit types as a slice.
    ///
    /// Useful for iteration or displaying all available units.
    pub fn all() -> &'static [UnitType] {
        &[
            // Dwarf
            UnitType::DwarfYoungWarrior,
            UnitType::DwarfWarrior,
            UnitType::DwarfVeteranWarrior,
            // Elf
            UnitType::ElfWarrior,
            UnitType::ElfArcher,
            UnitType::ElfMage,
            // Goblin
            UnitType::GoblinGrunt,
            UnitType::GoblinChief,
            // Human Noble
            UnitType::HumanNoble,
            UnitType::HumanPrince,
            UnitType::HumanKing,
            // Human Knight
            UnitType::HumanSquire,
            UnitType::HumanKnight,
            UnitType::HumanGrandKnight,
            UnitType::HumanKnightCommander,
            // Orc
            UnitType::OrcYoungSwordsman,
            UnitType::OrcSwordsman,
            UnitType::OrcEliteSwordsman,
        ]
    }
}

impl std::fmt::Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for UnitType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Dwarf
            "Dwarf Young Warrior" => Ok(UnitType::DwarfYoungWarrior),
            "Dwarf Warrior" => Ok(UnitType::DwarfWarrior),
            "Dwarf Veteran Warrior" => Ok(UnitType::DwarfVeteranWarrior),

            // Elf
            "Elf Warrior" => Ok(UnitType::ElfWarrior),
            "Elf Archer" => Ok(UnitType::ElfArcher),
            "Elf Mage" => Ok(UnitType::ElfMage),

            // Goblin
            "Goblin Grunt" => Ok(UnitType::GoblinGrunt),
            "Goblin Chief" => Ok(UnitType::GoblinChief),

            // Human Noble
            "Human Noble" => Ok(UnitType::HumanNoble),
            "Human Prince" => Ok(UnitType::HumanPrince),
            "Human King" => Ok(UnitType::HumanKing),

            // Human Knight
            "Human Squire" => Ok(UnitType::HumanSquire),
            "Human Knight" => Ok(UnitType::HumanKnight),
            "Human Grand Knight" => Ok(UnitType::HumanGrandKnight),
            "Human Knight Commander" => Ok(UnitType::HumanKnightCommander),

            // Orc
            "Orc Young Swordsman" => Ok(UnitType::OrcYoungSwordsman),
            "Orc Swordsman" => Ok(UnitType::OrcSwordsman),
            "Orc Elite Swordsman" => Ok(UnitType::OrcEliteSwordsman),

            _ => Err(format!("Unknown unit type: {}", s)),
        }
    }
}
