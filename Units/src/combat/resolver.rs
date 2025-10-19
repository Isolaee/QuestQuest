use crate::unit::Unit;
use combat::CombatResult;

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
