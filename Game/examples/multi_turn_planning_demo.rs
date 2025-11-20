/// Demo showcasing multi-turn planning capabilities.
///
/// This example demonstrates how AI units can set long-term goals and plan
/// strategically across multiple turns rather than just reacting to immediate situations.
use game::{GameObject, GameUnit, GameWorld, Team};
use graphics::HexCoord;
use units::{Terrain, UnitFactory};

fn main() {
    println!("=== Multi-Turn Planning Demo ===\n");

    // Create a world
    let mut world = GameWorld::new(15);
    world.generate_terrain();
    world.turn_system.start_game();

    // Create a player unit at origin
    let player = UnitFactory::create_human_warrior(
        "PlayerHero".to_string(),
        HexCoord::new(0, 0),
        Terrain::Grasslands,
    );
    let mut gp = GameUnit::new(player);
    gp.set_team(Team::Player);
    let player_id = world.add_unit(gp);

    // Create enemy units with different long-term goals

    // Enemy 1: Simple single-turn planner (baseline)
    let enemy1 = UnitFactory::create_goblin_grunt(
        "Goblin-SingleTurn".to_string(),
        HexCoord::new(10, 5),
        Terrain::Grasslands,
    );
    let mut ge1 = GameUnit::new_with_team(enemy1, Team::Enemy);
    ge1.set_plan_horizon(1); // Single turn only
    let e1_id = world.add_unit(ge1);

    // Enemy 2: Reach a specific position (multi-turn goal)
    let enemy2 = UnitFactory::create_goblin_grunt(
        "Goblin-Seeker".to_string(),
        HexCoord::new(8, 8),
        Terrain::Grasslands,
    );
    let mut ge2 = GameUnit::new_with_team(enemy2, Team::Enemy);

    // Set long-term goal: reach defensive area
    let target_pos = HexCoord::new(2, 2);
    let goal = ai::goals::LongTermGoal::ReachArea {
        area_centers: vec![ai::HexCoord {
            q: target_pos.q,
            r: target_pos.r,
        }],
        reason: "defensive_position".to_string(),
    };
    ge2.set_long_term_goal(Some(goal.to_string()));
    ge2.set_plan_horizon(5); // Plan 5 turns ahead
    let e2_id = world.add_unit(ge2);

    // Enemy 3: Eliminate target (aggressive pursuit)
    let enemy3 = UnitFactory::create_goblin_grunt(
        "Goblin-Hunter".to_string(),
        HexCoord::new(-8, 3),
        Terrain::Grasslands,
    );
    let mut ge3 = GameUnit::new_with_team(enemy3, Team::Enemy);

    let kill_goal = ai::goals::LongTermGoal::KillAllEnemies {
        search_radius: None, // Unlimited range
    };
    ge3.set_long_term_goal(Some(kill_goal.to_string()));
    ge3.set_plan_horizon(4); // Plan 4 turns ahead
    let e3_id = world.add_unit(ge3);

    println!("=== Unit Setup ===");
    println!(
        "Player: {} at {:?}",
        world.units[&player_id].name(),
        world.units[&player_id].position()
    );
    println!("\nEnemy Units:");
    println!(
        "1. {} at {:?}",
        world.units[&e1_id].name(),
        world.units[&e1_id].position()
    );
    println!("   Planning: Single-turn (reactive)");
    println!("   Horizon: {} turn", world.units[&e1_id].plan_horizon());

    println!(
        "\n2. {} at {:?}",
        world.units[&e2_id].name(),
        world.units[&e2_id].position()
    );
    println!("   Planning: Multi-turn strategic");
    println!("   Horizon: {} turns", world.units[&e2_id].plan_horizon());
    println!("   Goal: Reach position {:?}", target_pos);
    if let Some(goal_str) = world.units[&e2_id].long_term_goal() {
        println!("   Goal String: {}", goal_str);
    }

    println!(
        "\n3. {} at {:?}",
        world.units[&e3_id].name(),
        world.units[&e3_id].position()
    );
    println!("   Planning: Multi-turn tactical");
    println!("   Horizon: {} turns", world.units[&e3_id].plan_horizon());
    println!("   Goal: Kill all enemies");
    if let Some(goal_str) = world.units[&e3_id].long_term_goal() {
        println!("   Goal String: {}", goal_str);
    }

    // Test goal parsing
    println!("\n=== Goal Serialization/Deserialization Test ===");

    let test_goals = vec![
        ai::goals::LongTermGoal::ReachArea {
            area_centers: vec![ai::HexCoord { q: 5, r: 3 }],
            reason: "high_ground".to_string(),
        },
        ai::goals::LongTermGoal::KillAllEnemies {
            search_radius: Some(10),
        },
        ai::goals::LongTermGoal::Protect {
            targets: vec![ai::HexCoord { q: 0, r: 0 }, ai::HexCoord { q: 1, r: 0 }],
            reason: "formation".to_string(),
        },
        ai::goals::LongTermGoal::SiegeCastle {
            castle_id: "castle_1".to_string(),
        },
    ];

    for goal in test_goals {
        let serialized = goal.to_string();
        println!("\nOriginal: {:?}", goal);
        println!("Serialized: {}", serialized);

        if let Some(deserialized) = ai::goals::LongTermGoal::from_string(&serialized) {
            println!("Deserialized: {:?}", deserialized);
            println!("✓ Round-trip successful");
        } else {
            println!("✗ Deserialization failed");
        }
    }

    // Test goal decomposition
    println!("\n=== Goal Decomposition Test ===");

    let ws = world.extract_detailed_world_state(Team::Enemy);

    if let Some(goal_str) = world.units[&e2_id].long_term_goal() {
        if let Some(long_term_goal) = ai::goals::LongTermGoal::from_string(goal_str) {
            println!("\nDecomposing: {:?}", long_term_goal);

            if let Some(short_term) = long_term_goal.decompose(&ws, &e2_id.to_string()) {
                println!("Short-term goal for current turn:");
                println!("  Key: {}", short_term.key);
                println!("  Value: {:?}", short_term.value);
            } else {
                println!("Could not decompose goal (may already be achieved)");
            }

            let is_achieved = long_term_goal.is_achieved(&ws, &e2_id.to_string());
            println!("Goal achieved: {}", is_achieved);
        }
    }

    // Test distance calculation
    println!("\n=== Distance Calculations ===");
    let e2_pos = world.units[&e2_id].position();
    println!("Goblin-Seeker current position: {:?}", e2_pos);
    println!("Target position: {:?}", target_pos);
    println!("Distance to target: {} hexes", e2_pos.distance(target_pos));

    let e3_pos = world.units[&e3_id].position();
    let player_pos = world.units[&player_id].position();
    println!("\nGoblin-Hunter current position: {:?}", e3_pos);
    println!("Player position: {:?}", player_pos);
    println!("Distance to player: {} hexes", e3_pos.distance(player_pos));

    // Demonstrate planning horizon benefits
    println!("\n=== Planning Horizon Benefits ===");
    println!("\nSingle-Turn Planning (Horizon = 1):");
    println!("  • Plans only for immediate turn");
    println!("  • Reactive: responds to current situation");
    println!("  • May miss long-term opportunities");
    println!("  • Lower computational cost");

    println!("\nMulti-Turn Planning (Horizon = 3-5):");
    println!("  • Plans several turns ahead");
    println!("  • Strategic: works toward long-term goals");
    println!("  • Can coordinate complex maneuvers");
    println!("  • Maintains objective across turns");
    println!("  • Higher computational cost but smarter behavior");

    println!("\n=== Key Features Implemented ===");
    println!("✓ ai_long_term_goal field - stores strategic objectives");
    println!("✓ ai_plan_horizon field - controls planning depth");
    println!("✓ LongTermGoal enum - multiple goal types");
    println!("✓ Goal serialization - save/load goals as strings");
    println!("✓ Goal decomposition - break down into short-term steps");
    println!("✓ Goal achievement checking - know when objectives complete");
    println!("✓ HexCoord distance calculation - spatial reasoning");

    println!("\n=== Next Steps for Full Implementation ===");
    println!("1. Integrate with GameWorld::update() to use long-term goals");
    println!("2. Add turn simulation for multi-turn lookahead");
    println!("3. Implement path planning algorithms");
    println!("4. Add goal priority system");
    println!("5. Create goal-specific heuristics for better planning");
    println!("6. Add goal interruption/adaptation when situation changes");

    println!("\n=== Demo Complete ===");
}
