use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use graphics::HexCoord;

pub struct DwarfArcher {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl DwarfArcher {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let base = BaseUnit::new(name, position, Race::Dwarf, UnitClass::Archer, terrain);
        // TODO: Add specific archer attacks
        let attacks = Vec::new();
        Self { base, attacks }
    }
}

// Use the macro to implement all standard Unit trait methods
crate::impl_unit_delegate!(DwarfArcher);
