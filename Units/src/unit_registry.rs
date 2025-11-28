//! Unit registry for dynamic unit creation using the inventory pattern.
//!
//! This module provides a registry system that automatically discovers all unit types
//! at compile time using procedural macros and the inventory crate.
//!
//! ## Usage
//!
//! Units are registered automatically using the `#[register_unit]` attribute:
//!
//! ```rust,ignore
//! #[register_unit(
//!     type_name = "Human Warrior",
//!     description = "A versatile human warrior",
//!     default_terrain = Terrain::Grasslands,
//!     race = "Human",
//!     class = "Warrior"
//! )]
//! ```

use crate::unit_race::Terrain;
use crate::unit_trait::Unit;
use graphics::HexCoord;
use std::collections::HashMap;

/// Type alias for a unit constructor function
pub type UnitConstructor = fn(String, HexCoord) -> Box<dyn Unit>;

/// Metadata about a unit type
#[derive(Clone)]
pub struct UnitTypeInfo {
    /// The unit type name (e.g., "Human Warrior")
    pub type_name: &'static str,
    /// Description of the unit
    pub description: &'static str,
    /// Suggested default terrain for this unit
    pub default_terrain: Terrain,
    /// Race of the unit
    pub race: &'static str,
    /// Unit class (e.g., "Warrior", "Archer", "Mage")
    pub class: &'static str,
    /// Constructor function
    pub constructor: UnitConstructor,
}

// Collect all registered unit types using inventory
inventory::collect!(UnitTypeInfo);

/// Global unit registry that builds itself from inventory submissions
pub struct UnitRegistry {
    units: HashMap<String, UnitTypeInfo>,
}

impl UnitRegistry {
    /// Creates a new registry and populates it from inventory submissions
    pub fn new() -> Self {
        let mut units = HashMap::new();

        // Collect all unit types submitted via inventory
        for unit_info in inventory::iter::<UnitTypeInfo> {
            units.insert(unit_info.type_name.to_string(), unit_info.clone());
        }

        Self { units }
    }

    /// Gets a unit type info by name
    pub fn get(&self, type_name: &str) -> Option<&UnitTypeInfo> {
        self.units.get(type_name)
    }

    /// Creates a unit by type name
    pub fn create_unit(
        &self,
        type_name: &str,
        name: String,
        position: HexCoord,
    ) -> Result<Box<dyn Unit>, String> {
        match self.units.get(type_name) {
            Some(info) => Ok((info.constructor)(name, position)),
            None => Err(format!("Unknown unit type: '{}'", type_name)),
        }
    }

    /// Returns all registered unit type names
    pub fn get_all_types(&self) -> Vec<&str> {
        self.units.keys().map(|s| s.as_str()).collect()
    }

    /// Checks if a unit type is registered
    pub fn is_registered(&self, type_name: &str) -> bool {
        self.units.contains_key(type_name)
    }

    /// Gets all unit types for a specific race
    pub fn get_by_race(&self, race: &str) -> Vec<&UnitTypeInfo> {
        self.units
            .values()
            .filter(|info| info.race.eq_ignore_ascii_case(race))
            .collect()
    }

    /// Gets all unit types for a specific class
    pub fn get_by_class(&self, class: &str) -> Vec<&UnitTypeInfo> {
        self.units
            .values()
            .filter(|info| info.class.eq_ignore_ascii_case(class))
            .collect()
    }
}

impl Default for UnitRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro to submit a unit type to the inventory
/// This is used internally - units should use this in their module files
#[macro_export]
macro_rules! submit_unit {
    ($unit_type:ty, $type_name:expr, $desc:expr, $terrain:expr, $race:expr, $class:expr) => {
        inventory::submit! {
            $crate::unit_registry::UnitTypeInfo {
                type_name: $type_name,
                description: $desc,
                default_terrain: $terrain,
                race: $race,
                class: $class,
                constructor: |name, pos| {
                    Box::new(<$unit_type>::new(name, pos))
                },
            }
        }
    };
}
