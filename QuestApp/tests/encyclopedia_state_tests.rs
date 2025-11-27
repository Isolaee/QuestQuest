//! Encyclopedia State Tests
//!
//! Tests for the EncyclopediaState handler including visibility
//! management and state verification.

use questapp::game_scene::states::encyclopedia::EncyclopediaState;

#[test]
fn test_encyclopedia_state_new() {
    let state = EncyclopediaState::new();
    assert!(!state.visible);
}

#[test]
fn test_encyclopedia_state_default() {
    let state = EncyclopediaState::default();
    assert!(!state.visible);
}

#[test]
fn test_encyclopedia_state_starts_hidden() {
    let state = EncyclopediaState::new();
    assert!(!state.visible);
}

#[test]
fn test_encyclopedia_visibility_can_be_changed() {
    let mut state = EncyclopediaState::new();
    assert!(!state.visible);

    state.visible = true;
    assert!(state.visible);

    state.visible = false;
    assert!(!state.visible);
}

#[test]
fn test_encyclopedia_toggle_visibility() {
    let mut state = EncyclopediaState::new();
    let initial = state.visible;

    state.visible = !state.visible;
    assert_eq!(state.visible, !initial);

    state.visible = !state.visible;
    assert_eq!(state.visible, initial);
}

#[test]
fn test_encyclopedia_multiple_toggles() {
    let mut state = EncyclopediaState::new();
    assert!(!state.visible);

    for _ in 0..10 {
        state.visible = !state.visible;
    }

    // After even number of toggles, should be back to original
    assert!(!state.visible);
}

#[test]
fn test_encyclopedia_direct_field_access() {
    let mut state = EncyclopediaState::new();

    // Direct read
    let vis = state.visible;
    assert!(!vis);

    // Direct write
    state.visible = true;
    assert!(state.visible);
}

#[test]
fn test_encyclopedia_state_multiple_instances() {
    let state1 = EncyclopediaState::new();
    let mut state2 = EncyclopediaState::new();

    state2.visible = true;

    assert!(!state1.visible);
    assert!(state2.visible);
}

#[test]
fn test_encyclopedia_state_clone() {
    let state1 = EncyclopediaState::new();
    let state2 = state1.clone();

    assert_eq!(state1.visible, state2.visible);
}

#[test]
fn test_encyclopedia_state_clone_with_visibility() {
    let mut state1 = EncyclopediaState::new();
    state1.visible = true;

    let state2 = state1.clone();
    assert!(state2.visible);
}

#[test]
fn test_encyclopedia_state_debug() {
    let state = EncyclopediaState::new();
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("EncyclopediaState"));
    assert!(debug_str.contains("visible"));
}

#[test]
fn test_encyclopedia_state_is_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<EncyclopediaState>();
    assert_sync::<EncyclopediaState>();
}

#[test]
fn test_encyclopedia_boolean_properties() {
    let mut state = EncyclopediaState::new();

    // Boolean should only have two values
    state.visible = true;
    assert!(state.visible);

    state.visible = false;
    assert!(!state.visible);

    // Using boolean algebra
    state.visible = false;
    assert!(!state.visible);

    state.visible = true;
    assert!(state.visible);
}
