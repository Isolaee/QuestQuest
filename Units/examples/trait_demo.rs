/// Example demonstrating the trait-based unit system
/// This example shows how to use UnitFactory to create units and work with them
/// polymorphically using the Unit trait.
use graphics::HexCoord;
use units::{Race, Terrain, Unit, UnitClass, UnitFactory};

fn main() {
    println!("🎮 Trait-Based Unit System Demo");
    println!("================================\n");

    // Create units using the factory
    println!("Creating units with UnitFactory...\n");

    let warrior = UnitFactory::create_unit(
        "Thorin the Bold".to_string(),
        HexCoord::new(0, 0),
        Race::Dwarf,
        UnitClass::Warrior,
        Terrain::Mountain,
    );

    let archer = UnitFactory::create_unit(
        "Legolas Greenleaf".to_string(),
        HexCoord::new(3, -2),
        Race::Elf,
        UnitClass::Archer,
        Terrain::Forest0,
    );

    let mage = UnitFactory::create_unit_with_level(
        "Gandalf the Grey".to_string(),
        HexCoord::new(-2, 3),
        Race::Human,
        UnitClass::Mage,
        5,   // level
        500, // experience
        Terrain::Grasslands,
    );

    // Display unit information using trait methods
    println!("⚔️ CREATED UNITS:");
    warrior.display_unit_info();
    archer.display_unit_info();
    mage.display_unit_info();

    // Demonstrate polymorphism - store units in a Vec<Box<dyn Unit>>
    println!("\n📦 POLYMORPHIC STORAGE:");
    let mut units: Vec<Box<dyn Unit>> = vec![warrior, archer, mage];

    for unit in &units {
        println!("  - {}", unit.get_summary());
    }

    // Demonstrate trait methods
    println!("\n🎯 TRAIT METHODS DEMO:");

    // Check movement
    let target_pos = HexCoord::new(1, 1);
    if let Some(unit) = units.get_mut(0) {
        println!("\n{} attempting to move to {:?}", unit.name(), target_pos);
        if unit.can_move_to(target_pos) {
            if unit.move_to(target_pos) {
                println!("✅ Move successful! New position: {:?}", unit.position());
            }
        } else {
            println!(
                "❌ Target too far! Movement range: {}",
                unit.combat_stats().movement_speed
            );
        }
    }

    // Check attack range
    if let Some(unit) = units.get(1) {
        println!("\n{} checking attack range:", unit.name());
        let enemy_pos = HexCoord::new(5, -3);
        if unit.can_attack(enemy_pos) {
            println!("✅ Can attack target at {:?}", enemy_pos);
        } else {
            println!(
                "❌ Target out of range! Attack range: {}",
                unit.combat_stats().attack_range
            );
        }
    }

    // Experience and leveling
    if let Some(unit) = units.get_mut(0) {
        println!("\n{} gaining experience:", unit.name());
        println!("Current level: {}", unit.level());
        println!("Current experience: {}", unit.experience());

        let leveled_up = unit.add_experience(150);
        if leveled_up {
            println!("🌟 LEVEL UP! New level: {}", unit.level());
        } else {
            println!("Progress: {:.1}%", unit.level_progress() * 100.0);
        }
    }

    // Display final status
    println!("\n📊 FINAL STATUS:");
    for unit in &units {
        unit.display_quick_info();
    }

    println!("\n✅ Demo completed!");
}
