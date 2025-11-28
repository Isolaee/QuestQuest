use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct ElfArcher {
    base: BaseUnit,
}

impl ElfArcher {
    pub fn new(name: String, position: HexCoord) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            85,                                 // health
            13,                                 // base attack (elves are skilled archers)
            6 + Race::Elf.get_movement_bonus(), // movement speed (very fast)
            RangeCategory::Range,               // range category
            Resistances::new(
                // resistances (light armor)
                10, // blunt
                15, // pierce
                10, // fire
                5,  // dark
                20, // slash
                10, // crush
            ),
            13, // attack_strength
            1,  // attacks_per_round
        );

        let mut base = BaseUnit::new(
            name,
            position,
            Race::Elf,
            "Elf Archer".to_string(),
            "A legendary elf archer with unmatched precision. Elven archers are the finest marksmen in the realm, capable of picking off enemies from great distances while remaining nearly invisible in forests.".to_string(),
            None,
            vec![],
            combat_stats,
        );

        base.attacks = vec![
            Attack::ranged("Elven Longbow", 13, 1, DamageType::Pierce, 4),
            Attack::ranged("Quick Shot", 8, 2, DamageType::Pierce, 3),
            Attack::melee("Dagger", 5, 1, DamageType::Slash),
        ];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for ElfArcher {
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
    ElfArcher,
    "Elf Archer",
    "A precise elf archer",
    Terrain::Forest0,
    "Elf",
    "Archer"
);
