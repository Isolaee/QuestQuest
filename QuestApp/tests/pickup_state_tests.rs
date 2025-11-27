//! Pickup State Tests
//!
//! Tests for the PickupState handler including state creation
//! and field verification.

use questapp::game_scene::states::pickup::PickupState;
use uuid::Uuid;

#[test]
fn test_pickup_state_new() {
    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();
    let item_name = "Health Potion".to_string();

    let state = PickupState::new(unit_id, item_id, item_name.clone());

    assert_eq!(state.unit_id, unit_id);
    assert_eq!(state.item_id, item_id);
    assert_eq!(state.item_name, item_name);
}

#[test]
fn test_pickup_state_different_items() {
    let unit_id = Uuid::new_v4();
    let item_id1 = Uuid::new_v4();
    let item_id2 = Uuid::new_v4();

    let state1 = PickupState::new(unit_id, item_id1, "Sword".to_string());
    let state2 = PickupState::new(unit_id, item_id2, "Shield".to_string());

    assert_eq!(state1.unit_id, state2.unit_id);
    assert_ne!(state1.item_id, state2.item_id);
    assert_ne!(state1.item_name, state2.item_name);
}

#[test]
fn test_pickup_state_field_access() {
    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();
    let item_name = "Mana Potion".to_string();

    let state = PickupState::new(unit_id, item_id, item_name.clone());

    // Direct field access
    assert_eq!(state.unit_id, unit_id);
    assert_eq!(state.item_id, item_id);
    assert_eq!(state.item_name, item_name);
}

#[test]
fn test_pickup_state_empty_item_name() {
    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();
    let item_name = String::new();

    let state = PickupState::new(unit_id, item_id, item_name.clone());

    assert_eq!(state.item_name, "");
    assert!(state.item_name.is_empty());
}

#[test]
fn test_pickup_state_long_item_name() {
    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();
    let item_name = "The Legendary Sword of Ultimate Power and Glory".to_string();

    let state = PickupState::new(unit_id, item_id, item_name.clone());

    assert_eq!(state.item_name, item_name);
    assert_eq!(state.item_name.len(), item_name.len());
}

#[test]
fn test_pickup_state_special_characters_in_name() {
    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();
    let item_name = "Potion +5 (Rare) 🧪".to_string();

    let state = PickupState::new(unit_id, item_id, item_name.clone());

    assert_eq!(state.item_name, item_name);
}

#[test]
fn test_pickup_state_multiple_units_same_item() {
    let unit_id1 = Uuid::new_v4();
    let unit_id2 = Uuid::new_v4();
    let item_id = Uuid::new_v4();
    let item_name = "Shared Item".to_string();

    let state1 = PickupState::new(unit_id1, item_id, item_name.clone());
    let state2 = PickupState::new(unit_id2, item_id, item_name.clone());

    assert_ne!(state1.unit_id, state2.unit_id);
    assert_eq!(state1.item_id, state2.item_id);
    assert_eq!(state1.item_name, state2.item_name);
}

#[test]
fn test_pickup_state_uuid_independence() {
    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();

    assert_ne!(unit_id, item_id);

    let state = PickupState::new(unit_id, item_id, "Item".to_string());

    assert_ne!(state.unit_id, state.item_id);
}

#[test]
fn test_pickup_state_string_ownership() {
    let unit_id = Uuid::new_v4();
    let item_id = Uuid::new_v4();
    let item_name = "Test Item".to_string();

    let state = PickupState::new(unit_id, item_id, item_name.clone());

    // Original string should still be valid
    assert_eq!(item_name, "Test Item");
    // State should have its own copy
    assert_eq!(state.item_name, "Test Item");
}

#[test]
fn test_pickup_state_is_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<PickupState>();
    assert_sync::<PickupState>();
}
