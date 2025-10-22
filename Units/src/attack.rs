//! Attack definitions and combat actions.
//!
//! This module provides the [`Attack`] structure that represents specific
//! combat actions units can perform, including damage, range, and damage type.

use combat::DamageType;
use serde::{Deserialize, Serialize};

/// Represents a specific attack that a unit can perform.
///
/// Attacks have a name, damage value, damage type, and range. Units can have
/// multiple attacks (e.g., basic sword attack, power strike, shield bash).
///
/// # Examples
///
/// ```rust
/// use units::Attack;
/// use combat::DamageType;
///
/// // Create a melee sword attack
/// let sword_attack = Attack::melee("Sword Strike", 15, 1, DamageType::Slash);
/// assert_eq!(sword_attack.range, 1); // Melee range
///
/// // Create a ranged bow attack
/// let bow_attack = Attack::ranged("Bow Shot", 12, 2, DamageType::Pierce, 5);
/// assert_eq!(bow_attack.range, 5); // Can reach 5 hexes away
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attack {
    /// Name of the attack (e.g., "Sword Slash", "Axe Cleave", "Fire Arrow")
    pub name: String,
    /// Damage this attack deals
    pub damage: u32,
    /// Type of damage dealt (affects resistances)
    pub damage_type: DamageType,
    /// Range of this attack in hexes (1 = melee only)
    pub range: i32,
    /// Description of the attack for tooltips and logs
    pub description: String,
}

impl Attack {
    /// Creates a new attack with custom parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - The attack's display name
    /// * `damage` - Base damage dealt
    /// * `damage_type` - Type of damage (affects enemy resistances)
    /// * `range` - Maximum range in hexes (minimum 1 for melee)
    /// * `description` - Flavor text or mechanical description
    ///
    /// # Returns
    ///
    /// A new `Attack` instance. Range is clamped to a minimum of 1.
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

    /// Creates a basic melee attack with range 1.
    ///
    /// # Arguments
    ///
    /// * `name` - The attack's display name
    /// * `damage` - Base damage dealt
    /// * `_attack_times` - (Currently unused) Number of strikes
    /// * `damage_type` - Type of damage dealt
    ///
    /// # Returns
    ///
    /// A melee attack with range 1 and a generic description.
    pub fn melee(
        name: impl Into<String>,
        damage: u32,
        _attack_times: u32,
        damage_type: DamageType,
    ) -> Self {
        Self::new(name, damage, damage_type, 1, "A basic melee attack")
    }

    /// Creates a basic ranged attack with specified range.
    ///
    /// # Arguments
    ///
    /// * `name` - The attack's display name
    /// * `damage` - Base damage dealt
    /// * `_attack_times` - (Currently unused) Number of shots
    /// * `damage_type` - Type of damage dealt
    /// * `range` - Maximum range in hexes
    ///
    /// # Returns
    ///
    /// A ranged attack with the specified range and a generic description.
    pub fn ranged(
        name: impl Into<String>,
        damage: u32,
        _attack_times: u32,
        damage_type: DamageType,
        range: i32,
    ) -> Self {
        Self::new(name, damage, damage_type, range, "A ranged attack")
    }

    /// Creates a siege attack for attacking structures or fortifications.
    ///
    /// # Arguments
    ///
    /// * `name` - The attack's display name
    /// * `damage` - Base damage dealt
    /// * `_attack_times` - (Currently unused) Number of volleys
    /// * `damage_type` - Type of damage dealt
    /// * `range` - Maximum range in hexes
    ///
    /// # Returns
    ///
    /// A siege attack with the specified range.
    pub fn siege(
        name: impl Into<String>,
        damage: u32,
        _attack_times: u32,
        damage_type: DamageType,
        range: i32,
    ) -> Self {
        Self::new(name, damage, damage_type, range, "A siege attack")
    }

    /// Checks if this attack can reach a target at the given distance.
    ///
    /// # Arguments
    ///
    /// * `distance` - Distance to the target in hexes
    ///
    /// # Returns
    ///
    /// `true` if the target is within range (distance > 0 and <= range), `false` otherwise.
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
