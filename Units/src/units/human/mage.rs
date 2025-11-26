use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct HumanMage {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl HumanMage {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            70,                                   // health (mages are fragile)
            10,                                   // base attack
            4 + Race::Human.get_movement_bonus(), // movement speed
            RangeCategory::Range,                 // range category (magic attacks are like ranged)
            Resistances::new(
                // resistances (robes provide some magic resistance)
                5,  // blunt
                5,  // pierce
                20, // fire
                20, // dark
                5,  // slash
                5,  // crush
            ),
            10, // attack_strength
            1,  // attacks_per_round
        );

        let base = BaseUnit::new(
            name,
            position,
            Race::Human,
            "Human Mage".to_string(),
            "A human mage wielding arcane powers. Masters of elemental magic, they can devastate enemies from afar. Frail in close combat but deadly at range with fire and dark magic.".to_string(),
            terrain,
            combat_stats,
        );

        let attacks = vec![
            Attack::ranged("Fireball", 15, 1, DamageType::Fire, 3),
            Attack::ranged("Ice Blast", 12, 1, DamageType::Fire, 3), // Use Fire for cold spells
            Attack::melee("Staff Strike", 4, 1, DamageType::Blunt),
        ];

        Self { base, attacks }
    }
}

crate::impl_unit_delegate!(HumanMage);

crate::submit_unit!(
    HumanMage,
    "Human Mage",
    "A powerful human mage",
    Terrain::Grasslands,
    "Human",
    "Mage"
);
