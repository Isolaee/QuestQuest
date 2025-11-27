//! # Level Up System Example
//!
//! This example demonstrates the centralized level-up system with:
//! 1. Evolution-based level-ups (transforming to next unit)
//! 2. Incremental level-ups (stat boosts for max-level units)
//!
//! ## Key Features:
//! - Evolution: Transform into next unit type with new stats/attacks
//! - Incremental: +2 max HP, +1 attack for max-level units
//! - Equipment preserved automatically
//! - Previous/Next unit type tracking

use graphics::HexCoord;
use units::units::OrcYoungSwordsman;
use units::{Terrain, Unit};

fn main() {
    println!("=== Evolution & Incremental Level-Up Demo ===\n");

    // Create a Level 1 Young Swordsman
    let mut unit: Box<dyn Unit> = Box::new(OrcYoungSwordsman::new(
        "Gruk".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    ));

    println!("Created: {} (Level {})", unit.name(), unit.level());
    println!(
        "HP: {}/{}",
        unit.combat_stats().health,
        unit.combat_stats().max_health
    );
    println!("Attack: {}", unit.combat_stats().base_attack);
    // EVOLUTION 1: Young Swordsman → Swordsman
    println!("--- Gruk gains 200 XP ---");
    if unit.add_experience(200) {
        println!("🎉 Ready to evolve! (XP: {})\n", unit.experience());

        println!("→ Evolving into next form...");
        if let Some(evolved) = unit.evolve(true) {
            println!(
                "✓ Evolved to {} (Level {})",
                evolved.unit_type(),
                evolved.level()
            );
            println!(
                "✓ HP: {}/{} (healed to full)",
                evolved.combat_stats().health,
                evolved.combat_stats().max_health
            );
            println!("✓ Attack: {}", evolved.combat_stats().base_attack);
            println!("✓ New attacks unlocked!\n");
            unit = evolved;
        } else {
            println!("✗ No evolution available\n");
        }
    }

    // EVOLUTION 2: Swordsman → Elite Swordsman
    println!("--- Gruk gains 250 more XP (450 total) ---");
    if unit.add_experience(250) {
        println!("🎉 Ready to evolve again! (XP: {})\n", unit.experience());

        if unit.has_next_evolution() {
            println!("→ Evolving into elite form...");
            if let Some(evolved) = unit.evolve(false) {
                println!(
                    "✓ Evolved to {} (Level {})",
                    evolved.unit_type(),
                    evolved.level()
                );
                println!(
                    "✓ HP: {}/{}",
                    evolved.combat_stats().health,
                    evolved.combat_stats().max_health
                );
                println!("✓ Attack: {}", evolved.combat_stats().base_attack);
                println!("✓ Max evolution reached!\n");
                unit = evolved;
            }
        }
    }

    // INCREMENTAL LEVEL-UPS: Elite Swordsman at max level
    println!("--- Gruk continues training and gains 350 more XP (800 total) ---");
    if unit.add_experience(350) {
        println!(
            "🎉 Ready to level up (max evolution)! (XP: {})\n",
            unit.experience()
        );

        // Check if unit has no next evolution (max level reached)
        println!("→ No more evolutions. Gaining incremental power...");
        let old_hp = unit.combat_stats().max_health;
        let old_attack = unit.combat_stats().base_attack;

        unit.perform_level_up_incremental(true);

        println!(
            "✓ {} gained experience (Level {})",
            unit.unit_type(),
            unit.level()
        );
        println!(
            "✓ HP: {} → {} (+2, healed to full)",
            old_hp,
            unit.combat_stats().max_health
        );
        println!(
            "✓ Attack: {} → {} (+1)",
            old_attack,
            unit.combat_stats().base_attack
        );
        println!("✓ Same attacks, but stronger!\n");
    }

    // Another incremental level-up
    println!("--- Gruk gains 450 more XP (veteran warrior, 1250 total) ---");
    if unit.add_experience(450) {
        println!(
            "🎉 Another incremental level! (XP: {})\n",
            unit.experience()
        );

        let old_hp = unit.combat_stats().max_health;
        let old_attack = unit.combat_stats().base_attack;

        unit.perform_level_up_incremental(false);

        println!("✓ {} Level {}", unit.unit_type(), unit.level());
        println!(
            "✓ Max HP: {} → {} (+2)",
            old_hp,
            unit.combat_stats().max_health
        );
        println!(
            "✓ Attack: {} → {} (+1)",
            old_attack,
            unit.combat_stats().base_attack
        );
    }

    println!("\n=== Summary ===");
    println!("✓ Evolution-based level-ups: Transform into next unit type");
    println!("✓ Incremental level-ups: +2 HP, +1 attack for max-level units");
    println!("✓ Equipment preserved through all level-ups");
    println!("✓ Previous/Next unit tracking in evolution chain");
    println!("✓ All logic centralized in BaseUnit");
}
