//! # Game Crate
//!
//! The `game` crate provides the core game world management and object system for QuestQuest.
//! It acts as the central integration point between graphics, units, items, and combat systems.
//!
//! ## Features
//!
//! - **Game Objects**: Base trait system for all interactive game entities
//! - **Game World**: Manages terrain, units, and interactive objects in a hex-based grid
//! - **Combat System**: Handles combat initiation, confirmation, and resolution
//! - **Team Management**: Tracks unit affiliations (Player, Enemy, Neutral)
//!
//! ## Module Organization
//!
//! - [`objects`]: Defines the `GameObject` trait and implementations for terrain, units, and interactive objects
//! - [`world`]: Provides the `GameWorld` structure for managing all game entities and interactions
//! - [`turn_system`]: Manages turn-based gameplay mechanics
//!
//! ## Examples
//!
//! ```no_run
//! use game::{GameWorld, GameUnit, Team};
//! use graphics::HexCoord;
//!
//! // Create a new game world
//! let mut world = GameWorld::new(10);
//! world.generate_terrain();
//!
//! // Add units to the world
//! // let unit = GameUnit::new(Box::new(some_unit));
//! // world.add_unit(unit);
//! ```

pub mod objects;
pub mod turn_system;
pub mod world;

pub use objects::*;
pub use turn_system::*;
pub use world::*;

// Re-export commonly used types from dependencies
pub use graphics::{HexCoord, SpriteType, Vec2};
pub use units::{Item, Race, Unit, UnitClass};
