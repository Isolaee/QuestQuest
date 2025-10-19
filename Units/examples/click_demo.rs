use ::units::*;
use graphics::HexCoord;

fn main() {
    println!("🖱️  Unit Click Demo");
    println!("==================\n");

    // Create a test unit using the new system
    let mut warrior = UnitFactory::create_unit(
        "Thorin Ironbeard".to_string(),
        HexCoord::new(2, -1),
        Race::Dwarf,
        UnitClass::Warrior,
        Terrain::Mountain,
    );

    // Add some experience and level up for demonstration
    warrior.add_experience(150);
    warrior.add_experience(250); // Should level up to level 2

    // Damage the unit to show health in different states
    let max_health = warrior.combat_stats().max_health;
    warrior.take_damage((max_health * 2 / 3) as u32); // Take 2/3 damage, leaving 1/3 health

    // Create some equipment
    let sword = Item::new(
        "Orcrist".to_string(),
        "An ancient elvish blade that glows blue when orcs are near".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 8,
            attacks: Vec::new(),
            range_modifier: 0,
            range_type_override: None,
        },
    );

    let armor = Item::new(
        "Mithril Chainmail".to_string(),
        "Lightweight but incredibly strong armor made of mithril".to_string(),
        ItemProperties::Armor {
            defense_bonus: 12,
            movement_penalty: -1, // Light armor, minimal penalty
        },
    );

    let ring = Item::new(
        "Ring of Power".to_string(),
        "A magical ring that enhances the wearer's abilities".to_string(),
        ItemProperties::Accessory {
            attack_bonus: 2,
            defense_bonus: 2,
            health_bonus: 10,
            movement_bonus: 1,
        },
    );

    // Add items to inventory
    warrior.add_item_to_inventory(sword.clone());
    warrior.add_item_to_inventory(armor.clone());
    warrior.add_item_to_inventory(ring.clone());

    // Equip some items
    let _ = warrior.equip_item(sword.id);
    let _ = warrior.equip_item(armor.id);
    let _ = warrior.equip_item(ring.id);

    // Add some extra inventory items
    let potion = Item::new(
        "Health Potion".to_string(),
        "Restores 50 health points".to_string(),
        ItemProperties::Consumable {
            uses: 3,
            effect: ConsumableEffect::Heal { amount: 50 },
        },
    );
    warrior.add_item_to_inventory(potion);

    println!("Click on the unit to see detailed information:");
    println!("───────────────────────────────────────────────");

    // Simulate clicking on the unit
    warrior.on_click();

    println!("\nFor comparison, here's the quick info:");
    println!("─────────────────────────────────────");
    warrior.display_quick_info();

    // Test with a healthy unit
    println!("\n\n🖱️  Second Unit (Full Health)");
    println!("════════════════════════════════\n");

    let mut archer = UnitFactory::create_unit(
        "Legolas Greenleaf".to_string(),
        HexCoord::new(0, 3),
        Race::Elf,
        UnitClass::Archer,
        Terrain::Forest0,
    );

    archer.add_experience(250); // Add experience
    archer.on_click();
}
