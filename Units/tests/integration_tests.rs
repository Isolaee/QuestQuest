use graphics::HexCoord;
use units::*;

#[test]
fn test_unit_creation() {
    let position = HexCoord::new(0, 0);
    let unit = Unit::new(
        "Test Warrior".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    assert_eq!(unit.name, "Test Warrior");
    assert_eq!(unit.position, position);
    assert_eq!(unit.race, Race::Human);
    assert_eq!(unit.class, UnitClass::Warrior);
    assert_eq!(unit.level, 1);
    assert_eq!(unit.experience, 0);
    assert!(unit.is_alive());
}

#[test]
fn test_unit_builder() {
    let position = HexCoord::new(1, 1);
    let unit = units::unit::UnitBuilder::new("Built Unit", position, Race::Elf, UnitClass::Archer)
        .with_level(3)
        .with_experience(500)
        .build();

    assert_eq!(unit.name, "Built Unit");
    assert_eq!(unit.level, 3);
    assert_eq!(unit.experience, 500);
    assert_eq!(unit.race, Race::Elf);
    assert_eq!(unit.class, UnitClass::Archer);
}

#[test]
fn test_combat_stats_calculation() {
    let position = HexCoord::new(0, 0);
    let unit = Unit::new(
        "Fighter".to_string(),
        position,
        Race::Dwarf,        // +2 defense, +0 attack
        UnitClass::Warrior, // +3 defense, +2 attack
        Terrain::Grasslands,
    );

    // Dwarf Warrior should have base attack (2+0=2)
    // Defense is now terrain-based: Dwarf in Grasslands = 54% hit chance
    assert_eq!(unit.combat_stats.attack, 2);
    assert_eq!(unit.get_defense(), 54); // Terrain-based defense
}

#[test]
fn test_equipment_bonuses() {
    let position = HexCoord::new(0, 0);
    let mut unit = Unit::new(
        "Equipped Fighter".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    // Create a weapon with attack bonus
    let sword = Item::new(
        "Iron Sword".to_string(),
        "A sturdy iron sword".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 3,
            range_modifier: 0,
            range_type_override: None,
        },
    );

    let sword_id = sword.id;
    unit.add_item_to_inventory(sword);
    let _ = unit.equip_item(sword_id);

    // Check that attack bonus is applied
    assert!(unit.combat_stats.attack >= 5); // Base 2 + sword 3
}

#[test]
fn test_experience_and_leveling() {
    let position = HexCoord::new(0, 0);
    let mut unit = Unit::new(
        "Leveling Unit".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let initial_health = unit.combat_stats.max_health;

    // Add enough experience to level up (level 1 needs 100 exp for level 2)
    let leveled_up = unit.add_experience(100);

    assert!(leveled_up);
    assert_eq!(unit.level, 2);
    assert!(unit.combat_stats.max_health > initial_health); // Health should increase
    assert_eq!(unit.combat_stats.health, unit.combat_stats.max_health); // Should be full health after level up
}

#[test]
fn test_movement_validation() {
    let start_pos = HexCoord::new(0, 0);
    let unit = Unit::new(
        "Mover".to_string(),
        start_pos,
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let movement_speed = unit.combat_stats.movement_speed;

    // Should be able to move within movement range
    let close_pos = HexCoord::new(1, 0);
    assert!(unit.can_move_to(close_pos));

    // Should not be able to move beyond movement range
    let far_pos = HexCoord::new(movement_speed + 1, 0);
    assert!(!unit.can_move_to(far_pos));
}

#[test]
fn test_attack_range() {
    let position = HexCoord::new(0, 0);

    // Melee unit
    let warrior = Unit::new(
        "Warrior".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    assert_eq!(warrior.combat_stats.range_type, RangeType::Melee);
    assert!(warrior.can_attack(HexCoord::new(1, 0))); // 1 hex away
    assert!(!warrior.can_attack(HexCoord::new(2, 0))); // 2 hexes away

    // Ranged unit
    let archer = Unit::new(
        "Archer".to_string(),
        position,
        Race::Human,
        UnitClass::Archer,
        Terrain::Grasslands,
    );

    assert_eq!(archer.combat_stats.range_type, RangeType::Ranged);
    assert!(archer.can_attack(HexCoord::new(3, 0))); // 3 hexes away
    assert!(!archer.can_attack(HexCoord::new(4, 0))); // 4 hexes away
}

#[test]
fn test_damage_calculation() {
    let position = HexCoord::new(0, 0);
    let attacker = Unit::new(
        "Attacker".to_string(),
        position,
        Race::Orc,          // +2 attack
        UnitClass::Warrior, // +2 attack
        Terrain::Grasslands,
    );

    let defender = Unit::new(
        "Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Dwarf,        // +2 defense
        UnitClass::Warrior, // +3 defense
        Terrain::Grasslands,
    );

    let damage = attacker.calculate_damage_to(&defender);
    assert!(damage >= 1); // Should always deal at least 1 damage
}

#[test]
fn test_race_bonuses() {
    // Test different racial bonuses
    let races_and_expected = [
        (Race::Human, (0, 48, 0)),  // Balanced - 48% defense in grasslands
        (Race::Elf, (1, 45, 1)),    // +1 attack, 45% defense in grasslands, +1 movement
        (Race::Dwarf, (0, 54, -1)), // +0 attack, 54% defense in grasslands, -1 movement
        (Race::Orc, (2, 52, 0)),    // +2 attack, 52% defense in grasslands, +0 movement
    ];

    for (race, (expected_attack, expected_defense, expected_movement)) in races_and_expected {
        assert_eq!(race.get_attack_bonus(), expected_attack);
        assert_eq!(race.get_base_defense(Terrain::Grasslands), expected_defense);
        assert_eq!(race.get_movement_bonus(), expected_movement);
    }
}

#[test]
fn test_class_abilities() {
    // Test class-specific stats
    let warrior = UnitClass::Warrior;
    let mage = UnitClass::Mage;
    let archer = UnitClass::Archer;

    assert_eq!(warrior.get_default_range(), RangeType::Melee);
    assert_eq!(archer.get_default_range(), RangeType::Ranged);
    assert_eq!(mage.get_default_range(), RangeType::Ranged);

    assert!(warrior.get_base_health() > mage.get_base_health()); // Warriors tankier than mages
    assert!(warrior.get_defense_bonus() > mage.get_defense_bonus()); // Warriors have better defense
}
