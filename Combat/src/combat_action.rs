//! # Combat Action Module
//!
//! Defines the possible actions a unit can take during combat.
//! These actions form the basis for player choices and AI behavior.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CombatAction {
    /// Deal damage to a target
    Attack {
        /// Amount of damage to deal
        damage: i32,
    },
    /// Restore health to a target
    Heal {
        /// Amount of health to restore
        amount: i32,
    },
    /// Increase defense for one turn (reduces incoming damage)
    Defend,
    /// Take no action this turn
    Skip,
}

impl CombatAction {
    /// Returns the name of the combat action as a string slice.
    pub fn get_name(&self) -> &'static str {
        match self {
            CombatAction::Attack { .. } => "Attack",
            CombatAction::Heal { .. } => "Heal",
            CombatAction::Defend => "Defend",
            CombatAction::Skip => "Skip",
        }
    }
}
