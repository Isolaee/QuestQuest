//! # AI Crate - Enemy AI and Planning Logic
//!
//! The `ai` crate provides Goal-Oriented Action Planning (GOAP) for non-player units.
//! It contains no game state - instead, it provides planning algorithms that generate
//! action sequences based on input world state.
//!
//! ## Architecture Role
//!
//! - **Pure Planning Logic**: Generates action plans, contains no game state
//! - **Consumed by**: `Game/ScenarioWorld` which calls `plan_for_team()`
//! - **Not Responsible For**: Unit state, combat resolution, turn management, UI
//!
//! ## Separation of Concerns
//!
//! - `AI` crate: "What should the AI do?" (planning algorithms)
//! - `Combat` crate: "How is combat resolved?" (combat logic)
//! - `Game/ScenarioWorld`: "Execute AI actions" (state updates)
//! - `QuestApp`: "Show AI actions" (presentation)
//!
//! ## Features
//!
//! This crate provides a minimal GOAP planner with:
//! - Simple WorldState model for representing game state
//! - Action templates/instances for possible actions
//! - Forward A* planner with bounded search
//! - Team-based planning for coordinated AI behavior

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
