# Unit Abilities - Quick Reference

## Creating Abilities

### Passive Ability
```rust
let passive = Ability::Passive(PassiveAbility::new(
    "Name",
    "Description",
    PassiveTrigger::Always,
    PassiveEffect::AttackBonusPercent(20),
));
```

### Active Ability
```rust
let active = Ability::Active(ActiveAbility::new(
    "Name",
    "Description",
    3,                        // cooldown (turns)
    5,                        // range
    TargetType::SingleEnemy,
    ActiveEffect::Damage { amount: 50, damage_type: DamageType::Fire },
));
```

### Aura Ability
```rust
let aura = Ability::Aura(AuraAbility::new(
    "Name",
    "Description",
    2,                     // range
    AuraTarget::Allies,
    AuraEffect::AttackBonus(5),
));
```

## Unit Trait Methods

```rust
// Add/remove abilities
unit.add_ability(ability);
unit.remove_ability(ability_id);

// Use active abilities
if unit.is_ability_ready(ability_id) {
    unit.use_active_ability(ability_id)?;
}

// Tick cooldowns (call at turn start)
unit.tick_abilities();

// Query abilities
let passives = unit.get_passive_abilities();
let actives = unit.get_active_abilities();
let auras = unit.get_aura_abilities();
let all = unit.abilities();

// Check auras at position
let auras_here = unit.get_auras_at_position(target_pos);
```

## Common Passive Triggers

```rust
PassiveTrigger::Always                    // Stat bonuses
PassiveTrigger::OnDealDamage              // Lifesteal, bonus damage
PassiveTrigger::OnTakeDamage              // Thorns, shields
PassiveTrigger::OnHealthBelow(30)         // Low health enrage
PassiveTrigger::OnKill                    // Execute rewards
PassiveTrigger::OnTurnStart               // Regeneration
PassiveTrigger::OnTerrain(Terrain::Forest) // Terrain bonuses
```

## Common Active Effects

```rust
// Damage
ActiveEffect::Damage { amount: 50, damage_type: DamageType::Fire }

// Healing
ActiveEffect::Heal { amount: 50 }
ActiveEffect::HealPercent { percent: 30 }

// Buffs
ActiveEffect::Buff { 
    stat: BuffStat::Attack, 
    amount: 20, 
    duration: 3 
}

// Crowd Control
ActiveEffect::Stun { duration: 1 }
ActiveEffect::Root { duration: 2 }

// Movement
ActiveEffect::Teleport
ActiveEffect::Knockback { distance: 2 }
```

## Common Aura Effects

```rust
// Stat buffs
AuraEffect::AttackBonus(5)
AuraEffect::DefenseBonus(10)

// Percentage buffs
AuraEffect::AttackBonusPercent(15)
AuraEffect::DamageReduction { percent: 20 }

// Over time
AuraEffect::Regeneration(5)
AuraEffect::DamageOverTime { 
    damage: 3, 
    damage_type: DamageType::Dark 
}

// Debuffs
AuraEffect::Slow { amount: 1 }
AuraEffect::Fear { percent: 20 }
```

## Stat Integration

Passive abilities with `PassiveTrigger::Always` are automatically applied:

```rust
// Passives are applied automatically during stat recalculation
unit.recalculate_stats();

// Attack includes: base + level + equipment + passive bonuses
let total_attack = unit.combat_stats().get_total_attack();
```

## Example: Tank Unit

```rust
// Passive: Extra HP
unit.add_ability(Ability::Passive(PassiveAbility::new(
    "Fortitude", "+50 HP",
    PassiveTrigger::Always,
    PassiveEffect::HealthBonus(50),
)));

// Passive: Reflect damage
unit.add_ability(Ability::Passive(PassiveAbility::new(
    "Thorns", "Reflects 25% damage",
    PassiveTrigger::OnTakeDamage,
    PassiveEffect::ReflectDamage { percent: 25 },
)));

// Active: Taunt/Shield
unit.add_ability(Ability::Active(ActiveAbility::new(
    "Shield Wall", "Gain 100 shield",
    4, 0, TargetType::SelfOnly,
    ActiveEffect::Shield { amount: 100, duration: 2 },
)));

// Aura: Protect allies
unit.add_ability(Ability::Aura(AuraAbility::new(
    "Defender's Aura", "Allies take 15% less damage",
    2, AuraTarget::Allies,
    AuraEffect::DamageReduction { percent: 15 },
)));
```

## Example: Damage Dealer

```rust
// Passive: Critical chance
unit.add_ability(Ability::Passive(PassiveAbility::new(
    "Precision", "+15% crit chance",
    PassiveTrigger::Always,
    PassiveEffect::CriticalChance { percent: 15 },
)));

// Passive: Lifesteal
unit.add_ability(Ability::Passive(PassiveAbility::new(
    "Vampirism", "20% lifesteal",
    PassiveTrigger::OnDealDamage,
    PassiveEffect::Lifesteal { percent: 20 },
)));

// Active: Power strike
unit.add_ability(Ability::Active(ActiveAbility::new(
    "Crushing Blow", "Deal massive damage",
    3, 1, TargetType::SingleEnemy,
    ActiveEffect::Damage { amount: 80, damage_type: DamageType::Slash },
)));

// Aura: Attack boost
unit.add_ability(Ability::Aura(AuraAbility::new(
    "Battle Fury", "Allies gain +10% attack",
    2, AuraTarget::Allies,
    AuraEffect::AttackBonusPercent(10),
)));
```

## Example: Support/Healer

```rust
// Passive: Regen
unit.add_ability(Ability::Passive(PassiveAbility::new(
    "Natural Healing", "Heal 5 HP per turn",
    PassiveTrigger::OnTurnStart,
    PassiveEffect::Heal(5),
)));

// Active: Group heal
unit.add_ability(Ability::Active(ActiveAbility::new(
    "Mass Heal", "Heal all nearby allies",
    4, 3, TargetType::AllAllies,
    ActiveEffect::Heal { amount: 40 },
)));

// Active: Cleanse
unit.add_ability(Ability::Active(ActiveAbility::new(
    "Purify", "Remove debuffs from ally",
    3, 3, TargetType::SingleAlly,
    ActiveEffect::Cleanse,
)));

// Aura: Healing aura
unit.add_ability(Ability::Aura(AuraAbility::new(
    "Healing Presence", "Allies regenerate 5 HP/turn",
    3, AuraTarget::AlliesAndSelf,
    AuraEffect::Regeneration(5),
)));
```

## See Full Documentation

- **`Units/docs/ABILITIES.md`** - Complete reference
- **`Units/examples/abilities_demo.rs`** - Working example
- **`Units/tests/ability_tests.rs`** - Test examples
