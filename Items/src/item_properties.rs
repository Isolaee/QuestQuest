use serde::{Deserialize, Serialize};

/// Range type for attacks
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeType {
    Melee,
    Ranged,
    Siege,
}

impl RangeType {
    /// Get the base attack range for this range type (in hexes)
    pub fn base_range(&self) -> i32 {
        match self {
            RangeType::Melee => 1,  // 1 hex away
            RangeType::Ranged => 3, // 3 hexes away
            RangeType::Siege => 6,  // 6 hexes away
        }
    }

    /// Get the name of this range type as a string
    pub fn name(&self) -> &'static str {
        match self {
            RangeType::Melee => "Melee",
            RangeType::Ranged => "Ranged",
            RangeType::Siege => "Siege",
        }
    }
}

impl std::fmt::Display for RangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeType::Melee => write!(f, "Melee"),
            RangeType::Ranged => write!(f, "Ranged"),
            RangeType::Siege => write!(f, "Siege"),
        }
    }
}

/// Properties that an item can have
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemProperties {
    Weapon {
        attack_bonus: i32,
        range_modifier: i32,
        range_type_override: Option<RangeType>,
    },
    Armor {
        defense_bonus: i32,
        movement_penalty: i32,
    },
    Accessory {
        health_bonus: i32,
        attack_bonus: i32,
        defense_bonus: i32,
        movement_bonus: i32,
    },
    Consumable {
        uses: i32,
        effect: ConsumableEffect,
    },
}

/// Effects that consumable items can have
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConsumableEffect {
    Heal {
        amount: i32,
    },
    Buff {
        attack_bonus: i32,
        defense_bonus: i32,
        duration: i32,
    },
    Restore {
        health: i32,
    },
}
