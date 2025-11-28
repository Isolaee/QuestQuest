use units::UnitType;

#[test]
fn test_as_str() {
    assert_eq!(UnitType::OrcSwordsman.as_str(), "Orc Swordsman");
    assert_eq!(UnitType::DwarfWarrior.as_str(), "Dwarf Warrior");
    assert_eq!(UnitType::HumanNoble.as_str(), "Human Noble");
    assert_eq!(UnitType::ElfArcher.as_str(), "Elf Archer");
}

#[test]
fn test_from_str() {
    assert_eq!(
        "Orc Swordsman".parse::<UnitType>().ok(),
        Some(UnitType::OrcSwordsman)
    );
    assert_eq!(
        "Dwarf Warrior".parse::<UnitType>().ok(),
        Some(UnitType::DwarfWarrior)
    );
    assert_eq!(
        "Human Noble".parse::<UnitType>().ok(),
        Some(UnitType::HumanNoble)
    );
    assert_eq!("Unknown Unit".parse::<UnitType>().ok(), None);
    assert_eq!("".parse::<UnitType>().ok(), None);
}

#[test]
fn test_roundtrip() {
    // Verify all unit types can be converted to string and back
    for unit_type in UnitType::all() {
        assert_eq!(
            unit_type.as_str().parse::<UnitType>().ok(),
            Some(*unit_type),
            "Roundtrip failed for {:?}",
            unit_type
        );
    }
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", UnitType::OrcSwordsman), "Orc Swordsman");
    assert_eq!(
        format!("{}", UnitType::DwarfVeteranWarrior),
        "Dwarf Veteran Warrior"
    );
    assert_eq!(
        format!("{}", UnitType::HumanKnightCommander),
        "Human Knight Commander"
    );
}

#[test]
fn test_all_unit_types() {
    let all_types = UnitType::all();

    // Verify we have the expected number of unit types
    assert_eq!(all_types.len(), 18, "Should have 18 unit types");

    // Verify some key types are present
    assert!(all_types.contains(&UnitType::DwarfYoungWarrior));
    assert!(all_types.contains(&UnitType::OrcEliteSwordsman));
    assert!(all_types.contains(&UnitType::HumanKing));
    assert!(all_types.contains(&UnitType::ElfMage));
}

#[test]
fn test_unit_type_uniqueness() {
    // Ensure all string representations are unique
    let mut seen = std::collections::HashSet::new();
    for unit_type in UnitType::all() {
        let str_rep = unit_type.as_str();
        assert!(
            seen.insert(str_rep),
            "Duplicate string representation found: {}",
            str_rep
        );
    }
}

#[test]
fn test_dwarf_line() {
    assert_eq!(UnitType::DwarfYoungWarrior.as_str(), "Dwarf Young Warrior");
    assert_eq!(UnitType::DwarfWarrior.as_str(), "Dwarf Warrior");
    assert_eq!(
        UnitType::DwarfVeteranWarrior.as_str(),
        "Dwarf Veteran Warrior"
    );
}

#[test]
fn test_orc_line() {
    assert_eq!(UnitType::OrcYoungSwordsman.as_str(), "Orc Young Swordsman");
    assert_eq!(UnitType::OrcSwordsman.as_str(), "Orc Swordsman");
    assert_eq!(UnitType::OrcEliteSwordsman.as_str(), "Orc Elite Swordsman");
}

#[test]
fn test_human_noble_line() {
    assert_eq!(UnitType::HumanNoble.as_str(), "Human Noble");
    assert_eq!(UnitType::HumanPrince.as_str(), "Human Prince");
    assert_eq!(UnitType::HumanKing.as_str(), "Human King");
}

#[test]
fn test_human_knight_line() {
    assert_eq!(UnitType::HumanSquire.as_str(), "Human Squire");
    assert_eq!(UnitType::HumanKnight.as_str(), "Human Knight");
    assert_eq!(UnitType::HumanGrandKnight.as_str(), "Human Grand Knight");
    assert_eq!(
        UnitType::HumanKnightCommander.as_str(),
        "Human Knight Commander"
    );
}

#[test]
fn test_elf_units() {
    assert_eq!(UnitType::ElfWarrior.as_str(), "Elf Warrior");
    assert_eq!(UnitType::ElfArcher.as_str(), "Elf Archer");
    assert_eq!(UnitType::ElfMage.as_str(), "Elf Mage");
}

#[test]
fn test_goblin_units() {
    assert_eq!(UnitType::GoblinGrunt.as_str(), "Goblin Grunt");
    assert_eq!(UnitType::GoblinChief.as_str(), "Goblin Chief");
}
