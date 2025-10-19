use graphics::HexCoord;
use units::*;

fn main() {
    println!("🖱️  Unit Click Demo");
    println!("==================\n");

    // Create a test unit
    let mut warrior = Unit::new(
        "Thorin Ironbeard".to_string(),
        HexCoord::new(2, -1),
        unit_race::Race::Dwarf,
        unit_class::UnitClass::Warrior,
        unit_race::Terrain::Mountain,
    );

    // Add some experience and damage for demonstration
    warrior.experience = 150;
    warrior.level = 2;
    warrior.recalculate_stats();

    // Damage the unit to show health bar
    warrior.combat_stats.health = warrior.combat_stats.max_health / 3; // 1/3 health remaining

    // Create some equipment
    let sword = Item::new(
        "Orcrist".to_string(),
        "An ancient elvish blade that glows blue when orcs are near".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 8,
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

    let mut archer = Unit::new(
        "Legolas Greenleaf".to_string(),
        HexCoord::new(0, 3),
        unit_race::Race::Elf,
        unit_class::UnitClass::Archer,
        unit_race::Terrain::Forest0,
    );

    archer.experience = 250; // Ready to level up
    archer.on_click();
}
