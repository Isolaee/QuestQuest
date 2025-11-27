//! Encyclopedia State
//!
//! Handles the encyclopedia/wiki display where the player can:
//! - Browse game information
//! - Switch between categories (Units, Terrain, Mechanics)
//! - Scroll through entries
//! - Close the encyclopedia

/// Encyclopedia state handler
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EncyclopediaState {
    pub visible: bool,
}

impl EncyclopediaState {
    /// Creates a new encyclopedia state with the panel hidden
    ///
    /// # Returns
    ///
    /// A new `EncyclopediaState` with `visible` set to `false`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::states::encyclopedia::EncyclopediaState;
    ///
    /// let encyclopedia = EncyclopediaState::new();
    /// assert_eq!(encyclopedia.visible, false);
    /// ```
    pub fn new() -> Self {
        Self { visible: false }
    }

    /// Check if encyclopedia is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
