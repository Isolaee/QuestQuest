use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct HumanArcher {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl HumanArcher {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            80,                                   // health
            12,                                   // base attack
            5 + Race::Human.get_movement_bonus(), // movement speed (archers are faster)
            RangeCategory::Range,                 // range category
            Resistances::new(
                // resistances (light armor)
                10, // blunt
                10, // pierce
                5,  // fire
                5,  // dark
                15, // slash
                10, // crush
            ),
            12, // attack_strength
            1,  // attacks_per_round
        );

        let base = BaseUnit::new(
            name,
            position,
            Race::Human,
            "Human Archer".to_string(),
            terrain,
            combat_stats,
        );

        let attacks = vec![
            Attack::ranged("Longbow", 12, 1, combat::DamageType::Pierce, 3),
            Attack::melee("Short Blade", 6, 1, combat::DamageType::Slash),
        ];

        Self { base, attacks }
    }
}

crate::impl_unit_delegate!(HumanArcher);
