//! Combat types and utilities used by units and combat resolver.
//!
//! This module centralizes damage types, resistances, range categories,
//! combat statistics, and combat result/action types. It was previously
//! defined in the separate `combat` crate; for workspace cohesion these
//! types live here so other crates (like `combat`) can depend on `units`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Blunt,
    Pierce,
    Fire,
    Dark,
    Slash,
    Crush,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeCategory {
    Melee,
    Range,
    Siege,
}

impl RangeCategory {
    pub fn base_range(&self) -> i32 {
        match self {
            RangeCategory::Melee => 1,
            RangeCategory::Range => 3,
            RangeCategory::Siege => 5,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resistances {
    pub blunt: u8,
    pub pierce: u8,
    pub fire: u8,
    pub dark: u8,
    pub slash: u8,
    pub crush: u8,
}

impl Resistances {
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
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatStats {
    pub health: i32,
    pub max_health: i32,
    pub base_attack: u32,
    pub attack_modifier: i32,
    pub attack_strength: u32,
    pub attacks_per_round: u32,
    pub movement_speed: i32,
    pub range_category: RangeCategory,
    pub attack_range: i32,
    pub resistances: Resistances,
    pub terrain_hit_chance: u8,
    pub attacked_this_turn: bool,
}

impl CombatStats {
    pub fn new(
        max_health: i32,
        base_attack: u32,
        movement_speed: i32,
        range_category: RangeCategory,
        resistances: Resistances,
    ) -> Self {
        Self::new_with_attacks(
            max_health,
            base_attack,
            movement_speed,
            range_category,
            resistances,
            base_attack,
            1,
        )
    }

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
            attacks_per_round: attacks_per_round.max(1),
            movement_speed,
            range_category,
            attack_range: range_category.base_range(),
            resistances,
            terrain_hit_chance: 75,
            attacked_this_turn: false,
        }
    }

    pub fn reset_turn_flags(&mut self) {
        self.attacked_this_turn = false;
    }

    pub fn get_total_attack(&self) -> u32 {
        (self.base_attack as i32 + self.attack_modifier).max(0) as u32
    }

    pub fn set_terrain_hit_chance(&mut self, chance: u8) {
        self.terrain_hit_chance = chance.min(100);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

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

    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health = (self.health - damage.max(0)).max(0);
        !self.is_alive()
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount.max(0)).min(self.max_health);
    }

    pub fn health_percentage(&self) -> f32 {
        if self.max_health > 0 {
            self.health as f32 / self.max_health as f32
        } else {
            0.0
        }
    }

    pub fn calculate_damage_to(&self, target: &CombatStats, damage_type: DamageType) -> u32 {
        let base_damage = self.get_total_attack();
        let resistance = target.resistances.get_resistance(damage_type);
        let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
        let final_damage = (base_damage as f32 * resistance_multiplier) as u32;
        final_damage.max(1)
    }

    pub fn calculate_total_round_damage(
        &self,
        target: &CombatStats,
        damage_type: DamageType,
    ) -> u32 {
        let resistance = target.resistances.get_resistance(damage_type);
        let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);

        let modified_strength = (self.attack_strength as i32 + self.attack_modifier).max(0) as u32;
        let total_damage = modified_strength * self.attacks_per_round;
        let final_damage = (total_damage as f32 * resistance_multiplier) as u32;
        final_damage.max(1)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CombatResult {
    pub attacker_damage_dealt: u32,
    pub defender_damage_dealt: u32,
    pub attacker_hit: bool,
    pub defender_hit: bool,
    pub attacker_casualties: u32,
    pub defender_casualties: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CombatAction {
    Attack { damage: i32 },
    Heal { amount: i32 },
    Defend,
    Skip,
}

impl CombatAction {
    pub fn get_name(&self) -> &'static str {
        match self {
            CombatAction::Attack { .. } => "Attack",
            CombatAction::Heal { .. } => "Heal",
            CombatAction::Defend => "Defend",
            CombatAction::Skip => "Skip",
        }
    }
}
