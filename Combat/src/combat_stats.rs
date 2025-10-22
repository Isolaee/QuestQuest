//! # Combat Statistics Module
//!
//! Defines the core data structures for combat including damage types, resistances,
//! range categories, and comprehensive unit combat statistics.
//!
//! ## Key Concepts
//!
//! - **Damage Types**: Different types of damage that can be resisted differently
//! - **Resistances**: Percentage-based damage reduction (0-100%)
//! - **Range Categories**: Determine attack distance and counter-attack eligibility
//! - **Multi-Attack**: Units can have multiple attacks per combat round

use serde::{Deserialize, Serialize};

/// Types of damage that can be dealt in combat.
///
/// Each damage type can be resisted independently, allowing for tactical
/// unit composition and equipment choices. For example, heavily armored
/// units might have high resistance to Slash but low resistance to Blunt.
///
/// # Variants
///
/// - `Blunt`: Crushing damage from maces, clubs, etc.
/// - `Pierce`: Penetrating damage from arrows, spears, etc.
/// - `Fire`: Elemental fire damage
/// - `Dark`: Dark magic damage
/// - `Slash`: Cutting damage from swords, axes, etc.
/// - `Crush`: Heavy impact damage from hammers, siege weapons, etc.
///
/// # Examples
///
/// ```
/// use combat::DamageType;
///
/// let sword_damage = DamageType::Slash;
/// let fire_spell = DamageType::Fire;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    /// Crushing damage (maces, clubs)
    Blunt,
    /// Penetrating damage (arrows, spears)
    Pierce,
    /// Elemental fire damage
    Fire,
    /// Dark magic damage
    Dark,
    /// Cutting damage (swords, axes)
    Slash,
    /// Heavy impact damage (hammers, siege)
    Crush,
}

/// Range category determining attack distance and counter-attack rules.
///
/// Range categories affect both the maximum distance a unit can attack from
/// and whether defenders can counter-attack. Melee units can be counter-attacked,
/// while ranged units attack from safety.
///
/// # Examples
///
/// ```
/// use combat::RangeCategory;
///
/// let melee = RangeCategory::Melee;
/// assert_eq!(melee.base_range(), 1);
///
/// let archer = RangeCategory::Range;
/// assert_eq!(archer.base_range(), 3);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeCategory {
    /// Close combat (range 1), allows counter-attacks
    Melee,
    /// Ranged combat (range 3), no counter-attacks
    Range,
    /// Siege combat (range 5), no counter-attacks
    Siege,
}

impl RangeCategory {
    /// Returns the base attack range for this category.
    ///
    /// # Returns
    ///
    /// - Melee: 1
    /// - Range: 3
    /// - Siege: 5
    pub fn base_range(&self) -> i32 {
        match self {
            RangeCategory::Melee => 1,
            RangeCategory::Range => 3,
            RangeCategory::Siege => 5,
        }
    }
}

/// Resistance values for each damage type as percentage multipliers.
///
/// Resistances reduce incoming damage of specific types. A resistance of 0
/// means no reduction, while 100 means complete immunity. The actual damage
/// taken is: `damage * (1 - resistance / 100)`.
///
/// # Examples
///
/// ```
/// use combat::{Resistances, DamageType};
///
/// let mut resistances = Resistances::new(20, 10, 0, 5, 30, 15);
/// assert_eq!(resistances.get_resistance(DamageType::Slash), 30);
///
/// resistances.set_resistance(DamageType::Fire, 50);
/// assert_eq!(resistances.get_resistance(DamageType::Fire), 50);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resistances {
    /// Resistance to blunt damage (0-100%)
    pub blunt: u8,
    /// Resistance to pierce damage (0-100%)
    pub pierce: u8,
    /// Resistance to fire damage (0-100%)
    pub fire: u8,
    /// Resistance to dark damage (0-100%)
    pub dark: u8,
    /// Resistance to slash damage (0-100%)
    pub slash: u8,
    /// Resistance to crush damage (0-100%)
    pub crush: u8,
}

impl Resistances {
    /// Creates new resistances with specified values.
    ///
    /// Values are automatically clamped to the range 0-100.
    ///
    /// # Arguments
    ///
    /// * `blunt` - Blunt damage resistance (0-100)
    /// * `pierce` - Pierce damage resistance (0-100)
    /// * `fire` - Fire damage resistance (0-100)
    /// * `dark` - Dark damage resistance (0-100)
    /// * `slash` - Slash damage resistance (0-100)
    /// * `crush` - Crush damage resistance (0-100)
    ///
    /// # Examples
    ///
    /// ```
    /// use combat::Resistances;
    ///
    /// let resistances = Resistances::new(25, 15, 0, 10, 30, 20);
    /// ```
    pub fn new(blunt: u8, pierce: u8, fire: u8, dark: u8, slash: u8, crush: u8) -> Self {
        Self {
            blunt: blunt.min(100),
            pierce: pierce.min(100),
            fire: fire.min(100),
            dark: dark.min(100),
            slash: slash.min(100),
            crush: crush.min(100),
        }
    }

    /// Returns the resistance value for a specific damage type.
    ///
    /// # Arguments
    ///
    /// * `damage_type` - The type of damage to query
    ///
    /// # Returns
    ///
    /// Resistance value as u8 (0-100)
    pub fn get_resistance(&self, damage_type: DamageType) -> u8 {
        match damage_type {
            DamageType::Blunt => self.blunt,
            DamageType::Pierce => self.pierce,
            DamageType::Fire => self.fire,
            DamageType::Dark => self.dark,
            DamageType::Slash => self.slash,
            DamageType::Crush => self.crush,
        }
    }

    /// Sets the resistance value for a specific damage type.
    ///
    /// The value is automatically clamped to 0-100.
    ///
    /// # Arguments
    ///
    /// * `damage_type` - The type of damage to modify
    /// * `value` - New resistance value (will be clamped to 0-100)
    pub fn set_resistance(&mut self, damage_type: DamageType, value: u8) {
        let clamped = value.min(100);
        match damage_type {
            DamageType::Blunt => self.blunt = clamped,
            DamageType::Pierce => self.pierce = clamped,
            DamageType::Fire => self.fire = clamped,
            DamageType::Dark => self.dark = clamped,
            DamageType::Slash => self.slash = clamped,
            DamageType::Crush => self.crush = clamped,
        }
    }
}

impl Default for Resistances {
    /// Creates default resistances with all values set to 0 (no resistance).
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0)
    }
}

/// Comprehensive combat statistics for a unit.
///
/// `CombatStats` encapsulates all the numerical values that define a unit's
/// combat capabilities, including health, damage, attack frequency, range,
/// and damage resistances.
///
/// # Multi-Attack System
///
/// Units can attack multiple times per combat round using `attacks_per_round`.
/// Each attack deals `attack_strength` damage (modified by `attack_modifier`),
/// allowing for units that make many weak attacks or few strong attacks.
///
/// # Examples
///
/// ```
/// use combat::{CombatStats, RangeCategory, Resistances, DamageType};
///
/// // Create a basic warrior
/// let warrior = CombatStats::new(
///     100,                    // max health
///     20,                     // base attack
///     5,                      // movement speed
///     RangeCategory::Melee,
///     Resistances::default()
/// );
///
/// // Create an archer with multiple attacks
/// let archer = CombatStats::new_with_attacks(
///     80,                     // max health
///     15,                     // base attack
///     6,                      // movement speed
///     RangeCategory::Range,
///     Resistances::default(),
///     8,                      // attack strength per shot
///     3                       // 3 attacks per round
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatStats {
    /// Current health points
    pub health: i32,
    /// Maximum health points
    pub max_health: i32,
    /// Base attack damage before modifiers
    pub base_attack: u32,
    /// Additional attack damage from equipment, buffs, or debuffs
    pub attack_modifier: i32,
    /// Damage dealt by each individual attack
    pub attack_strength: u32,
    /// Number of attacks this unit makes per combat round
    pub attacks_per_round: u32,
    /// Movement speed on the game map
    pub movement_speed: i32,
    /// Range category (Melee, Range, or Siege)
    pub range_category: RangeCategory,
    /// Actual attack range in tiles
    pub attack_range: i32,
    /// Damage resistances as percentage multipliers
    pub resistances: Resistances,
    /// Hit chance percentage affected by terrain and positioning (0-100)
    pub terrain_hit_chance: u8,
}

impl CombatStats {
    /// Creates new combat stats with default attack parameters.
    ///
    /// This constructor sets `attack_strength` equal to `base_attack` and
    /// `attacks_per_round` to 1, suitable for standard single-attack units.
    ///
    /// # Arguments
    ///
    /// * `max_health` - Maximum health points
    /// * `base_attack` - Base attack damage
    /// * `movement_speed` - Movement speed on the map
    /// * `range_category` - Attack range category
    /// * `resistances` - Damage resistances
    ///
    /// # Examples
    ///
    /// ```
    /// use combat::{CombatStats, RangeCategory, Resistances};
    ///
    /// let stats = CombatStats::new(
    ///     100,
    ///     20,
    ///     5,
    ///     RangeCategory::Melee,
    ///     Resistances::default()
    /// );
    /// assert_eq!(stats.health, 100);
    /// assert_eq!(stats.attacks_per_round, 1);
    /// ```
    pub fn new(
        max_health: i32,
        base_attack: u32,
        movement_speed: i32,
        range_category: RangeCategory,
        resistances: Resistances,
    ) -> Self {
        // Default: 1 attack per round with strength equal to base_attack
        Self::new_with_attacks(
            max_health,
            base_attack,
            movement_speed,
            range_category,
            resistances,
            base_attack, // attack_strength defaults to base_attack
            1,           // attacks_per_round defaults to 1
        )
    }

    /// Creates new combat stats with custom attack parameters.
    ///
    /// This constructor allows full control over the multi-attack system,
    /// enabling units that make multiple attacks with different damage values.
    ///
    /// # Arguments
    ///
    /// * `max_health` - Maximum health points
    /// * `base_attack` - Base attack damage (used for display)
    /// * `movement_speed` - Movement speed on the map
    /// * `range_category` - Attack range category
    /// * `resistances` - Damage resistances
    /// * `attack_strength` - Damage per individual attack
    /// * `attacks_per_round` - Number of attacks per combat round (min 1)
    ///
    /// # Examples
    ///
    /// ```
    /// use combat::{CombatStats, RangeCategory, Resistances};
    ///
    /// // Create a unit that attacks 3 times for 10 damage each
    /// let multi_attacker = CombatStats::new_with_attacks(
    ///     100,
    ///     30,  // total base attack
    ///     5,
    ///     RangeCategory::Melee,
    ///     Resistances::default(),
    ///     10,  // 10 damage per attack
    ///     3    // 3 attacks
    /// );
    /// assert_eq!(multi_attacker.attacks_per_round, 3);
    /// ```
    pub fn new_with_attacks(
        max_health: i32,
        base_attack: u32,
        movement_speed: i32,
        range_category: RangeCategory,
        resistances: Resistances,
        attack_strength: u32,
        attacks_per_round: u32,
    ) -> Self {
        Self {
            health: max_health,
            max_health,
            base_attack,
            attack_modifier: 0,
            attack_strength,
            attacks_per_round: attacks_per_round.max(1), // At least 1 attack
            movement_speed,
            range_category,
            attack_range: range_category.base_range(),
            resistances,
            terrain_hit_chance: 75, // Default 75% hit chance
        }
    }

    /// Returns the total attack damage including modifiers.
    ///
    /// Combines `base_attack` with `attack_modifier`, clamped to 0 minimum.
    ///
    /// # Returns
    ///
    /// Total attack as u32 (base + modifier, minimum 0)
    pub fn get_total_attack(&self) -> u32 {
        (self.base_attack as i32 + self.attack_modifier).max(0) as u32
    }

    /// Sets the terrain-based hit chance.
    ///
    /// Different terrain types can affect accuracy (e.g., forest provides cover).
    /// Value is clamped to 0-100.
    ///
    /// # Arguments
    ///
    /// * `chance` - Hit chance percentage (0-100)
    pub fn set_terrain_hit_chance(&mut self, chance: u8) {
        self.terrain_hit_chance = chance.min(100);
    }

    /// Checks if the unit is alive.
    ///
    /// # Returns
    ///
    /// `true` if health is greater than 0, `false` otherwise
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Applies damage with resistance calculation.
    ///
    /// Damage is reduced based on the unit's resistance to the specific damage type.
    /// Final damage is calculated as: `base_damage * (1 - resistance / 100)`.
    ///
    /// # Arguments
    ///
    /// * `base_damage` - Base damage before resistance
    /// * `damage_type` - Type of damage being dealt
    ///
    /// # Returns
    ///
    /// Actual damage taken after resistance calculation
    pub fn take_damage_with_resistance(
        &mut self,
        base_damage: u32,
        damage_type: DamageType,
    ) -> u32 {
        let resistance = self.resistances.get_resistance(damage_type);
        let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
        let actual_damage = (base_damage as f32 * resistance_multiplier) as u32;

        self.health = (self.health - actual_damage as i32).max(0);
        actual_damage
    }

    /// Applies raw damage without resistance calculation.
    ///
    /// # Arguments
    ///
    /// * `damage` - Damage to apply (negative values are ignored)
    ///
    /// # Returns
    ///
    /// `true` if the unit died from this damage, `false` otherwise
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health = (self.health - damage.max(0)).max(0);
        !self.is_alive()
    }

    /// Restores health to the unit.
    ///
    /// Health cannot exceed `max_health`. Negative amounts are ignored.
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount of health to restore
    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount.max(0)).min(self.max_health);
    }

    /// Returns the health as a percentage.
    ///
    /// Useful for health bars and status displays.
    ///
    /// # Returns
    ///
    /// Health percentage as f32 (0.0 = dead, 1.0 = full health)
    pub fn health_percentage(&self) -> f32 {
        if self.max_health > 0 {
            self.health as f32 / self.max_health as f32
        } else {
            0.0
        }
    }

    /// Calculates damage that would be dealt to a target considering resistance.
    ///
    /// This is a preview calculation that doesn't actually apply damage.
    ///
    /// # Arguments
    ///
    /// * `target` - Target unit's combat stats
    /// * `damage_type` - Type of damage to deal
    ///
    /// # Returns
    ///
    /// Final damage after resistance (minimum 1)
    pub fn calculate_damage_to(&self, target: &CombatStats, damage_type: DamageType) -> u32 {
        let base_damage = self.get_total_attack();
        let resistance = target.resistances.get_resistance(damage_type);
        let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
        let final_damage = (base_damage as f32 * resistance_multiplier) as u32;
        final_damage.max(1) // Always at least 1 damage
    }

    /// Calculates total damage for all attacks in a combat round.
    ///
    /// Multiplies individual attack damage by `attacks_per_round` and applies
    /// target's resistance. Useful for damage prediction in combat UI.
    ///
    /// # Arguments
    ///
    /// * `target` - Target unit's combat stats
    /// * `damage_type` - Type of damage to deal
    ///
    /// # Returns
    ///
    /// Total damage for all attacks after resistance (minimum 1)
    pub fn calculate_total_round_damage(
        &self,
        target: &CombatStats,
        damage_type: DamageType,
    ) -> u32 {
        let resistance = target.resistances.get_resistance(damage_type);
        let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);

        // Total damage = (attack_strength + modifiers) * attacks_per_round * resistance
        let modified_strength = (self.attack_strength as i32 + self.attack_modifier).max(0) as u32;
        let total_damage = modified_strength * self.attacks_per_round;
        let final_damage = (total_damage as f32 * resistance_multiplier) as u32;

        final_damage.max(1) // Always at least 1 damage
    }
}
