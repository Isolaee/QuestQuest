use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct ElfWarrior {
    base: BaseUnit,
}

impl ElfWarrior {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            100,                                // health (lighter armor than humans)
            14,                                 // base attack
            4 + Race::Elf.get_movement_bonus(), // movement speed (elves are fast)
            RangeCategory::Melee,               // range category
            Resistances::new(
                // resistances (medium armor)
                20, // blunt
                25, // pierce (chainmail)
                15, // fire
                10, // dark
                25, // slash
                20, // crush
            ),
            14, // attack_strength
            1,  // attacks_per_round
        );

        let mut base = BaseUnit::new(
            name,
            position,
            Race::Elf,
            "Elf Warrior".to_string(),
            "An agile elf warrior with enhanced speed and grace. Elves are natural forest dwellers, gaining significant bonuses in woodland terrain. Swift and deadly with blade and shield.".to_string(),
            terrain,
            graphics::SpriteType::Unit,
            None,
            None,
            combat_stats,
        );

        base.attacks = vec![
            Attack::melee("Elven Blade", 14, 1, DamageType::Slash),
            Attack::melee("Shield Bash", 8, 1, DamageType::Blunt),
        ];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for ElfWarrior {
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
    ElfWarrior,
    "Elf Warrior",
    "An agile elf warrior",
    Terrain::Forest0,
    "Elf",
    "Warrior"
);
