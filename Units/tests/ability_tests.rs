//! Tests for the unit abilities system

use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;
use units::{ability::*, Attack, BaseUnit, Race, Unit};

// Test unit wrapper that implements Unit trait
struct TestUnit {
    base: BaseUnit,
}

impl TestUnit {
    fn new() -> Self {
        let stats = CombatStats::new(100, 10, 4, RangeCategory::Melee, Resistances::default());

        let base = BaseUnit::new(
            "Test Unit".to_string(),
            HexCoord::new(0, 0),
            Race::Human,
            "Test".to_string(),
            "Test unit".to_string(),
            None,
            vec![],
            stats,
        );

        Self { base }
    }
}

impl Unit for TestUnit {
    fn base(&self) -> &BaseUnit {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseUnit {
        &mut self.base
    }

    fn attacks(&self) -> &[Attack] {
        &self.base.attacks
    }
}

fn create_test_unit() -> BaseUnit {
    let stats = CombatStats::new(100, 10, 4, RangeCategory::Melee, Resistances::default());

    BaseUnit::new(
        "Test Unit".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        "Test".to_string(),
        "Test unit".to_string(),
        None,
        vec![],
        stats,
    )
}

#[test]
fn test_passive_ability_creation() {
    let passive = PassiveAbility::new(
        "Test Passive",
        "A test passive ability",
        PassiveTrigger::Always,
        PassiveEffect::AttackBonus(10),
    );

    assert_eq!(passive.name, "Test Passive");
    assert_eq!(passive.trigger, PassiveTrigger::Always);
}

#[test]
fn test_passive_stat_bonus() {
    let mut unit = create_test_unit();
    let base_attack = unit.combat_stats.get_total_attack() as i32;

    // Add passive attack bonus
    unit.add_ability(Ability::Passive(PassiveAbility::new(
        "Strength",
        "+10 attack",
        PassiveTrigger::Always,
        PassiveEffect::AttackBonus(10),
    )));

    unit.recalculate_stats();
    assert_eq!(unit.cached_attack, base_attack + 10);
}

#[test]
fn test_passive_percentage_bonus() {
    let mut unit = create_test_unit();
    let base_attack = unit.combat_stats.get_total_attack() as i32;

    // Add 50% attack bonus
    unit.add_ability(Ability::Passive(PassiveAbility::new(
        "Berserker",
        "+50% attack",
        PassiveTrigger::Always,
        PassiveEffect::AttackBonusPercent(50),
    )));

    unit.recalculate_stats();
    let expected = base_attack + (base_attack * 50 / 100);
    assert_eq!(unit.cached_attack, expected);
}

#[test]
fn test_multiple_passive_bonuses() {
    let mut unit = create_test_unit();
    let base_attack = unit.combat_stats.get_total_attack() as i32;

    // Add flat bonus
    unit.add_ability(Ability::Passive(PassiveAbility::new(
        "Strength",
        "+10 attack",
        PassiveTrigger::Always,
        PassiveEffect::AttackBonus(10),
    )));

    // Add percentage bonus (applied after flat)
    unit.add_ability(Ability::Passive(PassiveAbility::new(
        "Rage",
        "+20% attack",
        PassiveTrigger::Always,
        PassiveEffect::AttackBonusPercent(20),
    )));

    unit.recalculate_stats();
    let after_flat = base_attack + 10;
    let expected = after_flat + (after_flat * 20 / 100);
    assert_eq!(unit.cached_attack, expected);
}

#[test]
fn test_active_ability_creation() {
    let active = ActiveAbility::new(
        "Fireball",
        "A fire spell",
        3,
        5,
        TargetType::SingleEnemy,
        ActiveEffect::Damage {
            amount: 50,
            damage_type: DamageType::Fire,
        },
    );

    assert_eq!(active.name, "Fireball");
    assert_eq!(active.cooldown_max, 3);
    assert_eq!(active.range, 5);
    assert!(active.is_ready());
}

#[test]
fn test_active_ability_cooldown() {
    let mut active = ActiveAbility::new(
        "Test",
        "Test ability",
        3,
        0,
        TargetType::SelfOnly,
        ActiveEffect::Heal { amount: 50 },
    );

    assert!(active.is_ready());

    active.use_ability();
    assert!(!active.is_ready());
    assert_eq!(active.cooldown_current, 3);

    active.tick_cooldown();
    assert_eq!(active.cooldown_current, 2);

    active.tick_cooldown();
    assert_eq!(active.cooldown_current, 1);

    active.tick_cooldown();
    assert_eq!(active.cooldown_current, 0);
    assert!(active.is_ready());
}

#[test]
fn test_use_active_ability() {
    let mut unit = TestUnit::new();

    let heal = Ability::Active(ActiveAbility::new(
        "Heal",
        "Restore HP",
        2,
        0,
        TargetType::SelfOnly,
        ActiveEffect::Heal { amount: 50 },
    ));
    let heal_id = heal.id();

    unit.add_ability(heal);

    // Should be ready initially
    assert!(unit.is_ability_ready(heal_id));

    // Use ability
    assert!(unit.use_active_ability(heal_id).is_ok());

    // Should be on cooldown
    assert!(!unit.is_ability_ready(heal_id));
    assert_eq!(unit.ability_state().get_cooldown(heal_id), 2);
}

#[test]
fn test_ability_cooldown_ticking() {
    let mut unit = TestUnit::new();

    let ability = Ability::Active(ActiveAbility::new(
        "Test",
        "Test",
        3,
        0,
        TargetType::SelfOnly,
        ActiveEffect::Heal { amount: 10 },
    ));
    let ability_id = ability.id();

    unit.add_ability(ability);
    unit.use_active_ability(ability_id).unwrap();

    // Initial cooldown
    assert_eq!(unit.ability_state().get_cooldown(ability_id), 3);

    // Tick 1
    unit.tick_abilities();
    assert_eq!(unit.ability_state().get_cooldown(ability_id), 2);

    // Tick 2
    unit.tick_abilities();
    assert_eq!(unit.ability_state().get_cooldown(ability_id), 1);

    // Tick 3
    unit.tick_abilities();
    assert_eq!(unit.ability_state().get_cooldown(ability_id), 0);
    assert!(unit.is_ability_ready(ability_id));
}

#[test]
fn test_aura_ability_creation() {
    let aura = AuraAbility::new(
        "Leadership",
        "Buff allies",
        2,
        AuraTarget::Allies,
        AuraEffect::AttackBonus(5),
    );

    assert_eq!(aura.name, "Leadership");
    assert_eq!(aura.range, 2);
    assert_eq!(aura.target_type, AuraTarget::Allies);
}

#[test]
fn test_aura_range_check() {
    let aura = AuraAbility::new(
        "Test Aura",
        "Test",
        3,
        AuraTarget::Allies,
        AuraEffect::AttackBonus(5),
    );

    let source = HexCoord::new(0, 0);
    let near = HexCoord::new(1, 1);
    let far = HexCoord::new(5, 5);

    assert!(aura.is_in_range(source, near));
    assert!(!aura.is_in_range(source, far));
}

#[test]
fn test_add_remove_ability() {
    let mut unit = create_test_unit();

    let ability = Ability::Passive(PassiveAbility::new(
        "Test",
        "Test",
        PassiveTrigger::Always,
        PassiveEffect::AttackBonus(5),
    ));
    let ability_id = ability.id();

    // Add ability
    unit.add_ability(ability);
    assert_eq!(unit.abilities.len(), 1);
    assert!(unit.find_ability(ability_id).is_some());

    // Remove ability
    assert!(unit.remove_ability(ability_id));
    assert_eq!(unit.abilities.len(), 0);
    assert!(unit.find_ability(ability_id).is_none());
}

#[test]
fn test_filter_abilities_by_type() {
    let mut unit = TestUnit::new();

    // Add one of each type
    unit.add_ability(Ability::Passive(PassiveAbility::new(
        "Passive",
        "Test",
        PassiveTrigger::Always,
        PassiveEffect::AttackBonus(5),
    )));

    unit.add_ability(Ability::Active(ActiveAbility::new(
        "Active",
        "Test",
        2,
        0,
        TargetType::SelfOnly,
        ActiveEffect::Heal { amount: 10 },
    )));

    unit.add_ability(Ability::Aura(AuraAbility::new(
        "Aura",
        "Test",
        2,
        AuraTarget::Allies,
        AuraEffect::AttackBonus(5),
    )));

    // Filter by type
    assert_eq!(unit.get_passive_abilities().len(), 1);
    assert_eq!(unit.get_active_abilities().len(), 1);
    assert_eq!(unit.get_aura_abilities().len(), 1);
    assert_eq!(unit.abilities().len(), 3);
}

#[test]
fn test_ability_state_effects() {
    let mut state = AbilityState::new();
    let ability_id = Uuid::new_v4();

    let effect = ActiveEffectInstance::new(
        ActiveEffect::Buff {
            stat: BuffStat::Attack,
            amount: 20,
            duration: 3,
        },
        3,
        None,
    );

    // Add effect
    state.add_effect(ability_id, effect);
    assert!(state.active_effects.contains_key(&ability_id));

    // Tick effects
    state.tick();
    let remaining = state.active_effects.get(&ability_id).unwrap();
    assert_eq!(remaining.duration, 2);

    // Tick until expired
    state.tick();
    state.tick();
    assert!(!state.active_effects.contains_key(&ability_id));
}

#[test]
fn test_cannot_use_ability_on_cooldown() {
    let mut unit = TestUnit::new();

    let ability = Ability::Active(ActiveAbility::new(
        "Test",
        "Test",
        2,
        0,
        TargetType::SelfOnly,
        ActiveEffect::Heal { amount: 10 },
    ));
    let ability_id = ability.id();

    unit.add_ability(ability);

    // Use once - should succeed
    assert!(unit.use_active_ability(ability_id).is_ok());

    // Try to use again - should fail (on cooldown)
    assert!(unit.use_active_ability(ability_id).is_err());
}

#[test]
fn test_passive_health_bonus() {
    let mut unit = create_test_unit();
    let base_health = unit.combat_stats.max_health;

    unit.add_ability(Ability::Passive(PassiveAbility::new(
        "Vitality",
        "+50 HP",
        PassiveTrigger::Always,
        PassiveEffect::HealthBonus(50),
    )));

    unit.recalculate_stats();
    assert_eq!(unit.cached_max_health, base_health + 50);
}

#[test]
fn test_passive_movement_bonus() {
    let mut unit = create_test_unit();
    let base_movement = unit.combat_stats.movement_speed;

    unit.add_ability(Ability::Passive(PassiveAbility::new(
        "Speed",
        "+2 movement",
        PassiveTrigger::Always,
        PassiveEffect::MovementBonus(2),
    )));

    unit.recalculate_stats();
    assert_eq!(unit.cached_movement, base_movement + 2);
}

use uuid::Uuid;
