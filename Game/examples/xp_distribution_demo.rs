/// Example demonstrating the XP distribution system
///
/// This shows how units gain experience when defeating enemies:
/// - Killer gets level² XP
/// - Adjacent allies get 2 × level XP
///
/// Run with: cargo run --package game --example xp_distribution_demo
use game::{GameObject, GameUnit, GameWorld, Team};
use graphics::HexCoord;
use units::{OrcEliteSwordsman, Unit, UnitFactory};

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       XP Distribution System Demo                        ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║ Rules:                                                    ║");
    println!("║ • Killer gets level² XP                                   ║");
    println!("║ • Adjacent allies get 2 × level XP                        ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Create a game world
    let mut world = GameWorld::new(10);

    // Create player units
    println!("Creating Player Team:");

    // Main attacker - Level 2 Dwarf Warrior at (0, 0)
    let attacker = UnitFactory::create_dwarf_warrior("Gimli".to_string(), HexCoord::new(0, 0));
    println!(
        "  • {} (Level {}, {} XP)",
        attacker.name(),
        attacker.level(),
        attacker.experience()
    );
    let attacker_unit = GameUnit::new_with_team(attacker, Team::Player);
    let attacker_id = attacker_unit.id();
    world.add_unit(attacker_unit);

    // Adjacent ally 1 - Level 1 Dwarf Young Warrior at (1, 0)
    let ally1 = UnitFactory::create_dwarf_young_warrior("Ally 1".to_string(), HexCoord::new(1, 0));
    println!(
        "  • {} (Level {}, {} XP) - Adjacent to Thorin",
        ally1.name(),
        ally1.level(),
        ally1.experience()
    );
    let ally1_unit = GameUnit::new_with_team(ally1, Team::Player);
    let ally1_id = ally1_unit.id();
    world.add_unit(ally1_unit);

    // Adjacent ally 2 - Level 1 Dwarf Young Warrior at (0, 1)
    let ally2 = UnitFactory::create_dwarf_young_warrior("Ally 2".to_string(), HexCoord::new(0, 1));
    println!(
        "  • {} (Level {}, {} XP) - Adjacent to Thorin",
        ally2.name(),
        ally2.level(),
        ally2.experience()
    );
    let ally2_unit = GameUnit::new_with_team(ally2, Team::Player);
    let ally2_id = ally2_unit.id();
    world.add_unit(ally2_unit);

    // Non-adjacent ally - Level 2 Dwarf Warrior at (3, 3)
    let distant = UnitFactory::create_dwarf_warrior("Distant".to_string(), HexCoord::new(3, 3));
    println!(
        "  • {} (Level {}, {} XP) - Far from battle",
        distant.name(),
        distant.level(),
        distant.experience()
    );
    let distant_unit = GameUnit::new_with_team(distant, Team::Player);
    let distant_id = distant_unit.id();
    world.add_unit(distant_unit);

    // Create enemy unit - Level 3 Elite Swordsman at (1, -1)
    println!("\nCreating Enemy:");
    let enemy = OrcEliteSwordsman::new("Enemy".to_string(), HexCoord::new(1, -1));
    println!(
        "  • {} (Level {}, {} HP)",
        enemy.name(),
        enemy.level(),
        enemy.combat_stats().health
    );
    let enemy_boxed: Box<dyn Unit> = Box::new(enemy);
    let mut enemy_unit = GameUnit::new_with_team(enemy_boxed, Team::Enemy);
    let enemy_id = enemy_unit.id();

    // Weaken the enemy so one hit will kill it
    enemy_unit.unit_mut().take_damage(145); // Reduce to 5 HP (one hit will kill)
    println!(
        "    (Weakened to {} HP for demo)",
        enemy_unit.unit().combat_stats().health
    );

    world.add_unit(enemy_unit);

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║ Combat Begins!                                            ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("Expected XP Distribution:");
    println!("  • Thorin (killer): 3² = 9 XP");
    println!("  • Balin (adjacent): 2 × 3 = 6 XP");
    println!("  • Dwalin (adjacent): 2 × 3 = 6 XP");
    println!("  • Gimli (distant): 0 XP\n");

    // Show pre-combat state
    println!("Before Combat:");
    if let Some(unit) = world.get_unit(attacker_id) {
        println!("  {} XP: {}", unit.unit().name(), unit.unit().experience());
    }
    if let Some(unit) = world.get_unit(ally1_id) {
        println!("  {} XP: {}", unit.unit().name(), unit.unit().experience());
    }
    if let Some(unit) = world.get_unit(ally2_id) {
        println!("  {} XP: {}", unit.unit().name(), unit.unit().experience());
    }
    if let Some(unit) = world.get_unit(distant_id) {
        println!("  {} XP: {}", unit.unit().name(), unit.unit().experience());
    }

    // Execute combat - Thorin attacks Azog
    println!("\n▶ Thorin attacks Azog...\n");
    if let Err(e) = world.request_combat(attacker_id, enemy_id) {
        println!("Combat request error: {}", e);
    } else if let Err(e) = world.execute_pending_combat() {
        println!("Combat execution error: {}", e);
    }

    // Show post-combat state
    println!("\nAfter Combat:");
    if let Some(unit) = world.get_unit(attacker_id) {
        println!(
            "  {} XP: {} (+{})",
            unit.unit().name(),
            unit.unit().experience(),
            unit.unit().experience() - 50
        ); // Started at 50
    }
    if let Some(unit) = world.get_unit(ally1_id) {
        println!(
            "  {} XP: {} (+{})",
            unit.unit().name(),
            unit.unit().experience(),
            unit.unit().experience()
        ); // Started at 0
    }
    if let Some(unit) = world.get_unit(ally2_id) {
        println!(
            "  {} XP: {} (+{})",
            unit.unit().name(),
            unit.unit().experience(),
            unit.unit().experience()
        ); // Started at 0
    }
    if let Some(unit) = world.get_unit(distant_id) {
        println!(
            "  {} XP: {} (+{})",
            unit.unit().name(),
            unit.unit().experience(),
            unit.unit().experience() - 50
        ); // Started at 50
    }

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║ Demo Complete!                                            ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}
