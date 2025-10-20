use crate::unit_race::Terrain;
use crate::unit_trait::Unit;
use crate::units::*;
use graphics::HexCoord;

/// Factory for creating unit instances
/// Note: With the removal of UnitClass, units are now identified by their specific type names
pub struct UnitFactory;

impl UnitFactory {
    /// Create a human warrior
    pub fn create_human_warrior(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(HumanWarrior::new(name, position, terrain))
    }

    /// Create a human archer
    pub fn create_human_archer(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(HumanArcher::new(name, position, terrain))
    }

    /// Create a human mage
    pub fn create_human_mage(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(HumanMage::new(name, position, terrain))
    }

    /// Create an elf warrior
    pub fn create_elf_warrior(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(ElfWarrior::new(name, position, terrain))
    }

    /// Create an elf archer
    pub fn create_elf_archer(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(ElfArcher::new(name, position, terrain))
    }

    /// Create an elf mage
    pub fn create_elf_mage(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(ElfMage::new(name, position, terrain))
    }

    /// Create a dwarf warrior
    pub fn create_dwarf_warrior(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(DwarfWarrior::new(name, position, terrain))
    }

    /// Create a dwarf archer
    pub fn create_dwarf_archer(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(DwarfArcher::new(name, position, terrain))
    }

    /// Create a dwarf mage
    pub fn create_dwarf_mage(name: String, position: HexCoord, terrain: Terrain) -> Box<dyn Unit> {
        Box::new(DwarfMage::new(name, position, terrain))
    }

    /// Create a goblin grunt
    pub fn create_goblin_grunt(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(GoblinGrunt::new(name, position, terrain))
    }

    /// Create a goblin chief
    pub fn create_goblin_chief(
        name: String,
        position: HexCoord,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        Box::new(GoblinChief::new(name, position, terrain))
    }
}
