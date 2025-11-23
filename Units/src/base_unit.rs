//! Base unit implementation and shared data structures.
//!
//! This module provides [`BaseUnit`], which contains the common data and functionality
//! shared by all concrete unit implementations. It handles stat caching, equipment
//! bonuses, and level progression.

use crate::attack::Attack;
use crate::unit_race::{Race, Terrain};
use combat::CombatStats;
use graphics::HexCoord;
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
/// - `current_terrain`: The terrain type at the unit's position
///
/// # Examples
///
/// ```rust,no_run
/// use units::{BaseUnit, Race, Terrain};
/// use combat::{CombatStats, RangeCategory, Resistances};
/// use graphics::HexCoord;
///
/// let stats = CombatStats::new(100, 10, 5, RangeCategory::Melee, Resistances::default());
/// let unit = BaseUnit::new(
///     "Warrior".to_string(),
///     HexCoord::new(0, 0),
///     Race::Human,
///     "Human Warrior".to_string(),
///     Terrain::Grasslands,
///     stats,
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseUnit {
    // Identity
    pub id: UnitId,
    pub name: String,
    pub position: HexCoord,
    pub race: Race,
    pub unit_type: String, // e.g., "Human Warrior", "Orc Grunt", etc.

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

    // Environment
    pub current_terrain: Terrain,
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
    /// * `terrain` - The terrain at the starting position
    /// * `combat_stats` - Base combat statistics
    ///
    /// # Returns
    ///
    /// A new `BaseUnit` instance at level 1 with 0 experience.
    pub fn new(
        name: String,
        position: HexCoord,
        race: Race,
        unit_type: String,
        terrain: Terrain,
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
            level: 1,
            experience: 0,
            combat_stats,
            equipment: Equipment::default(),
            inventory: Vec::new(),
            cached_defense: 0, // Will be calculated from resistances
            cached_attack: base_attack,
            cached_movement: base_movement,
            cached_max_health: max_health,
            current_terrain: terrain,
            terrain_defenses: None,
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
    /// * `terrain` - The terrain at the starting position
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
        level: i32,
        experience: i32,
        terrain: Terrain,
        combat_stats: CombatStats,
    ) -> Self {
        let mut base = Self::new(name, position, race, unit_type, terrain, combat_stats);
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

        // Update terrain hit chance based on per-unit mapping (if any) or race and current terrain
        let hit_chance = if let Some(map) = &self.terrain_defenses {
            // If the unit provides a mapping for the current terrain, use it, otherwise fallback to race
            map.get(&self.current_terrain)
                .copied()
                .unwrap_or_else(|| self.race.get_terrain_hit_chance(self.current_terrain))
        } else {
            self.race.get_terrain_hit_chance(self.current_terrain)
        };

        self.combat_stats.set_terrain_hit_chance(hit_chance);
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
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use units::BaseUnit;
    /// # use combat::{CombatStats, RangeCategory, Resistances};
    /// # use graphics::HexCoord;
    /// # use units::{Race, Terrain};
    /// # let initial_stats = CombatStats::new(100, 10, 4, RangeCategory::Melee, Resistances::new(10, 10, 10, 10, 10, 10));
    /// # let mut unit = BaseUnit::new("Test".into(), HexCoord::new(0,0), Race::Human, "Warrior".into(), Terrain::Grasslands, initial_stats);
    /// let new_stats = CombatStats::new(150, 15, 4, RangeCategory::Melee, Resistances::new(15, 15, 15, 15, 15, 15));
    /// unit.apply_level_up_stats(new_stats, true); // Level up and heal to full
    /// ```
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

    /// Get XP needed to reach next level from current level
    pub fn xp_to_next_level(&self) -> i32 {
        Self::xp_required_for_level(self.level + 1)
    }

    /// Get remaining XP needed to level up
    pub fn xp_remaining_for_level_up(&self) -> i32 {
        (self.xp_to_next_level() - self.experience).max(0)
    }

    /// Check if unit has enough XP to level up
    pub fn can_level_up(&self) -> bool {
        self.experience >= self.xp_to_next_level()
    }

    /// Add experience and check if leveled up
    ///
    /// # Arguments
    ///
    /// * `xp` - Amount of experience to add
    ///
    /// # Returns
    ///
    /// Returns `true` if the unit now has enough XP to level up, `false` otherwise.
    /// Note: This does NOT automatically level up the unit - it only adds XP and checks
    /// the threshold. The caller must handle the actual level-up process.
    pub fn add_experience(&mut self, xp: i32) -> bool {
        self.experience += xp;
        self.can_level_up()
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
    ) -> Vec<Attack> {
        // Increment level
        self.level += 1;

        // Update unit type name
        self.unit_type = new_unit_type;

        // Apply new stats (preserves equipment)
        self.apply_level_up_stats(new_stats, heal_to_full);

        // Return new attacks for caller to update their attacks field
        new_attacks
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
    /// # Returns
    ///
    /// Empty vector (attacks unchanged for incremental level-ups)
    pub fn level_up_incremental(&mut self, heal_to_full: bool) -> Vec<Attack> {
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

        // Return empty vector - attacks stay the same
        vec![]
    }
}
