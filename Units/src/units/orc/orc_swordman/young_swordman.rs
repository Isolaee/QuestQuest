use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 1 Orc Swordsman - Entry level warrior unit.
///
/// Young Swordsmen are inexperienced fighters who are just beginning their martial training.
/// They have basic combat abilities and can evolve into regular Swordsmen with experience.
///
/// # Evolution Chain
/// - **Previous**: None (first in chain)
/// - **Current**: Orc Young Swordsman (Level 1)
/// - **Next**: Orc Swordsman (Level 2)
///
/// # Stats
/// - **HP**: 35
/// - **Attack**: 4
/// - **Movement**: 4 + race bonus
/// - **Range**: Melee
/// - **XP to Next Level**: 200 (level² × 50)
pub struct OrcYoungSwordsman {
    base: BaseUnit,
}

impl OrcYoungSwordsman {
    // ===== UNIT PROPERTIES =====

    /// Level 1 - Young Swordsman (Entry level)
    const LEVEL: i32 = 1;

    // // Evolution chain
    // const PREVIOUS_UNIT_TYPE: Option<&'static str> = None; // First in chain
    // const NEXT_UNIT_TYPE: &'static str = "Orc Swordsman"; // Evolves to Swordsman

    // Base Stats
    const BASE_HEALTH: i32 = 35;
    const BASE_ATTACK: u32 = 4;
    const BASE_MOVEMENT: i32 = 4;
    const ATTACK_STRENGTH: u32 = 12;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances (light armor - young and inexperienced)
    const RESISTANCE_BLUNT: u8 = 0;
    const RESISTANCE_PIERCE: u8 = 15;
    const RESISTANCE_FIRE: u8 = 10;
    const RESISTANCE_DARK: u8 = 20;
    const RESISTANCE_SLASH: u8 = 10;
    const RESISTANCE_CRUSH: u8 = 10;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Orc;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Orc Young Swordsman";

    // Experience
    const STARTING_EXPERIENCE: i32 = 0;

    // ===== XP PROGRESSION =====

    /// Custom XP formula for Human Squires - faster leveling for starting units
    /// Formula: level² × 25 (half of default)
    /// - Level 1→2: 100 XP
    /// - Level 2→3: 225 XP
    /// - Level 3→4: 400 XP
    pub fn xp_formula(level: i32) -> i32 {
        if level <= 1 {
            return 0;
        }
        level * level * 25
    }

    // ===== ATTACK DEFINITIONS =====

    /// Basic sword slash - weak but reliable
    fn sword_slash() -> Attack {
        Attack::melee(
            "Sword Slash",
            5, // damage
            1, // range (melee)
            DamageType::Slash,
        )
    }

    /// Clumsy thrust attack
    fn awkward_thrust() -> Attack {
        Attack::melee(
            "Awkward Thrust",
            3, // damage
            2, // range (melee)
            DamageType::Pierce,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Orc Young Swordsman.
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

        // Create base unit with level 1
        let mut base = BaseUnit::new(
            name,
            position,
            Self::RACE,
            Self::UNIT_TYPE.to_string(),
            "An inexperienced orc fighter beginning their warrior training. Young Swordsmen are aggressive but lack refinement in combat. They favor direct assault and evolve into proper Orc Swordsmen with battle experience.".to_string(),
            None,
            vec![crate::unit_type::UnitType::OrcSwordsman],
            combat_stats,
        );

        // Set level explicitly (BaseUnit defaults to 1, but being explicit)
        base.level = Self::LEVEL;
        base.experience = Self::STARTING_EXPERIENCE;

        // Define available attacks for level 1
        base.attacks = vec![Self::sword_slash(), Self::awkward_thrust()];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for OrcYoungSwordsman {
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
    OrcYoungSwordsman,
    "Orc Young Swordsman",
    "A young orc swordsman, level 1",
    Terrain::Grasslands,
    "Orc",
    "Swordsman"
);
