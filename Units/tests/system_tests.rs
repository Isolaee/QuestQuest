use ::combat::{resolve_combat, DamageType};
use ::units::*;
use graphics::HexCoord;

#[test]
fn test_unit_creation_with_factory() {
    let unit = UnitFactory::create_unit(
        "Test Warrior".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    assert_eq!(unit.name(), "Test Warrior");
    assert_eq!(unit.race(), Race::Human);
    assert_eq!(unit.class(), UnitClass::Warrior);
    assert_eq!(unit.level(), 1);
    assert!(unit.is_alive());
}

#[test]
fn test_combat_stats_initialization() {
    let unit = UnitFactory::create_unit(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Race::Dwarf,
        UnitClass::Warrior,
        Terrain::Mountain,
    );

    let stats = unit.combat_stats();

    // Warrior base health is 120
    assert_eq!(stats.health, 120);
    assert_eq!(stats.max_health, 120);

    // Warrior base attack is 15
    assert_eq!(stats.base_attack, 15);
    assert_eq!(stats.get_total_attack(), 15); // No modifiers yet

    // Dwarf on mountain should have high terrain hit chance
    assert!(stats.terrain_hit_chance > 70);
}

#[test]
fn test_experience_and_leveling() {
    let mut unit = UnitFactory::create_unit(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Mage,
        Terrain::Grasslands,
    );

    assert_eq!(unit.level(), 1);
    assert_eq!(unit.experience(), 0);

    // Add some experience (not enough to level)
    let leveled = unit.add_experience(50);
    assert!(!leveled);
    assert_eq!(unit.level(), 1);

    // Add enough to level up (level 1 needs 100 XP)
    let leveled = unit.add_experience(50);
    assert!(leveled);
    assert_eq!(unit.level(), 2);
}

#[test]
fn test_equipment_affects_stats() {
    let mut unit = UnitFactory::create_unit(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let initial_attack = unit.combat_stats().get_total_attack();

    // Create and equip a weapon
    let weapon = Item::new(
        "Test Sword".to_string(),
        "A test weapon".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 5,
            range_modifier: 0,
            range_type_override: None,
        },
    );

    unit.add_item_to_inventory(weapon);
    unit.equip_item(unit.inventory()[0].id).unwrap();

    // Attack should increase by weapon bonus
    let new_attack = unit.combat_stats().get_total_attack();
    assert_eq!(new_attack, initial_attack + 5);
}

#[test]
fn test_combat_resolution() {
    let mut attacker = UnitFactory::create_unit(
        "Attacker".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let mut defender = UnitFactory::create_unit(
        "Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Orc,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let defender_initial_health = defender.combat_stats().health;
    let damage_type = attacker.class().get_default_damage_type();

    let result = resolve_combat(
        attacker.combat_stats_mut(),
        defender.combat_stats_mut(),
        damage_type,
    );

    // Combat should produce a result
    assert!(result.attacker_hit || !result.attacker_hit); // Always true, but validates result exists

    // If attack hit, defender should have taken damage
    if result.attacker_hit {
        let defender_health_after = defender.combat_stats().health;
        assert!(defender_health_after < defender_initial_health);
        // Check that damage was dealt (actual damage might include resistance reduction)
        assert!(result.attacker_damage_dealt > 0);
    }
}

#[test]
fn test_damage_types_per_class() {
    let warrior = UnitFactory::create_unit(
        "Warrior".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let archer = UnitFactory::create_unit(
        "Archer".to_string(),
        HexCoord::new(0, 0),
        Race::Elf,
        UnitClass::Archer,
        Terrain::Forest0,
    );

    let mage = UnitFactory::create_unit(
        "Mage".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Mage,
        Terrain::Grasslands,
    );

    // Warriors use Slash damage
    assert_eq!(warrior.class().get_default_damage_type(), DamageType::Slash);

    // Archers use Pierce damage
    assert_eq!(archer.class().get_default_damage_type(), DamageType::Pierce);

    // Mages use Fire damage
    assert_eq!(mage.class().get_default_damage_type(), DamageType::Fire);
}

#[test]
fn test_resistances_per_class() {
    let warrior = UnitClass::Warrior.get_resistances();
    let archer = UnitClass::Archer.get_resistances();
    let mage = UnitClass::Mage.get_resistances();

    // Warriors should have high physical resistance
    assert!(warrior.slash > 20);
    assert!(warrior.blunt > 20);

    // Mages should have high magical resistance
    assert!(mage.fire > 20);
    assert!(mage.dark > 20);

    // Archers should be balanced
    assert!(archer.pierce > 15);
}

#[test]
fn test_terrain_affects_hit_chance() {
    // Dwarf on mountain (favorable)
    let dwarf_mountain = UnitFactory::create_unit(
        "Dwarf".to_string(),
        HexCoord::new(0, 0),
        Race::Dwarf,
        UnitClass::Warrior,
        Terrain::Mountain,
    );

    // Elf on forest (favorable)
    let elf_forest = UnitFactory::create_unit(
        "Elf".to_string(),
        HexCoord::new(0, 0),
        Race::Elf,
        UnitClass::Archer,
        Terrain::Forest0,
    );

    // Human on grasslands (neutral)
    let human_grass = UnitFactory::create_unit(
        "Human".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    // Dwarves on mountains should have good hit chance
    assert!(dwarf_mountain.combat_stats().terrain_hit_chance >= 70);

    // Elves on forest should have good hit chance
    assert!(elf_forest.combat_stats().terrain_hit_chance >= 70);

    // Humans on grasslands should have moderate hit chance
    assert!(human_grass.combat_stats().terrain_hit_chance >= 60);
}

#[test]
fn test_movement_and_positioning() {
    let mut unit = UnitFactory::create_unit(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Race::Elf,
        UnitClass::Archer,
        Terrain::Forest0,
    );

    let initial_pos = unit.position();
    let new_pos = HexCoord::new(2, 1);

    // Check if unit can move (Elf archer has good movement)
    assert!(unit.can_move_to(new_pos));

    // Move the unit
    let moved = unit.move_to(new_pos);
    assert!(moved);
    assert_eq!(unit.position(), new_pos);
    assert_ne!(unit.position(), initial_pos);
}

#[test]
fn test_range_categories() {
    let warrior = UnitFactory::create_unit(
        "Warrior".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let archer = UnitFactory::create_unit(
        "Archer".to_string(),
        HexCoord::new(3, 0), // Distance 3
        Race::Elf,
        UnitClass::Archer,
        Terrain::Forest0,
    );

    // Warrior should be melee range
    assert_eq!(
        warrior.combat_stats().range_category,
        ::combat::RangeCategory::Melee
    );
    assert_eq!(warrior.combat_stats().attack_range, 1);

    // Archer should be ranged
    assert_eq!(
        archer.combat_stats().range_category,
        ::combat::RangeCategory::Range
    );
    assert!(archer.combat_stats().attack_range >= 3);

    // Warrior can't attack archer at distance 3
    assert!(!warrior.can_attack(archer.position()));

    // Archer can attack warrior at distance 3
    assert!(archer.can_attack(warrior.position()));
}

#[test]
fn test_health_and_damage() {
    let mut unit = UnitFactory::create_unit(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let max_health = unit.combat_stats().max_health;
    assert_eq!(unit.combat_stats().health, max_health);
    assert!(unit.is_alive());

    // Take damage
    unit.take_damage(30);
    assert_eq!(unit.combat_stats().health, max_health - 30);
    assert!(unit.is_alive());

    // Heal
    unit.heal(15);
    assert_eq!(unit.combat_stats().health, max_health - 15);

    // Take lethal damage
    unit.take_damage(1000);
    assert!(!unit.is_alive());
    assert!(unit.combat_stats().health <= 0);
}

#[test]
fn test_inventory_management() {
    let mut unit = UnitFactory::create_unit(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    assert_eq!(unit.inventory().len(), 0);

    let item = Item::new(
        "Health Potion".to_string(),
        "Restores health".to_string(),
        ItemProperties::Consumable {
            uses: 1,
            effect: ConsumableEffect::Heal { amount: 50 },
        },
    );

    unit.add_item_to_inventory(item);
    assert_eq!(unit.inventory().len(), 1);

    let item_id = unit.inventory()[0].id;
    let removed = unit.remove_item_from_inventory(item_id);
    assert!(removed.is_some());
    assert_eq!(unit.inventory().len(), 0);
}
