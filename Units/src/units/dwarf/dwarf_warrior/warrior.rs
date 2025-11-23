use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 2 Dwarf Warrior - Intermediate defender unit.
///
/// Warriors are experienced dwarven fighters with improved combat abilities and formidable defenses.
/// They evolve from Young Warriors and can further evolve into Veteran Warriors.
///
/// # Evolution Chain
/// - **Previous**: Dwarf Young Warrior (Level 1)
/// - **Current**: Dwarf Warrior (Level 2)
/// - **Next**: Dwarf Veteran Warrior (Level 3)
///
/// # Stats
/// - **HP**: 140
/// - **Attack**: 14
/// - **Movement**: 3 + race bonus
/// - **Range**: Melee
/// - **XP to Next Level**: 200 (level² × 50)
pub struct DwarfWarrior {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl DwarfWarrior {
    // ===== UNIT PROPERTIES =====

    /// Level 2 - Warrior
    const LEVEL: i32 = 2;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: &'static str = "Dwarf Young Warrior"; // Evolved from Young Warrior
    const NEXT_UNIT_TYPE: &'static str = "Dwarf Veteran Warrior"; // Evolves to Veteran Warrior

    // Base Stats
    const BASE_HEALTH: i32 = 140;
    const BASE_ATTACK: u32 = 14;
    const BASE_MOVEMENT: i32 = 3;
    const ATTACK_STRENGTH: u32 = 14;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances (dwarven plate armor - trained defender)
    const RESISTANCE_BLUNT: u8 = 32;
    const RESISTANCE_PIERCE: u8 = 25;
    const RESISTANCE_FIRE: u8 = 22;
    const RESISTANCE_DARK: u8 = 15;
    const RESISTANCE_SLASH: u8 = 35;
    const RESISTANCE_CRUSH: u8 = 38;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Dwarf;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Dwarf Warrior";

    // ===== ATTACK DEFINITIONS =====

    /// Powerful axe swing
    fn heavy_axe() -> Attack {
        Attack::melee(
            "Heavy Axe",
            14, // damage
            1,  // range (melee)
            DamageType::Slash,
        )
    }

    /// Improved shield bash
    fn shield_bash() -> Attack {
        Attack::melee(
            "Shield Bash",
            10, // damage
            1,  // range (melee)
            DamageType::Crush,
        )
    }

    /// New ability - crushing hammer strike
    fn hammer_strike() -> Attack {
        Attack::melee(
            "Hammer Strike",
            16, // damage
            1,  // range (melee)
            DamageType::Crush,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Dwarf Warrior.
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

        // Create base unit with level 2
        let mut base = BaseUnit::new(
            name,
            position,
            Self::RACE,
            Self::UNIT_TYPE.to_string(),
            terrain,
            combat_stats,
        );

        // Set to level 2
        base.level = Self::LEVEL;
        base.experience = 50; // Carried over from level 1

        // Define available attacks for level 2
        let attacks = vec![
            Self::heavy_axe(),
            Self::shield_bash(),
            Self::hammer_strike(),
        ];

        Self { base, attacks }
    }

    // ===== LEVEL PROGRESSION DATA =====

    /// Returns the previous unit type in evolution chain.
    ///
    /// Returns `Some("Dwarf Young Warrior")` as the previous evolution stage.
    pub fn get_previous_unit_type() -> Option<String> {
        Some(Self::PREVIOUS_UNIT_TYPE.to_string())
    }

    /// Returns the next unit type in evolution chain.
    ///
    /// Returns `Some("Dwarf Veteran Warrior")` as the next evolution stage.
    pub fn get_next_unit_type() -> Option<String> {
        Some(Self::NEXT_UNIT_TYPE.to_string())
    }

    /// Check if this unit has a next evolution.
    ///
    /// Returns `true` since Warrior can evolve to Veteran Warrior.
    pub fn has_next_evolution() -> bool {
        true
    }

    /// Returns the combat stats for the next evolution level (Dwarf Veteran Warrior - Level 3).
    ///
    /// # Stats Progression
    /// - HP: 140 → 175 (+35)
    /// - Attack: 14 → 17 (+3)
    /// - Resistances increased significantly
    pub fn get_next_level_stats() -> CombatStats {
        CombatStats::new_with_attacks(
            175,                                  // health (+35)
            17,                                   // base attack (+3)
            3 + Race::Dwarf.get_movement_bonus(), // movement (same)
            RangeCategory::Melee,
            Resistances::new(
                40, // blunt (+8)
                32, // pierce (+7)
                28, // fire (+6)
                18, // dark (+3)
                42, // slash (+7)
                45, // crush (+7)
            ),
            17, // attack_strength (+3)
            1,  // attacks_per_round (same)
        )
    }

    /// Returns the attacks available at the next level
    pub fn get_next_level_attacks() -> Vec<Attack> {
        vec![
            Attack::melee("Mighty Axe", 20, 1, DamageType::Slash),
            Attack::melee("Shield Slam", 14, 1, DamageType::Crush),
            Attack::melee("War Hammer", 22, 1, DamageType::Crush),
            Attack::melee("Cleaving Strike", 18, 1, DamageType::Slash),
        ]
    }
}

// Use the macro to implement all standard Unit trait methods
crate::impl_unit_delegate!(DwarfWarrior);
