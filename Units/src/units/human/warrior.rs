use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct HumanWarrior {
    base: BaseUnit,
}

impl HumanWarrior {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        // Define combat stats specific to Human Warrior
        let combat_stats = CombatStats::new_with_attacks(
            120,                                  // health
            15,                                   // base attack
            3 + Race::Human.get_movement_bonus(), // movement speed
            RangeCategory::Melee,                 // range category
            Resistances::new(
                // resistances (heavy armor)
                30, // blunt
                20, // pierce
                10, // fire
                10, // dark
                35, // slash
                25, // crush
            ),
            15, // attack_strength
            1,  // attacks_per_round
        );

        let mut base = BaseUnit::new(
            name,
            position,
            Race::Human,
            "Human Warrior".to_string(),
            "A versatile human warrior with balanced stats. Humans excel at adaptation and can thrive in various terrains. Armed with sword and bow, they form the backbone of most armies.".to_string(),
            terrain,
            graphics::SpriteType::Unit,
            None, // No previous evolution
            None, // No next evolution
            combat_stats,
        );

        // Define default attacks for human warrior
        base.attacks = vec![
            Attack::melee("Sword Slash", 15, 1, DamageType::Slash),
            Attack::ranged("Shoddy Bow", 1, 2, DamageType::Pierce, 2),
        ];

        Self { base }
    }

    /// Add a new attack to this warrior's repertoire
    pub fn add_attack(&mut self, attack: Attack) {
        self.base.attacks.push(attack);
    }
}

// Implement the Unit trait with minimal boilerplate
impl crate::unit_trait::Unit for HumanWarrior {
    fn base(&self) -> &BaseUnit {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseUnit {
        &mut self.base
    }

    fn attacks(&self) -> &[Attack] {
        &self.base.attacks
    }

    // Evolution methods work automatically - no overrides needed
}

// Register this unit type with the global registry
crate::submit_unit!(
    HumanWarrior,
    "Human Warrior",
    "A versatile human warrior",
    Terrain::Grasslands,
    "Human",
    "Warrior"
);
