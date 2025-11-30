//! # Combat Resolver Module
//!
//! Contains the core combat resolution algorithm that handles turn-based
//! combat with alternating attacks, hit chance rolls, and damage calculation.
//!
//! Combat flows as follows:
//! 1. Requests to initiate combat are made via the `iniate_combat` function.
//! 2. The `execute_combat` function executes the combat round, alternating
//!    attacks between the attacker and defender until both have expended their
//!    allowed attacks for the turn.
//! 3. Each attack involves rolling for hit chance twice; both rolls must succeed
//!    for the attack to hit.
//! 4. Damage is calculated based on attack damage, modified by any resistances
//!    the target has against the attack's damage type.

use rand::Rng;
use units::combat::{CombatResult, DamageType, RangeCategory};
use units::unit_trait::Unit;

/// Resolves a combat encounter between an attacker and defender unit.
pub fn resolve_combat<U: Unit>(
    attacker: &mut U,
    defender: &mut U,
    damage_type: DamageType,
) -> CombatResult {
    // If the initiating unit has already attacked this game turn, abort combat.
    if attacker.combat_stats().attacked_this_turn {
        return CombatResult::default();
    }

    let mut result = CombatResult::default();

    // Determine total attacks for each combatant, capped at 1 per turn.
    let mut attacker_attacks_remaining: i32 = 1;
    let mut defender_attacks_remaining: i32 =
        if attacker.combat_stats().range_category == RangeCategory::Melee {
            1
        } else {
            0 // Ranged attackers don't get counter-attacked
        };

    let mut is_attacker_turn = true; // Attacker goes first

    while (attacker_attacks_remaining > 0 || defender_attacks_remaining > 0)
        && attacker.is_alive()
        && defender.is_alive()
    {
        if is_attacker_turn && attacker_attacks_remaining > 0 {
            // If already flagged as attacked, skip remaining attacker attacks.
            if attacker.base().attacked_this_round {
                attacker_attacks_remaining = 0;
                is_attacker_turn = false;
                continue;
            }

            // Hit chance: use precomputed terrain/defense field in CombatStats.
            let hit_chance = attacker.combat_stats().terrain_hit_chance as u32;

            if roll_hit(hit_chance) {
                // true = hit
                result.attacker_hit = true;

                // Resistance lookup
                let resistance_pct = defender
                    .combat_stats()
                    .resistances
                    .get_resistance(damage_type); // expect 0..=100

                let resistance_multiplier = 1.0 - (resistance_pct as f32 / 100.0);

                let base_attack = attacker.base().cached_attack;
                let attack_modifier = attacker.combat_stats().attack_modifier;
                let modified_strength = (base_attack + attack_modifier).max(0) as u32;

                let damage = ((modified_strength as f32 * resistance_multiplier) as u32).max(1);

                // Apply damage
                defender.take_damage(damage);
                result.attacker_damage_dealt += damage;

                if !defender.is_alive() {
                    result.defender_casualties = 1;
                }
            }

            attacker_attacks_remaining = attacker_attacks_remaining.saturating_sub(1);
            attacker.combat_stats_mut().attacked_this_turn = true;
            is_attacker_turn = false;
        } else if !is_attacker_turn && defender_attacks_remaining > 0 {
            // Defender (counter) turn
            if defender.combat_stats().attacked_this_turn {
                defender_attacks_remaining = 0;
                is_attacker_turn = true;
                continue;
            }

            let hit_chance = defender.combat_stats().terrain_hit_chance as u32;

            if roll_hit(hit_chance) {
                result.defender_hit = true;
                let resistance = attacker.get_resistance(damage_type);
                let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
                let modified_strength = (defender.combat_stats().attack_strength as i32
                    + defender.combat_stats().attack_modifier)
                    .max(0) as u32;
                let counter_damage =
                    ((modified_strength as f32 * resistance_multiplier) as u32).max(1);

                attacker.take_damage(counter_damage);
                result.defender_damage_dealt += counter_damage;

                if !attacker.is_alive() {
                    result.attacker_casualties = 1;
                }
            }

            defender_attacks_remaining = defender_attacks_remaining.saturating_sub(1);
            defender.combat_stats_mut().attacked_this_turn = true;
            is_attacker_turn = true;
        } else {
            // Toggle turn if either combatant has no attacks remaining
            is_attacker_turn = !is_attacker_turn;
            continue;
        }
    }
    result
}

fn roll_hit(hit_chance: u32) -> bool {
    let mut rng = rand::thread_rng();
    let roll_1 = rng.gen_range(0..100);
    let roll_2 = rng.gen_range(0..100);
    roll_1 < hit_chance || roll_2 < hit_chance
}

// pub fn resolve_combat(
//     attacker: &mut CombatStats,
//     defender: &mut CombatStats,
//     damage_type: DamageType,
// ) -> CombatResult {
//     // If the initiating unit has already attacked this game turn, do not
//     // initiate combat at all. Return an empty CombatResult so callers can
//     // detect that no combat occurred.
//     if attacker.attacked_this_turn {
//         return CombatResult {
//             attacker_damage_dealt: 0,
//             defender_damage_dealt: 0,
//             attacker_hit: false,
//             defender_hit: false,
//             attacker_casualties: 0,
//             defender_casualties: 0,
//         };
//     }

//     let mut result = CombatResult {
//         attacker_damage_dealt: 0,
//         defender_damage_dealt: 0,
//         attacker_hit: false,
//         defender_hit: false,
//         attacker_casualties: 0,
//         defender_casualties: 0,
//     };

//     let mut rng = rand::thread_rng();

//     // Combat debugging header
//     println!("\n╔═══════════════════════════════════════════════════════════╗");
//     println!("║              ⚔️  COMBAT ROUND BEGINS!!!  ⚔️                 ║");
//     println!("╠═══════════════════════════════════════════════════════════╣");
//     println!(
//         "║ Attacker: {} HP, {} attacks this turn (cap 1), {} dmg/attack      ",
//         attacker.health, attacker.attacks_per_round, attacker.attack_strength
//     );
//     println!(
//         "║ Defender: {} HP, {} attacks this turn (cap 1), {} dmg/attack      ",
//         defender.health, defender.attacks_per_round, defender.attack_strength
//     );
//     println!(
//         "║ Range: {:?}, Damage Type: {:?}                    ",
//         attacker.range_category, damage_type
//     );
//     println!("╚═══════════════════════════════════════════════════════════╝\n");

//     // Determine total attacks for each combatant
//     // Enforce: a unit can only make one attack per turn. We still keep the
//     // `attacks_per_round` stat for display/AI decisions, but combat resolution
//     // will cap actual attacks to 1 for this turn.
//     let mut attacker_attacks_remaining = attacker.attacks_per_round.min(1);
//     let mut defender_attacks_remaining = if attacker.range_category == crate::RangeCategory::Melee {
//         defender.attacks_per_round.min(1)
//     } else {
//         0 // Ranged attackers don't get counter-attacked
//     };

//     let mut attack_number = 1;
//     let mut is_attacker_turn = true; // Attacker goes first

//     // Alternate attacks until both fighters have used all their attacks
//     while (attacker_attacks_remaining > 0 || defender_attacks_remaining > 0)
//         && attacker.is_alive()
//         && defender.is_alive()
//     {
//         if is_attacker_turn && attacker_attacks_remaining > 0 {
//             // ATTACKER'S TURN
//             if attacker.attacked_this_turn {
//                 println!("│ Attacker has already attacked this turn — skipping.");
//                 // Consume the available attack slot but do not perform damage
//                 attacker_attacks_remaining = 0;
//                 is_attacker_turn = false;
//                 attack_number += 1;
//                 continue;
//             }
//             println!(
//                 "┌─ Attack #{}: ATTACKER's Turn ─────────────────────────┐",
//                 attack_number
//             );

//             let hit_roll_1 = rng.gen_range(0..100);
//             let hit_roll_2 = rng.gen_range(0..100);
//             let hit_chance = attacker.terrain_hit_chance;

//             print!("│ Roll 1: {} vs {}% hit chance → ", hit_roll_1, hit_chance);
//             if hit_roll_1 < hit_chance {
//                 println!("✓ SUCCESS");
//             } else {
//                 println!("✗ FAIL");
//             }
//             print!("│ Roll 2: {} vs {}% hit chance → ", hit_roll_2, hit_chance);
//             if hit_roll_2 < hit_chance {
//                 println!("✓ SUCCESS");
//             } else {
//                 println!("✗ FAIL");
//             }

//             if hit_roll_1 < hit_chance && hit_roll_2 < hit_chance {
//                 println!("✓ DOUBLE SUCCESS! HIT!");
//                 result.attacker_hit = true;

//                 // Calculate damage for this single attack with resistance
//                 let resistance = defender.resistances.get_resistance(damage_type);
//                 let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
//                 let modified_strength =
//                     (attacker.attack_strength as i32 + attacker.attack_modifier).max(0) as u32;
//                 let damage = ((modified_strength as f32 * resistance_multiplier) as u32).max(1);

//                 let defender_hp_before = defender.health;
//                 defender.take_damage_with_resistance(damage, damage_type);
//                 let defender_hp_after = defender.health;

//                 println!(
//                     "│ Damage: {} ({}% resistance) → {} actual damage",
//                     modified_strength, resistance, damage
//                 );
//                 println!(
//                     "│ Defender HP: {} → {} (took {} damage)",
//                     defender_hp_before, defender_hp_after, damage
//                 );

//                 result.attacker_damage_dealt += damage;

//                 if !defender.is_alive() {
//                     println!("│ 💀 DEFENDER DEFEATED!");
//                     result.defender_casualties = 1;
//                 }
//             } else {
//                 println!("✗ MISS! Need two successes to hit.");
//             }

//             println!("└───────────────────────────────────────────────────────┘\n");

//             attacker_attacks_remaining -= 1;
//             // Mark attacker as having attacked this game turn so they cannot attack again
//             attacker.attacked_this_turn = true;
//             is_attacker_turn = false; // Switch to defender
//         } else if !is_attacker_turn && defender_attacks_remaining > 0 {
//             // DEFENDER'S TURN (Counter-attack)
//             if defender.attacked_this_turn {
//                 println!("│ Defender has already attacked this turn — skipping.");
//                 defender_attacks_remaining = 0;
//                 is_attacker_turn = true;
//                 attack_number += 1;
//                 continue;
//             }
//             println!(
//                 "┌─ Attack #{}: DEFENDER's Turn (Counter) ───────────────┐",
//                 attack_number
//             );

//             let counter_roll_1 = rng.gen_range(0..100);
//             let counter_roll_2 = rng.gen_range(0..100);
//             let hit_chance = defender.terrain_hit_chance;

//             print!(
//                 "│ Roll 1: {} vs {}% hit chance → ",
//                 counter_roll_1, hit_chance
//             );
//             if counter_roll_1 < hit_chance {
//                 println!("✓ SUCCESS");
//             } else {
//                 println!("✗ FAIL");
//             }
//             print!(
//                 "│ Roll 2: {} vs {}% hit chance → ",
//                 counter_roll_2, hit_chance
//             );
//             if counter_roll_2 < hit_chance {
//                 println!("✓ SUCCESS");
//             } else {
//                 println!("✗ FAIL");
//             }

//             if counter_roll_1 < hit_chance && counter_roll_2 < hit_chance {
//                 println!("✓ DOUBLE SUCCESS! HIT!");
//                 result.defender_hit = true;

//                 // Calculate counter damage for this single attack with resistance
//                 let resistance = attacker.resistances.get_resistance(damage_type);
//                 let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
//                 let modified_strength =
//                     (defender.attack_strength as i32 + defender.attack_modifier).max(0) as u32;
//                 let counter_damage =
//                     ((modified_strength as f32 * resistance_multiplier) as u32).max(1);

//                 let attacker_hp_before = attacker.health;
//                 attacker.take_damage_with_resistance(counter_damage, damage_type);
//                 let attacker_hp_after = attacker.health;

//                 println!(
//                     "│ Damage: {} ({}% resistance) → {} actual damage",
//                     modified_strength, resistance, counter_damage
//                 );
//                 println!(
//                     "│ Attacker HP: {} → {} (took {} damage)",
//                     attacker_hp_before, attacker_hp_after, counter_damage
//                 );

//                 result.defender_damage_dealt += counter_damage;

//                 if !attacker.is_alive() {
//                     println!("│ 💀 ATTACKER DEFEATED!");
//                     result.attacker_casualties = 1;
//                 }
//             } else {
//                 println!("✗ MISS! Need two successes to hit.");
//             }

//             println!("└───────────────────────────────────────────────────────┘\n");

//             defender_attacks_remaining -= 1;
//             // Mark defender as having attacked this game turn so they cannot attack again
//             defender.attacked_this_turn = true;
//             is_attacker_turn = true; // Switch back to attacker
//         } else {
//             // Skip turn if current fighter has no attacks remaining
//             is_attacker_turn = !is_attacker_turn;
//             continue;
//         }

//         attack_number += 1;
//     }

//     // Combat summary
//     println!("╔═══════════════════════════════════════════════════════════╗");
//     println!("║              🏁 COMBAT ROUND COMPLETE 🏁                  ║");
//     println!("╠═══════════════════════════════════════════════════════════╣");
//     println!(
//         "║ Attacker: {} HP remaining (dealt {} total dmg)",
//         attacker.health, result.attacker_damage_dealt
//     );
//     println!(
//         "║ Defender: {} HP remaining (dealt {} total dmg)",
//         defender.health, result.defender_damage_dealt
//     );
//     if result.attacker_casualties > 0 {
//         println!("║ 💀 Attacker DEFEATED!");
//     }
//     if result.defender_casualties > 0 {
//         println!("║ 💀 Defender DEFEATED!");
//     }
//     println!("╚═══════════════════════════════════════════════════════════╝\n");

//     result
// }
