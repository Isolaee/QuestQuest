//! Game Scene Module
//!
//! This module provides the core state management system for the game scene.
//! It implements a clean state machine architecture where each game state
//! (exploring, combat, menu, etc.) is handled by a separate, testable module.
//!
//! # Architecture
//!
//! The state management follows a hierarchical pattern:
//!
//! - [`GameSceneState`] - The top-level coordinator that manages state transitions
//!   and delegates operations to specific state handlers
//! - [`GameState`] - An enum representing all possible game states
//! - State handlers - Individual modules for each state (exploring, combat, etc.)
//!
//! # State Handlers
//!
//! - [`ExploringState`] - Manages unit selection, movement range calculation, and
//!   normal gameplay exploration
//! - [`CombatState`] - Handles combat confirmation dialogs (future implementation)
//! - [`PickupState`] - Manages item pickup prompts (future implementation)
//! - [`EncyclopediaState`] - Controls the in-game encyclopedia/wiki display
//!
//! # Examples
//!
//! ```rust,no_run
//! use questapp::game_scene::GameSceneState;
//! use questapp::game_scene::GameState;
//!
//! // Create a new game scene state manager
//! let mut game_state = GameSceneState::new();
//!
//! // Transition to encyclopedia state
//! game_state.transition_to(GameState::Encyclopedia);
//! ```

pub mod states;

pub use states::combat::CombatState;
pub use states::encyclopedia::EncyclopediaState;
pub use states::exploring::ExploringState;
pub use states::pickup::PickupState;
pub use states::submenu::SubmenuState;
pub use states::GameState;

/// Game scene state manager
///
/// `GameSceneState` is the central coordinator for all game states. It maintains
/// references to all state handlers and manages transitions between them.
///
/// # Fields
///
/// - `current_state` - The currently active game state
/// - `exploring` - Persistent handler for exploration mode
/// - `combat` - Optional handler for combat confirmation (created on-demand)
/// - `pickup` - Optional handler for item pickups (created on-demand)
/// - `encyclopedia` - Persistent handler for encyclopedia display
///
/// # State Lifecycle
///
/// - **Persistent states**: `exploring` and `encyclopedia` exist for the entire
///   game session and maintain their state across transitions
/// - **On-demand states**: `combat` and `pickup` are created when entering those
///   states and destroyed when exiting
///
/// # Examples
///
/// ```rust,no_run
/// use questapp::game_scene::GameSceneState;
/// use uuid::Uuid;
///
/// let mut state = GameSceneState::new();
///
/// // Select a unit in exploring state
/// let unit_id = Uuid::new_v4();
/// // state.exploring.select_unit(unit_id, &mut context);
/// ```
pub struct GameSceneState {
    /// Current game state
    pub current_state: GameState,

    /// Exploring state handler (persistent)
    pub exploring: ExploringState,

    /// Combat state handler (created on-demand)
    pub combat: Option<CombatState>,

    /// Pickup state handler (created on-demand)
    pub pickup: Option<PickupState>,

    /// Encyclopedia state handler
    pub encyclopedia: EncyclopediaState,

    /// Game submenu state handler (persistent)
    #[allow(dead_code)]
    pub submenu: SubmenuState,
}

impl GameSceneState {
    /// Creates a new game scene state manager
    ///
    /// Initializes all persistent state handlers and sets the initial state
    /// to [`GameState::Exploring`].
    ///
    /// # Returns
    ///
    /// A new `GameSceneState` with default state (Exploring mode)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::GameSceneState;
    ///
    /// let state = GameSceneState::new();
    /// assert!(state.current_state == questapp::game_scene::GameState::Exploring);
    /// ```
    pub fn new() -> Self {
        Self {
            current_state: GameState::default(),
            exploring: ExploringState::new(),
            combat: None,
            pickup: None,
            encyclopedia: EncyclopediaState::new(),
            submenu: SubmenuState::new(),
        }
    }

    /// Transitions to a new game state
    ///
    /// Handles cleanup of the old state and initialization of the new state.
    /// On-demand state handlers (combat, pickup) are created or destroyed as needed.
    ///
    /// # Arguments
    ///
    /// * `new_state` - The target state to transition to
    ///
    /// # State Cleanup
    ///
    /// When leaving:
    /// - `CombatConfirmation` - Destroys the combat state handler
    /// - `ItemPickup` - Destroys the pickup state handler
    ///
    /// # State Initialization
    ///
    /// When entering:
    /// - `CombatConfirmation` - Creates a new combat state with attacker/defender IDs
    /// - `ItemPickup` - Creates a new pickup state with unit, item, and item name
    /// - Other states use persistent handlers
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use questapp::game_scene::{GameSceneState, GameState};
    /// use uuid::Uuid;
    ///
    /// let mut state = GameSceneState::new();
    ///
    /// // Transition to encyclopedia
    /// state.transition_to(GameState::Encyclopedia);
    ///
    /// // Transition to combat
    /// let attacker = Uuid::new_v4();
    /// let defender = Uuid::new_v4();
    /// state.transition_to(GameState::CombatConfirmation {
    ///     attacker_id: attacker,
    ///     defender_id: defender,
    /// });
    /// ```
    pub fn transition_to(&mut self, new_state: GameState) {
        println!(
            "🔄 State transition: {:?} -> {:?}",
            self.current_state, new_state
        );

        // Clean up old state
        match &self.current_state {
            GameState::CombatConfirmation { .. } => {
                self.combat = None;
            }
            GameState::ItemPickup { .. } => {
                self.pickup = None;
            }
            _ => {}
        }

        // Initialize new state
        match &new_state {
            GameState::CombatConfirmation {
                attacker_id,
                defender_id,
            } => {
                self.combat = Some(CombatState::new(*attacker_id, *defender_id));
            }
            GameState::ItemPickup {
                unit_id,
                item_id,
                item_name,
            } => {
                self.pickup = Some(PickupState::new(*unit_id, *item_id, item_name.clone()));
            }
            GameState::Menu => {
                // Menu state is persistent
            }
            GameState::Encyclopedia => {
                // Encyclopedia state is persistent
            }
            GameState::Exploring => {
                // Exploring state is persistent
            }
            GameState::Animating { .. } => {
                // Animation state doesn't need special initialization
            }
            GameState::GameSubmenu => {
                // Submenu state is persistent
            }
        }

        self.current_state = new_state;
    }
}

impl Default for GameSceneState {
    fn default() -> Self {
        Self::new()
    }
}
