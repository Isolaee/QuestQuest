use items::RangeType;
use serde::{Deserialize, Serialize};

/// Combat statistics for a unit
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatStats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub movement_speed: i32,
    pub range_type: RangeType,
    pub attack_range: i32,
}

impl CombatStats {
    /// Create new combat stats
    pub fn new(
        max_health: i32,
        attack: i32,
        defense: i32,
        movement_speed: i32,
        range_type: RangeType,
    ) -> Self {
        Self {
            health: max_health,
            max_health,
            attack,
            defense,
            movement_speed,
            range_type,
            attack_range: range_type.base_range(),
        }
    }

    /// Check if the unit is alive
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Take damage and return true if unit dies
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health = (self.health - damage.max(0)).max(0);
        !self.is_alive()
    }

    /// Heal the unit
    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount.max(0)).min(self.max_health);
    }

    /// Get health percentage (0.0 to 1.0)
    pub fn health_percentage(&self) -> f32 {
        if self.max_health > 0 {
            self.health as f32 / self.max_health as f32
        } else {
            0.0
        }
    }

    /// Calculate damage dealt to another unit
    pub fn calculate_damage(&self, target: &CombatStats) -> i32 {
        let base_damage = self.attack;
        // Always at least 1 damage
        (base_damage - target.defense).max(1)
    }
}
