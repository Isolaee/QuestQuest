//! Encyclopedia System Demo
//!
//! This example demonstrates the dynamic encyclopedia system that generates
//! comprehensive documentation for all game content at runtime.
//!
//! Run with: `cargo run --package encyclopedia --example encyclopedia_demo`

use encyclopedia::Encyclopedia;

fn main() {
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("                    QUESTQUEST ENCYCLOPEDIA DEMO");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("\nInitializing encyclopedia (loading all game content)...\n");

    // Create encyclopedia - this dynamically loads all units, terrain, and mechanics
    let encyclopedia = Encyclopedia::new();

    // Display main index
    encyclopedia.display_index();

    println!("\n\nPress Enter to view the Unit Encyclopedia...");
    wait_for_enter();

    // Show unit index
    encyclopedia.display_unit_index();

    println!("\n\nPress Enter to view example unit entries...");
    wait_for_enter();

    // Show some example unit entries
    let example_units = vec![
        "Human Warrior",
        "Elf Archer",
        "Dwarf Veteran Warrior",
        "Orc Swordsman",
    ];

    for unit_name in example_units {
        println!("\n");
        if let Some(entry) = encyclopedia.get_unit_entry(unit_name) {
            entry.display();
        }
        println!("\n");
    }

    println!("\nPress Enter to view the Terrain Guide...");
    wait_for_enter();

    // Show terrain guide
    encyclopedia.display_terrain_guide();

    println!("\n\nPress Enter to view example terrain entries...");
    wait_for_enter();

    // Show some terrain entries
    let example_terrain = vec!["Grasslands", "Forest", "Mountain"];

    for terrain_name in example_terrain {
        println!("\n");
        if let Some(entry) = encyclopedia.get_terrain_entry(terrain_name) {
            entry.display();
        }
    }

    println!("\n\nPress Enter to view the Game Mechanics Index...");
    wait_for_enter();

    // Show mechanics index
    encyclopedia.display_mechanics_index();

    println!("\n\nPress Enter to view example mechanic entries...");
    wait_for_enter();

    // Show some mechanic entries
    let example_mechanics = vec!["Combat System", "Experience & Leveling", "Damage Types"];

    for mechanic_name in example_mechanics {
        println!("\n");
        if let Some(entry) = encyclopedia.get_mechanic_entry(mechanic_name) {
            entry.display();
        }
    }

    // Demonstrate search functionality
    println!("\n\nPress Enter to demonstrate search functionality...");
    wait_for_enter();

    println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║                         🔍 SEARCH DEMO                                 ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝\n");

    let search_queries = vec!["warrior", "forest", "damage"];

    for query in search_queries {
        println!("Searching for: \"{}\"", query);
        let results = encyclopedia.search(query);
        println!("  Found {} results:", results.len());
        for result in results.iter().take(3) {
            println!("    - {} ({})", result.title(), result.category());
        }
        println!();
    }

    // Demonstrate filtering
    println!("\nPress Enter to demonstrate filtering by race...");
    wait_for_enter();

    println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║                      🎯 FILTER BY RACE DEMO                            ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝\n");

    let elf_units = encyclopedia.units_by_race(units::Race::Elf);
    println!("Elf Units:");
    for unit in elf_units {
        println!("  • {} - {}", unit.unit_type, unit.class);
    }

    println!("\n\nPress Enter to demonstrate filtering by class...");
    wait_for_enter();

    let warriors = encyclopedia.units_by_class("Warrior");
    println!("\nWarrior Class Units:");
    for unit in warriors {
        println!("  • {} ({:?})", unit.unit_type, unit.race);
    }

    println!("\n\n╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║                      ✨ ENCYCLOPEDIA DEMO COMPLETE                     ║");
    println!("╠═══════════════════════════════════════════════════════════════════════╣");
    println!("║ The encyclopedia dynamically loaded:                                  ║");
    println!(
        "║   • {} unit types                                                    ║",
        encyclopedia.all_units().len()
    );
    println!(
        "║   • {} terrain types                                                 ║",
        encyclopedia.all_terrain().len()
    );
    println!(
        "║   • {} game mechanics                                                ║",
        encyclopedia.all_mechanics().len()
    );
    println!("║                                                                       ║");
    println!("║ All content was generated at runtime from actual game data!           ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝");
}

fn wait_for_enter() {
    use std::io::{stdin, Read};
    let mut buffer = [0u8; 1];
    let _ = stdin().read(&mut buffer);
}
