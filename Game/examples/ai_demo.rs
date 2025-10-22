use game::GameWorld;
use game::Team;
use graphics::HexCoord;
use units::unit_race::Terrain as UnitTerrain;
use units::units::human::warrior::HumanWarrior;

// Import the lightweight ai crate types to demonstrate planner features directly
use ai::{plan, plan_for_team, ActionInstance, ActionTemplate, FactValue, Goal, WorldState};
use std::collections::HashMap;

fn show_single_agent_demo() {
    println!("\n=== AI Demo: single-agent chained planning ===");

    // Start: at A
    let mut start = WorldState::new();
    start.insert("At".to_string(), FactValue::Str("A".to_string()));

    // Actions: chain A->B->C and some irrelevant actions to show robustness
    let move_ab = ActionTemplate {
        name: "MoveAtoB".to_string(),
        preconditions: vec![("At".to_string(), FactValue::Str("A".to_string()))],
        effects: vec![("At".to_string(), FactValue::Str("B".to_string()))],
        cost: 1.0,
    };
    let move_bc = ActionTemplate {
        name: "MoveBtoC".to_string(),
        preconditions: vec![("At".to_string(), FactValue::Str("B".to_string()))],
        effects: vec![("At".to_string(), FactValue::Str("C".to_string()))],
        cost: 1.0,
    };
    // Irrelevant action (should be ignored by planner)
    let dance = ActionTemplate {
        name: "Dance".to_string(),
        preconditions: vec![("Mood".to_string(), FactValue::Str("Happy".to_string()))],
        effects: vec![("At".to_string(), FactValue::Str("A".to_string()))],
        cost: 5.0,
    };

    let actions = vec![move_ab, move_bc, dance];
    let goal = Goal {
        key: "At".to_string(),
        value: FactValue::Str("C".to_string()),
    };

    println!("Planning from At=A to At=C with irrelevant actions present...");
    if let Some(plan_idx) = plan(&start, &actions, &goal, 1000) {
        println!("Plan found (template indices): {:?}", plan_idx);
        println!("Plan steps:");
        let mut total_cost = 0.0f32;
        for idx in plan_idx {
            let a = &actions[idx];
            println!(" - {} (cost {})", a.name, a.cost);
            total_cost += a.cost;
        }
        println!("Total plan cost: {}", total_cost);
    } else {
        println!("No plan found (within node limit)");
    }
}

fn show_team_demo() {
    println!("\n=== AI Demo: two-agent team planning ===");

    // World state tracks per-agent positions using keys Agent1At and Agent2At
    let mut start = WorldState::new();
    start.insert("Agent1At".to_string(), FactValue::Str("A".to_string()));
    start.insert("Agent2At".to_string(), FactValue::Str("A".to_string()));

    // Templates: agent-specific moves (we will ground them into instances with agent ids)
    // (templates are illustrative; we construct concrete instances below)

    // For this simple demo, we'll create concrete ActionInstances manually instead of
    // implementing full parameter grounding: create instances for Agent1 and Agent2.
    let mut actions_instances: Vec<ActionInstance> = Vec::new();
    // Agent1 actions
    actions_instances.push(ActionInstance {
        name: "A1_MoveAtoB".to_string(),
        preconditions: vec![("Agent1At".to_string(), FactValue::Str("A".to_string()))],
        effects: vec![("Agent1At".to_string(), FactValue::Str("B".to_string()))],
        cost: 1.0,
        agent: Some("Agent1".to_string()),
    });
    actions_instances.push(ActionInstance {
        name: "A1_MoveBtoC".to_string(),
        preconditions: vec![("Agent1At".to_string(), FactValue::Str("B".to_string()))],
        effects: vec![("Agent1At".to_string(), FactValue::Str("C".to_string()))],
        cost: 1.0,
        agent: Some("Agent1".to_string()),
    });

    // Agent2 actions
    actions_instances.push(ActionInstance {
        name: "A2_MoveAtoB".to_string(),
        preconditions: vec![("Agent2At".to_string(), FactValue::Str("A".to_string()))],
        effects: vec![("Agent2At".to_string(), FactValue::Str("B".to_string()))],
        cost: 1.5,
        agent: Some("Agent2".to_string()),
    });
    actions_instances.push(ActionInstance {
        name: "A2_MoveBtoC".to_string(),
        preconditions: vec![("Agent2At".to_string(), FactValue::Str("B".to_string()))],
        effects: vec![("Agent2At".to_string(), FactValue::Str("C".to_string()))],
        cost: 1.0,
        agent: Some("Agent2".to_string()),
    });

    // Global action that helps either agent (e.g., a teleporter that moves whoever is at B to C)
    actions_instances.push(ActionInstance {
        name: "TeleportBtoC".to_string(),
        preconditions: vec![("Agent1At".to_string(), FactValue::Str("B".to_string()))],
        effects: vec![("Agent1At".to_string(), FactValue::Str("C".to_string()))],
        cost: 0.5,
        agent: None,
    });
    // Note: teleport is only defined for Agent1 in effects for simplicity; in a real
    // grounded domain we'd have teleport effects for both agents or proper parameterization.

    // Goals per agent
    let mut goals_per_agent: HashMap<String, Vec<Goal>> = HashMap::new();
    goals_per_agent.insert(
        "Agent1".to_string(),
        vec![Goal {
            key: "Agent1At".to_string(),
            value: FactValue::Str("C".to_string()),
        }],
    );
    goals_per_agent.insert(
        "Agent2".to_string(),
        vec![Goal {
            key: "Agent2At".to_string(),
            value: FactValue::Str("C".to_string()),
        }],
    );

    let agent_order = vec!["Agent1".to_string(), "Agent2".to_string()];

    println!("Planning for team (Agent1, Agent2) with some shared/global actions...");
    let per_agent_plans = plan_for_team(
        &start,
        &actions_instances,
        &goals_per_agent,
        &agent_order,
        2000,
    );

    for agent in &agent_order {
        println!("Plan for {}:", agent);
        if let Some(plan) = per_agent_plans.get(agent) {
            if plan.is_empty() {
                println!(" - (empty)");
            } else {
                // Reconstruct agent-visible actions to map local indices to names
                let visible: Vec<&ActionInstance> = actions_instances
                    .iter()
                    .filter(|a| match &a.agent {
                        Some(id) => id == agent,
                        None => true,
                    })
                    .collect();
                for &local_idx in plan {
                    if let Some(a) = visible.get(local_idx) {
                        println!(" - {} (cost {})", a.name, a.cost);
                    }
                }
            }
        } else {
            println!(" - (no entry)");
        }
    }
}

fn main() {
    // Keep the original tiny GameWorld snippet to show integration
    println!("=== GameWorld quick run ===");
    let mut world = GameWorld::new(3);
    world.turn_system.start_game();

    // Add a player unit at (0,0)
    let player = HumanWarrior::new(
        "PlayerHero".to_string(),
        HexCoord::new(0, 0),
        UnitTerrain::Grasslands,
    );
    let mut player_unit = game::GameUnit::new(Box::new(player));
    player_unit.set_team(Team::Player);
    let player_id = world.add_unit(player_unit);

    // Add an enemy unit at (1,0)
    let enemy = HumanWarrior::new(
        "EnemyGrunt".to_string(),
        HexCoord::new(1, 0),
        UnitTerrain::Grasslands,
    );
    let enemy_unit = game::GameUnit::new_with_team(Box::new(enemy), Team::Enemy);
    let enemy_id = world.add_unit(enemy_unit);

    println!(
        "World created. Player: {:?}, Enemy: {:?}",
        player_id, enemy_id
    );

    // Ensure it's Enemy turn so AI will run
    world.turn_system.set_team_control(Team::Enemy, false);
    // Force turn to Enemy
    world.turn_system.end_turn(); // advance from Player to Enemy

    // Run AI for current team (Enemy)
    println!(
        "Running AI for current team: {:?}",
        world.turn_system.current_team()
    );
    world.run_ai_for_current_team();

    println!(
        "AI run complete. Pending combat: {:?}",
        world.pending_combat.is_some()
    );

    // Now run the direct ai demos
    show_single_agent_demo();
    show_team_demo();
}
