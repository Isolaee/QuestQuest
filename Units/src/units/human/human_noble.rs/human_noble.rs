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
/// - **HP**: 100
/// - **Attack**: 12
/// - **Movement**: 4
/// - **Range**: Melee
/// - **XP to Next Level**: 50 (level² × 50)

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

    // ===== ATTACK DEFINITIONS =====

    /// Basic sword strike
    fn sword_strike() -> Attack {
        Attack::melee(
            "Sword Strike",
            5, // damage
            1,  // range (melee)
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Human Noble unit
    /// 
    /// # Arguments
    /// * `name` - The unit's name
    /// * `position` - Starting position on the hex grid
    /// * `terrain` - The terrain type at the starting position
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
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
            terrain,
            graphics::SpriteType::HumanNoble,
            None,
            Some("Human Noble".to_string()),
            combat_stats,
        );

        // Set level explicitly
        base.level = Self::LEVEL;
        base.experience = Self::STARTING_EXPERIENCE;

        // Define available attacks for level 1
        base.attacks = vec![Self::sword_strike()];

        Self { base }
    }

    // ===== LEVEL PROGRESSION DATA =====

    /// Returns the previous unit type in evolution chain.
    ///
    /// Returns `None` for Young Warrior as it's the first in the chain.
    pub fn get_previous_unit_type() -> Option<String> {
        Self::PREVIOUS_UNIT_TYPE.map(|s| s.to_string())
    }

    /// Returns the next unit type in evolution chain.
    ///
    /// Returns `Some("Human Prince")` as the next evolution stage.
    pub fn get_next_unit_type() -> Option<String> {
        Some(Self::NEXT_UNIT_TYPE.to_string())
    }

    /// Returns the previous unit type in evolution chain (trait method).
    pub fn evolution_previous(&self) -> Option<String> {
        Self::get_previous_unit_type()
    }

    /// Returns the next unit type in evolution chain (trait method).
    pub fn evolution_next(&self) -> Option<String> {
        Self::get_next_unit_type()
    }

    /// Check if this unit has a next evolution.
    ///
    /// Returns `true` since Young Warrior can evolve to Warrior.
    pub fn has_next_evolution() -> bool {
        true
    }

    /// Returns the combat stats for the next evolution level (Human Prince - Level 2).
    ///
    /// # Stats Progression
    /// - HP: 40 → 46 (+6)
    /// - Attack: 5 → 8 (+3)
    /// - Resistances increased in relevant areas
    pub fn get_next_level_stats() -> CombatStats {
        CombatStats::new_with_attacks(
            46,                                  // health (+6)
            8,                                   // base attack (+3)
            3 + Race::Human.get_movement_bonus(), // movement (same)
            RangeCategory::Melee,
            Resistances::new(
                20, // blunt (+0)
                25, // pierce (+5)
                0, // fire (+0)
                20, // dark (+5)
                20, // slash (+10)
                10, // crush (+10)
            ),
            0, // attack_strength (+0)
            1,  // attacks_per_round (same)
        )
    }

    /// Returns the attacks available at the next level
    pub fn get_next_level_attacks() -> Vec<Attack> {
        vec![
            Attack::melee("Proficient Slash", 6, 2, DamageType::Slash),
            Attack::range("Small Crossbow", 10, 1, DamageType::Pierce),
        ]
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
}

crate::submit_unit!(
    HumanNoble,
    "Human Noble",
    "An inexperienced but sturdy human noble beginning their journey. Human Nobles excel in leadership and versatility on the battlefield. With experience, they will become threth on the battlefield.",
    Terrain::Plains,
    "Human",
    "Noble"
);