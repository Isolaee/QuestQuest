//! Items crate
//!
//! This crate contains item definitions, equipment handling, and item
//! properties used by units and the game systems. It provides a small API for
//! creating items, equipping/unequipping them, and inspecting their effects.
//!
//! Key types:
//! - [`Item`] and [`ItemId`]
//! - [`Equipment`]
//! - [`ItemProperties`], [`ItemAttack`], [`RangeType`]
//!
//! Example:
//!
//! ```rust
//! use items::{Item, item_definitions::create_iron_sword};
//!
//! let sword = create_iron_sword();
//! println!("Created item: {}", sword.name);
//! ```

mod equipment;
mod item;
pub mod item_definitions;
pub mod item_properties;

pub use equipment::Equipment;
pub use item::{Item, ItemId, ItemType};
pub use item_properties::{ConsumableEffect, DamageType, ItemAttack, ItemProperties, RangeType};
