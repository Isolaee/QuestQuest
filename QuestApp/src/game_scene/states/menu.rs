//! Menu State
//!
//! Handles the in-game menu where the player can:
//! - Continue playing
//! - Access settings
//! - Save the game
//! - Return to main menu
//! - Quit the game

/// Menu state handler
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MenuState;

impl MenuState {
    /// Creates a new menu state
    ///
    /// # Returns
    ///
    /// A new `MenuState` ready to handle menu interactions
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::states::menu::MenuState;
    ///
    /// let menu = MenuState::new();
    /// ```
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }
}
