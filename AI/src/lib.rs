//! Minimal GOAP skeleton for QuestQuest
//!
//! This crate provides a tiny, dependency-free GOAP planner prototype intended
//! for integration into the `Game` crate later. It focuses on a simple WorldState
//! model, Action templates/instances, and a forward A* planner with bounded search.

// Crate root: small re-exporting module to hold AI building blocks split across files.
pub mod action;
pub mod actions;
pub mod executor;
pub mod goals;
pub mod planner;
pub mod world_state;

pub use action::ground_action_from_template;
pub use action::{ActionInstance, ActionTemplate, Goal};
pub use actions::move_template;
pub use actions::AttackTemplate;
pub use executor::{ActionExecutor, RuntimeAction};
pub use goals::LongTermGoal as Goal2;
pub use planner::plan_for_team;
pub use planner::{plan, plan_instances, Plan};
pub use world_state::HexCoord;
pub use world_state::{FactValue, WorldState};
