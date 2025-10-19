use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use graphics::HexCoord;

pub struct ElfWarrior {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl ElfWarrior {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let base = BaseUnit::new(name, position, Race::Elf, UnitClass::Warrior, terrain);
        let attacks = Vec::new();
        Self { base, attacks }
    }
}

crate::impl_unit_delegate!(ElfWarrior);
