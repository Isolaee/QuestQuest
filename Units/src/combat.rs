use crate::unit::Unit;
use items::RangeType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct CombatResult {
    pub attacker_damage_dealt: u32,
    pub defender_damage_dealt: u32,
    pub attacker_hit: bool,
    pub defender_hit: bool,
    pub attacker_casualties: u32,
    pub defender_casualties: u32,
}

/// Calculate if an attack hits based on defender's terrain-based defense
fn calculate_hit(attacker: &Unit, defender: &Unit) -> bool {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};

    // Simple deterministic "random" based on unit stats
    let state = RandomState::new();
    let mut hasher = state.build_hasher();
    attacker.id.hash(&mut hasher);
    defender.id.hash(&mut hasher);
    attacker.get_attack_power().hash(&mut hasher);
    let hash = hasher.finish();

    // Get defender's hit chance % based on their race and current terrain
    let base_hit_chance = defender.get_defense();

    // Apply attacker's attack bonus (each point reduces hit chance by 2%)
    let attack_modifier = attacker.race.get_attack_bonus();

    // Calculate final hit chance (clamped between 10% and 95%)
    let final_hit_chance = (base_hit_chance as i32 - attack_modifier * 2).clamp(10, 95) as u8;

    // Pseudo-random roll 1-100
    let roll = ((hash % 100) + 1) as u8;

    // Hit if roll is <= hit chance
    roll <= final_hit_chance
}
/// Perform combat between two units
pub fn resolve_combat(attacker: &mut Unit, defender: &mut Unit) -> CombatResult {
    let mut result = CombatResult {
        attacker_damage_dealt: 0,
        defender_damage_dealt: 0,
        attacker_hit: false,
        defender_hit: false,
        attacker_casualties: 0,
        defender_casualties: 0,
    };

    // Attacker attacks defender
    result.attacker_hit = calculate_hit(attacker, defender);
    if result.attacker_hit {
        let damage = attacker.get_attack_power();
        result.attacker_damage_dealt = damage;
        defender.take_damage(damage);
    }

    // Defender counterattacks (if still alive)
    if defender.is_alive() {
        result.defender_hit = calculate_hit(defender, attacker);
        if result.defender_hit {
            let damage = defender.get_attack_power();
            result.defender_damage_dealt = damage;
            attacker.take_damage(damage);
        }
    }

    // Calculate casualties based on damage
    result.attacker_casualties = if attacker.is_alive() { 0 } else { 1 };
    result.defender_casualties = if defender.is_alive() { 0 } else { 1 };

    result
}

/// Resolve combat between unit stacks (groups of units)
pub fn resolve_stack_combat(attackers: &mut Vec<Unit>, defenders: &mut Vec<Unit>) -> CombatResult {
    let mut total_result = CombatResult {
        attacker_damage_dealt: 0,
        defender_damage_dealt: 0,
        attacker_hit: false,
        defender_hit: false,
        attacker_casualties: 0,
        defender_casualties: 0,
    };

    // Match units from each stack
    let min_count = attackers.len().min(defenders.len());

    for i in 0..min_count {
        if attackers[i].is_alive() && defenders[i].is_alive() {
            let result = resolve_combat(&mut attackers[i], &mut defenders[i]);

            total_result.attacker_damage_dealt += result.attacker_damage_dealt;
            total_result.defender_damage_dealt += result.defender_damage_dealt;
            total_result.attacker_casualties += result.attacker_casualties;
            total_result.defender_casualties += result.defender_casualties;

            // Track if any hits occurred
            total_result.attacker_hit = total_result.attacker_hit || result.attacker_hit;
            total_result.defender_hit = total_result.defender_hit || result.defender_hit;
        }
    }

    // Remove dead units
    attackers.retain(|u| u.is_alive());
    defenders.retain(|u| u.is_alive());

    total_result
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
