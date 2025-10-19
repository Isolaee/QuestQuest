use crate::base_unit::BaseUnit;
use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use graphics::HexCoord;

pub struct ElfMage {
    base: BaseUnit,
}

impl ElfMage {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let base = BaseUnit::new(name, position, Race::Elf, UnitClass::Mage, terrain);
        Self { base }
    }
}

crate::impl_unit_delegate!(ElfMage);
