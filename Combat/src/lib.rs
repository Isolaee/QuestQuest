//! # Combat Crate
//!
//! The `combat` crate provides a comprehensive turn-based combat system for QuestQuest.
//! It handles combat statistics, damage calculations, resistance modifiers, and combat
//! resolution with detailed turn-by-turn mechanics.
//!
//! ## Features
//!
//! - **Combat Statistics**: Comprehensive unit stats including health, attack, and resistances
//! - **Damage Types**: Multiple damage types (Slash, Pierce, Blunt, Crush, Fire, Dark) with resistance system
//! - **Range Categories**: Melee, Range, and Siege combat with counter-attack mechanics
//! - **Combat Resolution**: Turn-based alternating attacks with hit chance rolls
//! - **Multi-Attack System**: Support for units with multiple attacks per round
//!
//! ## Combat Flow
//!
//! 1. Attacker and defender stats are prepared
//! 2. Combat is resolved using `resolve_combat()`
//! 3. Attacks alternate between combatants based on `attacks_per_round`
//! 4. Hit chance is rolled for each attack
//! 5. Damage is calculated with resistance modifiers
//! 6. Combat continues until all attacks are exhausted or a unit is defeated

mod combat_resolver;

pub use combat_resolver::resolve_combat;

// Re-export core combat types from the `units` crate to avoid duplicating
// type definitions. The `units` crate owns the `combat` module which contains
// `CombatStats`, `DamageType`, `RangeCategory`, `Resistances`, `CombatResult`,
// and `CombatAction`.
pub use units::combat::{
    CombatAction, CombatResult, CombatStats, DamageType, RangeCategory, Resistances,
};
