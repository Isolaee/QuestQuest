/// Evolution System Demo
///
/// This example demonstrates the automatic unit evolution system:
/// - Units automatically evolve when they level up
/// - Inventory and equipment are preserved
/// - Young Warrior → Warrior → Veteran Warrior
/// - Max level units gain incremental stat boosts
///
/// Run with: cargo run --package game --example evolution_demo
use game::{GameObject, GameUnit, GameWorld, Team};
use graphics::HexCoord;
use units::{Terrain, Unit, UnitFactory};

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       Automatic Unit Evolution Demo                      ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║ Units evolve automatically when they gain enough XP      ║");
    println!("║ • Young Warrior → Warrior → Veteran Warrior              ║");
    println!("║ • Inventory and equipment are preserved                  ║");
    println!("║ • Max-level units get +2 HP, +1 attack per level         ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Create a game world
    let mut world = GameWorld::new(10);

    // Create a Level 1 Young Warrior at (0, 0)
    println!("Creating Level 1 Dwarf Young Warrior:");
    let young_warrior =
        UnitFactory::create_dwarf_young_warrior("Thorin".to_string(), HexCoord::new(0, 0));
    println!("  • Name: {}", young_warrior.name());
    println!("  • Level: {}", young_warrior.level());
    println!("  • Type: {}", young_warrior.unit_type());
    println!("  • HP: {}", young_warrior.combat_stats().max_health);
    println!("  • Attack: {}", young_warrior.combat_stats().base_attack);
    let evolutions = young_warrior.evolution_next();
    if evolutions.is_empty() {
        println!("  • Can evolve to: None");
    } else {
        let evolutions_str: Vec<String> = evolutions.iter().map(|e| format!("{:?}", e)).collect();
        println!("  • Can evolve to: {}", evolutions_str.join(", "));
    }

    let young_warrior_boxed: Box<dyn Unit> = young_warrior;
    let player_unit = GameUnit::new_with_team(young_warrior_boxed, Team::Player);
    let player_id = player_unit.id();
    world.add_unit(player_unit);

    // Create an enemy at (1, 0) for the warrior to defeat
    println!("\nCreating Enemy Level 1 Orc Young Swordsman:");
    let enemy1 = units::units::OrcYoungSwordsman::new("Grishnak".to_string(), HexCoord::new(1, 0));
    println!("  • Name: {}", enemy1.name());
    println!("  • Level: {}", enemy1.level());
    println!("  • HP: {}", enemy1.combat_stats().health);

    let enemy1_boxed: Box<dyn Unit> = Box::new(enemy1);
    let mut enemy1_unit = GameUnit::new_with_team(enemy1_boxed, Team::Enemy);
    let enemy1_id = enemy1_unit.id();

    // Weaken enemy so Thorin can kill it in one hit
    enemy1_unit.unit_mut().take_damage(95); // Reduce to 5 HP
    println!(
        "  (Weakened to {} HP for demo)",
        enemy1_unit.unit().combat_stats().health
    );

    world.add_unit(enemy1_unit);

    // Start turn system
    world.start_turn_based_game();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║ BATTLE 1: First Evolution                                ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // For this demo, we'll manually award XP to trigger level-up
    // Level 2 requires 2² × 50 = 200 XP total
    println!("Awarding XP to Thorin to demonstrate evolution...\n");
    if let Some(player) = world.get_unit_mut(player_id) {
        player.unit_mut().add_experience(199); // Set to 199, then killing enemy gives 1 more = 200 total
    }

    println!("Thorin attacks Grishnak...\n");
    if let Err(e) = world.request_combat(player_id, enemy1_id) {
        println!("Attack failed: {}", e);
    } else if let Err(e) = world.execute_pending_combat() {
        println!("Combat execution failed: {}", e);
    }

    // Check if player evolved
    if let Some(player) = world.get_unit(player_id) {
        println!("\n📊 After Battle 1:");
        println!("  • Name: {}", player.unit().name());
        println!("  • Level: {}", player.unit().level());
        println!("  • Type: {}", player.unit().unit_type());
        println!(
            "  • HP: {}/{}",
            player.unit().combat_stats().health,
            player.unit().combat_stats().max_health
        );
        println!("  • Attack: {}", player.unit().combat_stats().base_attack);
        println!("  • XP: {}", player.unit().experience());
        let evolutions = player.unit().evolution_next();
        if evolutions.is_empty() {
            println!("  • Evolution: MAX LEVEL (incremental growth)");
        } else {
            let evolutions_str: Vec<String> =
                evolutions.iter().map(|e| format!("{:?}", e)).collect();
            println!("  • Can evolve to: {}", evolutions_str.join(", "));
        }
    }

    // Create second enemy for next evolution
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║ BATTLE 2: Second Evolution                               ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("Creating Enemy Level 2 Orc Swordsman:");
    let enemy2 = units::units::OrcSwordsman::new("Ugluk".to_string(), HexCoord::new(2, 0));
    println!("  • Name: {}", enemy2.name());
    println!("  • Level: {}", enemy2.level());
    println!("  • HP: {}", enemy2.combat_stats().health);

    let enemy2_boxed: Box<dyn Unit> = Box::new(enemy2);
    let mut enemy2_unit = GameUnit::new_with_team(enemy2_boxed, Team::Enemy);
    let enemy2_id = enemy2_unit.id();

    // Weaken enemy so Thorin can kill it
    enemy2_unit.unit_mut().take_damage(110); // Reduce to 15 HP for demo
    println!(
        "  (Weakened to {} HP for demo)",
        enemy2_unit.unit().combat_stats().health
    );

    world.add_unit(enemy2_unit);

    // Award more XP for second evolution
    // Level 3 requires 3² × 50 = 450 XP total
    // Player should now have ~200 XP, needs 250 more
    println!("\nAwarding more XP for second evolution...\n");
    if let Some(player) = world.get_unit_mut(player_id) {
        player.unit_mut().add_experience(246); // +246 = 446 total, killing Level 2 enemy gives 4 more = 450
    }

    // Attack second enemy (multiple times to defeat it)
    println!("Thorin attacks Ugluk...\n");
    if let Err(e) = world.request_combat(player_id, enemy2_id) {
        println!("Attack failed: {}", e);
    } else if let Err(e) = world.execute_pending_combat() {
        println!("Combat execution failed: {}", e);
    }

    // Check if enemy is still alive and attack again if needed
    if world.get_unit(enemy2_id).is_some() {
        // End turn and start new turn to allow another attack
        world.end_current_turn();

        println!("Thorin attacks Ugluk again...\n");
        if let Err(e) = world.request_combat(player_id, enemy2_id) {
            println!("Attack failed: {}", e);
        } else if let Err(e) = world.execute_pending_combat() {
            println!("Combat execution failed: {}", e);
        }
    }

    // Check final state
    if let Some(player) = world.get_unit(player_id) {
        println!("\n📊 After Battle 2:");
        println!("  • Name: {}", player.unit().name());
        println!("  • Level: {}", player.unit().level());
        println!("  • Type: {}", player.unit().unit_type());
        println!(
            "  • HP: {}/{}",
            player.unit().combat_stats().health,
            player.unit().combat_stats().max_health
        );
        println!("  • Attack: {}", player.unit().combat_stats().base_attack);
        println!("  • XP: {}", player.unit().experience());
        let evolutions = player.unit().evolution_next();
        if evolutions.is_empty() {
            println!("  • Evolution: MAX LEVEL (incremental growth)");
        } else {
            let evolutions_str: Vec<String> =
                evolutions.iter().map(|e| format!("{:?}", e)).collect();
            println!("  • Can evolve to: {}", evolutions_str.join(", "));
        }
    }

    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║ Summary                                                   ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║ ✓ Units automatically evolve when they level up          ║");
    println!("║ ✓ Young Warrior → Warrior → Veteran Warrior              ║");
    println!("║ ✓ Stats increase with each evolution                     ║");
    println!("║ ✓ New attacks unlocked                                    ║");
    println!("║ ✓ Same unit ID preserved through evolution               ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}
