use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::combat::{CombatStats, DamageType, RangeCategory, Resistances};
use crate::unit_race::Race;
use graphics::HexCoord;

pub struct GoblinGrunt {
    base: BaseUnit,
}

impl GoblinGrunt {
    pub fn new(name: String, position: HexCoord) -> Self {
        // Define combat stats specific to Goblin Grunt
        // Goblins are weak but evasive
        let combat_stats = CombatStats::new_with_attacks(
            60,                   // health (low)
            8,                    // base attack (weak)
            4,                    // movement speed (fast)
            RangeCategory::Melee, // range category
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

        let mut base = BaseUnit::new(
            name,
            position,
            Race::Goblin,
            "Goblin Grunt".to_string(),
            "A weak but evasive goblin warrior. Goblins are cunning and fast, making up for their lack of strength with speed and numbers. They excel in swamps and dark places where they can ambush unwary foes.".to_string(),
            None,
            vec![],
            combat_stats,
        );

        // Define default attacks for goblin grunt
        base.attacks = vec![Attack::melee("Rusty Dagger", 8, 1, DamageType::Pierce)];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for GoblinGrunt {
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
