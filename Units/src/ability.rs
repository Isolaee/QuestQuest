//! Unit abilities system.
//!
//! This module provides a flexible ability system for units, supporting:
//! - **Passive abilities**: Automatic effects that trigger under certain conditions
//! - **Active abilities**: Player-activated powers with cooldowns
//! - **Aura abilities**: Area effects that buff allies or debuff enemies
//!
//! # Examples
//!
//! ```rust
//! use units::ability::{Ability, PassiveAbility, PassiveTrigger, PassiveEffect};
//!
//! // Create a passive ability that triggers on taking damage
//! let thorns = Ability::Passive(PassiveAbility::new(
//!     "Thorns",
//!     "Reflects 20% of damage back to attacker",
//!     PassiveTrigger::OnTakeDamage,
//!     PassiveEffect::ReflectDamage { percent: 20 },
//! ));
//! ```

use graphics::HexCoord;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for abilities.
pub type AbilityId = Uuid;

/// Main ability enum encompassing all ability types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Ability {
    /// Automatic effects that trigger under certain conditions
    Passive(PassiveAbility),
    /// Player-activated powers with cooldowns
    Active(ActiveAbility),
    /// Area effects that buff allies or debuff enemies
    Aura(AuraAbility),
}

impl Ability {
    /// Get the ability's unique identifier
    pub fn id(&self) -> AbilityId {
        match self {
            Ability::Passive(p) => p.id,
            Ability::Active(a) => a.id,
            Ability::Aura(a) => a.id,
        }
    }

    /// Get the ability's name
    pub fn name(&self) -> &str {
        match self {
            Ability::Passive(p) => &p.name,
            Ability::Active(a) => &a.name,
            Ability::Aura(a) => &a.name,
        }
    }

    /// Get the ability's description
    pub fn description(&self) -> &str {
        match self {
            Ability::Passive(p) => &p.description,
            Ability::Active(a) => &a.description,
            Ability::Aura(a) => &a.description,
        }
    }
}

// ===== Passive Abilities =====

/// Passive ability that triggers automatically under certain conditions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PassiveAbility {
    /// Unique identifier
    pub id: AbilityId,
    /// Display name
    pub name: String,
    /// Description for tooltips
    pub description: String,
    /// When this ability triggers
    pub trigger: PassiveTrigger,
    /// What effect it has
    pub effect: PassiveEffect,
}

impl PassiveAbility {
    /// Create a new passive ability
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        trigger: PassiveTrigger,
        effect: PassiveEffect,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            trigger,
            effect,
        }
    }
}

/// Conditions that trigger passive abilities.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassiveTrigger {
    /// Always active (stat bonuses)
    Always,
    /// When the unit deals damage
    OnDealDamage,
    /// When the unit takes damage
    OnTakeDamage,
    /// When the unit kills an enemy
    OnKill,
    /// When the unit's health drops below a threshold (percentage)
    OnHealthBelow(u8),
    /// At the start of each turn
    OnTurnStart,
    /// At the end of each turn
    OnTurnEnd,
    /// When the unit moves
    OnMove,
    /// When the unit attacks
    OnAttack,
    /// When the unit is attacked
    OnBeingAttacked,
    /// When an ally within range dies
    OnAllyDeath { range: i32 },
    /// When standing on specific terrain
    OnTerrain(crate::unit_race::Terrain),
}

/// Effects that passive abilities can have.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PassiveEffect {
    /// Increase attack by flat amount
    AttackBonus(i32),
    /// Increase attack by percentage
    AttackBonusPercent(u8),
    /// Increase defense by flat amount
    DefenseBonus(i32),
    /// Increase defense by percentage
    DefenseBonusPercent(u8),
    /// Increase max health by flat amount
    HealthBonus(i32),
    /// Increase max health by percentage
    HealthBonusPercent(u8),
    /// Increase movement by flat amount
    MovementBonus(i32),
    /// Heal a flat amount
    Heal(i32),
    /// Heal a percentage of max health
    HealPercent(u8),
    /// Reflect damage back to attacker (percentage)
    ReflectDamage { percent: u8 },
    /// Gain temporary shield (absorbs damage)
    Shield { amount: i32, duration: u32 },
    /// Deal additional damage
    BonusDamage(u32),
    /// Lifesteal - heal for percentage of damage dealt
    Lifesteal { percent: u8 },
    /// Increase critical hit chance
    CriticalChance { percent: u8 },
    /// Dodge chance - chance to avoid attacks
    DodgeChance { percent: u8 },
    /// Cleanse negative effects
    Cleanse,
    /// Gain experience bonus
    ExperienceBonus { percent: u8 },
    /// Multi-strike - additional attacks
    MultiStrike { attacks: u8 },
    /// Damage over time to attacker
    Thorns { damage: u32 },
}

// ===== Active Abilities =====

/// Active ability that must be manually activated by the player.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveAbility {
    /// Unique identifier
    pub id: AbilityId,
    /// Display name
    pub name: String,
    /// Description for tooltips
    pub description: String,
    /// Maximum cooldown in turns
    pub cooldown_max: u32,
    /// Current cooldown remaining (0 = ready)
    pub cooldown_current: u32,
    /// Range of the ability (0 = self only)
    pub range: i32,
    /// Targeting type
    pub target_type: TargetType,
    /// Effect when activated
    pub effect: ActiveEffect,
}

impl ActiveAbility {
    /// Create a new active ability
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        cooldown: u32,
        range: i32,
        target_type: TargetType,
        effect: ActiveEffect,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            cooldown_max: cooldown,
            cooldown_current: 0,
            range,
            target_type,
            effect,
        }
    }

    /// Check if the ability is ready to use
    pub fn is_ready(&self) -> bool {
        self.cooldown_current == 0
    }

    /// Use the ability (sets cooldown)
    pub fn use_ability(&mut self) {
        self.cooldown_current = self.cooldown_max;
    }

    /// Reduce cooldown by one turn
    pub fn tick_cooldown(&mut self) {
        if self.cooldown_current > 0 {
            self.cooldown_current -= 1;
        }
    }

    /// Reset cooldown to ready
    pub fn reset_cooldown(&mut self) {
        self.cooldown_current = 0;
    }
}

/// How an ability targets units.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetType {
    /// Affects only the caster
    SelfOnly,
    /// Targets a single ally
    SingleAlly,
    /// Targets a single enemy
    SingleEnemy,
    /// Targets any single unit
    SingleUnit,
    /// Affects all allies in range
    AllAllies,
    /// Affects all enemies in range
    AllEnemies,
    /// Affects all units in range
    AllUnits,
    /// Targets a specific hex position
    Position,
    /// Targets an area around a position
    Area { radius: i32 },
}

/// Effects that active abilities can have.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActiveEffect {
    /// Deal direct damage
    Damage {
        amount: u32,
        damage_type: combat::DamageType,
    },
    /// Heal target
    Heal { amount: i32 },
    /// Heal percentage of max health
    HealPercent { percent: u8 },
    /// Apply a buff to target
    Buff {
        stat: BuffStat,
        amount: i32,
        duration: u32,
    },
    /// Apply a debuff to target
    Debuff {
        stat: BuffStat,
        amount: i32,
        duration: u32,
    },
    /// Teleport to target position
    Teleport,
    /// Stun target (cannot act)
    Stun { duration: u32 },
    /// Root target (cannot move)
    Root { duration: u32 },
    /// Silence target (cannot use abilities)
    Silence { duration: u32 },
    /// Grant temporary shield
    Shield { amount: i32, duration: u32 },
    /// Cleanse all negative effects
    Cleanse,
    /// Revive fallen ally
    Revive { health_percent: u8 },
    /// Deal damage over time
    DamageOverTime {
        damage_per_turn: u32,
        duration: u32,
        damage_type: combat::DamageType,
    },
    /// Heal over time
    HealOverTime { heal_per_turn: i32, duration: u32 },
    /// Knockback target away
    Knockback { distance: i32 },
    /// Pull target closer
    Pull { distance: i32 },
    /// Swap positions with target
    Swap,
    /// Create a barrier that blocks movement
    Barrier { duration: u32 },
    /// Transform into another unit temporarily
    Transform { unit_type: String, duration: u32 },
}

/// Stats that can be buffed or debuffed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuffStat {
    Attack,
    Defense,
    MaxHealth,
    Movement,
    AttackRange,
    CriticalChance,
    DodgeChance,
}

// ===== Aura Abilities =====

/// Aura ability that affects units in range automatically.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuraAbility {
    /// Unique identifier
    pub id: AbilityId,
    /// Display name
    pub name: String,
    /// Description for tooltips
    pub description: String,
    /// Range of the aura in hexes
    pub range: i32,
    /// Who the aura affects
    pub target_type: AuraTarget,
    /// Effect applied to targets
    pub effect: AuraEffect,
}

impl AuraAbility {
    /// Create a new aura ability
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        range: i32,
        target_type: AuraTarget,
        effect: AuraEffect,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            range,
            target_type,
            effect,
        }
    }

    /// Check if a position is within aura range
    pub fn is_in_range(&self, source: HexCoord, target: HexCoord) -> bool {
        source.distance(target) <= self.range
    }
}

/// Who an aura affects.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuraTarget {
    /// Only affects allies (not self)
    Allies,
    /// Only affects allies including self
    AlliesAndSelf,
    /// Only affects enemies
    Enemies,
    /// Affects all units
    All,
    /// Only affects self
    SelfOnly,
}

/// Effects that auras can have.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AuraEffect {
    /// Increase attack
    AttackBonus(i32),
    /// Increase attack by percentage
    AttackBonusPercent(u8),
    /// Increase defense
    DefenseBonus(i32),
    /// Increase defense by percentage
    DefenseBonusPercent(u8),
    /// Increase max health
    HealthBonus(i32),
    /// Increase max health by percentage
    HealthBonusPercent(u8),
    /// Increase movement
    MovementBonus(i32),
    /// Heal per turn
    Regeneration(i32),
    /// Damage per turn
    DamageOverTime {
        damage: u32,
        damage_type: combat::DamageType,
    },
    /// Reduce damage taken
    DamageReduction { percent: u8 },
    /// Increase damage dealt
    DamageAmplification { percent: u8 },
    /// Increase critical hit chance
    CriticalChance { percent: u8 },
    /// Increase dodge chance
    DodgeChance { percent: u8 },
    /// Lifesteal
    Lifesteal { percent: u8 },
    /// Experience bonus
    ExperienceBonus { percent: u8 },
    /// Slow movement
    Slow { amount: i32 },
    /// Cannot act
    Stun,
    /// Cannot move
    Root,
    /// Cannot use abilities
    Silence,
    /// Fear - reduced damage dealt
    Fear { percent: u8 },
}

// ===== Ability Instance Tracker =====

/// Tracks active effects and cooldowns for a unit.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AbilityState {
    /// Active buffs/debuffs with their remaining duration
    pub active_effects: HashMap<AbilityId, ActiveEffectInstance>,
    /// Ability cooldowns
    pub cooldowns: HashMap<AbilityId, u32>,
}

impl AbilityState {
    /// Create a new empty ability state
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an active effect
    pub fn add_effect(&mut self, ability_id: AbilityId, effect: ActiveEffectInstance) {
        self.active_effects.insert(ability_id, effect);
    }

    /// Remove an active effect
    pub fn remove_effect(&mut self, ability_id: AbilityId) {
        self.active_effects.remove(&ability_id);
    }

    /// Tick down all effect durations and cooldowns
    pub fn tick(&mut self) {
        // Reduce effect durations
        let mut expired = Vec::new();
        for (id, effect) in self.active_effects.iter_mut() {
            if effect.duration > 0 {
                effect.duration -= 1;
            }
            if effect.duration == 0 {
                expired.push(*id);
            }
        }

        // Remove expired effects
        for id in expired {
            self.active_effects.remove(&id);
        }

        // Reduce cooldowns
        for cooldown in self.cooldowns.values_mut() {
            if *cooldown > 0 {
                *cooldown -= 1;
            }
        }
    }

    /// Check if an ability is on cooldown
    pub fn is_on_cooldown(&self, ability_id: AbilityId) -> bool {
        self.cooldowns.get(&ability_id).copied().unwrap_or(0) > 0
    }

    /// Get remaining cooldown for an ability
    pub fn get_cooldown(&self, ability_id: AbilityId) -> u32 {
        self.cooldowns.get(&ability_id).copied().unwrap_or(0)
    }

    /// Set ability cooldown
    pub fn set_cooldown(&mut self, ability_id: AbilityId, cooldown: u32) {
        self.cooldowns.insert(ability_id, cooldown);
    }
}

/// An active effect instance with remaining duration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveEffectInstance {
    /// The effect being applied
    pub effect: ActiveEffect,
    /// Remaining duration in turns (0 = expired)
    pub duration: u32,
    /// Source unit ID (for tracking who applied it)
    pub source_id: Option<crate::unit_trait::UnitId>,
}

impl ActiveEffectInstance {
    /// Create a new effect instance
    pub fn new(
        effect: ActiveEffect,
        duration: u32,
        source_id: Option<crate::unit_trait::UnitId>,
    ) -> Self {
        Self {
            effect,
            duration,
            source_id,
        }
    }
}
