use graphics::HexCoord;
use units::*;

#[test]
fn test_combat_damage_calculation() {
    let position = HexCoord::new(0, 0);
    let attacker = Unit::new(
        "Attacker".to_string(),
        position,
        Race::Orc,          // +2 attack
        UnitClass::Warrior, // +2 attack, total 4 attack
    );

    let defender = Unit::new(
        "Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Dwarf,        // +2 defense
        UnitClass::Warrior, // +3 defense, total 5 defense
    );

    let damage = attacker.calculate_damage_to(&defender);

    // Damage should be at least 1 (minimum damage rule)
    // With 4 attack vs 5 defense, should deal 1 damage
    assert_eq!(damage, 1);
}

#[test]
fn test_combat_high_damage() {
    let position = HexCoord::new(0, 0);
    let mut strong_attacker = Unit::new(
        "Strong Attacker".to_string(),
        position,
        Race::Orc,          // +2 attack
        UnitClass::Warrior, // +2 attack
    );

    // Add a powerful weapon
    let weapon = Item::new(
        "Great Sword".to_string(),
        "A massive two-handed sword".to_string(),
        item::ItemProperties::Weapon {
            attack_bonus: 10,
            range_modifier: 0,
            range_type_override: None,
        },
    );

    let weapon_id = weapon.id;
    strong_attacker.add_item_to_inventory(weapon);
    let _ = strong_attacker.equip_item(weapon_id);

    let weak_defender = Unit::new(
        "Weak Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Human,     // No bonuses
        UnitClass::Mage, // Low defense
    );

    let damage = strong_attacker.calculate_damage_to(&weak_defender);

    // Should deal significant damage
    assert!(damage > 5);
}

#[test]
fn test_death_and_revival() {
    let position = HexCoord::new(0, 0);
    let mut unit = Unit::new(
        "Mortal Unit".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
    );

    // Reduce health to 0
    unit.combat_stats.health = 0;
    assert!(!unit.is_alive());

    // Heal the unit
    unit.heal(10);
    assert!(unit.is_alive());
    assert_eq!(unit.combat_stats.health, 10);
}

#[test]
fn test_healing_cap() {
    let position = HexCoord::new(0, 0);
    let mut unit = Unit::new(
        "Healing Unit".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
    );

    let max_health = unit.combat_stats.max_health;

    // Try to overheal
    unit.heal(max_health * 2);

    // Should not exceed max health
    assert_eq!(unit.combat_stats.health, max_health);
}

#[test]
fn test_experience_thresholds() {
    let position = HexCoord::new(0, 0);
    let mut unit = Unit::new(
        "Learning Unit".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
    );

    // Test level 1 to 2 (needs 1*1*100 = 100 exp)
    assert!(!unit.add_experience(99)); // Should not level up
    assert_eq!(unit.level, 1);

    assert!(unit.add_experience(1)); // Should level up to 2
    assert_eq!(unit.level, 2);
    assert_eq!(unit.experience, 100);

    // Test level 2 to 3 (needs 2*2*100 = 400 total exp, so 300 more)
    assert!(!unit.add_experience(299)); // Should not level up yet
    assert_eq!(unit.level, 2);

    assert!(unit.add_experience(1)); // Should level up to 3
    assert_eq!(unit.level, 3);
    assert_eq!(unit.experience, 400);
}

#[test]
fn test_level_up_stat_increases() {
    let position = HexCoord::new(0, 0);
    let mut unit = Unit::new(
        "Growing Unit".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
    );

    let initial_health = unit.combat_stats.max_health;
    let initial_attack = unit.combat_stats.attack;
    let initial_defense = unit.combat_stats.defense;

    // Level up
    unit.add_experience(100);

    // Stats should increase
    assert!(unit.combat_stats.max_health > initial_health);
    assert!(unit.combat_stats.attack >= initial_attack); // May not always increase due to randomization
    assert!(unit.combat_stats.defense >= initial_defense);

    // Health should be full after level up
    assert_eq!(unit.combat_stats.health, unit.combat_stats.max_health);
}

#[test]
fn test_movement_range_calculation() {
    let start = HexCoord::new(0, 0);
    let unit = Unit::new(
        "Walker".to_string(),
        start,
        Race::Elf,         // +1 movement
        UnitClass::Archer, // Base movement
    );

    let movement_speed = unit.combat_stats.movement_speed;

    // Test exact movement range
    let target = HexCoord::new(movement_speed, 0);
    assert!(unit.can_move_to(target));

    // Test beyond movement range
    let too_far = HexCoord::new(movement_speed + 1, 0);
    assert!(!unit.can_move_to(too_far));

    // Test diagonal movement
    let diagonal = HexCoord::new(movement_speed / 2, movement_speed / 2);
    let distance = start.distance(diagonal);
    if distance <= movement_speed {
        assert!(unit.can_move_to(diagonal));
    } else {
        assert!(!unit.can_move_to(diagonal));
    }
}

#[test]
fn test_attack_range_with_modifiers() {
    let position = HexCoord::new(0, 0);
    let mut archer = Unit::new(
        "Archer".to_string(),
        position,
        Race::Human,
        UnitClass::Archer, // 3 range
    );

    // Add weapon with range modifier
    let longbow = Item::new(
        "Longbow".to_string(),
        "A bow with extended range".to_string(),
        item::ItemProperties::Weapon {
            attack_bonus: 1,
            range_modifier: 2, // +2 range
            range_type_override: None,
        },
    );

    let bow_id = longbow.id;
    archer.add_item_to_inventory(longbow);
    let _ = archer.equip_item(bow_id);

    // Should now have 5 range (3 + 2)
    assert!(archer.can_attack(HexCoord::new(5, 0)));
    assert!(!archer.can_attack(HexCoord::new(6, 0)));
}
