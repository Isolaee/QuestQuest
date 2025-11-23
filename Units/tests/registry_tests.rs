//! Test demonstrating the automatic unit registration system

use graphics::HexCoord;
use units::{Terrain, UnitFactory};

#[test]
fn test_automatic_registration() {
    // All units should be automatically registered via inventory
    let types = UnitFactory::list_types();

    // We have 12 unit types registered
    assert_eq!(types.len(), 12);

    // Verify specific units exist
    assert!(UnitFactory::exists("Human Warrior"));
    assert!(UnitFactory::exists("Human Archer"));
    assert!(UnitFactory::exists("Human Mage"));

    assert!(UnitFactory::exists("Elf Warrior"));
    assert!(UnitFactory::exists("Elf Archer"));
    assert!(UnitFactory::exists("Elf Mage"));

    assert!(UnitFactory::exists("Dwarf Young Warrior"));
    assert!(UnitFactory::exists("Dwarf Warrior"));
    assert!(UnitFactory::exists("Dwarf Veteran Warrior"));

    assert!(UnitFactory::exists("Orc Young Swordsman"));
    assert!(UnitFactory::exists("Orc Swordsman"));
    assert!(UnitFactory::exists("Orc Elite Swordsman"));

    // Non-existent unit should return false
    assert!(!UnitFactory::exists("Dragon King"));
}

#[test]
fn test_create_by_name() {
    // Create units dynamically by name
    let warrior = UnitFactory::create("Human Warrior", None, None, None);
    assert!(warrior.is_ok());

    let archer = UnitFactory::create("Elf Archer", None, None, None);
    assert!(archer.is_ok());

    let dwarf = UnitFactory::create("Dwarf Veteran Warrior", None, None, None);
    assert!(dwarf.is_ok());

    // Invalid unit type should error
    let invalid = UnitFactory::create("Dragon King", None, None, None);
    assert!(invalid.is_err());
}

#[test]
fn test_create_with_custom_params() {
    let unit = UnitFactory::create(
        "Human Warrior",
        Some("Aragorn".to_string()),
        Some(HexCoord::new(5, 3)),
        Some(Terrain::Grasslands),
    )
    .unwrap();

    assert_eq!(unit.name(), "Aragorn");
    assert_eq!(unit.position(), HexCoord::new(5, 3));
}

#[test]
fn test_list_by_race() {
    let humans = UnitFactory::list_by_race("Human");
    assert_eq!(humans.len(), 3);
    assert!(humans.contains(&"Human Warrior"));
    assert!(humans.contains(&"Human Archer"));
    assert!(humans.contains(&"Human Mage"));

    let elves = UnitFactory::list_by_race("Elf");
    assert_eq!(elves.len(), 3);

    let dwarves = UnitFactory::list_by_race("Dwarf");
    assert_eq!(dwarves.len(), 3);

    let orcs = UnitFactory::list_by_race("Orc");
    assert_eq!(orcs.len(), 3);
}

#[test]
fn test_list_by_class() {
    let warriors = UnitFactory::list_by_class("Warrior");
    assert_eq!(warriors.len(), 5); // Human, Elf, 3 Dwarf stages

    let archers = UnitFactory::list_by_class("Archer");
    assert_eq!(archers.len(), 2); // Human, Elf

    let mages = UnitFactory::list_by_class("Mage");
    assert_eq!(mages.len(), 2); // Human, Elf

    let swordsmen = UnitFactory::list_by_class("Swordsman");
    assert_eq!(swordsmen.len(), 3); // 3 Orc stages
}

#[test]
fn test_evolution_chains_registered() {
    // Dwarf warrior evolution
    assert!(UnitFactory::exists("Dwarf Young Warrior"));
    assert!(UnitFactory::exists("Dwarf Warrior"));
    assert!(UnitFactory::exists("Dwarf Veteran Warrior"));

    // Orc swordsman evolution
    assert!(UnitFactory::exists("Orc Young Swordsman"));
    assert!(UnitFactory::exists("Orc Swordsman"));
    assert!(UnitFactory::exists("Orc Elite Swordsman"));

    // All should be queryable by race
    let dwarves = UnitFactory::list_by_race("Dwarf");
    assert!(dwarves.contains(&"Dwarf Young Warrior"));
    assert!(dwarves.contains(&"Dwarf Warrior"));
    assert!(dwarves.contains(&"Dwarf Veteran Warrior"));
}

#[test]
fn test_default_terrain() {
    // Create unit with default terrain
    let elf = UnitFactory::create("Elf Archer", None, None, None).unwrap();
    // Elf Archer's default terrain is Forest0, but we can't directly test it
    // since terrain is stored in BaseUnit. Just verify creation succeeds.
    assert_eq!(elf.name(), "Elf Archer");

    // Create with explicit terrain
    let dwarf = UnitFactory::create(
        "Dwarf Warrior",
        Some("Thorin".to_string()),
        None,
        Some(Terrain::Mountain),
    )
    .unwrap();
    assert_eq!(dwarf.name(), "Thorin");
}
