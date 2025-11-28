use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 1 Dwarf Warrior - Entry level defender unit.
///
/// Young Warriors are inexperienced but sturdy dwarven fighters who are just beginning
/// their martial training. They have good defensive capabilities and can evolve into
/// regular Warriors with experience.
///
/// # Evolution Chain
/// - **Current**: Dwarf Young Warrior (Level 1)
/// - **Next**: Dwarf Warrior (Level 2)
///
/// # Stats
/// - **HP**: 110 (higher than orcs due to dwarven resilience)
/// - **Attack**: 11
/// - **Movement**: 3 + race bonus (slower than orcs)
/// - **Range**: Melee
/// - **XP to Next Level**: 50 (level² × 50)
pub struct DwarfYoungWarrior {
    base: BaseUnit,
}

impl DwarfYoungWarrior {
    // ===== UNIT PROPERTIES =====

    /// Level 1 - Young Warrior (Entry level)
    const LEVEL: i32 = 1;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: Option<&'static str> = None; // First in chain
    const NEXT_UNIT_TYPE: &'static str = "Dwarf Warrior"; // Evolves to Warrior

    // Base Stats - Dwarves are tougher but slower
    const BASE_HEALTH: i32 = 110;
    const BASE_ATTACK: u32 = 11;
    const BASE_MOVEMENT: i32 = 3;
    const ATTACK_STRENGTH: u32 = 11;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances (dwarven armor - naturally high resistance)
    const RESISTANCE_BLUNT: u8 = 25;
    const RESISTANCE_PIERCE: u8 = 20;
    const RESISTANCE_FIRE: u8 = 18; // Dwarves work with fire
    const RESISTANCE_DARK: u8 = 12;
    const RESISTANCE_SLASH: u8 = 28;
    const RESISTANCE_CRUSH: u8 = 30; // Very resistant to crushing

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Dwarf;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Dwarf Young Warrior";

    // Experience
    const STARTING_EXPERIENCE: i32 = 0;

    // ===== ATTACK DEFINITIONS =====

    /// Basic axe chop - reliable dwarven attack
    fn axe_chop() -> Attack {
        Attack::melee(
            "Axe Chop",
            11, // damage
            1,  // range (melee)
            DamageType::Slash,
        )
    }

    /// Shield bash - defensive attack
    fn shield_bash() -> Attack {
        Attack::melee(
            "Shield Bash",
            8, // damage
            1, // range (melee)
            DamageType::Crush,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Dwarf Young Warrior.
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

        // Create base unit with level 1
        let mut base = BaseUnit::new(
            name,
            position,
            Self::RACE,
            Self::UNIT_TYPE.to_string(),
            "An inexperienced but sturdy dwarven fighter beginning their martial training. Young Warriors excel in mountainous terrain and possess natural dwarven resilience. With experience, they evolve into seasoned Dwarf Warriors.".to_string(),
            terrain,
            graphics::SpriteType::DwarfWarrior,
            None,
            vec![crate::unit_type::UnitType::DwarfWarrior],
            combat_stats,
        );

        // Set level explicitly
        base.level = Self::LEVEL;
        base.experience = Self::STARTING_EXPERIENCE;

        // Define available attacks for level 1
        base.attacks = vec![Self::axe_chop(), Self::shield_bash()];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for DwarfYoungWarrior {
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
    DwarfYoungWarrior,
    "Dwarf Young Warrior",
    "A young dwarf warrior, level 1",
    Terrain::Mountain,
    "Dwarf",
    "Warrior"
);
