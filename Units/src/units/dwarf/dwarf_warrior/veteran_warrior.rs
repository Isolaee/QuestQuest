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
    attacks: Vec<Attack>,
}

impl DwarfVeteranWarrior {
    // ===== UNIT PROPERTIES =====

    /// Level 3 - Veteran Warrior (Max evolution)
    const LEVEL: i32 = 3;

    // Evolution chain
    const PREVIOUS_UNIT_TYPE: &'static str = "Dwarf Warrior"; // Evolved from Warrior
    const NEXT_UNIT_TYPE: Option<&'static str> = None; // Max evolution - no next form

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
            combat_stats,
        );

        // Set to level 3
        base.level = Self::LEVEL;
        base.experience = 250; // Carried over from level 2

        // Define available attacks for level 3
        let attacks = vec![
            Self::mighty_axe(),
            Self::shield_slam(),
            Self::war_hammer(),
            Self::cleaving_strike(),
        ];

        Self { base, attacks }
    }

    // ===== LEVEL PROGRESSION DATA =====

    /// Returns the previous unit type in evolution chain
    pub fn get_previous_unit_type() -> Option<String> {
        Some(Self::PREVIOUS_UNIT_TYPE.to_string())
    }

    /// Veteran Warrior is max evolution - no next unit type
    pub fn get_next_unit_type() -> Option<String> {
        Self::NEXT_UNIT_TYPE.map(|s| s.to_string())
    }

    /// Check if this unit has a next evolution
    pub fn has_next_evolution() -> bool {
        false
    }

    /// Veteran Warrior is max level - no next level stats yet
    /// Implement this if you add a level 4 (e.g., Champion)
    pub fn get_next_level_stats() -> Option<CombatStats> {
        None // Max level reached
    }

    /// Veteran Warrior is max level - no next level attacks yet
    pub fn get_next_level_attacks() -> Vec<Attack> {
        vec![] // Max level reached
    }

    // ===== CUSTOM METHODS =====

    /// Add a new attack to this unit's repertoire
    pub fn add_attack(&mut self, attack: Attack) {
        self.attacks.push(attack);
    }

    /// Remove an attack by name
    pub fn remove_attack(&mut self, name: &str) -> bool {
        if let Some(pos) = self.attacks.iter().position(|a| a.name == name) {
            self.attacks.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get all attack names
    pub fn get_attack_names(&self) -> Vec<&str> {
        self.attacks.iter().map(|a| a.name.as_str()).collect()
    }
}

// Use the macro to implement all standard Unit trait methods
crate::impl_unit_delegate!(DwarfVeteranWarrior);

crate::submit_unit!(
    DwarfVeteranWarrior,
    "Dwarf Veteran Warrior",
    "A veteran dwarf warrior, level 3",
    Terrain::Mountain,
    "Dwarf",
    "Warrior"
);
