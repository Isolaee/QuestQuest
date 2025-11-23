//! Structure System Example
//!
//! This example demonstrates how to use the structure system in QuestQuest,
//! including creating structures, occupying them, applying bonuses, and combat.

use graphics::HexCoord;
use units::structures::{Structure, StructureFactory};
use units::Team;
use uuid::Uuid;

fn main() {
    println!("=== QuestQuest Structure System Demo ===\n");

    // ===== Creating a Stone Wall =====
    println!("1. Creating a Stone Wall");
    println!("{}", "-".repeat(50));

    let mut wall = StructureFactory::create_stone_wall(HexCoord::new(10, 10), Team::Player);

    println!("Created: {}", wall.name());
    println!("Type: {:?}", wall.structure_type());
    println!("Position: {:?}", wall.position());
    println!("Team: {:?}", wall.team());
    println!(
        "Durability: {}/{}",
        wall.current_durability(),
        wall.max_durability()
    );
    println!("Max Occupants: {}", wall.max_occupants());
    println!("Blocks Movement: {}", wall.blocks_movement());
    println!();

    // ===== Checking Bonuses =====
    println!("2. Structure Bonuses");
    println!("{}", "-".repeat(50));

    println!("Defense Bonus: +{}", wall.defense_bonus());
    println!("Attack Bonus: +{}", wall.attack_bonus());
    println!("Range Bonus: +{}", wall.range_bonus());
    println!("Vision Bonus: +{}", wall.vision_bonus());
    println!("Healing per Turn: {} HP", wall.healing_per_turn());
    println!();

    // ===== Occupying the Structure =====
    println!("3. Occupying the Structure");
    println!("{}", "-".repeat(50));

    let unit1_id = Uuid::new_v4();
    let unit2_id = Uuid::new_v4();

    // First unit occupies
    match wall.add_occupant(unit1_id) {
        Ok(_) => println!("✓ Unit 1 ({}) successfully occupied the wall", unit1_id),
        Err(e) => println!("✗ Failed to occupy: {}", e),
    }

    println!("Current occupants: {}", wall.occupants().len());
    println!("Has space: {}", wall.has_space());

    // Second unit tries to occupy (should fail - only 1 allowed)
    match wall.add_occupant(unit2_id) {
        Ok(_) => println!("✓ Unit 2 ({}) successfully occupied the wall", unit2_id),
        Err(e) => println!("\n✗ Unit 2 cannot occupy: {}", e),
    }

    println!();

    // ===== Taking Damage =====
    println!("4. Combat Damage");
    println!("{}", "-".repeat(50));

    let initial_durability = wall.current_durability();

    // Normal attack (not very effective against stone walls)
    let normal_damage = wall.take_damage(50, false);
    println!("Normal attack dealt {} damage", normal_damage);
    println!(
        "Durability: {}/{} (-{})",
        wall.current_durability(),
        wall.max_durability(),
        initial_durability - wall.current_durability()
    );

    // Siege attack (very effective!)
    let before_siege = wall.current_durability();
    let siege_damage = wall.take_damage(50, true);
    println!(
        "\nSiege attack dealt {} damage (2.5x multiplier!)",
        siege_damage
    );
    println!(
        "Durability: {}/{} (-{})",
        wall.current_durability(),
        wall.max_durability(),
        before_siege - wall.current_durability()
    );
    println!();

    // ===== Repairing =====
    println!("5. Repairing the Structure");
    println!("{}", "-".repeat(50));

    // Manual repair
    let repaired = wall.repair(100);
    println!("Manually repaired {} durability", repaired);
    println!(
        "Durability: {}/{}",
        wall.current_durability(),
        wall.max_durability()
    );

    // Auto repair (happens each turn when occupied)
    let auto_repaired = wall.auto_repair();
    println!(
        "\nAuto-repair: {} durability (happens each turn while occupied)",
        auto_repaired
    );
    println!(
        "Durability: {}/{}",
        wall.current_durability(),
        wall.max_durability()
    );
    println!();

    // ===== Leaving the Structure =====
    println!("6. Units Leaving the Structure");
    println!("{}", "-".repeat(50));

    let removed = wall.remove_occupant(unit1_id);
    println!("Unit 1 left: {}", removed);

    println!("\nCurrent occupants: {}", wall.occupants().len());
    println!("Has space: {}", wall.has_space());
    println!();

    // ===== Movement Blocking =====
    println!("7. Movement and Passage");
    println!("{}", "-".repeat(50));

    println!("Blocks movement: {}", wall.blocks_movement());
    println!(
        "Player team can pass: {}",
        wall.can_pass_through(Team::Player)
    );
    println!(
        "Enemy team can pass: {}",
        wall.can_pass_through(Team::Enemy)
    );
    println!(
        "Neutral team can pass: {}",
        wall.can_pass_through(Team::Neutral)
    );
    println!();

    // ===== Terrain Restrictions =====
    println!("8. Terrain Building Restrictions");
    println!("{}", "-".repeat(50));

    use units::Terrain;

    println!("Can build on:");
    for terrain in wall.buildable_on() {
        println!("  - {:?}", terrain);
    }

    println!("\nTerrain checks:");
    println!("  Grasslands: {}", wall.can_build_on(Terrain::Grasslands));
    println!("  Hills: {}", wall.can_build_on(Terrain::Hills));
    println!("  Mountain: {}", wall.can_build_on(Terrain::Mountain));
    println!("  Swamp: {}", wall.can_build_on(Terrain::Swamp));
    println!();

    // ===== Destruction =====
    println!("9. Destroying the Structure");
    println!("{}", "-".repeat(50));

    // Deal massive siege damage to destroy it
    while !wall.is_destroyed() {
        let damage = wall.take_damage(100, true);
        println!(
            "Siege attack dealt {} damage. Durability: {}/{}",
            damage,
            wall.current_durability(),
            wall.max_durability()
        );
    }

    println!("\n✗ Structure destroyed!");
    println!("Is destroyed: {}", wall.is_destroyed());
    println!();

    // ===== Summary =====
    println!("=== Summary ===");
    println!("{}", "-".repeat(50));
    println!("Stone walls are powerful defensive structures that:");
    println!("  • Block all movement until destroyed");
    println!("  • Provide +15 defense bonus to occupant");
    println!("  • Can hold only 1 defending unit");
    println!("  • Have 500 durability with strong resistances");
    println!("  • Are vulnerable to siege weapons (2.5x damage)");
    println!("  • Auto-repair when occupied (5 HP/turn)");
    println!("\nUse walls to create defensive chokepoints and protect your territory!");
}
