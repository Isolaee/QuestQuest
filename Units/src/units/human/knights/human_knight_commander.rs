use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 2 Human Knight Commander
///
/// Human Knight Commanders are seasoned warriors who have proven their
/// worth on the battlefield. They lead from the front and inspire their
/// fellow knights with their tactical acumen and unwavering courage.
/// Their experience makes them formidable opponents.
///
/// # Evolution Chain
/// **Previous**: Human Knight (Level 1)
/// **Current**: Human Knight Commander (Level 2)
/// **Next**: Human Grand Knight (Level 3)
///
/// # Stats
/// - **HP**: 52
/// - **Attack**: 9
/// - **Movement**: 4
/// - **Range**: Melee
/// - **XP to Next Level**: 360 (level² × 40, moderate leveling)
pub struct HumanKnightCommander {
    base: BaseUnit,
}

impl HumanKnightCommander {
    // ===== UNIT PROPERTIES =====

    /// Level 2 - Knight Commander (mid level)
    const LEVEL: i32 = 2;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: Option<&'static str> = Some("Human Knight");
    const NEXT_UNIT_TYPE: &'static str = "Human Grand Knight";

    // Base Stats
    const BASE_HEALTH: i32 = 52;
    const BASE_ATTACK: u32 = 9;
    const BASE_MOVEMENT: i32 = 4;
    const ATTACK_STRENGTH: u32 = 0;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances - Enhanced holy protection
    const RESISTANCE_BLUNT: u8 = 30;
    const RESISTANCE_PIERCE: u8 = 25;
    const RESISTANCE_FIRE: u8 = 15;
    const RESISTANCE_DARK: u8 = 35;
    const RESISTANCE_SLASH: u8 = 30;
    const RESISTANCE_CRUSH: u8 = 20;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Human;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Human Knight Commander";

    // Experience
    const STARTING_EXPERIENCE: i32 = 0;

    // ===== XP PROGRESSION =====

    /// Custom XP formula for Human Knight Commanders - moderate leveling
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

    /// Commanding sword strike with tactical precision
    fn commanding_strike() -> Attack {
        Attack::melee(
            "Commanding Strike",
            7, // damage
            1, // attack_times
            DamageType::Slash,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Human Knight Commander unit
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
            "An experienced knight commander who leads through example. Human Knight Commanders are battle-tested warriors who combine superior combat skills with tactical leadership. Their presence on the battlefield inspires nearby allies, granting them increased combat effectiveness. They excel at coordinating attacks and defending their allies. With continued service, they will become grand knights.".to_string(),
            Some(crate::unit_type::UnitType::HumanKnight),
            vec![],
            combat_stats,
        );

        // Set level explicitly
        base.level = Self::LEVEL;
        base.experience = Self::STARTING_EXPERIENCE;

        // Define available attacks
        base.attacks = vec![Self::commanding_strike()];

        // Add leadership aura - buffs adjacent allies
        let leadership_aura = crate::ability::Ability::Aura(crate::ability::AuraAbility::new(
            "Knight Commander's Leadership",
            "Adjacent allies gain +1 attack",
            1, // range: 1 hex (adjacent)
            crate::ability::AuraTarget::Allies,
            crate::ability::AuraEffect::AttackBonus(1),
        ));
        base.add_ability(leadership_aura);

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for HumanKnightCommander {
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
    HumanKnightCommander,
    "Human Knight Commander",
    "An experienced knight commander who leads through example. Human Knight Commanders are battle-tested warriors who combine superior combat skills with tactical leadership. Their presence on the battlefield inspires nearby allies, granting them increased combat effectiveness. They excel at coordinating attacks and defending their allies. With continued service, they will become grand knights.",
    Terrain::Grasslands,
    "Human",
    "Knight Commander"
);
