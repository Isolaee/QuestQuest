//! Graphics crate
//!
//! High-level rendering utilities and helper types for the QuestQuest game.
//!
//! This crate provides:
//! - Core types for hexagonal maps and coordinate transforms (`core`)
//! - Math primitives used by rendering and gameplay (`math`)
//! - Renderer and shader helpers (`rendering`)
//! - Simple UI widgets used by the demo app (`ui`)
//!
//! Typical usage:
//!
//! ```rust
//! use graphics::Renderer;
//! use graphics::core::HexGrid;
//!
//! // Create a grid and renderer in your application
//! let mut grid = HexGrid::new();
//! // Renderer setup is application-specific; see `rendering::Renderer` for details
//! ```
//!
//! The crate re-exports commonly-used types from submodules for convenience.
pub mod core;
pub mod math;
pub mod movement_animation;
pub mod rendering;
pub mod ui;

// Re-export commonly used types
pub use core::{
    Camera, HexCoord, HexGrid, Hexagon, HighlightType, SpriteType, WorldHexLookupTable,
};
pub use math::Vec2;
pub use movement_animation::{find_path, UnitAnimation};
pub use rendering::{
    setup_dynamic_hexagons, AttackOption, CombatConfirmation, GuideBuilder, GuideDisplay,
    GuideEntry, GuideLibrary, MenuAction, Renderer,
};
pub use ui::{
    AttackDisplayInfo, EncyclopediaCategory, EncyclopediaPanel, UiPanel, UnitDisplayInfo,
};
