//! Game Scene State Manager Tests
//!
//! Tests for the GameSceneState coordinator including state transitions,
//! state handler management, and lifecycle.

use questapp::game_scene::{GameSceneState, GameState};
use uuid::Uuid;

#[test]
fn test_game_scene_state_new() {
    let state = GameSceneState::new();
    assert_eq!(state.current_state, GameState::Exploring);
}

#[test]
fn test_game_scene_state_default() {
    let state = GameSceneState::default();
    assert_eq!(state.current_state, GameState::Exploring);
}

#[test]
fn test_game_scene_state_starts_in_exploring() {
    let state = GameSceneState::new();
    assert_eq!(state.current_state, GameState::Exploring);
}

#[test]
fn test_transition_to_encyclopedia() {
    let mut state = GameSceneState::new();
    state.transition_to(GameState::Encyclopedia);
    assert_eq!(state.current_state, GameState::Encyclopedia);
}

#[test]
fn test_transition_to_combat() {
    let mut state = GameSceneState::new();
    let attacker = Uuid::new_v4();
    let defender = Uuid::new_v4();

    state.transition_to(GameState::CombatConfirmation {
        attacker_id: attacker,
        defender_id: defender,
    });

    match state.current_state {
        GameState::CombatConfirmation {
            attacker_id,
            defender_id,
        } => {
            assert_eq!(attacker_id, attacker);
            assert_eq!(defender_id, defender);
        }
        _ => panic!("Expected CombatConfirmation state"),
    }
}

#[test]
fn test_transition_to_item_pickup() {
    let mut state = GameSceneState::new();
    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();

    state.transition_to(GameState::ItemPickup {
        unit_id,
        item_id,
        item_name: "Test Item".to_string(),
    });

    match state.current_state {
        GameState::ItemPickup { .. } => {
            // Successfully transitioned
        }
        _ => panic!("Expected ItemPickup state"),
    }
}

#[test]
fn test_transition_to_menu() {
    let mut state = GameSceneState::new();
    state.transition_to(GameState::Menu);
    assert_eq!(state.current_state, GameState::Menu);
}

#[test]
fn test_transition_to_animating() {
    let mut state = GameSceneState::new();
    let unit_id = Uuid::new_v4();

    state.transition_to(GameState::Animating { unit_id });

    match state.current_state {
        GameState::Animating { unit_id: u } => {
            assert_eq!(u, unit_id);
        }
        _ => panic!("Expected Animating state"),
    }
}

#[test]
fn test_multiple_transitions() {
    let mut state = GameSceneState::new();

    // Start in Exploring
    assert_eq!(state.current_state, GameState::Exploring);

    // Go to Encyclopedia
    state.transition_to(GameState::Encyclopedia);
    assert_eq!(state.current_state, GameState::Encyclopedia);

    // Back to Exploring
    state.transition_to(GameState::Exploring);
    assert_eq!(state.current_state, GameState::Exploring);

    // To Menu
    state.transition_to(GameState::Menu);
    assert_eq!(state.current_state, GameState::Menu);
}

#[test]
fn test_exploring_handler_exists() {
    let state = GameSceneState::new();
    // Exploring handler should always exist
    assert_eq!(state.exploring.selected_unit(), None);
}

#[test]
fn test_encyclopedia_handler_exists() {
    let state = GameSceneState::new();
    // Encyclopedia handler should always exist
    assert!(!state.encyclopedia.visible);
}

#[test]
fn test_combat_handler_created_on_transition() {
    let mut state = GameSceneState::new();

    let attacker = Uuid::new_v4();
    let defender = Uuid::new_v4();

    state.transition_to(GameState::CombatConfirmation {
        attacker_id: attacker,
        defender_id: defender,
    });

    // Combat handler should be created
    assert!(state.combat.is_some());
}

#[test]
fn test_combat_handler_destroyed_on_exit() {
    let mut state = GameSceneState::new();

    let attacker = Uuid::new_v4();
    let defender = Uuid::new_v4();

    // Enter combat
    state.transition_to(GameState::CombatConfirmation {
        attacker_id: attacker,
        defender_id: defender,
    });
    assert!(state.combat.is_some());

    // Exit combat
    state.transition_to(GameState::Exploring);
    assert!(state.combat.is_none());
}

#[test]
fn test_pickup_handler_created_on_transition() {
    let mut state = GameSceneState::new();

    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();

    state.transition_to(GameState::ItemPickup {
        unit_id,
        item_id,
        item_name: "Item".to_string(),
    });

    // Pickup handler should be created
    assert!(state.pickup.is_some());
}

#[test]
fn test_pickup_handler_destroyed_on_exit() {
    let mut state = GameSceneState::new();

    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();

    // Enter pickup
    state.transition_to(GameState::ItemPickup {
        unit_id,
        item_id,
        item_name: "Item".to_string(),
    });
    assert!(state.pickup.is_some());

    // Exit pickup
    state.transition_to(GameState::Exploring);
    assert!(state.pickup.is_none());
}

#[test]
fn test_exploring_state_persists() {
    let mut state = GameSceneState::new();

    // Select a unit (directly manipulate for testing)
    state.exploring.selected_unit = Some(Uuid::new_v4());
    let selected = state.exploring.selected_unit;

    // Transition to encyclopedia and back
    state.transition_to(GameState::Encyclopedia);
    state.transition_to(GameState::Exploring);

    // Exploring state should persist
    assert_eq!(state.exploring.selected_unit, selected);
}

#[test]
fn test_encyclopedia_state_persists() {
    let mut state = GameSceneState::new();

    // Set encyclopedia visible
    state.encyclopedia.visible = true;

    // Transition away and back
    state.transition_to(GameState::Menu);
    state.transition_to(GameState::Encyclopedia);

    // Encyclopedia state should persist
    assert!(state.encyclopedia.visible);
}

#[test]
fn test_transition_to_same_state() {
    let mut state = GameSceneState::new();

    // Transition to same state should be safe
    state.transition_to(GameState::Exploring);
    assert_eq!(state.current_state, GameState::Exploring);

    state.transition_to(GameState::Exploring);
    assert_eq!(state.current_state, GameState::Exploring);
}

#[test]
fn test_game_scene_state_is_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<GameSceneState>();
    assert_sync::<GameSceneState>();
}
