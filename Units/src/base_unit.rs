//! Base unit implementation and shared data structures.
//!
//! This module provides [`BaseUnit`], which contains the common data and functionality
//! shared by all concrete unit implementations. It handles stat caching, equipment
//! bonuses, and level progression.

use crate::attack::Attack;
use crate::combat::{CombatStats, DamageType};
use crate::unit_race::{Race, Terrain};
use crate::unit_type::UnitType;
use graphics::{HexCoord, SpriteType};
use items::{Equipment, Item};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::unit_trait::UnitId;
use std::collections::HashMap;

/// Base unit structure containing common data for all unit types.
///
/// This structure holds the shared state that all concrete unit implementations use,
/// including identity, position, stats, equipment, and progression. It provides
/// methods for stat recalculation and equipment management.
///
/// # Fields
///
/// - `id`: Unique identifier for the unit
/// - `name`: Display name
/// - `position`: Current hex coordinate
/// - `race`: Character race
/// - `unit_type`: Type identifier (e.g., "Human Warrior")
/// - `experience`: Current experience points
/// - `level`: Current level
/// - `combat_stats`: Base combat statistics
/// - `equipment`: Currently equipped items
/// - `inventory`: Items in the unit's backpack
/// - `cached_*`: Pre-calculated values for performance
///

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseUnit {
    // Identity
    pub id: UnitId,
    pub name: String,
    pub position: HexCoord,
    pub race: Race,
    pub unit_type: String,   // e.g., "Human Warrior", "Orc Grunt", etc.
    pub description: String, // Lore and gameplay description for wiki

    // Progression
    pub experience: i32,
    pub level: i32,

    // Combat and stats
    pub combat_stats: CombatStats,
    pub equipment: Equipment,
    pub inventory: Vec<Item>,
    /// Optional per-unit terrain -> defense mappings (percentage 0-100).
    /// If present, these override the race-based terrain defense values.
    pub terrain_defenses: Option<HashMap<Terrain, u8>>,

    // Cached values (recalculated when equipment/level changes)
    pub cached_defense: i32,
    pub cached_attack: i32,
    pub cached_movement: i32,
    pub cached_max_health: i32,

    // Visual representation
    pub sprite_type: SpriteType,

    // Evolution chain
    pub evolution_previous: Option<UnitType>,
    /// Multiple possible evolution paths (empty if no evolution available)
    pub evolution_next: Vec<UnitType>,

    // Attacks (stored here so level-up methods can update them automatically)
    pub attacks: Vec<Attack>,
    pub attacked_this_round: bool,

    // Abilities
    pub abilities: Vec<crate::ability::Ability>,
    pub ability_state: crate::ability::AbilityState,
}

impl BaseUnit {
    /// Creates a new base unit with the specified combat stats.
    ///
    /// This constructor initializes a level 1 unit with no experience,
    /// empty equipment, and cached stats calculated from the base stats.
    ///
    /// # Arguments
    ///
    /// * `name` - The unit's display name
    /// * `position` - Starting position on the hex grid
    /// * `race` - The unit's race
    /// * `unit_type` - Type identifier (e.g., "Human Warrior")
    /// * `description` - Lore and gameplay description
    /// * `terrain` - The terrain at the starting position
    /// * `combat_stats` - Base combat statistics
    ///
    /// # Returns
    ///
    /// A new `BaseUnit` instance at level 1 with 0 experience.
    ///
    /// Convenience constructor for units that automatically uses `SpriteType::Unit`.
    ///
    /// This is the recommended constructor for creating game units, as it eliminates
    /// the need to specify the sprite type which is always `SpriteType::Unit` for units.
    ///
    /// # Arguments
    ///
    /// * `name` - The unit's display name
    /// * `position` - Starting position on the hex grid
    /// * `race` - The unit's race
    /// * `unit_type` - Type identifier (e.g., "Human Warrior")
    /// * `description` - Lore and gameplay description
    /// * `evolution_previous` - Optional previous evolution in the chain
    /// * `evolution_next` - Possible next evolutions
    /// * `combat_stats` - Base combat statistics
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        position: HexCoord,
        race: Race,
        unit_type: String,
        description: String,
        evolution_previous: Option<UnitType>,
        evolution_next: Vec<UnitType>,
        combat_stats: CombatStats,
    ) -> Self {
        Self::new_with_sprite(
            name,
            position,
            race,
            unit_type,
            description,
            SpriteType::Unit,
            evolution_previous,
            evolution_next,
            combat_stats,
        )
    }

    /// Creates a new base unit with a custom sprite type.
    ///
    /// This is the lower-level constructor that allows specifying any sprite type.
    /// Most code should use `BaseUnit::new()` instead, which defaults to `SpriteType::Unit`.
    ///
    /// # TODO: Refactor for Production
    ///
    /// This constructor has too many arguments (10/7 allowed by Clippy).
    /// For production, consider one of these improvements:
    /// - **Builder Pattern**: Create `BaseUnitBuilder` for fluent construction
    /// - **Config Struct**: Group related params (unit_type, description, sprite_type, evolutions)
    ///   into a `UnitDefinition` or `UnitTemplate` struct
    /// - **Factory Method**: Move construction logic to a factory that loads unit definitions
    ///   from configuration/data files
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_sprite(
        name: String,
        position: HexCoord,
        race: Race,
        unit_type: String,
        description: String,
        sprite_type: SpriteType,
        evolution_previous: Option<UnitType>,
        evolution_next: Vec<UnitType>,
        combat_stats: CombatStats,
    ) -> Self {
        let max_health = combat_stats.health;
        let base_attack = combat_stats.base_attack as i32;
        let base_movement = combat_stats.movement_speed;

        BaseUnit {
            id: Uuid::new_v4(),
            name,
            position,
            race,
            unit_type,
            description,
            level: 1,
            experience: 0,
            combat_stats,
            equipment: Equipment::default(),
            inventory: Vec::new(),
            cached_defense: 0, // Will be calculated from resistances
            cached_attack: base_attack,
            cached_movement: base_movement,
            cached_max_health: max_health,
            terrain_defenses: None,
            sprite_type,
            evolution_previous,
            evolution_next,
            attacks: Vec::new(), // Will be set by unit constructor
            attacked_this_round: false,
            abilities: Vec::new(), // Will be set by unit constructor
            ability_state: crate::ability::AbilityState::new(),
        }
    }

    /// Creates a base unit with a specific level and experience.
    ///
    /// This is useful for creating higher-level units or loading saved games.
    ///
    /// # Arguments
    ///
    /// * `name` - The unit's display name
    /// * `position` - Starting position on the hex grid
    /// * `race` - The unit's race
    /// * `unit_type` - Type identifier (e.g., "Human Warrior")
    /// * `level` - Initial level (minimum 1)
    /// * `experience` - Initial experience points (minimum 0)
    /// * `description` - Lore and gameplay description
    /// * `combat_stats` - Base combat statistics
    ///
    /// # Returns
    ///
    /// A new `BaseUnit` instance with the specified level and experience.
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_level(
        name: String,
        position: HexCoord,
        race: Race,
        unit_type: String,
        description: String,
        level: i32,
        experience: i32,
        evolution_previous: Option<UnitType>,
        evolution_next: Vec<UnitType>,
        combat_stats: CombatStats,
    ) -> Self {
        let mut base = Self::new(
            name,
            position,
            race,
            unit_type,
            description,
            evolution_previous,
            evolution_next,
            combat_stats,
        );
        base.level = level.max(1);
        base.experience = experience.max(0);
        base
    }

    /// Recalculates all derived stats based on base stats, equipment, and level.
    ///
    /// This method should be called after:
    /// - Equipping or unequipping items
    /// - Leveling up
    /// - Applying buffs or debuffs
    ///
    /// It updates:
    /// - `cached_max_health`: Base health + level bonuses + equipment bonuses
    /// - `cached_attack`: Base attack + level bonuses + equipment bonuses
    /// - `cached_movement`: Base movement + equipment bonuses
    /// - `cached_defense`: Sum of all resistances
    pub fn recalculate_stats(&mut self) {
        // Base stats from initial combat_stats
        let base_health = self.combat_stats.max_health;
        let base_attack = self.combat_stats.attack_strength;
        let base_movement = self.combat_stats.movement_speed;

        // Level bonuses (each level adds small bonuses)
        let level_health_bonus = (self.level - 1) * 5;
        let level_attack_bonus = (self.level - 1) / 2; // Every 2 levels

        // Equipment bonuses
        let equipment_attack = self.equipment.get_total_attack_bonus();
        let equipment_movement = self.equipment.get_total_movement_modifier();
        let equipment_health = self.equipment.get_total_health_bonus();

        // Calculate final stats
        self.cached_attack = (base_attack as i32) + level_attack_bonus + equipment_attack;
        self.cached_movement = (base_movement + equipment_movement).max(1);
        self.cached_max_health = base_health + level_health_bonus + equipment_health;

        // Update combat stats
        let current_health_percentage = self.combat_stats.health_percentage();
        self.combat_stats.base_attack = base_attack + (level_attack_bonus as u32);
        self.combat_stats.attack_modifier = equipment_attack;
        self.combat_stats.movement_speed = self.cached_movement;
        self.combat_stats.max_health = self.cached_max_health;

        // Maintain health percentage when max health changes
        self.combat_stats.health =
            (self.cached_max_health as f32 * current_health_percentage) as i32;

        // Update range from equipment if overridden
        if let Some(_range_override) = self.equipment.get_range_type_override() {
            // Range type override from equipment
            self.combat_stats.attack_range = self.combat_stats.range_category.base_range()
                + self.equipment.get_total_range_modifier();
        } else {
            // Keep current range category, just update range modifier from equipment
            self.combat_stats.attack_range = self.combat_stats.range_category.base_range()
                + self.equipment.get_total_range_modifier();
        }

        // Ensure minimum range of 1
        self.combat_stats.attack_range = self.combat_stats.attack_range.max(1);

        // Note: Terrain hit chance should be calculated when needed based on the unit's position
        // on the map, not stored as a cached value. The terrain is a property of the map tile,
        // not the unit itself.

        // Apply passive ability bonuses
        self.apply_passive_bonuses();
    }

    /// Calculate terrain hit chance for the given terrain.
    ///
    /// This should be called when determining combat effectiveness based on the terrain
    /// at the unit's current position on the map.
    ///
    /// # Arguments
    ///
    /// * `terrain` - The terrain type at the unit's position
    ///
    /// # Returns
    ///
    /// The hit chance percentage (0-100) for the given terrain
    pub fn get_terrain_hit_chance(&self, terrain: Terrain) -> u8 {
        if let Some(map) = &self.terrain_defenses {
            // If the unit provides a mapping for the given terrain, use it, otherwise fallback to race
            map.get(&terrain)
                .copied()
                .unwrap_or_else(|| self.race.get_terrain_hit_chance(terrain))
        } else {
            self.race.get_terrain_hit_chance(terrain)
        }
    }

    /// Get all hexagonal coordinates within movement range
    pub fn get_movement_range(&self) -> Vec<HexCoord> {
        let mut coords = Vec::new();
        let range = self.combat_stats.movement_speed;

        for q in -range..=range {
            for r in -range..=range {
                let coord = HexCoord::new(self.position.q + q, self.position.r + r);
                let distance = self.position.distance(coord);

                if distance > 0 && distance <= range {
                    coords.push(coord);
                }
            }
        }

        coords
    }

    /// Create a visual health bar
    pub fn create_health_bar(&self, current: i32, max: i32, width: usize) -> String {
        if max == 0 {
            return "░".repeat(width);
        }

        let filled = ((current as f32 / max as f32) * width as f32) as usize;
        let filled = filled.min(width);

        let bar_char = if current as f32 / max as f32 > 0.75 {
            "█"
        } else if current as f32 / max as f32 > 0.5 {
            "▓"
        } else if current as f32 / max as f32 > 0.25 {
            "▒"
        } else {
            "░"
        };

        let filled_part = bar_char.repeat(filled);
        let empty_part = "░".repeat(width - filled);

        format!("[{}{}]", filled_part, empty_part)
    }

    /// Apply new stats during level up, preserving equipment and inventory
    ///
    /// This method updates the unit's base combat stats and recalculates all cached values,
    /// but preserves the unit's equipment and inventory. It also optionally heals the unit.
    ///
    /// # Arguments
    ///
    /// * `new_stats` - The new base combat stats for the leveled-up unit
    /// * `heal_to_full` - Whether to restore the unit to full health after leveling up
    ///
    pub fn apply_level_up_stats(&mut self, new_stats: CombatStats, heal_to_full: bool) {
        // Store current health percentage before updating stats
        let current_health_percentage = if heal_to_full {
            1.0 // Will restore to 100%
        } else {
            self.combat_stats.health_percentage()
        };

        // Update base combat stats (this replaces the base stats with new level stats)
        self.combat_stats = new_stats;

        // Recalculate all cached values including equipment bonuses
        self.recalculate_stats();

        // Apply health based on option
        if heal_to_full {
            self.combat_stats.health = self.combat_stats.max_health;
        } else {
            // Maintain health percentage with new max health
            self.combat_stats.health =
                (self.combat_stats.max_health as f32 * current_health_percentage) as i32;
        }
    }

    /// Get the experience required for a specific level
    ///
    /// This uses a quadratic formula: level^2 * 50
    /// Level 1→2: 100 XP
    /// Level 2→3: 250 XP (total)
    /// Level 3→4: 450 XP (total)
    /// Level 4→5: 700 XP (total)
    ///
    /// # Arguments
    ///
    /// * `level` - The target level
    ///
    /// # Returns
    ///
    /// Total experience required to reach that level
    pub fn xp_required_for_level(level: i32) -> i32 {
        if level <= 1 {
            return 0;
        }
        // Quadratic progression: level^2 * 50
        level * level * 50
    }

    /// Add experience points
    ///
    /// # Arguments
    ///
    /// * `xp` - Amount of experience to add
    ///
    /// Note: The Unit trait's add_experience method handles level-up checking.
    /// This is just a simple XP addition helper.
    pub fn add_experience(&mut self, xp: i32) {
        self.experience += xp;
    }

    /// Perform a complete level-up with evolution to next unit type
    ///
    /// This method handles evolution to the next unit in the evolution chain:
    /// 1. Increments the level
    /// 2. Applies new combat stats
    /// 3. Replaces attacks with new level's attacks
    /// 4. Optionally heals to full HP
    /// 5. Updates unit type name
    ///
    /// # Arguments
    ///
    /// * `new_stats` - Combat stats for the new level
    /// * `new_attacks` - Attacks available at the new level
    /// * `new_unit_type` - New unit type name (e.g., "Orc Swordsman" → "Orc Elite Swordsman")
    /// * `heal_to_full` - Whether to restore unit to full health
    ///
    /// # Returns
    ///
    /// The new attacks vector (for updating the unit's attacks field)
    pub fn level_up_evolution(
        &mut self,
        new_stats: CombatStats,
        new_attacks: Vec<Attack>,
        new_unit_type: String,
        heal_to_full: bool,
    ) {
        // Increment level
        self.level += 1;

        // Update unit type name
        self.unit_type = new_unit_type;

        // Update attacks
        self.attacks = new_attacks;

        // Apply new stats (preserves equipment)
        self.apply_level_up_stats(new_stats, heal_to_full);
    }

    /// Perform incremental level-up for max-level units (no evolution)
    ///
    /// When a unit has no next evolution, it gains small incremental stat boosts:
    /// - +2 max HP
    /// - +1 attack
    ///
    /// # Arguments
    ///
    /// * `heal_to_full` - Whether to restore unit to full health
    ///
    pub fn level_up_incremental(&mut self, heal_to_full: bool) {
        // Increment level
        self.level += 1;

        // Small stat increases for max-level units
        self.combat_stats.max_health += 2;
        self.combat_stats.attack_strength += 1; // Increment attack_strength, not base_attack

        // Recalculate stats with equipment bonuses
        self.recalculate_stats();

        // Optionally heal to full
        if heal_to_full {
            self.combat_stats.health = self.combat_stats.max_health;
        } else {
            // Ensure health doesn't exceed new max
            self.combat_stats.health = self.combat_stats.health.min(self.combat_stats.max_health);
        }

        // Attacks stay the same for incremental level-up
    }

    // ===== Attack Creation Helpers =====

    /// Creates a melee attack with the specified parameters.
    ///
    /// This is a convenience method that units can use in their attack definitions
    /// to maintain consistency with the attack creation pattern from the old macro system.
    ///
    /// # Arguments
    ///
    /// * `name` - Display name of the attack (e.g., "Sword Slash")
    /// * `damage` - Base damage dealt by this attack
    /// * `attack_times` - Number of strikes (currently unused but kept for compatibility)
    /// * `damage_type` - Type of damage dealt (affects enemy resistances)
    ///
    /// # Returns
    ///
    /// A new melee `Attack` with range 1
    pub fn create_melee_attack(
        name: impl Into<String>,
        damage: u32,
        attack_times: u32,
        damage_type: DamageType,
    ) -> Attack {
        Attack::melee(name, damage, attack_times, damage_type)
    }

    /// Creates a ranged attack with the specified parameters.
    ///
    /// This is a convenience method that units can use in their attack definitions
    /// to maintain consistency with the attack creation pattern from the old macro system.
    ///
    /// # Arguments
    ///
    /// * `name` - Display name of the attack (e.g., "Bow Shot")
    /// * `damage` - Base damage dealt by this attack
    /// * `attack_times` - Number of shots (currently unused but kept for compatibility)
    /// * `damage_type` - Type of damage dealt (affects enemy resistances)
    /// * `range` - Maximum range in hexes (minimum 1)
    ///
    /// # Returns
    ///
    /// A new ranged `Attack` with the specified range
    pub fn create_ranged_attack(
        name: impl Into<String>,
        damage: u32,
        attack_times: u32,
        damage_type: DamageType,
        range: i32,
    ) -> Attack {
        Attack::ranged(name, damage, attack_times, damage_type, range)
    }

    // ===== Universal Attack Management Methods =====

    /// Add a new attack to the attacks vector.
    /// This is a helper for units that store their own attacks.
    pub fn add_attack_to_vec(attacks: &mut Vec<Attack>, attack: Attack) {
        attacks.push(attack);
    }

    /// Remove an attack by name from the attacks vector.
    /// Returns true if an attack was removed.
    pub fn remove_attack_from_vec(attacks: &mut Vec<Attack>, name: &str) -> bool {
        if let Some(pos) = attacks.iter().position(|a| a.name == name) {
            attacks.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get all attack names from the attacks vector.
    pub fn get_attack_names_from_vec(attacks: &[Attack]) -> Vec<&str> {
        attacks.iter().map(|a| a.name.as_str()).collect()
    }

    // ===== Ability Management Methods =====

    /// Add an ability to the unit
    pub fn add_ability(&mut self, ability: crate::ability::Ability) {
        self.abilities.push(ability);
    }

    /// Remove an ability by ID
    pub fn remove_ability(&mut self, ability_id: crate::ability::AbilityId) -> bool {
        if let Some(pos) = self.abilities.iter().position(|a| a.id() == ability_id) {
            self.abilities.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get all abilities
    pub fn get_abilities(&self) -> &[crate::ability::Ability] {
        &self.abilities
    }

    /// Get mutable reference to abilities
    pub fn get_abilities_mut(&mut self) -> &mut Vec<crate::ability::Ability> {
        &mut self.abilities
    }

    /// Find an ability by ID
    pub fn find_ability(
        &self,
        ability_id: crate::ability::AbilityId,
    ) -> Option<&crate::ability::Ability> {
        self.abilities.iter().find(|a| a.id() == ability_id)
    }

    /// Find a mutable ability by ID
    pub fn find_ability_mut(
        &mut self,
        ability_id: crate::ability::AbilityId,
    ) -> Option<&mut crate::ability::Ability> {
        self.abilities.iter_mut().find(|a| a.id() == ability_id)
    }

    /// Tick ability state (reduce cooldowns and durations)
    pub fn tick_abilities(&mut self) {
        self.ability_state.tick();
    }

    /// Apply passive ability bonuses to stats
    ///
    /// This should be called during `recalculate_stats()` to include
    /// passive ability bonuses in the final cached stats.
    pub fn apply_passive_bonuses(&mut self) {
        use crate::ability::{PassiveEffect, PassiveTrigger};

        for ability in &self.abilities {
            if let crate::ability::Ability::Passive(passive) = ability {
                // Only apply "Always" passive effects to stats
                if passive.trigger == PassiveTrigger::Always {
                    match &passive.effect {
                        PassiveEffect::AttackBonus(bonus) => {
                            self.cached_attack += bonus;
                        }
                        PassiveEffect::AttackBonusPercent(percent) => {
                            let bonus = (self.cached_attack * (*percent as i32)) / 100;
                            self.cached_attack += bonus;
                        }
                        PassiveEffect::DefenseBonus(bonus) => {
                            self.cached_defense += bonus;
                        }
                        PassiveEffect::DefenseBonusPercent(percent) => {
                            let bonus = (self.cached_defense * (*percent as i32)) / 100;
                            self.cached_defense += bonus;
                        }
                        PassiveEffect::HealthBonus(bonus) => {
                            self.cached_max_health += bonus;
                        }
                        PassiveEffect::HealthBonusPercent(percent) => {
                            let bonus = (self.cached_max_health * (*percent as i32)) / 100;
                            self.cached_max_health += bonus;
                        }
                        PassiveEffect::MovementBonus(bonus) => {
                            self.cached_movement += bonus;
                        }
                        _ => {
                            // Other passive effects are applied when their trigger conditions are met
                        }
                    }
                }
            }
        }
    }
}
