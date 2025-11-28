//! Example demonstrating the unit abilities system
//!
//! This example shows how to create and use passive, active, and aura abilities.

use combat::DamageType;
use graphics::HexCoord;
use units::{ability::*, Race, Terrain, UnitFactory};

fn main() {
    println!("=== Unit Abilities System Demo ===\n");

    // Create a warrior unit
    let mut warrior = UnitFactory::create(
        "Dwarf Warrior",
        Some("Thorin".to_string()),
        Some(HexCoord::new(0, 0)),
        Some(Terrain::Mountain),
    )
    .expect("Failed to create warrior");

    println!("Created: {}", warrior.name());
    println!(
        "Base Attack: {}\n",
        warrior.combat_stats().get_total_attack()
    );

    // ===== Passive Abilities =====
    println!("--- Adding Passive Abilities ---");

    // 1. Always-active stat boost
    let berserker_strength = Ability::Passive(PassiveAbility::new(
        "Berserker Strength",
        "Increases attack by 15%",
        PassiveTrigger::Always,
        PassiveEffect::AttackBonusPercent(15),
    ));
    warrior.add_ability(berserker_strength);
    println!("Added: Berserker Strength (passive +15% attack)");

    // 2. Defensive passive that triggers on taking damage
    let iron_skin = Ability::Passive(PassiveAbility::new(
        "Iron Skin",
        "Increases defense by 10",
        PassiveTrigger::Always,
        PassiveEffect::DefenseBonus(10),
    ));
    warrior.add_ability(iron_skin);
    println!("Added: Iron Skin (passive +10 defense)");

    // 3. Thorns - reflects damage when attacked
    let thorns = Ability::Passive(PassiveAbility::new(
        "Thorns",
        "Reflects 20% of damage back to attacker",
        PassiveTrigger::OnTakeDamage,
        PassiveEffect::ReflectDamage { percent: 20 },
    ));
    warrior.add_ability(thorns);
    println!("Added: Thorns (reflects 20% damage when hit)");

    // 4. Lifesteal on dealing damage
    let vampiric_strike = Ability::Passive(PassiveAbility::new(
        "Vampiric Strike",
        "Heals for 15% of damage dealt",
        PassiveTrigger::OnDealDamage,
        PassiveEffect::Lifesteal { percent: 15 },
    ));
    warrior.add_ability(vampiric_strike);
    println!("Added: Vampiric Strike (15% lifesteal)\n");

    // Recalculate stats to apply passive bonuses
    warrior.recalculate_stats();
    println!(
        "Attack after passives: {}\n",
        warrior.combat_stats().get_total_attack()
    );

    // ===== Active Abilities =====
    println!("--- Adding Active Abilities ---");

    // 1. Healing ability
    let healing_surge = Ability::Active(ActiveAbility::new(
        "Healing Surge",
        "Restores 50 HP, 3 turn cooldown",
        3, // cooldown
        0, // range (self only)
        TargetType::SelfOnly,
        ActiveEffect::Heal { amount: 50 },
    ));
    let healing_id = healing_surge.id();
    warrior.add_ability(healing_surge);
    println!("Added: Healing Surge (heal 50 HP, 3 turn CD)");

    // 2. Damage ability
    let power_strike = Ability::Active(ActiveAbility::new(
        "Power Strike",
        "Deals 40 slash damage, 2 turn cooldown",
        2,
        1, // melee range
        TargetType::SingleEnemy,
        ActiveEffect::Damage {
            amount: 40,
            damage_type: DamageType::Slash,
        },
    ));
    let power_strike_id = power_strike.id();
    warrior.add_ability(power_strike);
    println!("Added: Power Strike (40 slash damage, 2 turn CD)");

    // 3. Buff ability
    let battle_cry = Ability::Active(ActiveAbility::new(
        "Battle Cry",
        "Increases attack by 20 for 3 turns, 5 turn cooldown",
        5,
        0,
        TargetType::SelfOnly,
        ActiveEffect::Buff {
            stat: BuffStat::Attack,
            amount: 20,
            duration: 3,
        },
    ));
    let battle_cry_id = battle_cry.id();
    warrior.add_ability(battle_cry);
    println!("Added: Battle Cry (buff +20 attack for 3 turns, 5 turn CD)");

    // 4. Stun ability
    let shield_bash = Ability::Active(ActiveAbility::new(
        "Shield Bash",
        "Stuns enemy for 1 turn, 4 turn cooldown",
        4,
        1,
        TargetType::SingleEnemy,
        ActiveEffect::Stun { duration: 1 },
    ));
    warrior.add_ability(shield_bash);
    println!("Added: Shield Bash (stun for 1 turn, 4 turn CD)\n");

    // ===== Aura Abilities =====
    println!("--- Adding Aura Abilities ---");

    // 1. Ally buff aura
    let inspiring_presence = Ability::Aura(AuraAbility::new(
        "Inspiring Presence",
        "Allies within 2 hexes gain +5 attack",
        2, // range
        AuraTarget::Allies,
        AuraEffect::AttackBonus(5),
    ));
    warrior.add_ability(inspiring_presence);
    println!("Added: Inspiring Presence (allies +5 attack in 2 hex range)");

    // 2. Enemy debuff aura
    let intimidating_aura = Ability::Aura(AuraAbility::new(
        "Intimidating Aura",
        "Enemies within 1 hex deal 15% less damage",
        1,
        AuraTarget::Enemies,
        AuraEffect::Fear { percent: 15 },
    ));
    warrior.add_ability(intimidating_aura);
    println!("Added: Intimidating Aura (enemies -15% damage in 1 hex range)");

    // 3. Regeneration aura
    let healing_aura = Ability::Aura(AuraAbility::new(
        "Healing Aura",
        "Allies within 2 hexes regenerate 5 HP per turn",
        2,
        AuraTarget::AlliesAndSelf,
        AuraEffect::Regeneration(5),
    ));
    warrior.add_ability(healing_aura);
    println!("Added: Healing Aura (regenerate 5 HP per turn in 2 hex range)\n");

    // ===== Demonstrating Ability Usage =====
    println!("--- Using Abilities ---");

    println!("Turn 1:");
    println!(
        "  Checking if Healing Surge is ready: {}",
        warrior.is_ability_ready(healing_id)
    );

    if let Ok(()) = warrior.use_active_ability(healing_id) {
        println!("  ✓ Used Healing Surge!");
    }

    println!(
        "  Checking if Healing Surge is ready: {}",
        warrior.is_ability_ready(healing_id)
    );

    // Try using Power Strike
    if let Ok(()) = warrior.use_active_ability(power_strike_id) {
        println!("  ✓ Used Power Strike!");
    }

    println!("\nTurn 2:");
    warrior.tick_abilities();
    println!("  Abilities ticked (cooldowns reduced by 1)");
    println!(
        "  Healing Surge cooldown: {} turns",
        warrior.ability_state().get_cooldown(healing_id)
    );
    println!(
        "  Power Strike cooldown: {} turns",
        warrior.ability_state().get_cooldown(power_strike_id)
    );

    println!("\nTurn 3:");
    warrior.tick_abilities();
    println!(
        "  Power Strike is ready: {}",
        warrior.is_ability_ready(power_strike_id)
    );

    if let Ok(()) = warrior.use_active_ability(battle_cry_id) {
        println!("  ✓ Used Battle Cry! Attack buffed for 3 turns.");
    }

    // ===== Summary =====
    println!("\n--- Ability Summary ---");
    println!("Total abilities: {}", warrior.abilities().len());
    println!("  Passive: {}", warrior.get_passive_abilities().len());
    println!("  Active: {}", warrior.get_active_abilities().len());
    println!("  Aura: {}", warrior.get_aura_abilities().len());

    println!("\nPassive Abilities:");
    for ability in warrior.get_passive_abilities() {
        println!("  - {} (trigger: {:?})", ability.name, ability.trigger);
    }

    println!("\nActive Abilities:");
    for ability in warrior.get_active_abilities() {
        let cooldown = warrior.ability_state().get_cooldown(ability.id);
        let status = if cooldown > 0 {
            format!("CD: {}", cooldown)
        } else {
            "Ready".to_string()
        };
        println!("  - {} ({})", ability.name, status);
    }

    println!("\nAura Abilities:");
    for ability in warrior.get_aura_abilities() {
        println!(
            "  - {} (range: {}, affects: {:?})",
            ability.name, ability.range, ability.target_type
        );
    }

    println!("\n=== Demo Complete ===");
}
