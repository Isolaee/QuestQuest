//! Item Pickup State
//!
//! Handles item pickup prompts where the player can:
//! - Choose to pick up an item
//! - Decline and leave the item

use uuid::Uuid;

/// Item pickup state handler
#[allow(dead_code)]
pub struct PickupState {
    pub unit_id: Uuid,
    pub item_id: Uuid,
    pub item_name: String,
}

impl PickupState {
    /// Creates a new item pickup state
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit that can pick up the item
    /// * `item_id` - UUID of the interactive object containing the item
    /// * `item_name` - Display name of the item
    ///
    /// # Returns
    ///
    /// A new `PickupState` with the specified unit, item, and item name
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::states::pickup::PickupState;
    /// use uuid::Uuid;
    ///
    /// let unit_id = Uuid::new_v4();
    /// let item_id = Uuid::new_v4();
    /// let pickup = PickupState::new(
    ///     unit_id,
    ///     item_id,
    ///     "Sword of Truth".to_string()
    /// );
    /// ```
    pub fn new(unit_id: Uuid, item_id: Uuid, item_name: String) -> Self {
        Self {
            unit_id,
            item_id,
            item_name,
        }
    }
}
