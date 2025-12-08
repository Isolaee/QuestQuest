//! Structure units - concrete implementations of different structure types.
//!
//! This module contains all the specific structure implementations.

pub mod house;
pub mod stone_wall;

// Re-export for convenience
pub use house::House;
pub use stone_wall::StoneWall;
