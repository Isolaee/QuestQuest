//! Exploring State Tests
//!
//! Tests for the ExploringState handler including unit selection,
//! deselection, and movement range calculations.

use questapp::game_scene::states::exploring::ExploringState;
use uuid::Uuid;

#[test]
fn test_exploring_state_new() {
    let state = ExploringState::new();
    assert_eq!(state.selected_unit(), None);
    assert_eq!(state.movement_range().len(), 0);
}

#[test]
fn test_exploring_state_default() {
    let state = ExploringState::default();
    assert_eq!(state.selected_unit(), None);
    assert_eq!(state.movement_range().len(), 0);
}

#[test]
fn test_deselect_unit() {
    let mut state = ExploringState::new();

    // Manually set a selected unit (normally done via select_unit with context)
    state.selected_unit = Some(Uuid::new_v4());
    state.movement_range = vec![];

    // Deselect
    state.deselect_unit();

    assert_eq!(state.selected_unit(), None);
    assert_eq!(state.movement_range().len(), 0);
}

#[test]
fn test_selected_unit_getter() {
    let state = ExploringState::new();
    assert_eq!(state.selected_unit(), None);
}

#[test]
fn test_movement_range_getter() {
    let state = ExploringState::new();
    let range = state.movement_range();
    assert_eq!(range.len(), 0);
}

#[test]
fn test_deselect_clears_selection() {
    let mut state = ExploringState::new();
    let unit_id = Uuid::new_v4();

    // Simulate selection
    state.selected_unit = Some(unit_id);
    assert_eq!(state.selected_unit(), Some(unit_id));

    // Deselect should clear it
    state.deselect_unit();
    assert_eq!(state.selected_unit(), None);
}

#[test]
fn test_deselect_clears_movement_range() {
    let mut state = ExploringState::new();

    // Simulate having a movement range
    state.movement_range = vec![];

    // Deselect should clear it
    state.deselect_unit();
    assert_eq!(state.movement_range().len(), 0);
}

#[test]
fn test_multiple_deselects_are_safe() {
    let mut state = ExploringState::new();

    // Deselecting when nothing is selected should be safe
    state.deselect_unit();
    assert_eq!(state.selected_unit(), None);

    state.deselect_unit();
    assert_eq!(state.selected_unit(), None);
}

#[test]
fn test_exploring_state_field_access() {
    let mut state = ExploringState::new();
    let unit_id = Uuid::new_v4();

    // Direct field access for testing
    state.selected_unit = Some(unit_id);
    assert_eq!(state.selected_unit, Some(unit_id));

    state.selected_unit = None;
    assert_eq!(state.selected_unit, None);
}

#[test]
fn test_movement_range_empty_by_default() {
    let state = ExploringState::new();
    assert!(state.movement_range().is_empty());
}

#[test]
fn test_exploring_state_is_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<ExploringState>();
    assert_sync::<ExploringState>();
}
