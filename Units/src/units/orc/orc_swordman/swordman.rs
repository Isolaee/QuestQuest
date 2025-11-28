use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 2 Orc Swordsman - Intermediate warrior unit.
///
/// Swordsmen are experienced fighters with improved combat abilities and defenses.
/// They evolve from Young Swordsmen and can further evolve into Elite Swordsmen.
///
/// # Evolution Chain
/// - **Previous**: Orc Young Swordsman (Level 1)
/// - **Current**: Orc Swordsman (Level 2)
/// - **Next**: Orc Elite Swordsman (Level 3)
///
/// # Stats
/// - **HP**: 125
/// - **Attack**: 15
/// - **Movement**: 4 + race bonus
/// - **Range**: Melee
/// - **XP to Next Level**: 450 (level² × 50)
pub struct OrcSwordsman {
    base: BaseUnit,
}

impl OrcSwordsman {
    // ===== UNIT PROPERTIES =====

    /// Level 2 - Swordsman
    const LEVEL: i32 = 2;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: &'static str = "Orc Young Swordsman"; // Evolved from Young Swordsman
    const NEXT_UNIT_TYPE: &'static str = "Orc Elite Swordsman"; // Evolves to Elite Swordsman

    // Base Stats
    const BASE_HEALTH: i32 = 125;
    const BASE_ATTACK: u32 = 15;
    const BASE_MOVEMENT: i32 = 4;
    const ATTACK_STRENGTH: u32 = 15;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances (medium armor - trained warrior)
    const RESISTANCE_BLUNT: u8 = 28;
    const RESISTANCE_PIERCE: u8 = 20;
    const RESISTANCE_FIRE: u8 = 12;
    const RESISTANCE_DARK: u8 = 17;
    const RESISTANCE_SLASH: u8 = 32;
    const RESISTANCE_CRUSH: u8 = 25;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Orc;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Orc Swordsman";

    // ===== ATTACK DEFINITIONS =====

    /// Improved sword slash
    fn sword_slash() -> Attack {
        Attack::melee(
            "Sword Slash",
            15, // damage
            1,  // range (melee)
            DamageType::Slash,
        )
    }

    /// Trained thrust attack
    fn sword_thrust() -> Attack {
        Attack::melee(
            "Sword Thrust",
            12, // damage
            1,  // range (melee)
            DamageType::Pierce,
        )
    }

    /// New ability - power strike
    fn power_strike() -> Attack {
        Attack::melee(
            "Power Strike",
            18, // damage
            1,  // range (melee)
            DamageType::Slash,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Orc Swordsman.
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
            "A capable orc warrior with proven combat skills. Swordsmen are the backbone of orc warbands, combining raw strength with battle-tested tactics. Can evolve into Elite Swordsmen through further victories.".to_string(),
            graphics::SpriteType::OrcWarrior,
            Some(crate::unit_type::UnitType::OrcYoungSwordsman),
            vec![crate::unit_type::UnitType::OrcEliteSwordsman],
            combat_stats,
        );

        // Set to level 2
        base.level = Self::LEVEL;
        base.experience = 100; // Carried over from level 1

        // Define available attacks for level 2
        base.attacks = vec![
            Self::sword_slash(),
            Self::sword_thrust(),
            Self::power_strike(),
        ];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for OrcSwordsman {
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
    OrcSwordsman,
    "Orc Swordsman",
    "An experienced orc swordsman, level 2",
    Terrain::Grasslands,
    "Orc",
    "Swordsman"
);
