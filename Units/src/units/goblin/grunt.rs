use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct GoblinGrunt {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl GoblinGrunt {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        // Define combat stats specific to Goblin Grunt
        // Goblins are weak but evasive
        let combat_stats = CombatStats::new_with_attacks(
            60,                                    // health (low)
            8,                                     // base attack (weak)
            4 + Race::Goblin.get_movement_bonus(), // movement speed (fast)
            RangeCategory::Melee,                  // range category
            Resistances::new(
                // resistances (light armor, agile)
                15, // blunt
                20, // pierce
                10, // fire
                15, // dark
                20, // slash
                10, // crush
            ),
            8, // attack_strength (weak)
            1, // attacks_per_round
        );

        let base = BaseUnit::new(
            name,
            position,
            Race::Goblin,
            "Goblin Grunt".to_string(),
            terrain,
            combat_stats,
        );

        // Define default attacks for goblin grunt
        let attacks = vec![Attack::melee("Rusty Dagger", 8, 1, DamageType::Pierce)];

        Self { base, attacks }
    }
}

// Use the macro to implement all standard Unit trait methods
crate::impl_unit_delegate!(GoblinGrunt);
