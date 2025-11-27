//! Combat State Tests
//!
//! Tests for the CombatState handler including state creation
//! and field verification.

use questapp::game_scene::states::combat::CombatState;
use uuid::Uuid;

#[test]
fn test_combat_state_new() {
    let attacker = Uuid::new_v4();
    let defender = Uuid::new_v4();

    let state = CombatState::new(attacker, defender);

    assert_eq!(state.attacker_id, attacker);
    assert_eq!(state.defender_id, defender);
}

#[test]
fn test_combat_state_different_units() {
    let attacker = Uuid::new_v4();
    let defender = Uuid::new_v4();

    assert_ne!(attacker, defender);

    let state = CombatState::new(attacker, defender);

    assert_ne!(state.attacker_id, state.defender_id);
}

#[test]
fn test_combat_state_same_unit_both_sides() {
    // While illogical in gameplay, the state should still be creatable
    let unit = Uuid::new_v4();

    let state = CombatState::new(unit, unit);

    assert_eq!(state.attacker_id, unit);
    assert_eq!(state.defender_id, unit);
    assert_eq!(state.attacker_id, state.defender_id);
}

#[test]
fn test_combat_state_field_access() {
    let attacker = Uuid::new_v4();
    let defender = Uuid::new_v4();

    let state = CombatState::new(attacker, defender);

    // Direct field access
    let retrieved_attacker = state.attacker_id;
    let retrieved_defender = state.defender_id;

    assert_eq!(retrieved_attacker, attacker);
    assert_eq!(retrieved_defender, defender);
}

#[test]
fn test_combat_state_multiple_instances() {
    let attacker1 = Uuid::new_v4();
    let defender1 = Uuid::new_v4();
    let attacker2 = Uuid::new_v4();
    let defender2 = Uuid::new_v4();

    let state1 = CombatState::new(attacker1, defender1);
    let state2 = CombatState::new(attacker2, defender2);

    assert_eq!(state1.attacker_id, attacker1);
    assert_eq!(state1.defender_id, defender1);
    assert_eq!(state2.attacker_id, attacker2);
    assert_eq!(state2.defender_id, defender2);

    assert_ne!(state1.attacker_id, state2.attacker_id);
    assert_ne!(state1.defender_id, state2.defender_id);
}

#[test]
fn test_combat_state_uuid_preservation() {
    let attacker = Uuid::new_v4();
    let defender = Uuid::new_v4();

    let state = CombatState::new(attacker, defender);

    // Verify UUIDs are preserved exactly
    assert_eq!(state.attacker_id.to_string(), attacker.to_string());
    assert_eq!(state.defender_id.to_string(), defender.to_string());
}

#[test]
fn test_combat_state_is_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<CombatState>();
    assert_sync::<CombatState>();
}
