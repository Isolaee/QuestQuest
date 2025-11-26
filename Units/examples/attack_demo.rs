use combat::DamageType;
use graphics::HexCoord;
/// Example of how to use the attack system with units
///
/// This demonstrates:
/// 1. Creating a veteran warrior with default attacks
/// 2. Accessing the unit's attacks
/// 3. Adding custom attacks to veteran units
use units::{Attack, DwarfVeteranWarrior, Terrain, Unit};

fn main() {
    // Create a dwarf veteran warrior (Level 3) at position (0, 0)
    let warrior = DwarfVeteranWarrior::new(
        "Thorin".to_string(),
        HexCoord { q: 0, r: 0 },
        Terrain::Grasslands,
    );

    // Print the warrior's default attacks
    println!("=== {}'s Default Attacks ===", warrior.name());
    for attack in warrior.get_attacks() {
        println!(
            "  {} - Damage: {}, Type: {:?}, Range: {}",
            attack.name, attack.damage, attack.damage_type, attack.range
        );
        println!("    {}", attack.description);
    }

    // Add a custom attack
    // Note: With the new architecture, add_attack would need to be implemented on the unit
    // or use BaseUnit's static methods. For now, this example just shows the default attacks.
    let power_attack = Attack::new(
        "Devastating Swing",
        25,
        DamageType::Slash,
        1,
        "A powerful overhead strike that deals heavy damage",
    );

    // To add attacks with new architecture, the unit would expose a method like:
    // warrior.add_custom_attack(power_attack);
    // For this demo, we'll just print what would be added:
    println!("\n=== Would Add Custom Attack ===");
    println!(
        "  {} - Damage: {}, Type: {:?}",
        power_attack.name, power_attack.damage, power_attack.damage_type
    );

    // Check if an attack can reach a target
    if let Some(attack) = warrior.get_attacks().first() {
        println!("\n=== Range Check for '{}' ===", attack.name);
        println!("  Can reach distance 1: {}", attack.can_reach(1));
        println!("  Can reach distance 2: {}", attack.can_reach(2));
    }
}
