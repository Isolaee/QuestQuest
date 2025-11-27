//! Combat Confirmation State
//!
//! Handles the combat confirmation dialog where the player:
//! - Reviews attacker and defender stats
//! - Selects which attack to use
//! - Confirms or cancels combat

use uuid::Uuid;

/// Combat state handler
#[allow(dead_code)]
pub struct CombatState {
    pub attacker_id: Uuid,
    pub defender_id: Uuid,
}

impl CombatState {
    /// Creates a new combat confirmation state
    ///
    /// # Arguments
    ///
    /// * `attacker_id` - UUID of the attacking unit
    /// * `defender_id` - UUID of the defending unit
    ///
    /// # Returns
    ///
    /// A new `CombatState` with the specified attacker and defender
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::states::combat::CombatState;
    /// use uuid::Uuid;
    ///
    /// let attacker = Uuid::new_v4();
    /// let defender = Uuid::new_v4();
    /// let combat = CombatState::new(attacker, defender);
    /// ```
    pub fn new(attacker_id: Uuid, defender_id: Uuid) -> Self {
        Self {
            attacker_id,
            defender_id,
        }
    }
}
