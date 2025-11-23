//! Demo of the dynamic unit factory system
//!
//! This example shows how to create units dynamically by name without needing
//! separate factory methods for each unit type.

use graphics::HexCoord;
use units::{Terrain, UnitFactory};

fn main() {
    println!("=== Dynamic Unit Factory Demo ===\n");

    // List all available unit types
    println!("Available unit types:");
    let mut types = UnitFactory::list_types();
    types.sort();
    for (i, unit_type) in types.iter().enumerate() {
        println!("  {}. {}", i + 1, unit_type);
    }
    println!();

    // Create units using dynamic factory with defaults
    println!("Creating units with default settings:");

    match UnitFactory::create("Human Warrior", None, None, None) {
        Ok(unit) => println!("  ✓ Created {} at {:?}", unit.name(), unit.position()),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    match UnitFactory::create("Elf Archer", None, None, None) {
        Ok(unit) => println!("  ✓ Created {} at {:?}", unit.name(), unit.position()),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    match UnitFactory::create("Dwarf Veteran Warrior", None, None, None) {
        Ok(unit) => println!("  ✓ Created {} at {:?}", unit.name(), unit.position()),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!();

    // Create units with custom parameters
    println!("Creating units with custom parameters:");

    match UnitFactory::create(
        "Human Warrior",
        Some("Aragorn".to_string()),
        Some(HexCoord::new(5, 3)),
        Some(Terrain::Grasslands),
    ) {
        Ok(unit) => {
            println!("  ✓ Created {} at {:?}", unit.name(), unit.position());
            println!(
                "    HP: {}, Level: {}",
                unit.combat_stats().health,
                unit.level()
            );
        }
        Err(e) => println!("  ✗ Error: {}", e),
    }

    match UnitFactory::create(
        "Elf Archer",
        Some("Legolas".to_string()),
        Some(HexCoord::new(10, 7)),
        Some(Terrain::Forest0),
    ) {
        Ok(unit) => {
            println!("  ✓ Created {} at {:?}", unit.name(), unit.position());
            println!(
                "    HP: {}, Level: {}",
                unit.combat_stats().health,
                unit.level()
            );
        }
        Err(e) => println!("  ✗ Error: {}", e),
    }

    match UnitFactory::create(
        "Dwarf Veteran Warrior",
        Some("Gimli".to_string()),
        Some(HexCoord::new(3, 8)),
        Some(Terrain::Mountain),
    ) {
        Ok(unit) => {
            println!("  ✓ Created {} at {:?}", unit.name(), unit.position());
            println!(
                "    HP: {}, Level: {}",
                unit.combat_stats().health,
                unit.level()
            );
        }
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!();

    // Try to create an invalid unit type
    println!("Testing error handling:");
    match UnitFactory::create("Dragon King", None, None, None) {
        Ok(unit) => println!("  ✓ Created {}", unit.name()),
        Err(e) => println!("  ✗ Expected error: {}", e),
    }

    println!();

    // List units by race
    println!("Units by race:");
    println!("  Humans: {:?}", UnitFactory::list_by_race("Human"));
    println!("  Elves: {:?}", UnitFactory::list_by_race("Elf"));
    println!("  Dwarves: {:?}", UnitFactory::list_by_race("Dwarf"));
    println!("  Orcs: {:?}", UnitFactory::list_by_race("Orc"));

    println!();

    // List units by class
    println!("Units by class:");
    println!("  Warriors: {:?}", UnitFactory::list_by_class("Warrior"));
    println!("  Archers: {:?}", UnitFactory::list_by_class("Archer"));
    println!("  Mages: {:?}", UnitFactory::list_by_class("Mage"));
    println!("  Swordsmen: {:?}", UnitFactory::list_by_class("Swordsman"));

    println!();

    // Check if unit types exist
    println!("Checking unit existence:");
    println!(
        "  'Human Warrior' exists? {}",
        UnitFactory::exists("Human Warrior")
    );
    println!(
        "  'Dragon King' exists? {}",
        UnitFactory::exists("Dragon King")
    );
}
