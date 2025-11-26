//! # Encyclopedia System
//!
//! A dynamic, runtime-generated encyclopedia system for QuestQuest that provides
//! comprehensive information about units, terrain, combat mechanics, and gameplay systems.
//!
//! ## Features
//!
//! - **Unit Encyclopedia**: Automatically discovers and documents all registered units
//! - **Terrain Guide**: Complete information about terrain types and their effects
//! - **Mechanics Reference**: Combat systems, experience, equipment, and more
//! - **Dynamic Generation**: All content generated at runtime from actual game data
//! - **Search & Filter**: Find entries by category, race, class, or keywords
//!
//! ## Example
//!
//! ```rust,no_run
//! use encyclopedia::Encyclopedia;
//!
//! let encyclopedia = Encyclopedia::new();
//!
//! // View all units
//! encyclopedia.display_unit_index();
//!
//! // Look up specific unit
//! if let Some(entry) = encyclopedia.get_unit_entry("Human Warrior") {
//!     entry.display();
//! }
//!
//! // View terrain guide
//! encyclopedia.display_terrain_guide();
//! ```

pub mod encyclopedia;
pub mod entries;
pub mod formatters;

pub use encyclopedia::Encyclopedia;
pub use entries::{EncyclopediaEntry, MechanicEntry, TerrainEntry, UnitEntry};
