use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 3 Human Grand Knight
///
/// Human Grand Knights are the ultimate embodiment of knightly virtue and
/// martial excellence. They are legendary warriors clad in masterwork armor,
/// having perfected the art of combat through decades of battle. Grand Knights
/// are inspiring leaders who dominate the battlefield with multiple devastating
/// attacks and unmatched defensive prowess.
///
/// # Evolution Chain
/// **Previous**: Human Knight Commander (Level 2)
/// **Current**: Human Grand Knight (Level 3)
/// **Next**: None (final evolution)
///
/// # Stats
/// - **HP**: 60
/// - **Attack**: 13
/// - **Movement**: 5
/// - **Range**: Melee
/// - **XP to Next Level**: N/A (max level)
pub struct HumanGrandKnight {
    base: BaseUnit,
}

impl HumanGrandKnight {
    // ===== UNIT PROPERTIES =====

    /// Level 3 - Grand Knight (final evolution)
    const LEVEL: i32 = 3;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: Option<&'static str> = Some("Human Knight Commander");
    const NEXT_UNIT_TYPE: Option<&'static str> = None; // Final evolution

    // Base Stats
    const BASE_HEALTH: i32 = 60;
    const BASE_ATTACK: u32 = 13;
    const BASE_MOVEMENT: i32 = 5;
    const ATTACK_STRENGTH: u32 = 0;
    const ATTACKS_PER_ROUND: u32 = 2;

    // Resistances - Maximum holy protection
    const RESISTANCE_BLUNT: u8 = 35;
    const RESISTANCE_PIERCE: u8 = 35;
    const RESISTANCE_FIRE: u8 = 25;
    const RESISTANCE_DARK: u8 = 50;
    const RESISTANCE_SLASH: u8 = 35;
    const RESISTANCE_CRUSH: u8 = 30;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Human;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Human Grand Knight";

    // Experience
    const STARTING_EXPERIENCE: i32 = 0;

    // ===== XP PROGRESSION =====

    /// Custom XP formula for Human Grand Knights - N/A (final evolution)
    /// This is the maximum level for this evolution line
    pub fn xp_formula(level: i32) -> i32 {
        if level <= 1 {
            return 0;
        }
        // Grand Knights don't evolve further, but maintain formula for consistency
        level * level * 50
    }

    // ===== ATTACK DEFINITIONS ======

    /// Devastating masterful strike
    fn legendary_strike() -> Attack {
        Attack::melee(
            "Legendary Strike",
            9, // damage
            1, // attack_times
            DamageType::Slash,
        )
    }

    /// Secondary crushing blow with decades of experience
    fn crushing_blow() -> Attack {
        Attack::melee(
            "Crushing Blow",
            7, // damage
            1, // attack_times
            DamageType::Crush,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Human Grand Knight unit
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
            "A legendary grand knight of unparalleled skill. Human Grand Knights are the ultimate warriors, combining decades of combat mastery with unbreakable discipline. They inspire their allies and crush their enemies with devastating precision and overwhelming force.".to_string(),
            terrain,
            graphics::SpriteType::Unit,
            Some("Human Knight Commander".to_string()),
            None, // Final evolution - no next unit
            combat_stats,
        );

        // Set level explicitly
        base.level = Self::LEVEL;
        base.experience = Self::STARTING_EXPERIENCE;

        // Define available attacks - grand knights have multiple powerful attacks
        base.attacks = vec![Self::legendary_strike(), Self::crushing_blow()];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for HumanGrandKnight {
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
    HumanGrandKnight,
    "Human Grand Knight",
    "A legendary grand knight of unparalleled skill. Human Grand Knights are the ultimate warriors, combining decades of combat mastery with unbreakable discipline. They inspire their allies and crush their enemies with devastating precision and overwhelming force.",
    Terrain::Grasslands,
    "Human",
    "Grand Knight"
);
