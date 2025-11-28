use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 1 Human Noble
///
/// Human Nobles are the starting units for human players, embodying
/// the ideals of chivalry and leadership. They are versatile units
/// capable of both offense and defense, making them suitable for
/// various battlefield roles.
///
/// # Evolution Chain
/// **Previous**: None (first in chain)
/// **Current**: Human Noble (Level 1)
/// **Next**: Human Prince (Level 2)
///
/// # Stats
/// - **HP**: 40
/// - **Attack**: 5
/// - **Movement**: 4
/// - **Range**: Melee
/// - **XP to Next Level**: 100 (level² × 25, faster leveling for starter units)
pub struct HumanNoble {
    base: BaseUnit,
}

impl HumanNoble {
    // ===== UNIT PROPERTIES =====

    /// Level 1 - Noble (Entry level)
    const LEVEL: i32 = 1;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: Option<&'static str> = None; // First in chain
    const NEXT_UNIT_TYPE: &'static str = "Human Prince"; // Evolves to Prince

    // Base Stats
    const BASE_HEALTH: i32 = 40;
    const BASE_ATTACK: u32 = 5;
    const BASE_MOVEMENT: i32 = 4;
    const ATTACK_STRENGTH: u32 = 0;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances
    const RESISTANCE_BLUNT: u8 = 20;
    const RESISTANCE_PIERCE: u8 = 10;
    const RESISTANCE_FIRE: u8 = 0;
    const RESISTANCE_DARK: u8 = 10;
    const RESISTANCE_SLASH: u8 = 10;
    const RESISTANCE_CRUSH: u8 = 0;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Human;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Human Noble";

    // Experience
    const STARTING_EXPERIENCE: i32 = 0;

    // ===== XP PROGRESSION =====

    /// Custom XP formula for Human Nobles - faster leveling for starting units
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

    /// Basic sword strike
    fn sword_strike() -> Attack {
        Attack::melee(
            "Sword Strike",
            5, // damage
            1, // attack_times
            DamageType::Slash,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Human Noble unit
    ///
    /// # Arguments
    /// * `name` - The unit's name
    /// * `position` - Starting position on the hex grid
    pub fn new(name: String, position: HexCoord) -> Self {
        // Build combat stats from constants
        let combat_stats = CombatStats::new_with_attacks(
            Self::BASE_HEALTH,
            Self::BASE_ATTACK,
            Self::BASE_MOVEMENT + Self::RACE.get_movement_bonus(),
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
            "An inexperienced but sturdy human noble beginning their journey. Human Nobles excel in leadership and versatility on the battlefield. With experience, they will become threth on the battlefield.".to_string(),
            None,
            vec![crate::unit_type::UnitType::HumanPrince],
            combat_stats,
        );

        // Set level explicitly
        base.level = Self::LEVEL;
        base.experience = Self::STARTING_EXPERIENCE;

        // Define available attacks for level 1
        base.attacks = vec![Self::sword_strike()];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for HumanNoble {
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
    HumanNoble,
    "Human Noble",
    "An inexperienced but sturdy human noble beginning their journey. Human Nobles excel in leadership and versatility on the battlefield. With experience, they will become threth on the battlefield.",
    Terrain::Grasslands,
    "Human",
    "Noble"
);
