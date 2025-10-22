//! # Combat Action Module
//!
//! Defines the possible actions a unit can take during combat.
//! These actions form the basis for player choices and AI behavior.

use serde::{Deserialize, Serialize};

/// Represents an action that a unit can perform during combat.
///
/// Combat actions determine what happens during a unit's turn, from dealing
/// damage to healing allies or taking defensive stances.
///
/// # Variants
///
/// - `Attack`: Deal damage to a target
/// - `Heal`: Restore health to self or an ally
/// - `Defend`: Increase defensive stats for one turn
/// - `Skip`: Take no action this turn
///
/// # Examples
///
/// ```
/// use combat::CombatAction;
///
/// let attack = CombatAction::Attack { damage: 25 };
/// assert_eq!(attack.get_name(), "Attack");
///
/// let heal = CombatAction::Heal { amount: 10 };
/// assert_eq!(heal.get_name(), "Heal");
/// ```
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
    /// Returns the display name of this action.
    ///
    /// Used for UI display and logging.
    ///
    /// # Examples
    ///
    /// ```
    /// use combat::CombatAction;
    ///
    /// let action = CombatAction::Defend;
    /// assert_eq!(action.get_name(), "Defend");
    /// ```
    pub fn get_name(&self) -> &'static str {
        match self {
            CombatAction::Attack { .. } => "Attack",
            CombatAction::Heal { .. } => "Heal",
            CombatAction::Defend => "Defend",
            CombatAction::Skip => "Skip",
        }
    }
}
