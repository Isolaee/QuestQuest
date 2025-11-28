# Unit Abilities System

A comprehensive ability system for units in QuestQuest, supporting **Passive**, **Active**, and **Aura** abilities.

## Overview

The ability system provides three types of abilities:

1. **Passive Abilities** - Automatic effects triggered by conditions
2. **Active Abilities** - Player-activated powers with cooldowns
3. **Aura Abilities** - Area effects that buff allies or debuff enemies

## Quick Start

```rust
use units::{Unit, UnitFactory, ability::*};
use combat::DamageType;

// Create a unit
let mut warrior = UnitFactory::create("Dwarf Warrior", None, None, None)?;

// Add a passive ability
let berserker = Ability::Passive(PassiveAbility::new(
    "Berserker Rage",
    "Increases attack by 20%",
    PassiveTrigger::Always,
    PassiveEffect::AttackBonusPercent(20),
));
warrior.add_ability(berserker);

// Add an active ability
let heal = Ability::Active(ActiveAbility::new(
    "Healing Surge",
    "Restores 50 HP",
    3, // cooldown in turns
    0, // range (self)
    TargetType::SelfOnly,
    ActiveEffect::Heal { amount: 50 },
));
let heal_id = heal.id();
warrior.add_ability(heal);

// Use the active ability
if warrior.is_ability_ready(heal_id) {
    warrior.use_active_ability(heal_id)?;
}

// Add an aura ability
let inspiring = Ability::Aura(AuraAbility::new(
    "Inspiring Presence",
    "Allies within 2 hexes gain +5 attack",
    2, // range
    AuraTarget::Allies,
    AuraEffect::AttackBonus(5),
));
warrior.add_ability(inspiring);
```

## Passive Abilities

Passive abilities trigger automatically under certain conditions.

### Passive Triggers

- `Always` - Permanent stat bonuses (applied during stat calculation)
- `OnDealDamage` - When the unit deals damage
- `OnTakeDamage` - When the unit takes damage
- `OnKill` - When the unit kills an enemy
- `OnHealthBelow(percent)` - When health drops below threshold
- `OnTurnStart` - At the start of each turn
- `OnTurnEnd` - At the end of each turn
- `OnMove` - When the unit moves
- `OnAttack` - When the unit attacks
- `OnBeingAttacked` - When the unit is attacked
- `OnAllyDeath { range }` - When an ally dies within range
- `OnTerrain(terrain)` - When standing on specific terrain

### Passive Effects

```rust
// Stat bonuses (flat and percentage)
PassiveEffect::AttackBonus(10)
PassiveEffect::AttackBonusPercent(15)
PassiveEffect::DefenseBonus(5)
PassiveEffect::DefenseBonusPercent(20)
PassiveEffect::HealthBonus(50)
PassiveEffect::HealthBonusPercent(10)
PassiveEffect::MovementBonus(2)

// Healing
PassiveEffect::Heal(25)
PassiveEffect::HealPercent(10)

// Combat mechanics
PassiveEffect::ReflectDamage { percent: 20 }
PassiveEffect::Shield { amount: 50, duration: 3 }
PassiveEffect::BonusDamage(15)
PassiveEffect::Lifesteal { percent: 15 }
PassiveEffect::CriticalChance { percent: 10 }
PassiveEffect::DodgeChance { percent: 15 }
PassiveEffect::MultiStrike { attacks: 2 }
PassiveEffect::Thorns { damage: 10 }

// Utility
PassiveEffect::Cleanse
PassiveEffect::ExperienceBonus { percent: 20 }
```

### Example: Conditional Passive

```rust
// Enrage - increases attack when health is low
let enrage = Ability::Passive(PassiveAbility::new(
    "Enrage",
    "When below 30% health, gain +50% attack",
    PassiveTrigger::OnHealthBelow(30),
    PassiveEffect::AttackBonusPercent(50),
));

// Thorns - reflects damage when hit
let thorns = Ability::Passive(PassiveAbility::new(
    "Thorns",
    "Reflects 25% of damage back to attacker",
    PassiveTrigger::OnTakeDamage,
    PassiveEffect::ReflectDamage { percent: 25 },
));

// Vampiric Strike - heal on dealing damage
let vampiric = Ability::Passive(PassiveAbility::new(
    "Vampiric Strike",
    "Heals for 20% of damage dealt",
    PassiveTrigger::OnDealDamage,
    PassiveEffect::Lifesteal { percent: 20 },
));
```

## Active Abilities

Active abilities must be manually activated and have cooldowns.

### Target Types

```rust
TargetType::SelfOnly          // Only affects the caster
TargetType::SingleAlly        // Targets one ally
TargetType::SingleEnemy       // Targets one enemy
TargetType::SingleUnit        // Targets any unit
TargetType::AllAllies         // All allies in range
TargetType::AllEnemies        // All enemies in range
TargetType::AllUnits          // All units in range
TargetType::Position          // Specific hex position
TargetType::Area { radius }   // Area around position
```

### Active Effects

```rust
// Damage
ActiveEffect::Damage { amount: 50, damage_type: DamageType::Fire }
ActiveEffect::DamageOverTime { 
    damage_per_turn: 10, 
    duration: 3, 
    damage_type: DamageType::Dark 
}

// Healing
ActiveEffect::Heal { amount: 50 }
ActiveEffect::HealPercent { percent: 30 }
ActiveEffect::HealOverTime { heal_per_turn: 10, duration: 3 }

// Buffs and Debuffs
ActiveEffect::Buff { stat: BuffStat::Attack, amount: 20, duration: 3 }
ActiveEffect::Debuff { stat: BuffStat::Defense, amount: 15, duration: 2 }

// Crowd Control
ActiveEffect::Stun { duration: 1 }
ActiveEffect::Root { duration: 2 }
ActiveEffect::Silence { duration: 2 }

// Movement
ActiveEffect::Teleport
ActiveEffect::Knockback { distance: 2 }
ActiveEffect::Pull { distance: 1 }
ActiveEffect::Swap

// Utility
ActiveEffect::Shield { amount: 100, duration: 2 }
ActiveEffect::Cleanse
ActiveEffect::Revive { health_percent: 50 }
ActiveEffect::Barrier { duration: 3 }
ActiveEffect::Transform { unit_type: "Berserker".to_string(), duration: 5 }
```

### Example: Combat Abilities

```rust
// Fireball - damage ability
let fireball = Ability::Active(ActiveAbility::new(
    "Fireball",
    "Deals 60 fire damage to an enemy",
    2, // 2 turn cooldown
    5, // 5 hex range
    TargetType::SingleEnemy,
    ActiveEffect::Damage {
        amount: 60,
        damage_type: DamageType::Fire,
    },
));

// Battle Cry - self buff
let battle_cry = Ability::Active(ActiveAbility::new(
    "Battle Cry",
    "Increases attack by 30 for 3 turns",
    5, // 5 turn cooldown
    0, // self only
    TargetType::SelfOnly,
    ActiveEffect::Buff {
        stat: BuffStat::Attack,
        amount: 30,
        duration: 3,
    },
));

// Charge - teleport to enemy
let charge = Ability::Active(ActiveAbility::new(
    "Charge",
    "Teleport to target enemy",
    4,
    3, // 3 hex range
    TargetType::SingleEnemy,
    ActiveEffect::Teleport,
));
```

### Using Active Abilities

```rust
// Check if ability is ready
if unit.is_ability_ready(ability_id) {
    // Use the ability
    unit.use_active_ability(ability_id)?;
    println!("Ability used!");
}

// Tick cooldowns at turn start
unit.tick_abilities();

// Get cooldown remaining
let cooldown = unit.ability_state().get_cooldown(ability_id);
println!("Ready in {} turns", cooldown);
```

## Aura Abilities

Aura abilities provide automatic area effects.

### Aura Targets

```rust
AuraTarget::Allies         // Only allies (not self)
AuraTarget::AlliesAndSelf  // Allies including self
AuraTarget::Enemies        // Only enemies
AuraTarget::All            // All units
AuraTarget::SelfOnly       // Only self
```

### Aura Effects

```rust
// Stat bonuses
AuraEffect::AttackBonus(5)
AuraEffect::AttackBonusPercent(10)
AuraEffect::DefenseBonus(10)
AuraEffect::DefenseBonusPercent(15)
AuraEffect::HealthBonus(20)
AuraEffect::HealthBonusPercent(10)
AuraEffect::MovementBonus(1)

// Combat modifiers
AuraEffect::DamageReduction { percent: 20 }
AuraEffect::DamageAmplification { percent: 15 }
AuraEffect::CriticalChance { percent: 10 }
AuraEffect::DodgeChance { percent: 10 }
AuraEffect::Lifesteal { percent: 10 }

// Over time effects
AuraEffect::Regeneration(5)
AuraEffect::DamageOverTime { 
    damage: 3, 
    damage_type: DamageType::Dark 
}

// Crowd control
AuraEffect::Slow { amount: 1 }
AuraEffect::Stun
AuraEffect::Root
AuraEffect::Silence
AuraEffect::Fear { percent: 20 }

// Utility
AuraEffect::ExperienceBonus { percent: 25 }
```

### Example: Support Auras

```rust
// Leadership - buff allies
let leadership = Ability::Aura(AuraAbility::new(
    "Leadership",
    "Allies within 2 hexes gain +10% attack",
    2,
    AuraTarget::Allies,
    AuraEffect::AttackBonusPercent(10),
));

// Intimidation - debuff enemies
let intimidation = Ability::Aura(AuraAbility::new(
    "Intimidating Presence",
    "Enemies within 1 hex deal 20% less damage",
    1,
    AuraTarget::Enemies,
    AuraEffect::Fear { percent: 20 },
));

// Healing aura - regeneration
let healing_aura = Ability::Aura(AuraAbility::new(
    "Healing Aura",
    "Allies within 3 hexes regenerate 5 HP per turn",
    3,
    AuraTarget::AlliesAndSelf,
    AuraEffect::Regeneration(5),
));

// Damage aura - enemies take damage
let poison_aura = Ability::Aura(AuraAbility::new(
    "Poison Aura",
    "Enemies within 2 hexes take 5 poison damage per turn",
    2,
    AuraTarget::Enemies,
    AuraEffect::DamageOverTime {
        damage: 5,
        damage_type: DamageType::Dark,
    },
));
```

### Checking Aura Range

```rust
// Get all auras affecting a position
let auras = unit.get_auras_at_position(target_position);
for aura in auras {
    println!("Affected by: {}", aura.name);
}
```

## Ability Management

### Adding/Removing Abilities

```rust
// Add ability
unit.add_ability(ability);

// Remove ability by ID
unit.remove_ability(ability_id);

// Find ability
if let Some(ability) = unit.find_ability(ability_id) {
    println!("Found: {}", ability.name());
}
```

### Filtering Abilities

```rust
// Get all passive abilities
let passives = unit.get_passive_abilities();

// Get all active abilities
let actives = unit.get_active_abilities();

// Get all aura abilities
let auras = unit.get_aura_abilities();

// Get all abilities
let all = unit.abilities();
```

### Ability State Management

```rust
// Tick abilities (reduce cooldowns and durations)
unit.tick_abilities();

// Check cooldown
if unit.ability_state().is_on_cooldown(ability_id) {
    let remaining = unit.ability_state().get_cooldown(ability_id);
    println!("On cooldown: {} turns", remaining);
}

// Add active effect
unit.ability_state_mut().add_effect(
    ability_id,
    ActiveEffectInstance::new(effect, duration, source_id)
);

// Remove effect
unit.ability_state_mut().remove_effect(ability_id);
```

## Integration with Stat System

Passive abilities with `PassiveTrigger::Always` are automatically applied during `recalculate_stats()`:

```rust
// This automatically includes passive bonuses
unit.recalculate_stats();

// Attack now includes passive bonuses
let total_attack = unit.combat_stats().get_total_attack();
```

## Complete Example: Custom Unit

```rust
use units::{BaseUnit, Unit, ability::*};
use combat::{CombatStats, RangeCategory, Resistances, DamageType};
use graphics::{HexCoord, SpriteType};

// Create custom unit with abilities
fn create_berserker() -> Box<dyn Unit> {
    let stats = CombatStats::new(
        150, // health
        25,  // attack
        4,   // movement
        RangeCategory::Melee,
        Resistances::new(15, 10, 20, 5, 15, 10),
    );

    let mut unit = BaseUnit::new(
        "Berserker".to_string(),
        HexCoord::new(0, 0),
        Race::Human,
        "Berserker".to_string(),
        "A fierce warrior".to_string(),
        Terrain::Grasslands,
        SpriteType::Unit,
        None,
        vec![],
        stats,
    );

    // Passive: Always attack bonus
    unit.add_ability(Ability::Passive(PassiveAbility::new(
        "Savage Strength",
        "+20% attack",
        PassiveTrigger::Always,
        PassiveEffect::AttackBonusPercent(20),
    )));

    // Passive: Enrage when low health
    unit.add_ability(Ability::Passive(PassiveAbility::new(
        "Enrage",
        "+50% attack when below 30% health",
        PassiveTrigger::OnHealthBelow(30),
        PassiveEffect::AttackBonusPercent(50),
    )));

    // Active: Whirlwind attack
    unit.add_ability(Ability::Active(ActiveAbility::new(
        "Whirlwind",
        "Damages all nearby enemies",
        3,
        2,
        TargetType::AllEnemies,
        ActiveEffect::Damage {
            amount: 40,
            damage_type: DamageType::Slash,
        },
    )));

    // Aura: Intimidate enemies
    unit.add_ability(Ability::Aura(AuraAbility::new(
        "Battle Fury",
        "Enemies nearby deal 15% less damage",
        2,
        AuraTarget::Enemies,
        AuraEffect::Fear { percent: 15 },
    )));

    unit.recalculate_stats();
    Box::new(unit)
}
```

## See Also

- [abilities_demo.rs](../examples/abilities_demo.rs) - Complete working example
- [Unit trait documentation](../src/unit_trait.rs) - Full API reference
- [Ability module](../src/ability.rs) - All ability types and effects
