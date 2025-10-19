use serde::{Deserialize, Serialize};

/// Damage types for combat calculations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Blunt,
    Pierce,
    Fire,
    Dark,
    Slash,
    Crush,
}

/// Range category for attacks
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

/// Resistance values for different damage types (as % multipliers, 0-100)
/// 0 = no resistance, 100 = full immunity
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

/// Combat statistics for a unit
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatStats {
    pub health: i32,
    pub max_health: i32,
    /// Base attack damage (unsigned)
    pub base_attack: u32,
    /// Additional attack damage from equipment/buffs
    pub attack_modifier: i32,
    /// Movement speed
    pub movement_speed: i32,
    /// Range category
    pub range_category: RangeCategory,
    /// Actual attack range
    pub attack_range: i32,
    /// Resistances to different damage types (as % multipliers)
    pub resistances: Resistances,
    /// Hit chance percentage based on current terrain (0-100)
    pub terrain_hit_chance: u8,
}

impl CombatStats {
    /// Create new combat stats
    pub fn new(
        max_health: i32,
        base_attack: u32,
        movement_speed: i32,
        range_category: RangeCategory,
        resistances: Resistances,
    ) -> Self {
        Self {
            health: max_health,
            max_health,
            base_attack,
            attack_modifier: 0,
            movement_speed,
            range_category,
            attack_range: range_category.base_range(),
            resistances,
            terrain_hit_chance: 75, // Default 75% hit chance
        }
    }

    /// Get total attack damage (base + modifiers)
    pub fn get_total_attack(&self) -> u32 {
        (self.base_attack as i32 + self.attack_modifier).max(0) as u32
    }

    /// Set terrain-based hit chance (0-100%)
    pub fn set_terrain_hit_chance(&mut self, chance: u8) {
        self.terrain_hit_chance = chance.min(100);
    }

    /// Check if the unit is alive
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Take damage with resistance calculation, returns actual damage taken
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

    /// Take raw damage without resistance
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

    /// Calculate damage dealt considering target's resistance
    pub fn calculate_damage_to(&self, target: &CombatStats, damage_type: DamageType) -> u32 {
        let base_damage = self.get_total_attack();
        let resistance = target.resistances.get_resistance(damage_type);
        let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
        let final_damage = (base_damage as f32 * resistance_multiplier) as u32;
        final_damage.max(1) // Always at least 1 damage
    }
}
