//! Factory for creating unit instances.
//!
//! This module provides both the traditional factory methods for creating specific unit types
//! and a dynamic registry-based creation system using the inventory pattern.
//!
//! Units are automatically discovered at compile-time via the `inventory` crate and the
//! `submit_unit!` macro in each unit's implementation file.

use crate::unit_race::Terrain;
use crate::unit_registry::UnitRegistry;
use crate::unit_trait::Unit;
use crate::units::*;
use graphics::HexCoord;
use lazy_static::lazy_static;

lazy_static! {
    /// Global unit registry, populated automatically from inventory submissions
    static ref GLOBAL_REGISTRY: UnitRegistry = UnitRegistry::new();
}

/// Gets the global unit registry
fn get_registry() -> &'static UnitRegistry {
    &GLOBAL_REGISTRY
}

/// Factory for creating units
pub struct UnitFactory;

impl UnitFactory {
    /// Creates a unit dynamically by type name
    ///
    /// # Arguments
    /// * `type_name` - The unit type name (e.g., "Human Warrior", "Elf Archer")
    /// * `name` - Optional custom name for the unit (defaults to type name)
    /// * `position` - Optional hex coordinate (defaults to (0,0))
    /// * `terrain` - Optional terrain (defaults to unit's default terrain)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use units::UnitFactory;
    /// use graphics::HexCoord;
    /// use units::Terrain;
    ///
    /// // Create with all defaults
    /// let unit = UnitFactory::create("Human Warrior", None, None, None).unwrap();
    ///
    /// // Create with custom name
    /// let unit = UnitFactory::create("Elf Archer", Some("Legolas".to_string()), None, None).unwrap();
    ///
    /// // Create with custom position and terrain
    /// let unit = UnitFactory::create(
    ///     "Dwarf Warrior",
    ///     Some("Gimli".to_string()),
    ///     Some(HexCoord::new(5, 3)),
    ///     Some(Terrain::Mountain)
    /// ).unwrap();
    /// ```
    pub fn create(
        type_name: &str,
        name: Option<String>,
        position: Option<HexCoord>,
        terrain: Option<Terrain>,
    ) -> Result<Box<dyn Unit>, String> {
        let registry = get_registry();
        let info = registry
            .get(type_name)
            .ok_or_else(|| format!("Unknown unit type: '{}'", type_name))?;

        let unit_name = name.unwrap_or_else(|| type_name.to_string());
        let pos = position.unwrap_or_else(|| HexCoord::new(0, 0));
        let terr = terrain.unwrap_or(info.default_terrain);

        registry.create_unit(type_name, unit_name, pos, terr)
    }

    /// Lists all available unit types
    pub fn list_types() -> Vec<&'static str> {
        get_registry().get_all_types()
    }

    /// Gets all unit types for a specific race
    pub fn list_by_race(race: &str) -> Vec<&'static str> {
        get_registry()
            .get_by_race(race)
            .iter()
            .map(|info| info.type_name)
            .collect()
    }

    /// Gets all unit types for a specific class
    pub fn list_by_class(class: &str) -> Vec<&'static str> {
        get_registry()
            .get_by_class(class)
            .iter()
            .map(|info| info.type_name)
            .collect()
    }

    /// Checks if a unit type exists
    pub fn exists(type_name: &str) -> bool {
        get_registry().is_registered(type_name)
    }

    // ========== Legacy Individual Factory Methods ==========
    // These are kept for backward compatibility with existing code

    pub fn create_human_warrior(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(HumanWarrior::new(name, position, terrain))
    }

    pub fn create_human_archer(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(HumanArcher::new(name, position, terrain))
    }

    pub fn create_human_mage(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(HumanMage::new(name, position, terrain))
    }

    pub fn create_elf_warrior(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(ElfWarrior::new(name, position, terrain))
    }

    pub fn create_elf_archer(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(ElfArcher::new(name, position, terrain))
    }

    pub fn create_elf_mage(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(ElfMage::new(name, position, terrain))
    }

    pub fn create_dwarf_young_warrior(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(DwarfYoungWarrior::new(name, position, terrain))
    }

    pub fn create_dwarf_warrior(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(DwarfWarrior::new(name, position, terrain))
    }

    pub fn create_dwarf_veteran_warrior(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(DwarfVeteranWarrior::new(name, position, terrain))
    }

    pub fn create_goblin_grunt(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(GoblinGrunt::new(name, position, terrain))
    }

    pub fn create_goblin_chief(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(GoblinChief::new(name, position, terrain))
    }
}
