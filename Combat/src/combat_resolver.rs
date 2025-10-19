use crate::{CombatResult, CombatStats, DamageType};
use rand::Rng;

/// Resolve combat between two units
/// Returns the result of the combat encounter
pub fn resolve_combat(
    attacker: &mut CombatStats,
    defender: &mut CombatStats,
    damage_type: DamageType,
) -> CombatResult {
    let mut result = CombatResult {
        attacker_damage_dealt: 0,
        defender_damage_dealt: 0,
        attacker_hit: false,
        defender_hit: false,
        attacker_casualties: 0,
        defender_casualties: 0,
    };

    // Check if attacker hits based on terrain hit chance
    let mut rng = rand::thread_rng();
    let hit_roll = rng.gen_range(0..100);

    if hit_roll < attacker.terrain_hit_chance {
        result.attacker_hit = true;

        // Calculate damage with resistance
        let damage = attacker.calculate_damage_to(defender, damage_type);
        result.attacker_damage_dealt = damage;

        // Apply damage to defender
        defender.take_damage_with_resistance(damage, damage_type);

        // Check if defender died
        if !defender.is_alive() {
            result.defender_casualties = 1;
        }
    }

    // Defender counterattacks if still alive and in melee range
    if defender.is_alive() && attacker.range_category == crate::RangeCategory::Melee {
        let counter_roll = rng.gen_range(0..100);

        if counter_roll < defender.terrain_hit_chance {
            result.defender_hit = true;

            // Calculate counter damage with resistance
            let counter_damage = defender.calculate_damage_to(attacker, damage_type);
            result.defender_damage_dealt = counter_damage;

            // Apply damage to attacker
            attacker.take_damage_with_resistance(counter_damage, damage_type);

            // Check if attacker died
            if !attacker.is_alive() {
                result.attacker_casualties = 1;
            }
        }
    }

    result
}
