use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

/// Level 3 Dwarf Veteran Warrior - Maximum evolution defender unit.
///
/// Veteran Warriors are legendary dwarven champions with peak combat abilities and nearly
/// impenetrable defenses. They represent the pinnacle of the Dwarf Warrior evolution chain.
/// After reaching this stage, units continue to grow stronger through incremental stat boosts
/// (+2 HP, +1 attack per level).
///
/// # Evolution Chain
/// - **Previous**: Dwarf Warrior (Level 2)
/// - **Current**: Dwarf Veteran Warrior (Level 3+)
/// - **Next**: None (continues with incremental improvements)
///
/// # Stats
/// - **HP**: 175 (increases by +2 per level after 3)
/// - **Attack**: 17 (increases by +1 per level after 3)
/// - **Movement**: 3 + race bonus
/// - **Range**: Melee
/// - **XP per Level**: 450, 800, 1250... (level² × 50)
pub struct DwarfVeteranWarrior {
    base: BaseUnit,
}

impl DwarfVeteranWarrior {
    // ===== UNIT PROPERTIES =====

    /// Level 3 - Veteran Warrior (Max evolution)
    const LEVEL: i32 = 3;

    // Base Stats
    const BASE_HEALTH: i32 = 175;
    const BASE_ATTACK: u32 = 17;
    const BASE_MOVEMENT: i32 = 3;
    const ATTACK_STRENGTH: u32 = 17;
    const ATTACKS_PER_ROUND: u32 = 1;

    // Resistances (legendary dwarven armor - nearly impenetrable)
    const RESISTANCE_BLUNT: u8 = 40;
    const RESISTANCE_PIERCE: u8 = 32;
    const RESISTANCE_FIRE: u8 = 28;
    const RESISTANCE_DARK: u8 = 18;
    const RESISTANCE_SLASH: u8 = 42;
    const RESISTANCE_CRUSH: u8 = 45;

    // Range category
    const RANGE_CATEGORY: RangeCategory = RangeCategory::Melee;

    // Race
    const RACE: Race = Race::Dwarf;

    // Unit type identifier
    const UNIT_TYPE: &'static str = "Dwarf Veteran Warrior";

    // ===== ATTACK DEFINITIONS =====

    /// Primary melee attack - devastating axe swing
    fn mighty_axe() -> Attack {
        Attack::melee(
            "Mighty Axe",
            20, // damage
            1,  // range (melee)
            DamageType::Slash,
        )
    }

    /// Powerful shield attack
    fn shield_slam() -> Attack {
        Attack::melee(
            "Shield Slam",
            14, // damage
            1,  // range (melee)
            DamageType::Crush,
        )
    }

    /// Brutal war hammer strike
    fn war_hammer() -> Attack {
        Attack::melee(
            "War Hammer",
            22, // damage
            1,  // range (melee)
            DamageType::Crush,
        )
    }

    /// Wide cleaving attack
    fn cleaving_strike() -> Attack {
        Attack::melee(
            "Cleaving Strike",
            18, // damage
            1,  // range (melee)
            DamageType::Slash,
        )
    }

    // ===== CONSTRUCTOR =====

    /// Creates a new Dwarf Veteran Warrior.
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
            "An elite dwarven warrior who has survived countless battles. Veterans are the pinnacle of dwarven martial prowess, nearly impervious in mountain terrain. Their legendary resilience and devastating attacks make them feared opponents.".to_string(),
            terrain,
            graphics::SpriteType::DwarfWarrior,
            Some(crate::unit_type::UnitType::DwarfWarrior),
            vec![],
            combat_stats,
        );

        // Set to level 3
        base.level = Self::LEVEL;
        base.experience = 250; // Carried over from level 2

        // Define available attacks for level 3
        base.attacks = vec![
            Self::mighty_axe(),
            Self::shield_slam(),
            Self::war_hammer(),
            Self::cleaving_strike(),
        ];

        Self { base }
    }

    // ===== ATTACK MANAGEMENT =====
    // Note: Attack management methods (add_attack, remove_attack, get_attack_names)
    // are now static methods on BaseUnit and can be called directly on any unit's
    // attacks vector using BaseUnit::add_attack_to_vec(&mut self.attacks, attack).
    // Evolution chain is stored in BaseUnit and accessed via trait methods.
}

// Implement the Unit trait with minimal boilerplate
// Evolution chain is automatically handled by BaseUnit (stores evolution_previous and evolution_next)
// Attack management methods are now static on BaseUnit
impl crate::unit_trait::Unit for DwarfVeteranWarrior {
    fn base(&self) -> &BaseUnit {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseUnit {
        &mut self.base
    }

    fn attacks(&self) -> &[Attack] {
        &self.base.attacks
    }

    // Evolution methods work automatically - no overrides needed
    // BaseUnit handles attack updates directly
}

crate::submit_unit!(
    DwarfVeteranWarrior,
    "Dwarf Veteran Warrior",
    "A veteran dwarf warrior, level 3",
    Terrain::Mountain,
    "Dwarf",
    "Warrior"
);
