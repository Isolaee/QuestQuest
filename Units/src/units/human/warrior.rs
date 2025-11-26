use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct HumanWarrior {
    base: BaseUnit,
    attacks: Vec<Attack>,
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

        let base = BaseUnit::new(
            name,
            position,
            Race::Human,
            "Human Warrior".to_string(),
            "A versatile human warrior with balanced stats. Humans excel at adaptation and can thrive in various terrains. Armed with sword and bow, they form the backbone of most armies.".to_string(),
            terrain,
            combat_stats,
        );

        // Define default attacks for human warrior
        let attacks = vec![
            Attack::melee("Sword Slash", 15, 1, DamageType::Slash),
            Attack::ranged("Shoddy Bow", 1, 2, DamageType::Pierce, 2),
        ];

        Self { base, attacks }
    }

    /// Add a new attack to this warrior's repertoire
    pub fn add_attack(&mut self, attack: Attack) {
        self.attacks.push(attack);
    }
}

// Use the macro to implement all standard Unit trait methods
crate::impl_unit_delegate!(HumanWarrior);

// Register this unit type with the global registry
crate::submit_unit!(
    HumanWarrior,
    "Human Warrior",
    "A versatile human warrior",
    Terrain::Grasslands,
    "Human",
    "Warrior"
);
