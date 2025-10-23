//! Minimal GOAP skeleton for QuestQuest
//!
//! This crate provides a tiny, dependency-free GOAP planner prototype intended
//! for integration into the `Game` crate later. It focuses on a simple WorldState
//! model, Action templates/instances, and a forward A* planner with bounded search.

// Crate root: small re-exporting module to hold AI building blocks split across files.
pub mod action;
pub mod actions;
pub mod executor;
pub mod planner;
pub mod world_state;

pub use action::ground_action_from_template;
pub use action::{ActionInstance, ActionTemplate, Goal};
pub use actions::move_template;
pub use actions::AttackTemplate;
pub use executor::{ActionExecutor, RuntimeAction};
pub use planner::plan_for_team;
pub use planner::{plan, plan_instances, Plan};
pub use world_state::HexCoord;
pub use world_state::{FactValue, WorldState};

// The actual implementation has been split across modules. Unit tests live here
// and rely on the public re-exports above.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poof_of_concept_simple_plan() {
        // Start: at A
        let mut start = WorldState::new();
        use crate::world_state::HexCoord;
        start.insert("At".to_string(), FactValue::Hex(HexCoord { q: 0, r: 0 }));

        // Actions: MoveAtoB, MoveBtoC
        let a1 = move_template(HexCoord { q: 0, r: 0 }, HexCoord { q: 1, r: 0 }, 1.0);
        let a2 = move_template(HexCoord { q: 1, r: 0 }, HexCoord { q: 2, r: 0 }, 1.0);

        let actions = vec![a1, a2];
        let goal = Goal {
            key: "At".to_string(),
            value: FactValue::Hex(HexCoord { q: 2, r: 0 }),
        };

        let plan_res = plan(&start, &actions, &goal, 1000);
        assert!(plan_res.is_some());
        let plan = plan_res.unwrap();
        // Expect two actions: 0 then 1
        assert_eq!(plan, vec![0usize, 1usize]);
    }

    #[test]
    fn plan_and_execute_move_then_attack() {
        // World: agent at A, enemy at B with health 5
        let mut start = WorldState::new();
        use crate::world_state::HexCoord;
        start.insert("At".to_string(), FactValue::Hex(HexCoord { q: 0, r: 0 }));
        start.insert(
            "EnemyAt".to_string(),
            FactValue::Hex(HexCoord { q: 1, r: 0 }),
        );
        start.insert("EnemyHealth".to_string(), FactValue::Int(5));
        start.insert("EnemyAlive".to_string(), FactValue::Bool(true));

        // Templates: Move A->B, Attack at B (damage 5)
        let move_ab = move_template(HexCoord { q: 0, r: 0 }, HexCoord { q: 1, r: 0 }, 1.0);

        let attack_t = crate::actions::attack::AttackTemplate {
            name_base: "Attack".to_string(),
            damage: 5,
            cost: 1.0,
            range: 1,
        };

        // Ground templates into instances. AttackTemplate will produce an instance for EnemyAt=B
        let mut instances: Vec<ActionInstance> = Vec::new();
        instances.push(crate::action::ground_action_from_template(
            &move_ab,
            Some("agent1".to_string()),
        ));
        let mut att_instances = attack_t.ground_for_state(&start, Some("agent1".to_string()));
        instances.append(&mut att_instances);

        // Goal: EnemyAlive == false
        let goal = Goal {
            key: "EnemyAlive".to_string(),
            value: FactValue::Bool(false),
        };

        let plan_opt = plan_instances(&start, &instances, &goal, 1000);
        assert!(plan_opt.is_some(), "Planner should find a plan");
        let plan = plan_opt.unwrap();

        // Expect two steps: Move (index 0) then Attack (index 1)
        assert_eq!(plan.len(), 2);

        // Now execute plan: convert chosen instances into runtime actions and run executor
        let mut executor = ActionExecutor::new();
        let mut world = start.clone();

        for &idx in &plan {
            let ai = instances[idx].clone();
            // Use Timed action for Move (duration 1.0) and Instant for Attack
            let runtime = if ai.name.starts_with("Move") {
                RuntimeAction::Timed {
                    instance: ai,
                    duration: 1.0,
                    elapsed: 0.0,
                }
            } else {
                RuntimeAction::Instant(ai)
            };

            executor.start(runtime);
            // Simulate ticks until done
            let mut completed = false;
            for _ in 0..5 {
                if executor.update(0.5, &mut world) {
                    completed = true;
                    break;
                }
            }
            assert!(completed, "Action should complete within ticks");
        }

        // After execution enemy should be dead
        assert_eq!(world.get("EnemyAlive"), Some(&FactValue::Bool(false)));
        if let Some(FactValue::Int(h)) = world.get("EnemyHealth") {
            assert_eq!(*h, 0);
        }
        // Agent should be at B (hex coord q=1,r=0)
        assert_eq!(
            world.get("At"),
            Some(&FactValue::Hex(HexCoord { q: 1, r: 0 }))
        );
    }
}
