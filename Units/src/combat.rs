use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the range type of a unit's attacks
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RangeType {
    Melee,  // Adjacent hexes only
    Ranged, // Multiple hex range
    Siege,  // Long range, area effect
}

impl RangeType {
    /// Get the maximum attack range in hexes
    pub fn get_range_distance(self) -> i32 {
        match self {
            RangeType::Melee => 1,  // 1 hex away
            RangeType::Ranged => 3, // 3 hexes away
            RangeType::Siege => 6,  // 6 hexes away
        }
    }

    /// Get the display name
    pub fn get_name(self) -> &'static str {
        match self {
            RangeType::Melee => "Melee",
            RangeType::Ranged => "Ranged",
            RangeType::Siege => "Siege",
        }
    }
}

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
            attack_range: range_type.get_range_distance(),
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

/// Combat actions that a unit can perform
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CombatAction {
    Attack { damage: i32 },
    Heal { amount: i32 },
    Defend, // Increases defense for one turn
    Skip,   // Do nothing
}

impl CombatAction {
    /// Get the display name of the action
    pub fn get_name(&self) -> &'static str {
        match self {
            CombatAction::Attack { .. } => "Attack",
            CombatAction::Heal { .. } => "Heal",
            CombatAction::Defend => "Defend",
            CombatAction::Skip => "Skip",
        }
    }
}

impl fmt::Display for RangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RangeType::Melee => write!(f, "Melee"),
            RangeType::Ranged => write!(f, "Ranged"),
            RangeType::Siege => write!(f, "Siege"),
        }
    }
}
