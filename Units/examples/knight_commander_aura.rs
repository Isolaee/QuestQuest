//! Demonstration of Human Knight Commander's Leadership Aura

use graphics::HexCoord;
use units::{Terrain, UnitFactory};

fn main() {
    println!("=== Human Knight Commander Leadership Aura Demo ===\n");

    // Create a Knight Commander at position (0, 0)
    let commander = UnitFactory::create(
        "Human Knight Commander",
        Some("Sir Roland".to_string()),
        Some(HexCoord::new(0, 0)),
        Some(Terrain::Grasslands),
    )
    .expect("Failed to create Knight Commander");

    println!("Created: {}", commander.name());
    println!("Unit Type: {}", commander.unit_type());
    println!("Position: {:?}", commander.position());
    println!("Level: {}", commander.level());
    println!(
        "Base Attack: {}\n",
        commander.combat_stats().get_total_attack()
    );

    // Check abilities
    println!("--- Abilities ---");
    println!("Total abilities: {}", commander.abilities().len());

    let auras = commander.get_aura_abilities();
    println!("Aura abilities: {}", auras.len());

    if !auras.is_empty() {
        println!("\nAura Details:");
        for aura in auras {
            println!("  Name: {}", aura.name);
            println!("  Description: {}", aura.description);
            println!("  Range: {} hex", aura.range);
            println!("  Target: {:?}", aura.target_type);
            println!("  Effect: {:?}", aura.effect);
        }
    }

    // Demonstrate range
    println!("\n--- Adjacent Positions (affected by aura) ---");
    let adjacent_positions = vec![
        HexCoord::new(1, 0),  // Right
        HexCoord::new(-1, 0), // Left
        HexCoord::new(0, 1),  // Down-Right
        HexCoord::new(0, -1), // Up-Left
        HexCoord::new(1, -1), // Up-Right
        HexCoord::new(-1, 1), // Down-Left
    ];

    for pos in &adjacent_positions {
        let distance = commander.position().distance(*pos);
        println!(
            "  Position {:?} - Distance: {} - Affected: {}",
            pos,
            distance,
            distance <= 1
        );
    }

    // Test with a far position
    let far_position = HexCoord::new(3, 3);
    let far_distance = commander.position().distance(far_position);
    println!(
        "\n  Far Position {:?} - Distance: {} - Affected: {}",
        far_position,
        far_distance,
        far_distance <= 1
    );

    println!("\n--- Summary ---");
    println!("The Knight Commander provides a +1 attack bonus to all allied units");
    println!("within 1 hex (adjacent positions). This leadership aura makes the");
    println!("Knight Commander a valuable tactical unit for supporting frontline troops.\n");

    println!("=== Demo Complete ===");
}
