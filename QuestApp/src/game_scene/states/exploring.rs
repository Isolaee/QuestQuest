//! Exploring State
//!
//! Handles normal exploration mode where the player can:
//! - Select units by right-clicking
//! - Move units by left-clicking on valid hexes
//! - View unit information
//! - Initiate combat with enemy units

use super::GameState;
use game::{GameObject, GameWorld};
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
    /// Always returns `None` to stay in the exploring state
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::states::exploring::ExploringState;
    /// use uuid::Uuid;
    ///
    /// let mut exploring = ExploringState::new();
    /// let unit_id = Uuid::new_v4();
    ///
    /// // exploring.select_unit(unit_id, &mut context);
    /// // assert_eq!(exploring.selected_unit(), Some(unit_id));
    /// ```
    pub fn select_unit(&mut self, unit_id: Uuid, ctx: &mut ExploringContext) -> Option<GameState> {
        self.selected_unit = Some(unit_id);
        self.update_movement_range(ctx);
        None // Stay in exploring state
    }

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

    /// Updates the movement range for the currently selected unit
    ///
    /// Recalculates which hexes the selected unit can reach based on its
    /// current position and remaining movement points.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Context containing the game world
    ///
    /// # Behavior
    ///
    /// - If no unit is selected, clears the movement range
    /// - Otherwise, uses breadth-first search to calculate reachable hexes
    fn update_movement_range(&mut self, ctx: &ExploringContext) {
        self.movement_range.clear();

        if let Some(unit_id) = self.selected_unit {
            if let Some(unit) = ctx.game_world.get_unit(unit_id) {
                let start_pos = unit.position();
                let movement_points = unit.moves_left();

                // Calculate reachable hexes using pathfinding
                self.movement_range =
                    self.calculate_reachable_hexes(start_pos, movement_points, ctx.game_world);
            }
        }
    }

    /// Calculates all hexes reachable within the given movement points
    ///
    /// Uses a breadth-first search algorithm to find all hexes that can be
    /// reached from the start position within the movement cost budget.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting hex coordinate
    /// * `max_cost` - Maximum movement points available
    /// * `world` - Reference to the game world for terrain costs and obstacles
    ///
    /// # Returns
    ///
    /// A vector of all reachable hex coordinates (excluding the start hex)
    ///
    /// # Algorithm
    ///
    /// 1. Start at the unit's position with cost 0
    /// 2. For each hex, check all 6 neighbors
    /// 3. Calculate movement cost considering terrain
    /// 4. Skip impassable terrain (cost = i32::MAX)
    /// 5. Skip if total cost exceeds max_cost
    /// 6. Skip occupied hexes (via is_position_valid_for_movement)
    /// 7. Add valid neighbors to the queue
    /// 8. Track visited hexes to avoid recalculation
    fn calculate_reachable_hexes(
        &self,
        start: HexCoord,
        max_cost: i32,
        world: &GameWorld,
    ) -> Vec<HexCoord> {
        use std::collections::{HashMap, VecDeque};

        let mut reachable = Vec::new();
        let mut visited: HashMap<HexCoord, i32> = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back((start, 0));
        visited.insert(start, 0);

        while let Some((pos, cost)) = queue.pop_front() {
            if cost > 0 {
                reachable.push(pos);
            }

            for neighbor in pos.neighbors() {
                let move_cost = world.get_movement_cost(neighbor);
                if move_cost == i32::MAX {
                    continue; // Impassable
                }

                let new_cost = cost + move_cost;
                if new_cost > max_cost {
                    continue;
                }

                if let Some(&existing_cost) = visited.get(&neighbor) {
                    if new_cost >= existing_cost {
                        continue;
                    }
                }

                // Check if position is valid for movement
                if !world.is_position_valid_for_movement(neighbor, self.selected_unit) {
                    continue;
                }

                visited.insert(neighbor, new_cost);
                queue.push_back((neighbor, new_cost));
            }
        }

        reachable
    }

    /// Returns the UUID of the currently selected unit
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
