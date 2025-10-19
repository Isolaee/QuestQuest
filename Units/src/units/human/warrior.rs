use crate::base_unit::BaseUnit;
use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use graphics::HexCoord;

pub struct HumanWarrior {
    base: BaseUnit,
}

impl HumanWarrior {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let base = BaseUnit::new(name, position, Race::Human, UnitClass::Warrior, terrain);
        Self { base }
    }
}

// Use the macro to implement all standard Unit trait methods
crate::impl_unit_delegate!(HumanWarrior);
