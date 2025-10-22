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
//!
//! ## Examples
//!
//! ```ignore
//! use combat::{CombatStats, DamageType, RangeCategory, Resistances, resolve_combat};
//!
//! // Create combatants
//! let mut attacker = CombatStats::new(
//!     100,  // max health
//!     15,   // base attack
//!     5,    // movement speed
//!     RangeCategory::Melee,
//!     Resistances::default()
//! );
//!
//! let mut defender = CombatStats::new(
//!     80,
//!     12,
//!     4,
//!     RangeCategory::Melee,
//!     Resistances::default()
//! );
//!
//! // Resolve combat
//! let result = resolve_combat(&mut attacker, &mut defender, DamageType::Slash);
//! println!("Attacker dealt {} damage", result.attacker_damage_dealt);
//! ```

mod combat_action;
mod combat_resolver;
mod combat_result;
mod combat_stats;

pub use combat_action::CombatAction;
pub use combat_resolver::resolve_combat;
pub use combat_result::CombatResult;
pub use combat_stats::{CombatStats, DamageType, RangeCategory, Resistances};
