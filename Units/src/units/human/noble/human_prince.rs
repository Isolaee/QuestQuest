use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::combat::{CombatStats, DamageType, RangeCategory, Resistances};
use crate::unit_race::{Race, Terrain};
use graphics::HexCoord;

/// Level 2 Human Prince
///
/// Human Princes are the starting units for human players, embodying
/// the ideals of chivalry and leadership. They are versatile units
/// capable of both offense and defense, making them suitable for
/// various battlefield roles.
///
/// # Evolution Chain
/// **Previous**: Human Noble (Level 1)
/// **Current**: Human Prince (Level 2)
/// **Next**: Human King (Level 3)
///
/// # Stats
/// - **HP**: 46
/// - **Attack**: 8
/// - **Movement**: 4
/// - **Range**: Melee
/// - **XP to Next Level**: 360 (level² × 40, moderate leveling)
pub struct HumanPrince {
    base: BaseUnit,
}

impl HumanPrince {
    // ===== UNIT PROPERTIES =====

    /// Level 2 - Prince (mid level)
    const LEVEL: i32 = 2;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: Option<&'static str> = Some("Human Noble"); // First in chain
    const NEXT_UNIT_TYPE: &'static str = "Human King"; // Evolves to King

    // Base Stats
    const BASE_HEALTH: i32 = 46;
    const BASE_ATTACK: u32 = 8;
    const BASE_MOVEMENT: i32 = 4;
    const ATTACK_STRENGTH: u32 = 0;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances
    const RESISTANCE_BLUNT: u8 = 20;
    const RESISTANCE_PIERCE: u8 = 20;
    const RESISTANCE_FIRE: u8 = 0;
    const RESISTANCE_DARK: u8 = 20;
    const RESISTANCE_SLASH: u8 = 20;
    const RESISTANCE_CRUSH: u8 = 10;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Human;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Human Prince";

    // Experience
    const STARTING_EXPERIENCE: i32 = 0;

    // ===== XP PROGRESSION =====

    /// Custom XP formula for Human Princes - moderate leveling
    /// Formula: level² × 40
    /// - Level 2→3: 360 XP
    /// - Level 3→4: 640 XP
    /// - Level 4→5: 1000 XP
    pub fn xp_formula(level: i32) -> i32 {
        if level <= 1 {
            return 0;
        }
        level * level * 40
    }

    // ===== ATTACK DEFINITIONS ======

    /// Proficient slash attack
    fn proficient_slash() -> Attack {
        Attack::melee(
            "Proficient Slash",
            5, // damage
            1, // attack_times
            DamageType::Slash,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Human Prince unit
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
            "A skilled human prince with growing leadership abilities. Human Princes are experienced combatants who excel in both offense and defense. With continued training, they will become formidable kings.".to_string(),
            Some(crate::unit_type::UnitType::HumanNoble),
            vec![crate::unit_type::UnitType::HumanKing],
            combat_stats,
        );

        // Set level explicitly
        base.level = Self::LEVEL;
        base.experience = Self::STARTING_EXPERIENCE;

        // Define available attacks
        base.attacks = vec![Self::proficient_slash()];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for HumanPrince {
    fn base(&self) -> &BaseUnit {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseUnit {
        &mut self.base
    }

    fn attacks(&self) -> &[Attack] {
        &self.base.attacks
    }

    // Custom XP progression - moderate leveling
    fn xp_required_for_level(&self, level: i32) -> i32 {
        Self::xp_formula(level)
    }
}

crate::submit_unit!(
    HumanPrince,
    "Human Prince",
    "A skilled human prince with growing leadership abilities. Human Princes are experienced combatants who excel in both offense and defense. With continued training, they will become formidable kings.",
    Terrain::Grasslands,
    "Human",
    "Prince"
);
