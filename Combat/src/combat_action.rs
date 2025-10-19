use serde::{Deserialize, Serialize};

/// Combat actions that a unit can perform
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CombatAction {
    Attack { damage: i32 },
    Heal { amount: i32 },
    Defend, // Increases defense for one turn
    Skip,   // Do nothing
}

impl CombatAction {
    /// Get the display name of the action
    pub fn get_name(&self) -> &'static str {
        match self {
            CombatAction::Attack { .. } => "Attack",
            CombatAction::Heal { .. } => "Heal",
            CombatAction::Defend => "Defend",
            CombatAction::Skip => "Skip",
        }
    }
}
