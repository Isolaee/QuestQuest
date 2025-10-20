use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct DwarfArcher {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl DwarfArcher {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            110,                                  // health (tough but less than warrior)
            14,                                   // base attack
            4 + Race::Dwarf.get_movement_bonus(), // movement speed
            RangeCategory::Range,                 // range category
            Resistances::new(
                // resistances (medium armor)
                25, // blunt
                20, // pierce
                10, // fire
                5,  // dark
                20, // slash
                25, // crush
            ),
            14, // attack_strength
            1,  // attacks_per_round
        );

        let base = BaseUnit::new(
            name,
            position,
            Race::Dwarf,
            "Dwarf Archer".to_string(),
            terrain,
            combat_stats,
        );

        let attacks = vec![
            Attack::ranged("Crossbow Bolt", 14, 1, DamageType::Pierce, 3),
            Attack::melee("Hand Axe", 8, 1, DamageType::Slash),
        ];

        Self { base, attacks }
    }
}

// Use the macro to implement all standard Unit trait methods
crate::impl_unit_delegate!(DwarfArcher);
