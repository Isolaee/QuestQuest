// Unit creation and functionality tests using the new trait-based system
// Each unit type implements Unit trait with base(), base_mut(), and attacks() methods

use ::units::*;
use graphics::HexCoord;

// ===== New Trait Pattern Tests =====

#[test]
fn test_all_dwarf_units() {
    let young = UnitFactory::create_dwarf_young_warrior("Y".to_string(), HexCoord::new(0, 0));
    let warrior = UnitFactory::create_dwarf_warrior("W".to_string(), HexCoord::new(0, 0));
    let veteran = UnitFactory::create_dwarf_veteran_warrior("V".to_string(), HexCoord::new(0, 0));

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
    let grunt = UnitFactory::create_goblin_grunt("G".to_string(), HexCoord::new(0, 0));
    let chief = UnitFactory::create_goblin_chief("C".to_string(), HexCoord::new(0, 0));

    assert_eq!(grunt.race(), Race::Goblin);
    assert_eq!(chief.race(), Race::Goblin);

    assert_eq!(grunt.unit_type(), "Goblin Grunt");
    assert_eq!(chief.unit_type(), "Goblin Chief");

    // Chief should be stronger than grunt
    assert!(chief.combat_stats().max_health > grunt.combat_stats().max_health);
}
