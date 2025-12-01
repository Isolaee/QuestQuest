/// Tests for the new map JSON format with scenario info and team declarations
use game::scenario_instance::ScenarioWorld;

#[test]
fn test_new_format_parses_scenario_info() {
    let map_json = r#"{
  "Scenario": {
    "Name": "Test Battle",
    "Description": "A test scenario"
  },
  "Teams": [
    {
      "Name": "Player",
      "IsPlayerControlled": true,
      "Goal": "Win the battle"
    }
  ],
  "Map": [
    {"HexCoord": {"q": 0, "r": 0}, "SpriteType": "Grasslands", "Unit": null, "Item": null, "Structure": null}
  ]
}"#;

    let result = ScenarioWorld::parse_map_json(map_json);
    assert!(
        result.is_ok(),
        "Failed to parse new format: {:?}",
        result.err()
    );

    let parsed = result.unwrap();

    // Check scenario info
    assert!(parsed.scenario.is_some());
    let scenario = parsed.scenario.unwrap();
    assert_eq!(scenario.name, "Test Battle");
    assert_eq!(scenario.description, "A test scenario");

    // Check teams
    assert_eq!(parsed.teams.len(), 1);
    assert_eq!(parsed.teams[0].name, "Player");
    assert_eq!(parsed.teams[0].is_player_controlled, true);
    assert_eq!(parsed.teams[0].goal, "Win the battle");

    // Check map parsed correctly
    assert_eq!(parsed.terrain.len(), 1);
}

#[test]
fn test_new_format_with_multiple_teams() {
    let map_json = r#"{
  "Scenario": {
    "Name": "Three-way Battle",
    "Description": "A battle with three factions"
  },
  "Teams": [
    {
      "Name": "Player",
      "IsPlayerControlled": true,
      "Goal": "Defeat all enemies"
    },
    {
      "Name": "Enemy",
      "IsPlayerControlled": false,
      "Goal": "Defeat the player"
    },
    {
      "Name": "Neutral",
      "IsPlayerControlled": false,
      "Goal": "Survive"
    }
  ],
  "Map": [
    {"HexCoord": {"q": 0, "r": 0}, "SpriteType": "Grasslands", "Unit": ["Human_squire", "Player"], "Item": null, "Item": null, "Structure": null},
    {"HexCoord": {"q": 1, "r": 0}, "SpriteType": "Hills", "Unit": ["Orc_swordman", "Enemy"], "Item": null, "Structure": null}
  ]
}"#;

    let result = ScenarioWorld::parse_map_json(map_json);
    assert!(
        result.is_ok(),
        "Failed to parse new format with multiple teams: {:?}",
        result.err()
    );

    let parsed = result.unwrap();

    // Check teams
    assert_eq!(parsed.teams.len(), 3);
    assert_eq!(parsed.teams[0].name, "Player");
    assert_eq!(parsed.teams[1].name, "Enemy");
    assert_eq!(parsed.teams[2].name, "Neutral");

    // Check units were parsed
    assert_eq!(parsed.units.len(), 2);
}

#[test]
fn test_legacy_format_still_works() {
    let map_json = r#"[
  {"HexCoord": {"q": 0, "r": 0}, "SpriteType": "Grasslands", "Unit": null, "Item": null, "Structure": null},
  {"HexCoord": {"q": 1, "r": 0}, "SpriteType": "Hills", "Unit": ["Human_squire", "Player"], "Item": null, "Structure": null}
]"#;

    let result = ScenarioWorld::parse_map_json(map_json);
    assert!(
        result.is_ok(),
        "Failed to parse legacy format: {:?}",
        result.err()
    );

    let parsed = result.unwrap();

    // Legacy format should have no scenario info or teams
    assert!(parsed.scenario.is_none());
    assert_eq!(parsed.teams.len(), 0);

    // But map should still parse
    assert_eq!(parsed.terrain.len(), 2);
    assert_eq!(parsed.units.len(), 1);
}

#[test]
fn test_new_format_without_scenario_info() {
    let map_json = r#"{
  "Teams": [
    {
      "Name": "Player",
      "IsPlayerControlled": true,
      "Goal": "Win"
    }
  ],
  "Map": [
    {"HexCoord": {"q": 0, "r": 0}, "SpriteType": "Grasslands", "Unit": null, "Item": null, "Structure": null}
  ]
}"#;

    let result = ScenarioWorld::parse_map_json(map_json);
    assert!(
        result.is_ok(),
        "Failed to parse format without scenario: {:?}",
        result.err()
    );

    let parsed = result.unwrap();

    // Scenario should be None
    assert!(parsed.scenario.is_none());

    // But teams should still be there
    assert_eq!(parsed.teams.len(), 1);
}
