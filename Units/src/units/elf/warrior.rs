use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct ElfWarrior {
    base: BaseUnit,
    attacks: Vec<Attack>,
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

        let base = BaseUnit::new(
            name,
            position,
            Race::Elf,
            "Elf Warrior".to_string(),
            terrain,
            combat_stats,
        );

        let attacks = vec![
            Attack::melee("Elven Blade", 14, 1, DamageType::Slash),
            Attack::melee("Shield Bash", 8, 1, DamageType::Blunt),
        ];

        Self { base, attacks }
    }
}

crate::impl_unit_delegate!(ElfWarrior);
