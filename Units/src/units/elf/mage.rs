use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::combat::{CombatStats, DamageType, RangeCategory, Resistances};
use crate::unit_race::{Race, Terrain};
use graphics::HexCoord;

pub struct ElfMage {
    base: BaseUnit,
}

impl ElfMage {
    pub fn new(name: String, position: HexCoord) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            75,                   // health
            11,                   // base attack
            5,                    // movement speed
            RangeCategory::Range, // range category (magic attacks are like ranged)
            Resistances::new(
                // resistances (magical robes)
                5,  // blunt
                5,  // pierce
                25, // fire (strong fire resistance)
                25, // dark
                5,  // slash
                5,  // crush
            ),
            11, // attack_strength
            1,  // attacks_per_round
        );

        let mut base = BaseUnit::new(
            name,
            position,
            Race::Elf,
            "Elf Mage".to_string(),
            "An elf mage attuned to nature's magical energies. Ancient and wise, elf mages channel powerful arcane forces with finesse. Their connection to the natural world makes them particularly potent in forest environments.".to_string(),
            None,
            vec![],
            combat_stats,
        );

        base.attacks = vec![
            Attack::ranged("Arcane Bolt", 16, 1, DamageType::Fire, 3),
            Attack::ranged("Lightning Strike", 14, 1, DamageType::Fire, 3),
            Attack::ranged("Nature's Wrath", 12, 1, DamageType::Pierce, 2),
        ];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for ElfMage {
    fn base(&self) -> &BaseUnit {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseUnit {
        &mut self.base
    }

    fn attacks(&self) -> &[Attack] {
        &self.base.attacks
    }
}

crate::submit_unit!(
    ElfMage,
    "Elf Mage",
    "A mystical elf mage",
    Terrain::Forest0,
    "Elf",
    "Mage"
);
