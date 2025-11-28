use graphics::HexCoord;
use units::UnitFactory;

#[test]
fn test_human_knight_branching_evolution() {
    // Create a Human Knight
    let mut knight = UnitFactory::create(
        "Human Knight",
        Some("Sir Lancelot".to_string()),
        Some(HexCoord::new(0, 0)),
        None,
    )
    .expect("Failed to create Human Knight");

    // Check that it has two evolution paths (branching)
    let evolutions = knight.evolution_next();
    assert_eq!(
        evolutions.len(),
        2,
        "Human Knight should have 2 evolution paths"
    );
    assert!(evolutions.contains(&units::UnitType::HumanKnightCommander));
    assert!(evolutions.contains(&units::UnitType::HumanGrandKnight));

    // Add enough XP to level up
    knight.add_experience(100);
    assert!(knight.can_level_up());

    // Test evolution to Knight Commander (index 0)
    let commander = knight
        .evolve(0, true)
        .expect("Failed to evolve to Knight Commander");
    assert_eq!(commander.unit_type(), "Human Knight Commander");
    assert_eq!(commander.level(), 2);
    assert_eq!(commander.name(), "Sir Lancelot");

    // Verify commander has improved stats
    assert!(
        commander.combat_stats().max_health > 0,
        "Knight Commander should have positive health"
    );
    assert!(
        commander.combat_stats().movement_speed >= 3,
        "Knight Commander should have at least base movement"
    );
}

#[test]
fn test_knight_commander_evolution_chain() {
    // Test the Knight Commander -> Grand Knight path
    let mut knight = UnitFactory::create(
        "Human Knight",
        Some("Battle Leader".to_string()),
        Some(HexCoord::new(0, 0)),
        None,
    )
    .unwrap();

    knight.add_experience(100);

    // Evolve to Knight Commander (first branch - index 0)
    let commander = knight.evolve(0, true).unwrap();
    assert_eq!(commander.level(), 2);
    assert_eq!(commander.unit_type(), "Human Knight Commander");

    // Knight Commander is final form (no further evolution)
    assert!(commander.evolution_next().is_empty());

    // Test the other branch - Grand Knight (index 1)
    let mut knight2 = UnitFactory::create(
        "Human Knight",
        Some("Battle Leader".to_string()),
        Some(HexCoord::new(0, 0)),
        None,
    )
    .unwrap();
    knight2.add_experience(100);

    let grand_knight = knight2.evolve(1, true).unwrap();
    assert_eq!(grand_knight.level(), 2);
    assert_eq!(grand_knight.unit_type(), "Human Grand Knight");

    // Grand Knight is also final form
    assert!(grand_knight.evolution_next().is_empty());
}
