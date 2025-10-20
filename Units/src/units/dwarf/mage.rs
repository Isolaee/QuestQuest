use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct DwarfMage {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl DwarfMage {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            95,                                   // health (tougher than human/elf mages)
            12,                                   // base attack
            4 + Race::Dwarf.get_movement_bonus(), // movement speed
            RangeCategory::Range,                 // range category (magic attacks are like ranged)
            Resistances::new(
                // resistances (runic protections)
                10, // blunt
                10, // pierce
                30, // fire (forge magic)
                15, // dark
                10, // slash
                15, // crush
            ),
            12, // attack_strength
            1,  // attacks_per_round
        );

        let base = BaseUnit::new(
            name,
            position,
            Race::Dwarf,
            "Dwarf Mage".to_string(),
            terrain,
            combat_stats,
        );

        let attacks = vec![
            Attack::ranged("Rune of Fire", 15, 1, DamageType::Fire, 3),
            Attack::ranged("Stone Spear", 13, 1, DamageType::Blunt, 2),
            Attack::melee("Runic Hammer", 8, 1, DamageType::Blunt),
        ];

        Self { base, attacks }
    }
}

crate::impl_unit_delegate!(DwarfMage);
