use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::combat::{CombatStats, DamageType, RangeCategory, Resistances};
use crate::unit_race::{Race, Terrain};
use graphics::HexCoord;

/// Level 1 Human Knight
///
/// Human Knights are heavily armored warriors who serve as the frontline
/// defenders of humanity. They are trained in the art of mounted combat
/// and shield techniques, making them formidable defensive units with
/// solid offensive capabilities.
///
/// # Evolution Chain
/// **Previous**: None (first in chain)
/// **Current**: Human Knight (Level 1)
/// **Next**: Human Knight Commander (Level 2)
///
/// # Stats
/// - **HP**: 45
/// - **Attack**: 6
/// - **Movement**: 3
/// - **Range**: Melee
/// - **XP to Next Level**: 100 (level² × 25, faster leveling for starter units)
pub struct HumanKnight {
    base: BaseUnit,
}

impl HumanKnight {
    // ===== UNIT PROPERTIES =====

    /// Level 1 - Knight (Entry level)
    const LEVEL: i32 = 1;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: Option<&'static str> = None; // First in chain
    const NEXT_UNIT_TYPE: &'static str = "Human Knight Commander"; // Evolves to Knight Commander

    // Base Stats
    const BASE_HEALTH: i32 = 45;
    const BASE_ATTACK: u32 = 6;
    const BASE_MOVEMENT: i32 = 3;
    const ATTACK_STRENGTH: u32 = 0;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances - Higher physical resistances due to heavy armor
    const RESISTANCE_BLUNT: u8 = 25;
    const RESISTANCE_PIERCE: u8 = 20;
    const RESISTANCE_FIRE: u8 = 5;
    const RESISTANCE_DARK: u8 = 15;
    const RESISTANCE_SLASH: u8 = 25;
    const RESISTANCE_CRUSH: u8 = 10;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Human;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Human Knight";

    // Experience
    const STARTING_EXPERIENCE: i32 = 0;

    // ===== XP PROGRESSION =====

    /// Custom XP formula for Human Knights - faster leveling for starting units
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

    // ===== ATTACK DEFINITIONS ======

    /// Heavy sword swing with full armor weight
    fn armored_strike() -> Attack {
        Attack::melee(
            "Armored Strike",
            6, // damage
            1, // attack_times
            DamageType::Slash,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Human Knight unit
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
        let mut base = BaseUnit::new(
            name,
            position,
            Self::RACE,
            Self::UNIT_TYPE.to_string(),
            "A heavily armored human knight sworn to protect the innocent. Human Knights are defensive specialists who excel at holding the line and protecting allies. With training, they will become knight commanders.".to_string(),
            Some(crate::unit_type::UnitType::HumanSquire),
            vec![crate::unit_type::UnitType::HumanKnightCommander, crate::unit_type::UnitType::HumanGrandKnight],
            combat_stats,
        );

        // Set level explicitly
        base.level = Self::LEVEL;
        base.experience = Self::STARTING_EXPERIENCE;

        // Define available attacks for level 1
        base.attacks = vec![Self::armored_strike()];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for HumanKnight {
    fn base(&self) -> &BaseUnit {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseUnit {
        &mut self.base
    }

    fn attacks(&self) -> &[Attack] {
        &self.base.attacks
    }

    // Custom XP progression - faster leveling for starting units
    fn xp_required_for_level(&self, level: i32) -> i32 {
        Self::xp_formula(level)
    }
}

crate::submit_unit!(
    HumanKnight,
    "Human Knight",
    "A heavily armored human knight sworn to protect the innocent. Human Knights are defensive specialists who excel at holding the line and protecting allies. With training, they will become knight commanders.",
    Terrain::Grasslands,
    "Human",
    "Knight"
);
