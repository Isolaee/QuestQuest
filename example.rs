use combat::resolve_combat;
use graphics::HexCoord;
use items::{Item, ItemProperties, RangeType};
use units::unit_factory::UnitFactory;
use units::unit_race::Terrain;
use units::unit_trait::Unit;

fn main() {
    println!("🎮 QuestQuest: Hexagonal Game Engine Demo");
    println!("==========================================\n");

    // Create some units on a hexagonal grid using UnitFactory
    let mut warrior = UnitFactory::create_dwarf_warrior(
        "Thorin the Bold".to_string(),
        HexCoord::new(0, 0),
        Terrain::Mountain,
    );

    let mut archer = UnitFactory::create_elf_archer(
        "Legolas Greenleaf".to_string(),
        HexCoord::new(3, -2),
        Terrain::Forest0,
    );

    let mut mage = UnitFactory::create_human_mage(
        "Gandalf the Grey".to_string(),
        HexCoord::new(-2, 3),
        Terrain::Grasslands,
    );

    println!("⚔️ INITIAL UNITS:");
    print_unit_status(&*warrior);
    print_unit_status(&*archer);
    print_unit_status(&*mage);

    // Create some equipment
    let sword = Item::new(
        "Orcrist".to_string(),
        "An ancient elvish blade".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 5,
            range_modifier: 0,
            range_type_override: None,
            attacks: Vec::new(), // No special attacks for this example
        },
    );

    let longbow = Item::new(
        "Bow of the Galadhrim".to_string(),
        "A bow crafted in Lothlórien".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 3,
            range_modifier: 2,
            range_type_override: None,
            attacks: Vec::new(), // No special attacks for this example
        },
    );

    let staff = Item::new(
        "Staff of Power".to_string(),
        "A wizard's staff imbued with magic".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 2,
            range_modifier: 1,
            range_type_override: Some(RangeType::Ranged),
            attacks: Vec::new(), // No special attacks for this example
        },
    );

    // Equip items
    let sword_id = sword.id;
    let bow_id = longbow.id;
    let staff_id = staff.id;

    warrior.add_item_to_inventory(sword);
    archer.add_item_to_inventory(longbow);
    mage.add_item_to_inventory(staff);

    warrior.equip_item(sword_id).unwrap();
    archer.equip_item(bow_id).unwrap();
    mage.equip_item(staff_id).unwrap();

    println!("\n🛡️ AFTER EQUIPPING WEAPONS:");
    print_unit_status(&*warrior);
    print_unit_status(&*archer);
    print_unit_status(&*mage);

    // Simulate some combat
    println!("\n⚔️ COMBAT SIMULATION:");

    // Check if units can attack each other
    let archer_pos = archer.position();
    let warrior_pos = warrior.position();
    let mage_pos = mage.position();

    println!(
        "Distance from Archer to Warrior: {}",
        archer_pos.distance(warrior_pos)
    );
    println!(
        "Can Archer attack Warrior? {}",
        archer.can_attack(warrior_pos)
    );

    if archer.can_attack(warrior_pos) {
        let damage_type = archer
            .get_attacks()
            .first()
            .map(|a| a.damage_type)
            .unwrap_or(combat::DamageType::Slash);
        let result = resolve_combat(
            archer.combat_stats_mut(),
            warrior.combat_stats_mut(),
            damage_type,
        );

        if result.attacker_hit {
            println!(
                "Archer shoots at Warrior for {} damage!",
                result.attacker_damage_dealt
            );
        } else {
            println!("Archer's shot missed!");
        }
    }

    println!(
        "Distance from Warrior to Mage: {}",
        warrior_pos.distance(mage_pos)
    );
    println!("Can Warrior attack Mage? {}", warrior.can_attack(mage_pos));

    if !warrior.can_attack(mage_pos) {
        println!("Warrior moves closer to Mage...");
        // Move warrior closer
        let new_pos = HexCoord::new(-1, 2);
        warrior.move_to(new_pos);
        println!("Warrior moved to {:?}", warrior.position());

        if warrior.can_attack(mage_pos) {
            let damage_type = warrior
                .get_attacks()
                .first()
                .map(|a| a.damage_type)
                .unwrap_or(combat::DamageType::Slash);
            let result = resolve_combat(
                warrior.combat_stats_mut(),
                mage.combat_stats_mut(),
                damage_type,
            );

            if result.attacker_hit {
                println!(
                    "Warrior attacks Mage for {} damage!",
                    result.attacker_damage_dealt
                );
            } else {
                println!("Warrior's attack missed!");
            }
        }
    }

    // Level up demonstration
    println!("\n📈 LEVELING UP:");
    println!("Giving experience to all units...");

    if warrior.add_experience(100) {
        println!(
            "🎉 {} leveled up to level {}!",
            warrior.name(),
            warrior.level()
        );
    }

    if archer.add_experience(100) {
        println!(
            "🎉 {} leveled up to level {}!",
            archer.name(),
            archer.level()
        );
    }

    if mage.add_experience(100) {
        println!("🎉 {} leveled up to level {}!", mage.name(), mage.level());
    }

    println!("\n🏆 FINAL STATUS:");
    print_unit_status(&*warrior);
    print_unit_status(&*archer);
    print_unit_status(&*mage);

    // Demonstrate hexagonal coordinates
    println!("\n🗺️ HEXAGONAL GRID DEMONSTRATION:");
    println!("Current positions:");
    println!("  {}: {:?}", warrior.name(), warrior.position());
    println!("  {}: {:?}", archer.name(), archer.position());
    println!("  {}: {:?}", mage.name(), mage.position());

    println!("\nNeighbors of Warrior's position:");
    for (i, neighbor) in warrior.position().neighbors().iter().enumerate() {
        println!("  Direction {}: {:?}", i, neighbor);
    }

    println!("\nHexagonal distance examples:");
    let test_coords = [
        HexCoord::new(1, 0),
        HexCoord::new(0, 1),
        HexCoord::new(-1, 1),
        HexCoord::new(-1, 0),
        HexCoord::new(0, -1),
        HexCoord::new(1, -1),
    ];

    println!("Distance from Warrior to nearby hexes:");
    for coord in test_coords {
        let distance = warrior.position().distance(coord);
        println!("  {:?} -> distance: {}", coord, distance);
    }
}

fn print_unit_status(unit: &dyn Unit) {
    let equipment = unit.equipment();
    let weapon_name = equipment
        .weapon
        .as_ref()
        .map(|w| w.name.as_str())
        .unwrap_or("None");

    let stats = unit.combat_stats();
    println!(
        "📋 {} (Lv.{} {:?} {:?}):",
        unit.name(),
        unit.level(),
        unit.race(),
        unit.unit_type()
    );
    println!(
        "   Position: {:?} | Health: {}/{} | Attack: {} | Range: {} ({:?})",
        unit.position(),
        stats.health,
        stats.max_health,
        stats.get_total_attack(),
        stats.attack_range,
        stats.range_category
    );
    println!(
        "   Movement: {} | Weapon: {} | XP: {}",
        stats.movement_speed,
        weapon_name,
        unit.experience()
    );
    println!();
}
