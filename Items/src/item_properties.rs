use serde::{Deserialize, Serialize};

/// Type of damage dealt by an attack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Slash,
    Pierce,
    Blunt,
    Fire,
    Ice,
    Lightning,
    Poison,
    Holy,
    Dark,
}

/// Represents a specific attack that an item provides.
///
/// Used for weapon attacks which may contain multiple `ItemAttack` entries
/// (for weapons that strike multiple times or have special effects).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemAttack {
    /// Name of the attack (e.g., "Slash", "Stab")
    pub name: String,
    /// Damage this attack deals
    pub damage: u32,
    /// Type of damage dealt
    pub damage_type: DamageType,
    /// Number of times this attack hits per round
    pub attack_times: u32,
}

impl ItemAttack {
    /// Create a new item attack.
    pub fn new(
        name: impl Into<String>,
        damage: u32,
        attack_times: u32,
        damage_type: DamageType,
    ) -> Self {
        Self {
            name: name.into(),
            damage,
            damage_type,
            attack_times: attack_times.max(1), // At least 1 attack
        }
    }
}

/// Range type for attacks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeType {
    Melee,
    Ranged,
    Siege,
}

impl RangeType {
    /// Get the base attack range for this range type (in hexes).
    pub fn base_range(&self) -> i32 {
        match self {
            RangeType::Melee => 1,  // 1 hex away
            RangeType::Ranged => 3, // 3 hexes away
            RangeType::Siege => 6,  // 6 hexes away
        }
    }

    /// Get the name of this range type as a string.
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

/// Properties that an item can have.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemProperties {
    /// Weapon with combat-related properties
    Weapon {
        attack_bonus: i32,
        range_modifier: i32,
        range_type_override: Option<RangeType>,
        attacks: Vec<ItemAttack>,
    },
    /// Armor providing defense and potentially movement penalty
    Armor {
        defense_bonus: i32,
        movement_penalty: i32,
    },
    /// Accessories providing small bonuses
    Accessory {
        health_bonus: i32,
        attack_bonus: i32,
        defense_bonus: i32,
        movement_bonus: i32,
    },
    /// Consumable items with limited uses and effects
    Consumable { uses: i32, effect: ConsumableEffect },
}

/// Effects that consumable items can have.
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
