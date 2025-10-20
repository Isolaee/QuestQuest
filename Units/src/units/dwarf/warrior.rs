use crate::attack::Attack;
use crate::base_unit::BaseUnit;
use crate::unit_race::{Race, Terrain};
use combat::{CombatStats, DamageType, RangeCategory, Resistances};
use graphics::HexCoord;

pub struct DwarfWarrior {
    base: BaseUnit,
    attacks: Vec<Attack>,
}

impl DwarfWarrior {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            140,                                  // health (dwarves are tough)
            16,                                   // base attack (strong fighters)
            3 + Race::Dwarf.get_movement_bonus(), // movement speed (slow but sturdy)
            RangeCategory::Melee,                 // range category
            Resistances::new(
                // resistances (heavy armor, mountain resilience)
                40, // blunt (very resistant)
                25, // pierce
                15, // fire (forge-born)
                10, // dark
                30, // slash
                45, // crush (extremely resistant)
            ),
            16, // attack_strength
            1,  // attacks_per_round
        );

        let base = BaseUnit::new(
            name,
            position,
            Race::Dwarf,
            "Dwarf Warrior".to_string(),
            terrain,
            combat_stats,
        );

        // Define default attacks for dwarf warrior
        let attacks = vec![
            Attack::melee("Axe Chop", 16, 1, DamageType::Slash),
            Attack::melee("Shield Bash", 10, 1, DamageType::Blunt),
        ];

        Self { base, attacks }
    }

    /// Add a new attack to this warrior's repertoire
    pub fn add_attack(&mut self, attack: Attack) {
        self.attacks.push(attack);
    }
}

crate::impl_unit_delegate!(DwarfWarrior);
