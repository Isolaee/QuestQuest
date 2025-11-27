//! Game State Tests
//!
//! Tests for the GameState enum and state transitions

use questapp::game_scene::states::GameState;
use uuid::Uuid;

#[test]
fn test_game_state_default() {
    let state = GameState::default();
    assert_eq!(state, GameState::Exploring);
}

#[test]
fn test_game_state_equality() {
    let state1 = GameState::Exploring;
    let state2 = GameState::Exploring;
    assert_eq!(state1, state2);
}

#[test]
fn test_game_state_combat_confirmation() {
    let attacker = Uuid::new_v4();
    let defender = Uuid::new_v4();

    let state = GameState::CombatConfirmation {
        attacker_id: attacker,
        defender_id: defender,
    };

    match state {
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
fn test_game_state_item_pickup() {
    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();
    let item_name = "Health Potion".to_string();

    let state = GameState::ItemPickup {
        unit_id,
        item_id,
        item_name: item_name.clone(),
    };

    match state {
        GameState::ItemPickup {
            unit_id: u,
            item_id: i,
            item_name: n,
        } => {
            assert_eq!(u, unit_id);
            assert_eq!(i, item_id);
            assert_eq!(n, item_name);
        }
        _ => panic!("Expected ItemPickup state"),
    }
}

#[test]
fn test_game_state_encyclopedia() {
    let state = GameState::Encyclopedia;
    assert_eq!(state, GameState::Encyclopedia);
}

#[test]
fn test_game_state_menu() {
    let state = GameState::Menu;
    assert_eq!(state, GameState::Menu);
}

#[test]
fn test_game_state_animating() {
    let unit_id = Uuid::new_v4();
    let state = GameState::Animating { unit_id };

    match state {
        GameState::Animating { unit_id: u } => {
            assert_eq!(u, unit_id);
        }
        _ => panic!("Expected Animating state"),
    }
}

#[test]
fn test_game_state_clone() {
    let attacker = Uuid::new_v4();
    let defender = Uuid::new_v4();

    let state1 = GameState::CombatConfirmation {
        attacker_id: attacker,
        defender_id: defender,
    };
    let state2 = state1.clone();

    assert_eq!(state1, state2);
}

#[test]
fn test_game_state_debug() {
    let state = GameState::Exploring;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Exploring"));
}
