use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use combat::DamageType;
use graphics::HexCoord;

pub struct DwarfWarrior {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl DwarfWarrior {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let base = BaseUnit::new(name, position, Race::Dwarf, UnitClass::Warrior, terrain);

        // Define default attacks for dwarf warrior
        let attacks = vec![
            Attack::melee("Axe Chop", 15, 1, DamageType::Slash),
            Attack::new(
                "Shield Bash",
                10,
                DamageType::Blunt,
                1,
                "A stunning blow with the shield",
            ),
        ];

        Self { base, attacks }
    }

    /// Add a new attack to this warrior's repertoire
    pub fn add_attack(&mut self, attack: Attack) {
        self.attacks.push(attack);
    }
}

crate::impl_unit_delegate!(DwarfWarrior);
