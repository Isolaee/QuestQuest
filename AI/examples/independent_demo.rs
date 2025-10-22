use ai::{
    ground_action_from_template, plan, plan_instances, ActionTemplate, FactValue, Goal, WorldState,
};
use std::collections::HashMap;

fn main() {
    // Demonstration of independent per-agent planning using the ai crate's planner.
    // We'll create a small world with two agents that each have a simple goal.

    // Shared world state: both agents start at different locations and a box is at B
    let mut world = WorldState::new();
    world.insert("AgentAAt".to_string(), FactValue::Str("A".to_string()));
    world.insert("AgentBAt".to_string(), FactValue::Str("X".to_string()));
    world.insert("BoxAt".to_string(), FactValue::Str("B".to_string()));

    // Actions available to Agent A: MoveA_AtoB, PickupBoxAtB
    let a_move = ActionTemplate {
        name: "MoveA_AtoB".to_string(),
        preconditions: vec![("AgentAAt".to_string(), FactValue::Str("A".to_string()))],
        effects: vec![("AgentAAt".to_string(), FactValue::Str("B".to_string()))],
        cost: 1.0,
    };
    let a_pick = ActionTemplate {
        name: "PickupBoxAtB_forA".to_string(),
        preconditions: vec![
            ("AgentAAt".to_string(), FactValue::Str("B".to_string())),
            ("BoxAt".to_string(), FactValue::Str("B".to_string())),
        ],
        effects: vec![("BoxCarriedBy".to_string(), FactValue::Str("A".to_string()))],
        cost: 1.0,
    };

    // Actions for Agent B: MoveB_XtoB, PickupBoxAtB
    let b_move = ActionTemplate {
        name: "MoveB_XtoB".to_string(),
        preconditions: vec![("AgentBAt".to_string(), FactValue::Str("X".to_string()))],
        effects: vec![("AgentBAt".to_string(), FactValue::Str("B".to_string()))],
        cost: 1.0,
    };
    let b_pick = ActionTemplate {
        name: "PickupBoxAtB_forB".to_string(),
        preconditions: vec![
            ("AgentBAt".to_string(), FactValue::Str("B".to_string())),
            ("BoxAt".to_string(), FactValue::Str("B".to_string())),
        ],
        effects: vec![("BoxCarriedBy".to_string(), FactValue::Str("B".to_string()))],
        cost: 1.0,
    };

    // Each agent will plan independently using only their actions and a goal to carry the box.
    // Agent A planning
    let agent_a_actions = vec![a_move.clone(), a_pick.clone()];
    let a_goal = Goal {
        key: "BoxCarriedBy".to_string(),
        value: FactValue::Str("A".to_string()),
    };

    // Agent B planning
    let agent_b_actions = vec![b_move.clone(), b_pick.clone()];
    let b_goal = Goal {
        key: "BoxCarriedBy".to_string(),
        value: FactValue::Str("B".to_string()),
    };

    // Run planners separately
    println!("=== Independent planning demo ===");

    // Agent A uses the high-level `plan` that accepts templates
    if let Some(plan_a) = plan(&world, &agent_a_actions, &a_goal, 100) {
        println!("Agent A plan (template indices): {:?}", plan_a);
        for idx in plan_a {
            println!("  - {}", agent_a_actions[idx].name);
        }
    } else {
        println!("Agent A: no plan found within node limit");
    }

    // Agent B plans independently
    if let Some(plan_b) = plan(&world, &agent_b_actions, &b_goal, 100) {
        println!("Agent B plan (template indices): {:?}", plan_b);
        for idx in plan_b {
            println!("  - {}", agent_b_actions[idx].name);
        }
    } else {
        println!("Agent B: no plan found within node limit");
    }

    println!("=== End demo ===");
}
