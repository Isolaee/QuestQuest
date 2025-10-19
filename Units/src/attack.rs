use combat::DamageType;
use serde::{Deserialize, Serialize};

/// Represents a specific attack that a unit can perform
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attack {
    /// Name of the attack (e.g., "Sword Slash", "Axe Cleave", "Fire Arrow")
    pub name: String,
    /// Damage this attack deals
    pub damage: u32,
    /// Type of damage dealt
    pub damage_type: DamageType,
    /// Range of this attack (in hexes, 1 = melee)
    pub range: i32,
    /// Description of the attack
    pub description: String,
}

impl Attack {
    /// Create a new attack
    pub fn new(
        name: impl Into<String>,
        damage: u32,
        damage_type: DamageType,
        range: i32,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            damage,
            damage_type,
            range: range.max(1), // At least melee range
            description: description.into(),
        }
    }

    /// Create a basic melee attack
    pub fn melee(
        name: impl Into<String>,
        damage: u32,
        _attack_times: u32,
        damage_type: DamageType,
    ) -> Self {
        Self::new(name, damage, damage_type, 1, "A basic melee attack")
    }

    /// Create a basic ranged attack
    pub fn ranged(
        name: impl Into<String>,
        damage: u32,
        _attack_times: u32,
        damage_type: DamageType,
        range: i32,
    ) -> Self {
        Self::new(name, damage, damage_type, range, "A ranged attack")
    }

    pub fn siege(
        name: impl Into<String>,
        damage: u32,
        _attack_times: u32,
        damage_type: DamageType,
        range: i32,
    ) -> Self {
        Self::new(name, damage, damage_type, range, "A siege attack")
    }

    /// Check if this attack can reach the target at given distance
    pub fn can_reach(&self, distance: i32) -> bool {
        distance > 0 && distance <= self.range
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_melee_attack() {
        let attack = Attack::melee("Sword Strike", 15, 1, DamageType::Slash);
        assert_eq!(attack.name, "Sword Strike");
        assert_eq!(attack.damage, 15);
        assert_eq!(attack.damage_type, DamageType::Slash);
        assert_eq!(attack.range, 1);
    }

    #[test]
    fn test_create_ranged_attack() {
        let attack = Attack::ranged("Bow Shot", 12, 2, DamageType::Pierce, 3);
        assert_eq!(attack.name, "Bow Shot");
        assert_eq!(attack.damage, 12);
        assert_eq!(attack.damage_type, DamageType::Pierce);
        assert_eq!(attack.range, 3);
    }

    #[test]
    fn test_attack_range() {
        let melee = Attack::melee("Punch", 5, 1, DamageType::Blunt);
        assert!(melee.can_reach(1));
        assert!(!melee.can_reach(2));

        let ranged = Attack::ranged("Arrow", 10, 2, DamageType::Pierce, 3);
        assert!(ranged.can_reach(1));
        assert!(ranged.can_reach(3));
        assert!(!ranged.can_reach(4));
    }
}
