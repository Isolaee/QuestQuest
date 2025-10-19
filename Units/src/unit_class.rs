use combat::{DamageType, RangeCategory, Resistances};
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

    /// Get default range category for this class
    pub fn get_default_range(self) -> RangeCategory {
        match self {
            UnitClass::Warrior => RangeCategory::Melee,
            UnitClass::Archer => RangeCategory::Range,
            UnitClass::Mage => RangeCategory::Range,
            UnitClass::Rogue => RangeCategory::Melee,
            UnitClass::Cleric => RangeCategory::Melee,
            UnitClass::Paladin => RangeCategory::Melee,
            UnitClass::Ranger => RangeCategory::Range,
        }
    }

    /// Get default damage type for this class
    pub fn get_default_damage_type(self) -> DamageType {
        match self {
            UnitClass::Warrior => DamageType::Slash, // Sword
            UnitClass::Archer => DamageType::Pierce, // Arrows
            UnitClass::Mage => DamageType::Fire,     // Magic
            UnitClass::Rogue => DamageType::Pierce,  // Daggers
            UnitClass::Cleric => DamageType::Blunt,  // Mace
            UnitClass::Paladin => DamageType::Slash, // Holy sword
            UnitClass::Ranger => DamageType::Pierce, // Bow
        }
    }

    /// Get class resistances (% reduction for each damage type)
    pub fn get_resistances(self) -> Resistances {
        match self {
            UnitClass::Warrior => Resistances::new(
                30, // blunt - heavy armor
                20, // pierce - armor can be pierced
                10, // fire - armor conducts heat
                10, // dark - no special resistance
                35, // slash - armor protects well
                25, // crush - armor absorbs impact
            ),
            UnitClass::Archer => Resistances::new(
                10, // blunt - light armor
                25, // pierce - used to arrows
                5,  // fire - leather burns
                5,  // dark - no special resistance
                15, // slash - some protection
                10, // crush - minimal protection
            ),
            UnitClass::Mage => Resistances::new(
                5,  // blunt - robes offer no protection
                5,  // pierce - robes offer no protection
                40, // fire - fire resistance magic
                45, // dark - dark magic knowledge
                5,  // slash - robes offer no protection
                5,  // crush - robes offer no protection
            ),
            UnitClass::Rogue => Resistances::new(
                15, // blunt - agile dodging
                20, // pierce - light armor + agility
                10, // fire - leather armor
                15, // dark - shadow affinity
                20, // slash - agile dodging
                10, // crush - can dodge some impacts
            ),
            UnitClass::Cleric => Resistances::new(
                20, // blunt - medium armor
                15, // pierce - medium armor
                25, // fire - divine protection
                35, // dark - holy resistance
                20, // slash - medium armor
                20, // crush - divine protection
            ),
            UnitClass::Paladin => Resistances::new(
                35, // blunt - heavy armor + divine
                25, // pierce - heavy armor + divine
                30, // fire - divine protection
                40, // dark - holy aura
                40, // slash - heavy armor + divine
                30, // crush - heavy armor + divine
            ),
            UnitClass::Ranger => Resistances::new(
                15, // blunt - medium armor
                20, // pierce - nature's blessing
                15, // fire - nature affinity
                10, // dark - no special resistance
                20, // slash - medium armor
                15, // crush - nature's toughness
            ),
        }
    }

    /// Get base attack damage for this class
    pub fn get_base_attack(self) -> u32 {
        match self {
            UnitClass::Warrior => 15, // Strong melee
            UnitClass::Archer => 12,  // Ranged precision
            UnitClass::Mage => 10,    // Magic damage
            UnitClass::Rogue => 14,   // Sneak attack
            UnitClass::Cleric => 8,   // Support role
            UnitClass::Paladin => 13, // Divine might
            UnitClass::Ranger => 11,  // Nature bond
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
