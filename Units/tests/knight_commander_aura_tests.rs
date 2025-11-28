//! Test for Human Knight Commander's leadership aura ability

use graphics::HexCoord;
use units::{Terrain, UnitFactory};

#[test]
fn test_knight_commander_has_leadership_aura() {
    let commander = UnitFactory::create(
        "Human Knight Commander",
        Some("Test Commander".to_string()),
        Some(HexCoord::new(0, 0)),
        Some(Terrain::Grasslands),
    )
    .expect("Failed to create Knight Commander");

    // Check that the unit has abilities
    assert_eq!(
        commander.abilities().len(),
        1,
        "Knight Commander should have 1 ability"
    );

    // Check that it has an aura ability
    let auras = commander.get_aura_abilities();
    assert_eq!(auras.len(), 1, "Knight Commander should have 1 aura");

    // Verify the aura details
    let aura = auras[0];
    assert_eq!(aura.name, "Knight Commander's Leadership");
    assert_eq!(aura.range, 1, "Leadership aura should have range 1");

    // Verify it targets allies
    assert_eq!(aura.target_type, units::ability::AuraTarget::Allies);

    // Verify it gives +1 attack
    match aura.effect {
        units::ability::AuraEffect::AttackBonus(bonus) => {
            assert_eq!(bonus, 1, "Leadership aura should give +1 attack");
        }
        _ => panic!("Expected AttackBonus effect"),
    }
}

#[test]
fn test_knight_commander_aura_range() {
    let commander = UnitFactory::create(
        "Human Knight Commander",
        Some("Test Commander".to_string()),
        Some(HexCoord::new(0, 0)),
        Some(Terrain::Grasslands),
    )
    .expect("Failed to create Knight Commander");

    let auras = commander.get_aura_abilities();
    assert_eq!(auras.len(), 1);

    let aura = auras[0];
    let commander_pos = commander.position();

    // Test adjacent positions (should be in range)
    let adjacent = vec![
        HexCoord::new(1, 0),
        HexCoord::new(-1, 0),
        HexCoord::new(0, 1),
        HexCoord::new(0, -1),
        HexCoord::new(1, -1),
        HexCoord::new(-1, 1),
    ];

    for pos in adjacent {
        assert!(
            aura.is_in_range(commander_pos, pos),
            "Position {:?} should be in aura range",
            pos
        );
    }

    // Test far position (should NOT be in range)
    let far_pos = HexCoord::new(3, 3);
    assert!(
        !aura.is_in_range(commander_pos, far_pos),
        "Far position should not be in aura range"
    );
}
