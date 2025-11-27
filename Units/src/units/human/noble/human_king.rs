use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 3 Human King
///
/// Human Kings are the pinnacle of human leadership and martial prowess.
/// They are battle-hardened veterans who inspire their allies and strike
/// fear into their enemies. As the final evolution of the human noble line,
/// they excel in all aspects of combat and command.
///
/// # Evolution Chain
/// **Previous**: Human Prince (Level 2)
/// **Current**: Human King (Level 3)
/// **Next**: None (final evolution)
///
/// # Stats
/// - **HP**: 54
/// - **Attack**: 12
/// - **Movement**: 5
/// - **Range**: Melee
/// - **XP to Next Level**: N/A (max level)
pub struct HumanKing {
    base: BaseUnit,
}

impl HumanKing {
    // ===== UNIT PROPERTIES =====

    /// Level 3 - King (final evolution)
    const LEVEL: i32 = 3;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: Option<&'static str> = Some("Human Prince");
    const NEXT_UNIT_TYPE: Option<&'static str> = None; // Final evolution

    // Base Stats
    const BASE_HEALTH: i32 = 54;
    const BASE_ATTACK: u32 = 12;
    const BASE_MOVEMENT: i32 = 5;
    const ATTACK_STRENGTH: u32 = 0;
    const ATTACKS_PER_ROUND: u32 = 2;

    // Resistances
    const RESISTANCE_BLUNT: u8 = 30;
    const RESISTANCE_PIERCE: u8 = 30;
    const RESISTANCE_FIRE: u8 = 10;
    const RESISTANCE_DARK: u8 = 30;
    const RESISTANCE_SLASH: u8 = 30;
    const RESISTANCE_CRUSH: u8 = 20;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Human;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Human King";

    // Experience
    const STARTING_EXPERIENCE: i32 = 0;

    // ===== XP PROGRESSION =====

    /// Custom XP formula for Human Kings - N/A (final evolution)
    /// This is the maximum level for this evolution line
    pub fn xp_formula(level: i32) -> i32 {
        if level <= 1 {
            return 0;
        }
        // Kings don't evolve further, but maintain formula for consistency
        level * level * 50
    }

    // ===== ATTACK DEFINITIONS ======

    /// Masterful sword strike with royal authority
    fn royal_strike() -> Attack {
        Attack::melee(
            "Royal Strike",
            8, // damage
            1, // attack_times
            DamageType::Slash,
        )
    }

    /// Secondary devastating blow
    fn commanding_blow() -> Attack {
        Attack::melee(
            "Commanding Blow",
            6, // damage
            1, // attack_times
            DamageType::Crush,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Human King unit
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
            "A legendary human king who has mastered the art of war. Human Kings are supreme commanders who inspire their troops and dominate the battlefield with unmatched skill and authority.".to_string(),
            terrain,
            graphics::SpriteType::Unit,
            Some("Human Prince".to_string()),
            None, // Final evolution - no next unit
            combat_stats,
        );

        // Set level explicitly
        base.level = Self::LEVEL;
        base.experience = Self::STARTING_EXPERIENCE;

        // Define available attacks - kings have multiple powerful attacks
        base.attacks = vec![Self::royal_strike(), Self::commanding_blow()];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for HumanKing {
    fn base(&self) -> &BaseUnit {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseUnit {
        &mut self.base
    }

    fn attacks(&self) -> &[Attack] {
        &self.base.attacks
    }

    // Custom XP progression - N/A (final evolution)
    fn xp_required_for_level(&self, level: i32) -> i32 {
        Self::xp_formula(level)
    }
}

crate::submit_unit!(
    HumanKing,
    "Human King",
    "A legendary human king who has mastered the art of war. Human Kings are supreme commanders who inspire their troops and dominate the battlefield with unmatched skill and authority.",
    Terrain::Grasslands,
    "Human",
    "King"
);
