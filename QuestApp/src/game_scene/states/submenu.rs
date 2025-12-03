//! Game Submenu State
//!
//! Handles the right-click game submenu that appears when no unit is selected.
//! Currently provides access to actions like recruiting units.

use graphics::HexCoord;

/// Game submenu state handler
///
/// The submenu provides contextual actions that can be performed during gameplay.
/// Currently supports:
/// - Recruit: Open unit recruitment (to be implemented)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SubmenuState {
    /// Currently selected menu item (for keyboard navigation)
    selected_item: usize,
    /// Hex coordinate where the submenu was opened (for spawning units)
    recruit_position: Option<HexCoord>,
}

/// Available submenu items
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SubmenuItem {
    Recruit,
}

impl SubmenuState {
    /// Creates a new game submenu state
    ///
    /// # Returns
    ///
    /// A new `SubmenuState` ready to handle submenu interactions
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::states::submenu::SubmenuState;
    ///
    /// let submenu = SubmenuState::new();
    /// ```
    pub fn new() -> Self {
        Self {
            selected_item: 0,
            recruit_position: None,
        }
    }

    /// Sets the position where recruitment should occur
    pub fn set_recruit_position(&mut self, position: HexCoord) {
        self.recruit_position = Some(position);
    }

    /// Gets the stored recruit position
    pub fn recruit_position(&self) -> Option<HexCoord> {
        self.recruit_position
    }

    /// Gets all available menu items
    ///
    /// # Returns
    ///
    /// A vector of all submenu items
    #[allow(dead_code)]
    pub fn items(&self) -> Vec<SubmenuItem> {
        vec![SubmenuItem::Recruit]
    }

    /// Gets the currently selected item index
    #[allow(dead_code)]
    pub fn selected_item(&self) -> usize {
        self.selected_item
    }

    /// Moves selection to the next item
    #[allow(dead_code)]
    pub fn select_next(&mut self) {
        let item_count = self.items().len();
        if item_count > 0 {
            self.selected_item = (self.selected_item + 1) % item_count;
        }
    }

    /// Moves selection to the previous item
    #[allow(dead_code)]
    pub fn select_previous(&mut self) {
        let item_count = self.items().len();
        if item_count > 0 {
            self.selected_item = if self.selected_item == 0 {
                item_count - 1
            } else {
                self.selected_item - 1
            };
        }
    }

    /// Gets the label for a submenu item
    #[allow(dead_code)]
    pub fn item_label(item: SubmenuItem) -> &'static str {
        match item {
            SubmenuItem::Recruit => "Recruit",
        }
    }
}
