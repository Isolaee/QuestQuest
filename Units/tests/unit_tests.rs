// Unit creation and functionality tests using the new system
// Each unit type (HumanWarrior, GoblinGrunt, etc.) defines its own stats

use ::units::*;
use graphics::HexCoord;

// ===== Unit Creation Tests =====

#[test]
fn test_create_human_warrior() {
    let unit = UnitFactory::create_human_warrior(
        "Thorin".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    assert_eq!(unit.name(), "Thorin");
    assert_eq!(unit.race(), Race::Human);
    assert_eq!(unit.unit_type(), "Human Warrior");
    assert_eq!(unit.level(), 1);
    assert!(unit.is_alive());

    let stats = unit.combat_stats();
    assert_eq!(stats.health, 120); // Human Warrior has 120 HP
    assert_eq!(stats.max_health, 120);
}

#[test]
fn test_create_goblin_grunt() {
    let unit =
        UnitFactory::create_goblin_grunt("Gruk".to_string(), HexCoord::new(1, 1), Terrain::Swamp);

    assert_eq!(unit.name(), "Gruk");
    assert_eq!(unit.race(), Race::Goblin);
    assert_eq!(unit.unit_type(), "Goblin Grunt");
    assert_eq!(unit.level(), 1);

    let stats = unit.combat_stats();
    assert_eq!(stats.health, 60); // Goblin Grunt has 60 HP
    assert_eq!(stats.max_health, 60);
}

#[test]
fn test_create_elf_archer() {
    let unit = UnitFactory::create_elf_archer(
        "Legolas".to_string(),
        HexCoord::new(2, -1),
        Terrain::Forest0,
    );

    assert_eq!(unit.name(), "Legolas");
    assert_eq!(unit.race(), Race::Elf);
    assert_eq!(unit.unit_type(), "Elf Archer");

    let stats = unit.combat_stats();
    assert_eq!(stats.health, 85); // Elf Archer has 85 HP
}

#[test]
fn test_create_dwarf_warrior() {
    let unit = UnitFactory::create_dwarf_warrior(
        "Gimli".to_string(),
        HexCoord::new(-1, 2),
        Terrain::Mountain,
    );

    assert_eq!(unit.unit_type(), "Dwarf Warrior");
    assert_eq!(unit.race(), Race::Dwarf);
    assert_eq!(unit.level(), 2); // Level 2 warrior

    let stats = unit.combat_stats();
    assert_eq!(stats.health, 140); // Dwarf Warrior has 140 HP (very tough)
}

// ===== Combat Stats Tests =====

#[test]
fn test_combat_stats_different_units() {
    let warrior = UnitFactory::create_human_warrior(
        "Warrior".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    let mage = UnitFactory::create_human_mage(
        "Mage".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    let archer = UnitFactory::create_human_archer(
        "Archer".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    // Warriors have more health than mages
    assert!(warrior.combat_stats().max_health > mage.combat_stats().max_health);

    // Archers are faster than warriors
    assert!(archer.combat_stats().movement_speed > warrior.combat_stats().movement_speed);

    // All should start at full health
    assert_eq!(
        warrior.combat_stats().health,
        warrior.combat_stats().max_health
    );
    assert_eq!(mage.combat_stats().health, mage.combat_stats().max_health);
    assert_eq!(
        archer.combat_stats().health,
        archer.combat_stats().max_health
    );
}

#[test]
fn test_unit_attacks() {
    let warrior = UnitFactory::create_human_warrior(
        "Warrior".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    let attacks = warrior.get_attacks();

    // Human Warrior should have attacks defined
    assert!(attacks.len() > 0);

    // Check first attack exists
    assert!(!attacks[0].name.is_empty());
    assert!(attacks[0].damage > 0);
}

// ===== Experience and Leveling Tests =====

// ===== Movement Tests =====

#[test]
fn test_unit_movement() {
    let mut unit = UnitFactory::create_human_warrior(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    let start_pos = unit.position();
    let new_pos = HexCoord::new(1, 0);

    assert!(unit.move_to(new_pos));
    assert_eq!(unit.position(), new_pos);
    assert_ne!(unit.position(), start_pos);
}

#[test]
fn test_movement_range() {
    let unit = UnitFactory::create_human_warrior(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    let movement_range = unit.get_movement_range();

    // Should have some movement range based on movement speed
    assert!(movement_range.len() > 0);

    // Starting position should be included or nearby hexes
    assert!(movement_range
        .iter()
        .any(|&coord| coord.distance(HexCoord::new(0, 0)) <= unit.combat_stats().movement_speed));
}

// ===== Health and Damage Tests =====

#[test]
fn test_take_damage() {
    let mut unit = UnitFactory::create_human_warrior(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    let initial_health = unit.combat_stats().health;
    unit.take_damage(20);

    assert_eq!(unit.combat_stats().health, initial_health - 20);
    assert!(unit.is_alive());
}

#[test]
fn test_death_from_damage() {
    let mut unit = UnitFactory::create_goblin_grunt(
        "Weak Goblin".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    // Deal massive damage
    unit.take_damage(1000);

    assert!(!unit.is_alive());
    assert!(unit.combat_stats().health <= 0);
}

#[test]
fn test_healing() {
    let mut unit = UnitFactory::create_human_warrior(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    // Take some damage first
    unit.take_damage(30);
    let damaged_health = unit.combat_stats().health;

    // Heal
    unit.heal(15);

    assert_eq!(unit.combat_stats().health, damaged_health + 15);
}

#[test]
fn test_healing_cannot_exceed_max() {
    let mut unit = UnitFactory::create_human_warrior(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );

    let max_health = unit.combat_stats().max_health;

    // Try to overheal
    unit.heal(1000);

    // Should be capped at max health
    assert_eq!(unit.combat_stats().health, max_health);
}

// ===== Race Bonuses Tests =====

#[test]
fn test_race_terrain_bonuses() {
    let dwarf = UnitFactory::create_dwarf_warrior(
        "Dwarf".to_string(),
        HexCoord::new(0, 0),
        Terrain::Mountain,
    );

    let elf =
        UnitFactory::create_elf_warrior("Elf".to_string(), HexCoord::new(0, 0), Terrain::Forest0);

    // Both should have terrain bonuses reflected in their hit chance
    assert!(dwarf.combat_stats().terrain_hit_chance > 0);
    assert!(elf.combat_stats().terrain_hit_chance > 0);
}

// ===== Different Unit Types Tests =====

#[test]
fn test_all_human_units() {
    let warrior = UnitFactory::create_human_warrior(
        "W".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );
    let archer =
        UnitFactory::create_human_archer("A".to_string(), HexCoord::new(0, 0), Terrain::Grasslands);
    let mage =
        UnitFactory::create_human_mage("M".to_string(), HexCoord::new(0, 0), Terrain::Grasslands);

    assert_eq!(warrior.race(), Race::Human);
    assert_eq!(archer.race(), Race::Human);
    assert_eq!(mage.race(), Race::Human);

    assert_eq!(warrior.unit_type(), "Human Warrior");
    assert_eq!(archer.unit_type(), "Human Archer");
    assert_eq!(mage.unit_type(), "Human Mage");
}

#[test]
fn test_all_elf_units() {
    let warrior =
        UnitFactory::create_elf_warrior("W".to_string(), HexCoord::new(0, 0), Terrain::Forest0);
    let archer =
        UnitFactory::create_elf_archer("A".to_string(), HexCoord::new(0, 0), Terrain::Forest0);
    let mage = UnitFactory::create_elf_mage("M".to_string(), HexCoord::new(0, 0), Terrain::Forest0);

    assert_eq!(warrior.race(), Race::Elf);
    assert_eq!(archer.race(), Race::Elf);
    assert_eq!(mage.race(), Race::Elf);

    assert_eq!(warrior.unit_type(), "Elf Warrior");
    assert_eq!(archer.unit_type(), "Elf Archer");
    assert_eq!(mage.unit_type(), "Elf Mage");
}

#[test]
fn test_all_dwarf_units() {
    let young = UnitFactory::create_dwarf_young_warrior(
        "Y".to_string(),
        HexCoord::new(0, 0),
        Terrain::Mountain,
    );
    let warrior =
        UnitFactory::create_dwarf_warrior("W".to_string(), HexCoord::new(0, 0), Terrain::Mountain);
    let veteran = UnitFactory::create_dwarf_veteran_warrior(
        "V".to_string(),
        HexCoord::new(0, 0),
        Terrain::Mountain,
    );

    assert_eq!(young.race(), Race::Dwarf);
    assert_eq!(warrior.race(), Race::Dwarf);
    assert_eq!(veteran.race(), Race::Dwarf);

    assert_eq!(young.unit_type(), "Dwarf Young Warrior");
    assert_eq!(warrior.unit_type(), "Dwarf Warrior");
    assert_eq!(veteran.unit_type(), "Dwarf Veteran Warrior");

    // Check evolution progression
    assert_eq!(young.level(), 1);
    assert_eq!(warrior.level(), 2);
    assert_eq!(veteran.level(), 3);

    // Check health progression
    assert_eq!(young.combat_stats().max_health, 110);
    assert_eq!(warrior.combat_stats().max_health, 140);
    assert_eq!(veteran.combat_stats().max_health, 175);

    // Veteran should be strongest
    assert!(veteran.combat_stats().max_health > warrior.combat_stats().max_health);
    assert!(warrior.combat_stats().max_health > young.combat_stats().max_health);
}

#[test]
fn test_goblin_units() {
    let grunt =
        UnitFactory::create_goblin_grunt("G".to_string(), HexCoord::new(0, 0), Terrain::Swamp);
    let chief =
        UnitFactory::create_goblin_chief("C".to_string(), HexCoord::new(0, 0), Terrain::Swamp);

    assert_eq!(grunt.race(), Race::Goblin);
    assert_eq!(chief.race(), Race::Goblin);

    assert_eq!(grunt.unit_type(), "Goblin Grunt");
    assert_eq!(chief.unit_type(), "Goblin Chief");

    // Chief should be stronger than grunt
    assert!(chief.combat_stats().max_health > grunt.combat_stats().max_health);
}

// ===== Position and Coordinate Tests =====

#[test]
fn test_unit_position() {
    let coord = HexCoord::new(3, -2);
    let unit = UnitFactory::create_human_warrior("Test".to_string(), coord, Terrain::Grasslands);

    assert_eq!(unit.position(), coord);
    assert_eq!(unit.position().q, 3);
    assert_eq!(unit.position().r, -2);
}
