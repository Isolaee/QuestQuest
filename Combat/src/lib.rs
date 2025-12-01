//! # Combat Crate - Combat Resolution Logic
//!
//! The `combat` crate is responsible for all combat resolution logic in QuestQuest.
//! It contains no game state - instead, it provides pure functions that calculate
//! combat outcomes based on input stats.
//!
//! ## Architecture Role
//!
//! - **Pure Logic**: Contains only combat resolution algorithms, no state
//! - **Consumed by**: `Game/ScenarioWorld` which calls `resolve_combat()`
//! - **Not Responsible For**: Unit state, turn management, AI decisions, UI display
//!
//! ## Separation of Concerns
//!
//! - `Combat` crate: "How is combat resolved?" (pure logic)
//! - `Game/ScenarioWorld`: "When does combat happen?" (game state)
//! - `AI` crate: "Should I attack?" (decision making)
//! - `QuestApp`: "Show combat confirmation" (presentation)
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
//! 1. Attacker and defender stats are prepared by caller (ScenarioWorld)
//! 2. Combat is resolved using `resolve_combat()` (pure function)
//! 3. Attacks alternate between combatants based on `attacks_per_round`
//! 4. Hit chance is rolled for each attack
//! 5. Damage is calculated with resistance modifiers
//! 6. Combat continues until all attacks are exhausted or a unit is defeated
//! 7. Results returned to caller for state updates

mod combat_resolver;

pub use combat_resolver::resolve_combat;

// Re-export core combat types from the `units` crate to avoid duplicating
// type definitions. The `units` crate owns the `combat` module which contains
// `CombatStats`, `DamageType`, `RangeCategory`, `Resistances`, `CombatResult`,
// and `CombatAction`.
pub use units::combat::{
    CombatAction, CombatResult, CombatStats, DamageType, RangeCategory, Resistances,
};
