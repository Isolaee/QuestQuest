use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct GoblinChief {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl GoblinChief {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
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

        let base = BaseUnit::new(
            name,
            position,
            Race::Goblin,
            "Goblin Chief".to_string(),
            "A goblin leader with enhanced combat prowess. Chiefs command respect through strength and cunning, wielding better equipment than their underlings. Still maintains the goblin's characteristic agility.".to_string(),
            terrain,
            combat_stats,
        );

        // Define default attacks for goblin chief
        let attacks = vec![
            Attack::melee("Tribal Blade", 12, 1, DamageType::Slash),
            Attack::melee("War Cry", 10, 1, DamageType::Blunt), // Intimidation attack
        ];

        Self { base, attacks }
    }
}

// Use the macro to implement all standard Unit trait methods
crate::impl_unit_delegate!(GoblinChief);
