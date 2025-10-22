//! Factory for creating unit instances.
//!
//! This module provides [`UnitFactory`], which offers convenient factory methods
//! for creating different unit types. Each method returns a boxed trait object
//! implementing the [`Unit`] trait.

use crate::unit_race::Terrain;
use crate::unit_trait::Unit;
use crate::units::*;
use graphics::HexCoord;

/// Factory for creating unit instances.
///
/// `UnitFactory` provides static methods to create different types of units
/// without needing to know the concrete implementation types. All units are
/// returned as boxed trait objects (`Box<dyn Unit>`).
///
/// # Examples
///
/// ```rust,no_run
/// use units::UnitFactory;
/// use units::Terrain;
/// use graphics::HexCoord;
///
/// // Create a human warrior
/// let warrior = UnitFactory::create_human_warrior(
///     "Aragorn".to_string(),
///     HexCoord::new(0, 0),
///     Terrain::Grasslands,
/// );
///
/// // Create an elf archer
/// let archer = UnitFactory::create_elf_archer(
///     "Legolas".to_string(),
///     HexCoord::new(1, 0),
///     Terrain::Forest0,
/// );
/// ```
pub struct UnitFactory;

impl UnitFactory {
    /// Creates a human warrior unit.
    ///
    /// Warriors are melee combatants with high defense and slash damage.
    ///
    /// # Arguments
    ///
    /// * `name` - The unit's display name
    /// * `position` - Starting position on the hex grid
    /// * `terrain` - The terrain at the starting position
    ///
    /// # Returns
    ///
    /// A boxed `Unit` trait object representing a human warrior.
    pub fn create_human_warrior(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(HumanWarrior::new(name, position, terrain))
    }

    /// Creates a human archer unit.
    ///
    /// Archers are ranged combatants with pierce damage and moderate defense.
    pub fn create_human_archer(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(HumanArcher::new(name, position, terrain))
    }

    /// Creates a human mage unit.
    ///
    /// Mages are ranged spellcasters with fire damage and magical resistances.
    pub fn create_human_mage(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(HumanMage::new(name, position, terrain))
    }

    /// Creates an elf warrior unit.
    ///
    /// Elf warriors combine melee prowess with elven agility and forest bonuses.
    pub fn create_elf_warrior(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(ElfWarrior::new(name, position, terrain))
    }

    /// Creates an elf archer unit.
    ///
    /// Elf archers excel at ranged combat with superior accuracy and forest mobility.
    pub fn create_elf_archer(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(ElfArcher::new(name, position, terrain))
    }

    /// Creates an elf mage unit.
    ///
    /// Elf mages combine magical power with elven wisdom and nature affinity.
    pub fn create_elf_mage(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(ElfMage::new(name, position, terrain))
    }

    /// Creates a dwarf warrior unit.
    ///
    /// Dwarf warriors are heavily armored with exceptional mountain and hill bonuses.
    pub fn create_dwarf_warrior(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(DwarfWarrior::new(name, position, terrain))
    }

    /// Creates a dwarf archer unit.
    ///
    /// Dwarf archers use crossbows and have high durability despite their smaller range.
    pub fn create_dwarf_archer(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(DwarfArcher::new(name, position, terrain))
    }

    /// Creates a dwarf mage unit.
    ///
    /// Dwarf mages specialize in earth and fire magic with dwarven resilience.
    pub fn create_dwarf_mage(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(DwarfMage::new(name, position, terrain))
    }

    /// Creates a goblin grunt unit.
    ///
    /// Goblin grunts are weak but numerous enemies with swamp and forest advantages.
    pub fn create_goblin_grunt(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(GoblinGrunt::new(name, position, terrain))
    }

    /// Creates a goblin chief unit.
    ///
    /// Goblin chiefs are stronger goblin leaders with better stats and leadership abilities.
    pub fn create_goblin_chief(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(GoblinChief::new(name, position, terrain))
    }
}
