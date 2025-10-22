//! # Combat Resolver Module
//!
//! Contains the core combat resolution algorithm that handles turn-based
//! combat with alternating attacks, hit chance rolls, and damage calculation.

use crate::{CombatResult, CombatStats, DamageType};
use rand::Rng;

/// Resolves combat between two units with alternating attacks.
///
/// This is the main combat resolution function. It handles:
/// - Turn-based alternating attacks
/// - Hit chance rolls for each attack
/// - Damage calculation with resistance modifiers
/// - Multi-attack systems (both units can attack multiple times)
/// - Counter-attacks (only for melee range)
/// - Combat logging with detailed turn-by-turn output
///
/// # Combat Flow
///
/// 1. Attacker makes their first attack (if hit roll succeeds)
/// 2. If defender is in melee range, they counter-attack
/// 3. Attacks alternate until both units exhaust their `attacks_per_round`
/// 4. Combat ends when all attacks are used or a unit is defeated
///
/// # Ranged vs Melee
///
/// - **Melee**: Both attacker and defender can attack
/// - **Ranged/Siege**: Only attacker can attack (defender cannot counter)
///
/// # Arguments
///
/// * `attacker` - Mutable reference to attacking unit's stats
/// * `defender` - Mutable reference to defending unit's stats
/// * `damage_type` - Type of damage being dealt (affects resistance)
///
/// # Returns
///
/// `CombatResult` containing damage dealt, hits, and casualties
///
/// # Examples
///
/// ```ignore
/// use combat::{resolve_combat, CombatStats, DamageType, RangeCategory, Resistances};
///
/// let mut warrior = CombatStats::new(100, 20, 5, RangeCategory::Melee, Resistances::default());
/// let mut goblin = CombatStats::new(50, 10, 4, RangeCategory::Melee, Resistances::default());
///
/// let result = resolve_combat(&mut warrior, &mut goblin, DamageType::Slash);
///
/// if result.defender_casualties > 0 {
///     println!("Goblin defeated!");
/// }
/// ```
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

    let mut rng = rand::thread_rng();

    // Combat debugging header
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║              ⚔️  COMBAT ROUND BEGINS  ⚔️                 ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!(
        "║ Attacker: {} HP, {} attacks/round, {} dmg/attack      ",
        attacker.health, attacker.attacks_per_round, attacker.attack_strength
    );
    println!(
        "║ Defender: {} HP, {} attacks/round, {} dmg/attack      ",
        defender.health, defender.attacks_per_round, defender.attack_strength
    );
    println!(
        "║ Range: {:?}, Damage Type: {:?}                    ",
        attacker.range_category, damage_type
    );
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Determine total attacks for each combatant
    let mut attacker_attacks_remaining = attacker.attacks_per_round;
    let mut defender_attacks_remaining = if attacker.range_category == crate::RangeCategory::Melee {
        defender.attacks_per_round
    } else {
        0 // Ranged attackers don't get counter-attacked
    };

    let mut attack_number = 1;
    let mut is_attacker_turn = true; // Attacker goes first

    // Alternate attacks until both fighters have used all their attacks
    while (attacker_attacks_remaining > 0 || defender_attacks_remaining > 0)
        && attacker.is_alive()
        && defender.is_alive()
    {
        if is_attacker_turn && attacker_attacks_remaining > 0 {
            // ATTACKER'S TURN
            println!(
                "┌─ Attack #{}: ATTACKER's Turn ─────────────────────────┐",
                attack_number
            );

            let hit_roll_1 = rng.gen_range(0..100);
            let hit_roll_2 = rng.gen_range(0..100);
            let hit_chance = attacker.terrain_hit_chance;

            print!("│ Roll 1: {} vs {}% hit chance → ", hit_roll_1, hit_chance);
            if hit_roll_1 < hit_chance {
                println!("✓ SUCCESS");
            } else {
                println!("✗ FAIL");
            }
            print!("│ Roll 2: {} vs {}% hit chance → ", hit_roll_2, hit_chance);
            if hit_roll_2 < hit_chance {
                println!("✓ SUCCESS");
            } else {
                println!("✗ FAIL");
            }

            if hit_roll_1 < hit_chance && hit_roll_2 < hit_chance {
                println!("✓ DOUBLE SUCCESS! HIT!");
                result.attacker_hit = true;

                // Calculate damage for this single attack with resistance
                let resistance = defender.resistances.get_resistance(damage_type);
                let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
                let modified_strength =
                    (attacker.attack_strength as i32 + attacker.attack_modifier).max(0) as u32;
                let damage = ((modified_strength as f32 * resistance_multiplier) as u32).max(1);

                let defender_hp_before = defender.health;
                defender.take_damage_with_resistance(damage, damage_type);
                let defender_hp_after = defender.health;

                println!(
                    "│ Damage: {} ({}% resistance) → {} actual damage",
                    modified_strength, resistance, damage
                );
                println!(
                    "│ Defender HP: {} → {} (took {} damage)",
                    defender_hp_before, defender_hp_after, damage
                );

                result.attacker_damage_dealt += damage;

                if !defender.is_alive() {
                    println!("│ 💀 DEFENDER DEFEATED!");
                    result.defender_casualties = 1;
                }
            } else {
                println!("✗ MISS! Need two successes to hit.");
            }

            println!("└───────────────────────────────────────────────────────┘\n");

            attacker_attacks_remaining -= 1;
            is_attacker_turn = false; // Switch to defender
        } else if !is_attacker_turn && defender_attacks_remaining > 0 {
            // DEFENDER'S TURN (Counter-attack)
            println!(
                "┌─ Attack #{}: DEFENDER's Turn (Counter) ───────────────┐",
                attack_number
            );

            let counter_roll_1 = rng.gen_range(0..100);
            let counter_roll_2 = rng.gen_range(0..100);
            let hit_chance = defender.terrain_hit_chance;

            print!(
                "│ Roll 1: {} vs {}% hit chance → ",
                counter_roll_1, hit_chance
            );
            if counter_roll_1 < hit_chance {
                println!("✓ SUCCESS");
            } else {
                println!("✗ FAIL");
            }
            print!(
                "│ Roll 2: {} vs {}% hit chance → ",
                counter_roll_2, hit_chance
            );
            if counter_roll_2 < hit_chance {
                println!("✓ SUCCESS");
            } else {
                println!("✗ FAIL");
            }

            if counter_roll_1 < hit_chance && counter_roll_2 < hit_chance {
                println!("✓ DOUBLE SUCCESS! HIT!");
                result.defender_hit = true;

                // Calculate counter damage for this single attack with resistance
                let resistance = attacker.resistances.get_resistance(damage_type);
                let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
                let modified_strength =
                    (defender.attack_strength as i32 + defender.attack_modifier).max(0) as u32;
                let counter_damage =
                    ((modified_strength as f32 * resistance_multiplier) as u32).max(1);

                let attacker_hp_before = attacker.health;
                attacker.take_damage_with_resistance(counter_damage, damage_type);
                let attacker_hp_after = attacker.health;

                println!(
                    "│ Damage: {} ({}% resistance) → {} actual damage",
                    modified_strength, resistance, counter_damage
                );
                println!(
                    "│ Attacker HP: {} → {} (took {} damage)",
                    attacker_hp_before, attacker_hp_after, counter_damage
                );

                result.defender_damage_dealt += counter_damage;

                if !attacker.is_alive() {
                    println!("│ 💀 ATTACKER DEFEATED!");
                    result.attacker_casualties = 1;
                }
            } else {
                println!("✗ MISS! Need two successes to hit.");
            }

            println!("└───────────────────────────────────────────────────────┘\n");

            defender_attacks_remaining -= 1;
            is_attacker_turn = true; // Switch back to attacker
        } else {
            // Skip turn if current fighter has no attacks remaining
            is_attacker_turn = !is_attacker_turn;
            continue;
        }

        attack_number += 1;
    }

    // Combat summary
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║              🏁 COMBAT ROUND COMPLETE 🏁                  ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!(
        "║ Attacker: {} HP remaining (dealt {} total dmg)",
        attacker.health, result.attacker_damage_dealt
    );
    println!(
        "║ Defender: {} HP remaining (dealt {} total dmg)",
        defender.health, result.defender_damage_dealt
    );
    if result.attacker_casualties > 0 {
        println!("║ 💀 Attacker DEFEATED!");
    }
    if result.defender_casualties > 0 {
        println!("║ 💀 Defender DEFEATED!");
    }
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    result
}
