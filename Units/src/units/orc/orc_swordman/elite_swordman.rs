use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 3 Orc Elite Swordsman - Maximum evolution warrior unit.
///
/// Elite Swordsmen are master warriors with peak combat abilities and formidable defenses.
/// They represent the pinnacle of the Orc Swordsman evolution chain. After reaching this stage,
/// units continue to grow stronger through incremental stat boosts (+2 HP, +1 attack per level).
///
/// # Evolution Chain
/// - **Previous**: Orc Swordsman (Level 2)
/// - **Current**: Orc Elite Swordsman (Level 3+)
/// - **Next**: None (continues with incremental improvements)
///
/// # Stats
/// - **HP**: 150 (increases by +2 per level after 3)
/// - **Attack**: 18 (increases by +1 per level after 3)
/// - **Movement**: 4 + race bonus
/// - **Range**: Melee
/// - **XP per Level**: 800, 1250, 1800... (level² × 50)
pub struct OrcEliteSwordsman {
    base: BaseUnit,
}

impl OrcEliteSwordsman {
    // ===== UNIT PROPERTIES =====

    /// Level 3 - Elite Swordsman (Max evolution)
    const LEVEL: i32 = 3;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: &'static str = "Orc Swordsman"; // Evolved from Swordsman
    const NEXT_UNIT_TYPE: Option<&'static str> = None; // Max evolution - no next form

    // Base Stats
    const BASE_HEALTH: i32 = 150;
    const BASE_ATTACK: u32 = 18;
    const BASE_MOVEMENT: i32 = 4;
    const ATTACK_STRENGTH: u32 = 18;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances (heavy armor - elite warrior)
    const RESISTANCE_BLUNT: u8 = 35;
    const RESISTANCE_PIERCE: u8 = 25;
    const RESISTANCE_FIRE: u8 = 15;
    const RESISTANCE_DARK: u8 = 20;
    const RESISTANCE_SLASH: u8 = 40;
    const RESISTANCE_CRUSH: u8 = 30;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Orc;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Orc Elite Swordsman";

    // ===== ATTACK DEFINITIONS =====

    /// Primary melee attack - powerful overhead slash
    fn heavy_slash() -> Attack {
        Attack::melee(
            "Heavy Slash",
            22, // damage
            1,  // range (melee)
            DamageType::Slash,
        )
    }

    /// Secondary attack - quick thrust
    fn sword_thrust() -> Attack {
        Attack::melee(
            "Sword Thrust",
            15, // damage
            1,  // range (melee)
            DamageType::Pierce,
        )
    }

    /// Brutal crushing blow
    fn crushing_blow() -> Attack {
        Attack::melee(
            "Crushing Blow",
            18, // damage
            1,  // range (melee)
            DamageType::Crush,
        )
    }

    /// Defensive attack when threatened
    fn defensive_strike() -> Attack {
        Attack::melee(
            "Defensive Strike",
            12, // damage
            1,  // range (melee)
            DamageType::Slash,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Orc Elite Swordsman.
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
            "A battle-hardened orc warrior of exceptional prowess. Elite Swordsmen are veterans of countless conflicts, wielding their blades with deadly precision. Their ferocity in combat is matched only by their tactical cunning.".to_string(),
            terrain,
            graphics::SpriteType::OrcWarrior,
            Some("Orc Swordsman"),
            vec![],
            combat_stats,
        );

        // Set to level 3
        base.level = Self::LEVEL;
        base.experience = 250; // Carried over from level 2

        // Define available attacks for level 3
        base.attacks = vec![
            Self::heavy_slash(),
            Self::sword_thrust(),
            Self::crushing_blow(),
            Self::defensive_strike(),
        ];

        Self { base }
    }

    // ===== CUSTOM METHODS =====

    /// Add a new attack to this unit's repertoire
    pub fn add_attack(&mut self, attack: Attack) {
        self.base.attacks.push(attack);
    }

    /// Remove an attack by name
    pub fn remove_attack(&mut self, name: &str) -> bool {
        if let Some(pos) = self.base.attacks.iter().position(|a| a.name == name) {
            self.base.attacks.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get all attack names
    pub fn get_attack_names(&self) -> Vec<&str> {
        self.base.attacks.iter().map(|a| a.name.as_str()).collect()
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for OrcEliteSwordsman {
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
    OrcEliteSwordsman,
    "Orc Elite Swordsman",
    "An elite orc swordsman, level 3",
    Terrain::Grasslands,
    "Orc",
    "Swordsman"
);
