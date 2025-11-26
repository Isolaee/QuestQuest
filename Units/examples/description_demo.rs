//! Demo showcasing unit descriptions for wiki/gameplay display
//!
//! This example demonstrates how to access and display unit descriptions,
//! which provide lore and tactical information about each unit type.
//!
//! Run with: `cargo run --example description_demo`

use graphics::HexCoord;
use units::{Terrain, Unit, UnitFactory};

fn main() {
    println!("=== Unit Description Demo ===\n");
    println!("This demo shows the description property available for all units.");
    println!("Descriptions provide lore and gameplay information for wikis and tooltips.\n");

    // Create various unit types (using registered units only)
    let units = vec![
        UnitFactory::create("Human Warrior", None, None, None)
            .expect("Failed to create Human Warrior"),
        UnitFactory::create("Human Archer", None, None, None)
            .expect("Failed to create Human Archer"),
        UnitFactory::create("Human Mage", None, None, None).expect("Failed to create Human Mage"),
        UnitFactory::create("Elf Warrior", None, None, None).expect("Failed to create Elf Warrior"),
        UnitFactory::create("Elf Archer", None, None, None).expect("Failed to create Elf Archer"),
        UnitFactory::create("Elf Mage", None, None, None).expect("Failed to create Elf Mage"),
        UnitFactory::create("Dwarf Young Warrior", None, None, None)
            .expect("Failed to create Dwarf Young Warrior"),
        UnitFactory::create("Dwarf Warrior", None, None, None)
            .expect("Failed to create Dwarf Warrior"),
        UnitFactory::create("Dwarf Veteran Warrior", None, None, None)
            .expect("Failed to create Dwarf Veteran Warrior"),
        UnitFactory::create("Orc Young Swordsman", None, None, None)
            .expect("Failed to create Orc Young Swordsman"),
        UnitFactory::create("Orc Swordsman", None, None, None)
            .expect("Failed to create Orc Swordsman"),
        UnitFactory::create("Orc Elite Swordsman", None, None, None)
            .expect("Failed to create Orc Elite Swordsman"),
    ];

    // Display each unit's description
    for unit in units.iter() {
        println!("╔═══════════════════════════════════════════════════════════════════════");
        println!("║ Unit: {}", unit.name());
        println!("║ Type: {}", unit.unit_type());
        println!("║ Race: {:?}", unit.race());
        println!("║ Level: {}", unit.level());
        println!("╠═══════════════════════════════════════════════════════════════════════");
        println!("║ Description:");
        println!("║ {}", unit.description());
        println!("╚═══════════════════════════════════════════════════════════════════════");
        println!();
    }

    // Example: Using description in a wiki-style format
    println!("\n=== Wiki-Style Display ===\n");

    let warrior = UnitFactory::create(
        "Human Warrior",
        Some("Sir Galahad".to_string()),
        Some(HexCoord::new(5, 5)),
        Some(Terrain::Grasslands),
    )
    .expect("Failed to create warrior");

    display_wiki_entry(&*warrior);
}

fn display_wiki_entry(unit: &dyn Unit) {
    println!("┌─────────────────────────────────────────────────────────────────────┐");
    println!("│                        UNIT WIKI ENTRY                              │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ Name: {:60} │", unit.name());
    println!("│ Type: {:60} │", unit.unit_type());
    println!("│ Race: {:60} │", format!("{:?}", unit.race()));
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ DESCRIPTION                                                         │");
    println!("│                                                                     │");

    // Word wrap the description to fit in the box
    let desc = unit.description();
    let max_width = 67;
    let words: Vec<&str> = desc.split_whitespace().collect();
    let mut current_line = String::new();

    for word in words {
        if current_line.len() + word.len() + 1 > max_width {
            println!("│ {:<67} │", current_line);
            current_line = word.to_string();
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        println!("│ {:<67} │", current_line);
    }

    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ STATS                                                               │");
    println!(
        "│ Level: {:12}  HP: {:15}  Attack: {:15} │",
        unit.level(),
        unit.combat_stats().health,
        unit.combat_stats().attack_strength
    );
    println!(
        "│ Movement: {:9}  Defense: {:37} │",
        unit.combat_stats().movement_speed,
        unit.get_defense()
    );
    println!("└─────────────────────────────────────────────────────────────────────┘");
}
