use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use combat::DamageType;
use graphics::HexCoord;

pub struct HumanWarrior {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl HumanWarrior {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let base = BaseUnit::new(name, position, Race::Human, UnitClass::Warrior, terrain);

        // Define default attacks for human warrior
        let attacks = vec![
            Attack::melee("Sword Slash", 15, 1, DamageType::Slash),
            Attack::ranged("Shoddy Bow", 1, 2, DamageType::Pierce, 2),
        ];

        Self { base, attacks }
    }

    /// Add a new attack to this warrior's repertoire
    pub fn add_attack(&mut self, attack: Attack) {
        self.attacks.push(attack);
    }
}

// Use the macro to implement all standard Unit trait methods
crate::impl_unit_delegate!(HumanWarrior);
