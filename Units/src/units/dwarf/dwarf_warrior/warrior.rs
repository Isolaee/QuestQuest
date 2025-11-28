use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 2 Dwarf Warrior - Intermediate defender unit.
///
/// Warriors are experienced dwarven fighters with improved combat abilities and formidable defenses.
/// They evolve from Young Warriors and can further evolve into Veteran Warriors.
///
/// # Evolution Chain
/// - **Previous**: Dwarf Young Warrior (Level 1)
/// - **Current**: Dwarf Warrior (Level 2)
/// - **Next**: Dwarf Veteran Warrior (Level 3)
///
/// # Stats
/// - **HP**: 140
/// - **Attack**: 14
/// - **Movement**: 3 + race bonus
/// - **Range**: Melee
/// - **XP to Next Level**: 200 (level² × 50)
pub struct DwarfWarrior {
    base: BaseUnit,
}

impl DwarfWarrior {
    // ===== UNIT PROPERTIES =====

    /// Level 2 - Warrior
    const LEVEL: i32 = 2;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: &'static str = "Dwarf Young Warrior"; // Evolved from Young Warrior
    const NEXT_UNIT_TYPE: &'static str = "Dwarf Veteran Warrior"; // Evolves to Veteran Warrior

    // Base Stats
    const BASE_HEALTH: i32 = 140;
    const BASE_ATTACK: u32 = 14;
    const BASE_MOVEMENT: i32 = 3;
    const ATTACK_STRENGTH: u32 = 14;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances (dwarven plate armor - trained defender)
    const RESISTANCE_BLUNT: u8 = 32;
    const RESISTANCE_PIERCE: u8 = 25;
    const RESISTANCE_FIRE: u8 = 22;
    const RESISTANCE_DARK: u8 = 15;
    const RESISTANCE_SLASH: u8 = 35;
    const RESISTANCE_CRUSH: u8 = 38;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Dwarf;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Dwarf Warrior";

    /// Returns the sprite for this unit type
    pub fn sprite(&self) -> graphics::SpriteType {
        graphics::SpriteType::DwarfWarrior
    }

    // ===== ATTACK DEFINITIONS =====

    /// Powerful axe swing
    fn heavy_axe() -> Attack {
        Attack::melee(
            "Heavy Axe",
            14, // damage
            1,  // range (melee)
            DamageType::Slash,
        )
    }

    /// Improved shield bash
    fn shield_bash() -> Attack {
        Attack::melee(
            "Shield Bash",
            10, // damage
            1,  // range (melee)
            DamageType::Crush,
        )
    }

    /// New ability - crushing hammer strike
    fn hammer_strike() -> Attack {
        Attack::melee(
            "Hammer Strike",
            16, // damage
            1,  // range (melee)
            DamageType::Crush,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Dwarf Warrior.
    ///
    /// # Arguments
    /// * `name` - The unit's name
    /// * `position` - Starting position on the hex grid
    pub fn new(name: String, position: HexCoord) -> Self {
        // Build combat stats from constants
        let combat_stats = CombatStats::new_with_attacks(
            Self::BASE_HEALTH,
            Self::BASE_ATTACK,
            Self::BASE_MOVEMENT,
            Self::RANGE_CATEGORY,
            Resistances::new(
                Self::RESISTANCE_BLUNT,
                Self::RESISTANCE_PIERCE,
                Self::RESISTANCE_FIRE,
                Self::RESISTANCE_DARK,
                Self::RESISTANCE_SLASH,
                Self::RESISTANCE_CRUSH,
            ),
            Self::ATTACK_STRENGTH,
            Self::ATTACKS_PER_ROUND,
        );

        // Create base unit
        let mut base = BaseUnit::new_with_sprite(
            name,
            position,
            Self::RACE,
            Self::UNIT_TYPE.to_string(),
            "A battle-tested dwarven warrior with formidable defensive capabilities. Masters of mountain warfare, they wield axe and shield with deadly efficiency. Can evolve into elite Veteran Warriors with further experience.".to_string(),
            graphics::SpriteType::DwarfWarrior,
            Some(crate::unit_type::UnitType::DwarfYoungWarrior),
            vec![crate::unit_type::UnitType::DwarfVeteranWarrior],
            combat_stats,
        );

        // Set to level 2
        base.level = Self::LEVEL;
        base.experience = 50; // Carried over from level 1

        // Define available attacks for level 2
        base.attacks = vec![
            Self::heavy_axe(),
            Self::shield_bash(),
            Self::hammer_strike(),
        ];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for DwarfWarrior {
    fn base(&self) -> &BaseUnit {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseUnit {
        &mut self.base
    }

    fn attacks(&self) -> &[Attack] {
        &self.base.attacks
    }
}

crate::submit_unit!(
    DwarfWarrior,
    "Dwarf Warrior",
    "An experienced dwarf warrior, level 2",
    Terrain::Mountain,
    "Dwarf",
    "Warrior"
);
