//! Exploring State
//!
//! Handles normal exploration mode where the player can:
//! - Select units by right-clicking
//! - Move units by left-clicking on valid hexes
//! - View unit information
//! - Initiate combat with enemy units

use game::GameWorld;
use graphics::{HexCoord, HexGrid, Renderer, UiPanel};
use uuid::Uuid;

/// Context needed for exploring state operations
///
/// Contains references to all game systems required for exploration mode.
/// This struct is passed to state methods that need to interact with the
/// game world, rendering, or UI systems.
///
/// # Lifetime
///
/// The lifetime parameter `'a` ensures that all references remain valid
/// for the duration of the operation.
///
/// # Fields
///
/// * `game_world` - Mutable reference to the game world (units, terrain, objects)
/// * `hex_grid` - Reference to the hex grid for coordinate calculations
/// * `renderer` - Optional mutable reference to the renderer for visual updates
/// * `ui_panel` - Optional mutable reference to the UI panel
/// * `screen_width` - Screen width in pixels for coordinate conversions
/// * `screen_height` - Screen height in pixels for coordinate conversions
#[allow(dead_code)]
pub struct ExploringContext<'a> {
    pub game_world: &'a mut GameWorld,
    pub hex_grid: &'a HexGrid,
    pub renderer: Option<&'a mut Renderer>,
    pub ui_panel: Option<&'a mut UiPanel>,
    pub screen_width: f32,
    pub screen_height: f32,
}

/// Exploring state handler
///
/// Manages the exploration mode where players select units and view their
/// movement ranges. This is the primary gameplay state.
///
/// # State
///
/// * `selected_unit` - The UUID of the currently selected unit, or `None`
/// * `movement_range` - List of hexes the selected unit can move to
///
/// # Movement Range Calculation
///
/// When a unit is selected, the movement range is calculated using a
/// breadth-first search algorithm that considers:
/// - Terrain movement costs
/// - Unit's remaining movement points
/// - Occupied hexes (blocked by other units)
/// - Impassable terrain
///
/// # Examples
///
/// ```rust,no_run
/// use questapp::game_scene::states::exploring::ExploringState;
///
/// let mut exploring = ExploringState::new();
/// assert_eq!(exploring.selected_unit(), None);
///
/// exploring.deselect_unit();
/// assert_eq!(exploring.movement_range().len(), 0);
/// ```
pub struct ExploringState {
    /// Currently selected unit for movement
    pub selected_unit: Option<Uuid>,
    /// Valid movement hexes for the selected unit
    pub movement_range: Vec<HexCoord>,
}

impl ExploringState {
    /// Creates a new exploring state with no unit selected
    ///
    /// # Returns
    ///
    /// A new `ExploringState` with empty selection and movement range
    pub fn new() -> Self {
        Self {
            selected_unit: None,
            movement_range: Vec::new(),
        }
    }

    /// Selects a unit and calculates its movement range
    ///
    /// When a unit is selected, this method:
    /// 1. Updates the `selected_unit` field
    /// 2. Calculates all reachable hexes within the unit's movement points
    /// 3. Stores the results in `movement_range`
    ///
    /// # Arguments
    ///
    /// * `unit_id` - The UUID of the unit to select
    /// * `ctx` - Context containing the game world and other systems
    ///
    /// # Returns
    ///
    /// Deselects the currently selected unit
    ///
    /// Clears both the selected unit and its movement range. This is typically
    /// called when the player clicks on empty terrain or presses ESC.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::states::exploring::ExploringState;
    ///
    /// let mut exploring = ExploringState::new();
    /// exploring.deselect_unit();
    ///
    /// assert_eq!(exploring.selected_unit(), None);
    /// assert_eq!(exploring.movement_range().len(), 0);
    /// ```
    pub fn deselect_unit(&mut self) {
        self.selected_unit = None;
        self.movement_range.clear();
    }

    /// Sets the selected unit directly without calculating movement range
    ///
    /// Use this when you need to set the selection without having a GameWorld context.
    /// Typically followed by a call to `set_movement_range()`.
    ///
    /// # Arguments
    ///
    /// * `unit_id` - The unit to select, or None to deselect
    pub fn set_selected_unit(&mut self, unit_id: Option<Uuid>) {
        self.selected_unit = unit_id;
        if unit_id.is_none() {
            self.movement_range.clear();
        }
    }

    /// Sets the movement range directly
    ///
    /// Use this when you've calculated the movement range externally (e.g., using
    /// ScenarioWorld::all_legal_moves) and want to update the state.
    ///
    /// # Arguments
    ///
    /// * `range` - Vector of reachable hex coordinates
    pub fn set_movement_range(&mut self, range: Vec<HexCoord>) {
        self.movement_range = range;
    }

    /// Calculates all hexes reachable within the given movement points
    ///
    /// # Returns
    ///
    /// * `Some(Uuid)` - If a unit is selected
    /// * `None` - If no unit is selected
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::states::exploring::ExploringState;
    ///
    /// let exploring = ExploringState::new();
    /// assert_eq!(exploring.selected_unit(), None);
    /// ```
    pub fn selected_unit(&self) -> Option<Uuid> {
        self.selected_unit
    }

    /// Returns a slice of all hexes the selected unit can move to
    ///
    /// # Returns
    ///
    /// A slice of hex coordinates representing valid movement destinations.
    /// Empty if no unit is selected.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::states::exploring::ExploringState;
    ///
    /// let exploring = ExploringState::new();
    /// assert_eq!(exploring.movement_range().len(), 0);
    /// ```
    pub fn movement_range(&self) -> &[HexCoord] {
        &self.movement_range
    }
}

impl Default for ExploringState {
    fn default() -> Self {
        Self::new()
    }
}
