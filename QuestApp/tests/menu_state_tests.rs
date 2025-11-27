//! Menu State Tests
//!
//! Tests for the MenuState handler.

use questapp::game_scene::states::menu::MenuState;

#[test]
fn test_menu_state_new() {
    let state = MenuState::new();
    // MenuState is a unit struct, so just verify it can be created
    let _ = state;
}

#[test]
fn test_menu_state_multiple_instances() {
    let state1 = MenuState::new();
    let state2 = MenuState::new();

    // Unit structs should be equal
    assert_eq!(state1, state2);
}

#[test]
fn test_menu_state_default() {
    let state = MenuState;
    let new_state = MenuState::new();

    assert_eq!(state, new_state);
}

#[test]
fn test_menu_state_copy() {
    let state1 = MenuState::new();
    let state2 = state1; // Copy semantics for unit struct

    assert_eq!(state1, state2);
}

#[test]
fn test_menu_state_clone() {
    let state1 = MenuState::new();
    let state2 = state1;

    assert_eq!(state1, state2);
}

#[test]
fn test_menu_state_debug() {
    let state = MenuState::new();
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("MenuState"));
}

#[test]
fn test_menu_state_is_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<MenuState>();
    assert_sync::<MenuState>();
}

#[test]
fn test_menu_state_size() {
    use std::mem;

    // Unit struct should have zero size
    assert_eq!(mem::size_of::<MenuState>(), 0);
}
