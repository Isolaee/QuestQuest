//! Game State Management
//!
//! This module defines the core state machine for the game, including all possible
//! game states and their associated data. Each state represents a distinct mode of
//! gameplay with its own interaction patterns and UI requirements.
//!
//! # Game States
//!
//! The game can be in one of the following states at any time:
//!
//! ## Core States
//!
//! - **Exploring** - The default state where players can select units, view movement
//!   ranges, and initiate actions. This is the primary gameplay mode.
//!
//! - **Encyclopedia** - Displays the in-game wiki with information about units,
//!   terrain, and game mechanics. Non-blocking for unit selection.
//!
//! ## Dialog States (Future Implementation)
//!
//! - **CombatConfirmation** - Shows a confirmation dialog when a unit attempts to
//!   attack an enemy, displaying attack options and expected outcomes.
//!
//! - **ItemPickup** - Prompts the player when a unit moves to a hex containing an
//!   item, allowing them to pick it up or leave it.
//!
//! - **Menu** - The in-game pause menu for accessing settings, saving, etc.
//!
//! - **Animating** - A special state where unit movement or combat animations are
//!   playing, blocking user input until complete.
//!
//! # State Architecture
//!
//! Each state has its own module with a dedicated handler struct:
//! - `ExploringState` in [`exploring`]
//! - `CombatState` in [`combat`]
//! - `PickupState` in [`pickup`]
//! - `MenuState` in [`menu`]
//! - `EncyclopediaState` in [`encyclopedia`]
//!
//! # Examples
//!
//! ```rust,no_run
//! use questapp::game_scene::states::GameState;
//! use uuid::Uuid;
//!
//! // Create a new exploring state (default)
//! let state = GameState::default();
//! assert_eq!(state, GameState::Exploring);
//!
//! // Create a combat confirmation state
//! let attacker = Uuid::new_v4();
//! let defender = Uuid::new_v4();
//! let combat_state = GameState::CombatConfirmation {
//!     attacker_id: attacker,
//!     defender_id: defender,
//! };
//! ```

pub mod combat;
pub mod encyclopedia;
pub mod exploring;
pub mod menu;
pub mod pickup;

use uuid::Uuid;

/// Represents all possible game states
///
/// `GameState` is an enum that defines every possible state the game can be in.
/// Each variant may contain associated data needed for that specific state.
///
/// # Variants
///
/// * `Exploring` - Normal exploration and unit movement mode
/// * `CombatConfirmation` - Combat confirmation dialog with attacker and defender
/// * `ItemPickup` - Item pickup prompt with unit, item, and item name
/// * `Menu` - In-game pause menu
/// * `Encyclopedia` - Encyclopedia/wiki display
/// * `Animating` - Unit animation is playing (blocks input)
///
/// # Examples
///
/// ```rust,no_run
/// use questapp::game_scene::states::GameState;
///
/// match GameState::Exploring {
///     GameState::Exploring => println!("Player is exploring"),
///     GameState::Encyclopedia => println!("Reading the wiki"),
///     _ => println!("Other state"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub enum GameState {
    /// Normal exploration and unit movement mode
    #[default]
    Exploring,

    /// Combat confirmation dialog is active
    /// Contains attacker and defender IDs for reference
    #[allow(dead_code)]
    CombatConfirmation {
        attacker_id: Uuid,
        defender_id: Uuid,
    },

    /// Item pickup prompt is active
    ItemPickup {
        unit_id: Uuid,
        item_id: Uuid,
        item_name: String,
    },

    /// In-game menu is displayed
    #[allow(dead_code)]
    Menu,

    /// Encyclopedia/wiki is displayed
    Encyclopedia,

    /// Unit is currently animating movement
    /// May transition to other states when animation completes
    #[allow(dead_code)]
    Animating { unit_id: Uuid },
}
