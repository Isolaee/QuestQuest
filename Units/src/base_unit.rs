use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use combat::CombatStats;
use graphics::HexCoord;
use items::{Equipment, Item};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::unit_trait::UnitId;

/// Base unit structure containing common data for all unit types
/// This holds the shared state that all concrete unit implementations use
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseUnit {
    // Identity
    pub id: UnitId,
    pub name: String,
    pub position: HexCoord,
    pub race: Race,
    pub class: UnitClass,

    // Progression
    pub experience: i32,
    pub level: i32,

    // Combat and stats
    pub combat_stats: CombatStats,
    pub equipment: Equipment,
    pub inventory: Vec<Item>,

    // Cached values (recalculated when equipment/level changes)
    pub cached_defense: i32,
    pub cached_attack: i32,
    pub cached_movement: i32,
    pub cached_max_health: i32,

    // Environment
    pub current_terrain: Terrain,
}

impl BaseUnit {
    /// Create a new base unit with default values
    pub fn new(
        name: String,
        position: HexCoord,
        race: Race,
        class: UnitClass,
        terrain: Terrain,
    ) -> Self {
        let base_health = class.get_base_health();
        let base_attack = class.get_attack_bonus() + race.get_attack_bonus();
        let base_defense = class.get_defense_bonus();
        let base_movement = class.get_movement_speed() + race.get_movement_bonus();
        let range_type = class.get_default_range();

        let combat_stats = CombatStats::new(
            base_health,
            base_attack,
            base_defense,
            base_movement,
            range_type,
        );

        Self {
            id: Uuid::new_v4(),
            name,
            position,
            race,
            class,
            experience: 0,
            level: 1,
            combat_stats,
            equipment: Equipment::new(),
            inventory: Vec::new(),
            cached_defense: base_defense,
            cached_attack: base_attack,
            cached_movement: base_movement,
            cached_max_health: base_health,
            current_terrain: terrain,
        }
    }

    /// Create a base unit with specific level and experience
    pub fn new_with_level(
        name: String,
        position: HexCoord,
        race: Race,
        class: UnitClass,
        level: i32,
        experience: i32,
        terrain: Terrain,
    ) -> Self {
        let mut base = Self::new(name, position, race, class, terrain);
        base.level = level.max(1);
        base.experience = experience.max(0);
        base
    }

    /// Recalculate all derived stats based on base stats, equipment, and level
    pub fn recalculate_stats(&mut self) {
        // Base stats from race and class
        let base_health = self.class.get_base_health();
        let base_attack = self.class.get_attack_bonus() + self.race.get_attack_bonus();
        let base_defense = self.class.get_defense_bonus();
        let base_movement = self.class.get_movement_speed() + self.race.get_movement_bonus();

        // Level bonuses (each level adds small bonuses)
        let level_health_bonus = (self.level - 1) * 5;
        let level_attack_bonus = (self.level - 1) / 2; // Every 2 levels
        let level_defense_bonus = (self.level - 1) / 3; // Every 3 levels

        // Equipment bonuses
        let equipment_attack = self.equipment.get_total_attack_bonus();
        let equipment_defense = self.equipment.get_total_defense_bonus();
        let equipment_movement = self.equipment.get_total_movement_modifier();
        let equipment_health = self.equipment.get_total_health_bonus();

        // Calculate final stats
        self.cached_attack = base_attack + level_attack_bonus + equipment_attack;
        self.cached_defense = base_defense + level_defense_bonus + equipment_defense;
        self.cached_movement = (base_movement + equipment_movement).max(1);
        self.cached_max_health = base_health + level_health_bonus + equipment_health;

        // Update combat stats
        let current_health_percentage = self.combat_stats.health_percentage();
        self.combat_stats.attack = self.cached_attack;
        self.combat_stats.defense = self.cached_defense;
        self.combat_stats.movement_speed = self.cached_movement;
        self.combat_stats.max_health = self.cached_max_health;

        // Maintain health percentage when max health changes
        self.combat_stats.health =
            (self.cached_max_health as f32 * current_health_percentage) as i32;

        // Update range type from equipment if overridden
        if let Some(range_override) = self.equipment.get_range_type_override() {
            self.combat_stats.range_type = range_override;
            self.combat_stats.attack_range =
                range_override.base_range() + self.equipment.get_total_range_modifier();
        } else {
            let default_range = self.class.get_default_range();
            self.combat_stats.range_type = default_range;
            self.combat_stats.attack_range =
                default_range.base_range() + self.equipment.get_total_range_modifier();
        }

        // Ensure minimum range of 1
        self.combat_stats.attack_range = self.combat_stats.attack_range.max(1);
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
}
