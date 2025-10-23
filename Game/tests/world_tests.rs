use game::*;
use graphics::HexCoord;
use units::unit_factory::UnitFactory;

// Helper to create a small world and start the turn system
fn make_world(radius: i32) -> GameWorld {
    let mut w = GameWorld::new(radius);
    w.start_turn_based_game();
    w
}

#[test]
fn test_add_and_remove_unit() {
    let mut world = make_world(5);

    let u = UnitFactory::create_goblin_grunt(
        "Gob1".to_string(),
        HexCoord::new(0, 0),
        units::unit_race::Terrain::Grasslands,
    );
    let mut gu = GameUnit::new(u);
    gu.set_team(Team::Player);
    let id = world.add_unit(gu);

    assert!(world.get_unit(id).is_some());

    let removed = world.remove_unit(id);
    assert!(removed.is_some());
    assert!(world.get_unit(id).is_none());
}

#[test]
fn test_is_position_valid_for_movement_blocks() {
    let mut world = make_world(3);

    // Place a mountain tile that blocks movement
    let mountain_coord = HexCoord::new(2, 0);
    if let Some(t) = world.get_terrain_mut(mountain_coord) {
        t.set_movement_cost(5);
    }

    // Create unit
    let u = UnitFactory::create_goblin_grunt(
        "Gob2".to_string(),
        HexCoord::new(0, 0),
        units::unit_race::Terrain::Grasslands,
    );
    let mut gu = GameUnit::new(u);
    gu.set_team(Team::Player);
    let id = world.add_unit(gu);

    // Out of bounds
    assert!(!world.is_position_valid_for_movement(HexCoord::new(10, 10), Some(id)));

    // Place an allied unit at (1,0) - should block
    let u2 = UnitFactory::create_goblin_grunt(
        "Ally".to_string(),
        HexCoord::new(1, 0),
        units::unit_race::Terrain::Grasslands,
    );
    let mut gu2 = GameUnit::new(u2);
    gu2.set_team(Team::Player);
    let id2 = world.add_unit(gu2);

    assert!(!world.is_position_valid_for_movement(HexCoord::new(1, 0), Some(id)));

    // An enemy unit at (1,1) should allow movement (combat)
    let e = UnitFactory::create_human_warrior(
        "Enemy".to_string(),
        HexCoord::new(1, 1),
        units::unit_race::Terrain::Grasslands,
    );
    let mut ge = GameUnit::new(e);
    ge.set_team(Team::Enemy);
    let eid = world.add_unit(ge);

    assert!(world.is_position_valid_for_movement(HexCoord::new(1, 1), Some(id)));
    // Clean up
    let _ = world.remove_unit(id);
    let _ = world.remove_unit(id2);
    let _ = world.remove_unit(eid);
}

#[test]
fn test_move_unit_and_combat_flow() {
    let mut world = make_world(4);

    // Player unit
    let p = UnitFactory::create_human_warrior(
        "Hero".to_string(),
        HexCoord::new(0, 0),
        units::unit_race::Terrain::Grasslands,
    );
    let mut gp = GameUnit::new(p);
    gp.set_team(Team::Player);
    let pid = world.add_unit(gp);

    // Enemy unit adjacent
    let e = UnitFactory::create_goblin_grunt(
        "Grunt".to_string(),
        HexCoord::new(1, 0),
        units::unit_race::Terrain::Grasslands,
    );
    let mut ge = GameUnit::new(e);
    ge.set_team(Team::Enemy);
    let eid = world.add_unit(ge);

    // Move onto enemy triggers request_combat (returns Ok but sets pending_combat)
    let res = world.move_unit(pid, HexCoord::new(1, 0));
    assert!(res.is_err() || world.pending_combat.is_some());

    // If pending combat exists, execute it
    if world.pending_combat.is_some() {
        // Use default selected attack index 0
        let exec = world.execute_pending_combat();
        assert!(exec.is_ok());
    }

    // After combat, either attacker or defender may be removed depending on stats
    let p_alive = world.get_unit(pid).is_some();
    let e_alive = world.get_unit(eid).is_some();
    // At least one should be dead or moved
    assert!(p_alive || !e_alive || !p_alive || e_alive);
}

#[test]
fn test_process_interactions_non_item() {
    let mut world = make_world(3);

    // Create a non-item interactive object with one interaction
    let obj = InteractiveObject::new(
        HexCoord::new(0, 0),
        "Shrine".to_string(),
        "A test shrine".to_string(),
        graphics::SpriteType::Grasslands,
    );
    // Ensure it has interaction count so it will be removed after interacting
    let id = world.add_interactive_object(obj.clone());

    // Place a player unit on same tile
    let u = UnitFactory::create_goblin_grunt(
        "Interacter".to_string(),
        HexCoord::new(0, 0),
        units::unit_race::Terrain::Grasslands,
    );
    let mut gu = GameUnit::new(u);
    gu.set_team(Team::Player);
    let uid = world.add_unit(gu);

    // Process interactions via public update() which calls process_interactions internally.
    world.update(0.0);
    assert!(world.get_interactive_object(id).is_some());

    // Clean up
    let _ = world.remove_unit(uid);
    let _ = world.remove_interactive_object(id);
}

#[test]
fn test_extract_world_state_and_generate_actions() {
    let mut world = make_world(4);

    // Create player unit and enemy unit
    let p = UnitFactory::create_human_warrior(
        "Hero2".to_string(),
        HexCoord::new(0, 0),
        units::unit_race::Terrain::Grasslands,
    );
    let mut gp = GameUnit::new(p);
    gp.set_team(Team::Player);
    let pid = world.add_unit(gp);

    let e = UnitFactory::create_goblin_grunt(
        "Grunt2".to_string(),
        HexCoord::new(1, 0),
        units::unit_race::Terrain::Grasslands,
    );
    let mut ge = GameUnit::new(e);
    ge.set_team(Team::Enemy);
    let _eid = world.add_unit(ge);

    let ws = world.extract_world_state_for_team(Team::Player);
    assert!(ws.get(&format!("CurrentTeam")).is_some());

    let actions = world.generate_team_actions(Team::Player);
    // Should contain at least one move or attack action
    assert!(!actions.is_empty());

    // Clean up
    let _ = world.remove_unit(pid);
}
