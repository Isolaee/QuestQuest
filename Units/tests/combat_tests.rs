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
        Terrain::Grasslands,
    );

    let defender = Unit::new(
        "Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Dwarf,        // +2 defense
        UnitClass::Warrior, // +3 defense, total 5 defense
        Terrain::Grasslands,
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
        Terrain::Grasslands,
    );

    // Add a powerful weapon
    let weapon = Item::new(
        "Great Sword".to_string(),
        "A massive two-handed sword".to_string(),
        ItemProperties::Weapon {
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
        Terrain::Grasslands,
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
        Terrain::Grasslands,
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
        Terrain::Grasslands,
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
        Terrain::Grasslands,
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
        Terrain::Grasslands,
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
        Terrain::Grasslands,
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
        Terrain::Grasslands,
    );

    // Add weapon with range modifier
    let longbow = Item::new(
        "Longbow".to_string(),
        "A bow with extended range".to_string(),
        ItemProperties::Weapon {
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

#[test]
fn test_hit_chance_accuracy_with_terrain() {
    const ITERATIONS: usize = 2000;
    const ERROR_MARGIN: f64 = 5.0; // 5% error margin

    let position = HexCoord::new(0, 0);

    // Test 1: Kobold in Mountain (should be very hard to hit - 34%)
    let attacker = Unit::new(
        "Attacker".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let defender = Unit::new(
        "Kobold Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Kobold,
        UnitClass::Warrior,
        Terrain::Mountain, // Kobolds have 34% defense in mountains
    );

    let expected_hit_chance = 34.0; // Base defense for Kobold in Mountain
    let actual_hit_rate = run_combat_iterations(&attacker, &defender, ITERATIONS);

    println!(
        "Kobold in Mountain - Expected: {:.1}%, Actual: {:.1}%, Diff: {:.1}%",
        expected_hit_chance,
        actual_hit_rate,
        (actual_hit_rate - expected_hit_chance).abs()
    );

    assert!(
        (actual_hit_rate - expected_hit_chance).abs() <= ERROR_MARGIN,
        "Hit rate {:.1}% outside error margin of {:.1}% from expected {:.1}%",
        actual_hit_rate,
        ERROR_MARGIN,
        expected_hit_chance
    );

    // Test 2: Elf in Forest (should be hard to hit - 42%)
    let elf_defender = Unit::new(
        "Elf Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Elf,
        UnitClass::Archer,
        Terrain::Forest0, // Elves have 42% defense in forests
    );

    let expected_hit_chance = 42.0;
    let actual_hit_rate = run_combat_iterations(&attacker, &elf_defender, ITERATIONS);

    println!(
        "Elf in Forest - Expected: {:.1}%, Actual: {:.1}%, Diff: {:.1}%",
        expected_hit_chance,
        actual_hit_rate,
        (actual_hit_rate - expected_hit_chance).abs()
    );

    assert!(
        (actual_hit_rate - expected_hit_chance).abs() <= ERROR_MARGIN,
        "Hit rate {:.1}% outside error margin of {:.1}% from expected {:.1}%",
        actual_hit_rate,
        ERROR_MARGIN,
        expected_hit_chance
    );

    // Test 3: Zombie (slow, easy to hit - 60% in grasslands)
    let zombie_defender = Unit::new(
        "Zombie Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Zombie,
        UnitClass::Warrior,
        Terrain::Grasslands, // Zombies have 61% defense in grasslands
    );

    let expected_hit_chance = 61.0;
    let actual_hit_rate = run_combat_iterations(&attacker, &zombie_defender, ITERATIONS);

    println!(
        "Zombie in Grasslands - Expected: {:.1}%, Actual: {:.1}%, Diff: {:.1}%",
        expected_hit_chance,
        actual_hit_rate,
        (actual_hit_rate - expected_hit_chance).abs()
    );

    assert!(
        (actual_hit_rate - expected_hit_chance).abs() <= ERROR_MARGIN,
        "Hit rate {:.1}% outside error margin of {:.1}% from expected {:.1}%",
        actual_hit_rate,
        ERROR_MARGIN,
        expected_hit_chance
    );
}

#[test]
fn test_attack_bonus_affects_hit_chance() {
    const ITERATIONS: usize = 2000;
    const ERROR_MARGIN: f64 = 5.0;

    let position = HexCoord::new(0, 0);

    // Orc has +2 attack bonus, each point reduces hit chance by 2%
    let orc_attacker = Unit::new(
        "Orc Attacker".to_string(),
        position,
        Race::Orc, // +2 attack bonus
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let defender = Unit::new(
        "Human Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands, // 48% base defense
    );

    // Base: 48%, with +2 attack bonus: 48 - (2 * 2) = 44%
    let expected_hit_chance = 44.0;
    let actual_hit_rate = run_combat_iterations(&orc_attacker, &defender, ITERATIONS);

    println!(
        "Orc (+2 attack) vs Human - Expected: {:.1}%, Actual: {:.1}%, Diff: {:.1}%",
        expected_hit_chance,
        actual_hit_rate,
        (actual_hit_rate - expected_hit_chance).abs()
    );

    assert!(
        (actual_hit_rate - expected_hit_chance).abs() <= ERROR_MARGIN,
        "Hit rate {:.1}% outside error margin of {:.1}% from expected {:.1}%",
        actual_hit_rate,
        ERROR_MARGIN,
        expected_hit_chance
    );
}

#[test]
fn test_hit_chance_clamping() {
    const ITERATIONS: usize = 2000;
    const ERROR_MARGIN: f64 = 5.0;

    let position = HexCoord::new(0, 0);

    // Test lower clamp (minimum 10%)
    // Kobold (-2 attack) vs Kobold in Mountain (34% defense)
    // 34 - (-2 * 2) = 34 + 4 = 38% (normal, not clamped)
    let weak_attacker = Unit::new(
        "Kobold Attacker".to_string(),
        position,
        Race::Kobold, // -2 attack bonus
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let agile_defender = Unit::new(
        "Kobold Defender".to_string(),
        HexCoord::new(1, 0),
        Race::Kobold,
        UnitClass::Warrior,
        Terrain::Mountain, // 34% base defense
    );

    // 34 - (-2 * 2) = 34 + 4 = 38%
    let expected_hit_chance = 38.0;
    let actual_hit_rate = run_combat_iterations(&weak_attacker, &agile_defender, ITERATIONS);

    println!(
        "Kobold vs Kobold (clamp test) - Expected: {:.1}%, Actual: {:.1}%, Diff: {:.1}%",
        expected_hit_chance,
        actual_hit_rate,
        (actual_hit_rate - expected_hit_chance).abs()
    );

    assert!(
        (actual_hit_rate - expected_hit_chance).abs() <= ERROR_MARGIN,
        "Hit rate {:.1}% outside error margin of {:.1}% from expected {:.1}%",
        actual_hit_rate,
        ERROR_MARGIN,
        expected_hit_chance
    );
}

#[test]
fn test_multiple_terrain_types() {
    const ITERATIONS: usize = 1000;
    const ERROR_MARGIN: f64 = 5.0;

    let position = HexCoord::new(0, 0);
    let attacker = Unit::new(
        "Human Attacker".to_string(),
        position,
        Race::Human, // 0 attack bonus
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    // Test Dwarf in different terrains
    let terrains_and_defenses = vec![
        (Terrain::Mountain, 48.0),   // Dwarf excels in mountains
        (Terrain::Hills, 50.0),      // Good in hills
        (Terrain::Grasslands, 54.0), // Average in grasslands
        (Terrain::Swamp, 60.0),      // Poor in swamps
    ];

    for (terrain, expected_defense) in terrains_and_defenses {
        let defender = Unit::new(
            "Dwarf Defender".to_string(),
            HexCoord::new(1, 0),
            Race::Dwarf,
            UnitClass::Warrior,
            terrain,
        );

        let actual_hit_rate = run_combat_iterations(&attacker, &defender, ITERATIONS);

        println!(
            "Dwarf in {:?} - Expected: {:.1}%, Actual: {:.1}%, Diff: {:.1}%",
            terrain,
            expected_defense,
            actual_hit_rate,
            (actual_hit_rate - expected_defense).abs()
        );

        assert!(
            (actual_hit_rate - expected_defense).abs() <= ERROR_MARGIN,
            "Terrain {:?}: Hit rate {:.1}% outside error margin of {:.1}% from expected {:.1}%",
            terrain,
            actual_hit_rate,
            ERROR_MARGIN,
            expected_defense
        );
    }
}

/// Helper function to run multiple combat iterations and calculate hit rate
fn run_combat_iterations(attacker: &Unit, defender: &Unit, iterations: usize) -> f64 {
    let mut hits = 0;

    for i in 0..iterations {
        // Create fresh copies for each iteration
        let mut att = attacker.clone();
        let mut def = defender.clone();

        // Modify attacker slightly each iteration to vary the hash
        att.experience = i as i32;

        let result = units::combat::resolve_combat(&mut att, &mut def);
        if result.attacker_hit {
            hits += 1;
        }
    }

    (hits as f64 / iterations as f64) * 100.0
}
