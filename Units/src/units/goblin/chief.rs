use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::Race;
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct GoblinChief {
    base: BaseUnit,
}

impl GoblinChief {
    pub fn new(name: String, position: HexCoord) -> Self {
        // Define combat stats specific to Goblin Chief
        // Stronger than regular goblins, still agile
        let combat_stats = CombatStats::new_with_attacks(
            90,                                    // health (moderate)
            12,                                    // base attack (decent)
            4 + Race::Goblin.get_movement_bonus(), // movement speed (fast)
            RangeCategory::Melee,                  // range category
            Resistances::new(
                // resistances (better armor than grunt)
                20, // blunt
                25, // pierce
                15, // fire
                20, // dark
                25, // slash
                15, // crush
            ),
            12, // attack_strength
            1,  // attacks_per_round
        );

        let mut base = BaseUnit::new(
            name,
            position,
            Race::Goblin,
            "Goblin Chief".to_string(),
            "A goblin leader with enhanced combat prowess. Chiefs command respect through strength and cunning, wielding better equipment than their underlings. Still maintains the goblin's characteristic agility.".to_string(),
            None,
            vec![],
            combat_stats,
        );

        // Define default attacks for goblin chief
        base.attacks = vec![
            Attack::melee("Tribal Blade", 12, 1, DamageType::Slash),
            Attack::melee("War Cry", 10, 1, DamageType::Blunt), // Intimidation attack
        ];

        Self { base }
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for GoblinChief {
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
