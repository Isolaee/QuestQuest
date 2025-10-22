use combat::{resolve_combat, CombatStats, DamageType, RangeCategory, Resistances};

#[test]
fn unit_cannot_attack_twice_and_flag_resets() {
    let mut attacker = CombatStats::new_with_attacks(
        100, // max_health
        20,  // base_attack
        5,   // movement_speed
        RangeCategory::Melee,
        Resistances::default(),
        10, // attack_strength
        3,  // attacks_per_round (would normally allow multiple)
    );

    let mut defender = CombatStats::new(80, 12, 4, RangeCategory::Melee, Resistances::default());

    // First combat: attacker should be able to attack once; flag set
    let _res1 = resolve_combat(&mut attacker, &mut defender, DamageType::Slash);
    assert!(
        attacker.attacked_this_turn,
        "Attacker should be marked as having attacked"
    );

    // Try to run combat again in same turn: attacker should be skipped (no additional attacks)
    let defender_before = defender.health;
    let res2 = resolve_combat(&mut attacker, &mut defender, DamageType::Slash);
    // Since attacker already attacked this turn, combat should not even start
    assert_eq!(res2.attacker_damage_dealt, 0);
    assert_eq!(res2.defender_damage_dealt, 0);
    assert!(
        !res2.attacker_hit && !res2.defender_hit,
        "No hits should be recorded"
    );
    // Defender health should be unchanged
    assert_eq!(
        defender_before, defender.health,
        "Defender should not take damage when attacker already attacked this turn"
    );

    // Reset turn flags and verify attacker can attack again
    attacker.reset_turn_flags();
    defender.reset_turn_flags();
    assert!(
        !attacker.attacked_this_turn,
        "Flag should be cleared after reset"
    );

    // Combat now should proceed and possibly change defender health
    let _res3 = resolve_combat(&mut attacker, &mut defender, DamageType::Slash);
    assert!(
        attacker.attacked_this_turn,
        "Attacker should be marked again after attacking in new turn"
    );
}
