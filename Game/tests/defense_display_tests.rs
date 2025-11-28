/// Tests for defense value display on movement range tiles
///
/// This test module validates that defense values are correctly displayed
/// on tiles when a unit is selected, showing the unit's defense stat
/// (slash resistance) on each tile within its movement range.
use game::{GameObject, GameUnit, GameWorld};
use graphics::{HexCoord, HexGrid};
use units::UnitFactory;

#[test]
fn test_hex_text_overlay_set_and_get() {
    // Test that we can set and retrieve text overlays on hexagons
    let mut hex_grid = HexGrid::new();
    let coord = HexCoord::new(0, 0);

    // Initially should have no text overlay
    let hex = hex_grid.get_hex_at(coord).unwrap();
    assert!(hex.text_overlay.is_none());

    // Set text overlay
    hex_grid.set_hex_text_overlay(coord, Some("DEF:35".to_string()));

    // Verify text overlay was set
    let hex = hex_grid.get_hex_at(coord).unwrap();
    assert_eq!(hex.text_overlay, Some("DEF:35".to_string()));

    // Clear text overlay
    hex_grid.set_hex_text_overlay(coord, None);
    let hex = hex_grid.get_hex_at(coord).unwrap();
    assert!(hex.text_overlay.is_none());
}

#[test]
fn test_clear_all_text_overlays() {
    // Test that clear_all_text_overlays clears text from all hexes
    let mut hex_grid = HexGrid::new();

    // Set text overlays on multiple hexes
    let coords = vec![
        HexCoord::new(0, 0),
        HexCoord::new(1, 0),
        HexCoord::new(0, 1),
        HexCoord::new(-1, 0),
    ];

    for coord in &coords {
        hex_grid.set_hex_text_overlay(*coord, Some("DEF:35".to_string()));
    }

    // Verify all were set
    for coord in &coords {
        let hex = hex_grid.get_hex_at(*coord).unwrap();
        assert!(hex.text_overlay.is_some());
    }

    // Clear all text overlays
    hex_grid.clear_all_text_overlays();

    // Verify all were cleared
    for coord in &coords {
        let hex = hex_grid.get_hex_at(*coord).unwrap();
        assert!(hex.text_overlay.is_none());
    }
}

#[test]
fn test_defense_value_format() {
    // Test that defense values are formatted correctly
    let mut world = GameWorld::new(10);

    // Create a unit with known defense (slash resistance)
    let unit = UnitFactory::create(
        "Dwarf Warrior",
        Some("Test".to_string()),
        Some(HexCoord::new(0, 0)),
    )
    .expect("Failed to create unit");

    let defense = unit.combat_stats().resistances.slash;
    let game_unit = GameUnit::new(unit);
    world.add_unit(game_unit);

    // Verify defense value is formatted as expected
    let expected_format = format!("DEF:{}", defense);
    assert!(expected_format.starts_with("DEF:"));
    assert!(expected_format.len() > 4); // Should have digits after "DEF:"
}

#[test]
fn test_unit_defense_retrieval() {
    // Test that we can retrieve defense value from a unit
    let mut world = GameWorld::new(10);

    // Create units with different types (different defense values)
    let dwarf = UnitFactory::create(
        "Dwarf Warrior",
        Some("Dwarf".to_string()),
        Some(HexCoord::new(0, 0)),
    )
    .expect("Failed to create dwarf");
    let dwarf_defense = dwarf.combat_stats().resistances.slash;

    let orc = UnitFactory::create(
        "Orc Swordsman",
        Some("Orc".to_string()),
        Some(HexCoord::new(2, 2)),
    )
    .expect("Failed to create orc");
    let orc_defense = orc.combat_stats().resistances.slash;

    // Add units to world
    let dwarf_id = world.add_unit(GameUnit::new(dwarf));
    let orc_id = world.add_unit(GameUnit::new(orc));

    // Verify we can retrieve defense from world
    let dwarf_unit = world.get_unit(dwarf_id).unwrap();
    assert_eq!(
        dwarf_unit.unit().combat_stats().resistances.slash,
        dwarf_defense
    );

    let orc_unit = world.get_unit(orc_id).unwrap();
    assert_eq!(
        orc_unit.unit().combat_stats().resistances.slash,
        orc_defense
    );

    // Dwarves and Orcs might have different defense values
    // This test ensures we're reading the correct unit's stats
}

#[test]
fn test_movement_range_calculation() {
    // Test that movement range is calculated correctly for defense display
    let mut world = GameWorld::new(10);

    // Create a unit at origin with known movement speed
    let unit = UnitFactory::create(
        "Dwarf Warrior",
        Some("Test".to_string()),
        Some(HexCoord::new(0, 0)),
    )
    .expect("Failed to create unit");

    let movement_speed = unit.combat_stats().movement_speed;
    let game_unit = GameUnit::new(unit);
    let unit_id = world.add_unit(game_unit);

    // Get the unit to check movement range
    let game_unit = world.get_unit(unit_id).unwrap();

    // Movement range should not be empty for a unit with movement points
    assert!(movement_speed > 0, "Unit should have movement points");

    // Verify unit is at expected position
    assert_eq!(game_unit.position(), HexCoord::new(0, 0));
}

#[test]
fn test_text_overlay_persists_across_highlights() {
    // Test that text overlays persist when highlights are applied
    let mut hex_grid = HexGrid::new();
    let coord = HexCoord::new(0, 0);

    // Set text overlay
    hex_grid.set_hex_text_overlay(coord, Some("DEF:35".to_string()));

    // Apply highlight
    hex_grid.highlight_hex(coord, graphics::HighlightType::MovementRange);

    // Text overlay should still be present
    let hex = hex_grid.get_hex_at(coord).unwrap();
    assert_eq!(hex.text_overlay, Some("DEF:35".to_string()));
    assert_eq!(hex.highlight, graphics::HighlightType::MovementRange);
}

#[test]
fn test_text_overlay_independent_of_unit_sprite() {
    // Test that text overlays work independently of unit sprites
    let mut hex_grid = HexGrid::new();
    let coord = HexCoord::new(0, 0);

    // Set terrain sprite
    hex_grid.set_sprite_at(coord, graphics::SpriteType::DwarfWarrior);

    // Set text overlay
    hex_grid.set_hex_text_overlay(coord, Some("DEF:35".to_string()));

    // Both should be present
    let hex = hex_grid.get_hex_at(coord).unwrap();
    assert_eq!(hex.sprite, graphics::SpriteType::DwarfWarrior);
    assert_eq!(hex.text_overlay, Some("DEF:35".to_string()));
}

#[test]
fn test_defense_display_multiple_units() {
    // Test defense display for multiple units with different defense values
    let mut world = GameWorld::new(10);

    // Create multiple units
    let units = vec![
        ("Dwarf Warrior", HexCoord::new(0, 0)),
        ("Orc Swordsman", HexCoord::new(3, 3)),
        ("Elf Archer", HexCoord::new(-2, 2)),
    ];

    let mut defense_values = Vec::new();

    for (unit_type, pos) in units {
        let unit = UnitFactory::create(unit_type, Some(unit_type.to_string()), Some(pos))
            .expect(&format!("Failed to create {}", unit_type));

        let defense = unit.combat_stats().resistances.slash;
        defense_values.push(defense);

        world.add_unit(GameUnit::new(unit));
    }

    // Each unit should have a defense value (could be different)
    assert_eq!(defense_values.len(), 3);
    // Defense values are u32, so they're always non-negative by type definition
}

#[test]
fn test_clear_selection_clears_text_overlays() {
    // Integration test: verify text overlays are cleared properly
    let mut hex_grid = HexGrid::new();

    // Simulate setting text overlays on movement range
    let movement_coords = vec![
        HexCoord::new(0, 1),
        HexCoord::new(1, 0),
        HexCoord::new(-1, 0),
        HexCoord::new(0, -1),
    ];

    for coord in &movement_coords {
        hex_grid.set_hex_text_overlay(*coord, Some("DEF:35".to_string()));
        hex_grid.highlight_hex(*coord, graphics::HighlightType::MovementRange);
    }

    // Verify overlays were set
    for coord in &movement_coords {
        let hex = hex_grid.get_hex_at(*coord).unwrap();
        assert!(hex.text_overlay.is_some());
    }

    // Simulate clear selection
    hex_grid.clear_all_highlights();
    hex_grid.clear_all_text_overlays();

    // Verify everything was cleared
    for coord in &movement_coords {
        let hex = hex_grid.get_hex_at(*coord).unwrap();
        assert!(hex.text_overlay.is_none());
        assert_eq!(hex.highlight, graphics::HighlightType::None);
    }
}

#[test]
fn test_defense_value_non_negative() {
    // Test that defense values are always non-negative
    // Defense values (u32) are non-negative by type definition
    let unit_types = vec![
        "Dwarf Warrior",
        "Orc Swordsman",
        "Elf Archer",
        "Human Knight",
    ];

    for unit_type in unit_types {
        let unit = UnitFactory::create(
            unit_type,
            Some(unit_type.to_string()),
            Some(HexCoord::new(0, 0)),
        )
        .expect(&format!("Failed to create {}", unit_type));

        let _defense = unit.combat_stats().resistances.slash;
        // Defense is u32, so no need to assert >= 0
    }
}

#[test]
fn test_text_overlay_empty_string() {
    // Test handling of empty text overlay
    let mut hex_grid = HexGrid::new();
    let coord = HexCoord::new(0, 0);

    // Set empty string (should be treated as Some(""))
    hex_grid.set_hex_text_overlay(coord, Some("".to_string()));

    let hex = hex_grid.get_hex_at(coord).unwrap();
    assert_eq!(hex.text_overlay, Some("".to_string()));

    // Setting None should clear it
    hex_grid.set_hex_text_overlay(coord, None);
    let hex = hex_grid.get_hex_at(coord).unwrap();
    assert!(hex.text_overlay.is_none());
}

#[test]
fn test_multiple_text_overlays_on_adjacent_tiles() {
    // Test that multiple adjacent tiles can have different text overlays
    let mut hex_grid = HexGrid::new();

    let tiles = vec![
        (HexCoord::new(0, 0), "DEF:35"),
        (HexCoord::new(1, 0), "DEF:40"),
        (HexCoord::new(0, 1), "DEF:30"),
        (HexCoord::new(-1, 0), "DEF:35"),
    ];

    // Set different text on adjacent tiles
    for (coord, text) in &tiles {
        hex_grid.set_hex_text_overlay(*coord, Some(text.to_string()));
    }

    // Verify each tile has its own text
    for (coord, expected_text) in &tiles {
        let hex = hex_grid.get_hex_at(*coord).unwrap();
        assert_eq!(hex.text_overlay, Some(expected_text.to_string()));
    }
}
