use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct ElfMage {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl ElfMage {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            75,                                 // health
            11,                                 // base attack
            5 + Race::Elf.get_movement_bonus(), // movement speed
            RangeCategory::Range,               // range category (magic attacks are like ranged)
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

        let base = BaseUnit::new(
            name,
            position,
            Race::Elf,
            "Elf Mage".to_string(),
            terrain,
            combat_stats,
        );

        let attacks = vec![
            Attack::ranged("Arcane Bolt", 16, 1, DamageType::Fire, 3),
            Attack::ranged("Lightning Strike", 14, 1, DamageType::Fire, 3),
            Attack::ranged("Nature's Wrath", 12, 1, DamageType::Pierce, 2),
        ];

        Self { base, attacks }
    }
}

crate::impl_unit_delegate!(ElfMage);
