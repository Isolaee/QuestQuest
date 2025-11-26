//! Main Encyclopedia system that dynamically generates content at runtime

use crate::entries::{EncyclopediaEntry, MechanicEntry, TerrainEntry, UnitEntry};
use std::collections::HashMap;
use units::{Race, Terrain, UnitFactory};

/// The main Encyclopedia that holds all dynamically generated entries
pub struct Encyclopedia {
    units: HashMap<String, UnitEntry>,
    terrain: HashMap<String, TerrainEntry>,
    mechanics: HashMap<String, MechanicEntry>,
}

impl Encyclopedia {
    /// Create a new Encyclopedia by dynamically loading all content
    pub fn new() -> Self {
        let mut encyclopedia = Encyclopedia {
            units: HashMap::new(),
            terrain: HashMap::new(),
            mechanics: HashMap::new(),
        };

        // Load all units dynamically from the registry
        encyclopedia.load_units();

        // Load all terrain types
        encyclopedia.load_terrain();

        // Load game mechanics
        encyclopedia.load_mechanics();

        encyclopedia
    }

    /// Dynamically load all registered units
    fn load_units(&mut self) {
        let unit_types = UnitFactory::list_types();

        for unit_type in unit_types {
            if let Ok(entry) = UnitEntry::from_unit_type(unit_type) {
                self.units.insert(unit_type.to_string(), entry);
            }
        }

        // Evolution chains are now automatically populated from unit data
    }

    /// Load all terrain types
    fn load_terrain(&mut self) {
        let terrain_types = vec![
            Terrain::Grasslands,
            Terrain::Forest0,
            Terrain::Forest1,
            Terrain::Mountain,
            Terrain::Hills,
            Terrain::Swamp,
            Terrain::HauntedWoods,
        ];

        for terrain in terrain_types {
            let entry = TerrainEntry::new(terrain);
            self.terrain.insert(terrain.name().to_string(), entry);
        }
    }

    /// Load game mechanics documentation
    fn load_mechanics(&mut self) {
        // Combat System
        self.mechanics.insert(
            "Combat System".to_string(),
            MechanicEntry::new(
                "Combat System".to_string(),
                "Core Mechanics".to_string(),
                "Combat in QuestQuest is turn-based with units taking actions based on their movement and attack capabilities.".to_string(),
                vec![
                    "Attacks are calculated using attack strength vs. defense".to_string(),
                    "Damage is modified by resistances and damage types".to_string(),
                    "Terrain affects hit chance and defense values".to_string(),
                    "Range determines if units can attack (melee vs. ranged)".to_string(),
                ],
                vec![
                    "A Human Warrior (15 attack) vs Goblin Grunt (low defense)".to_string(),
                    "Elf Archer attacking from 3 hexes away with pierce damage".to_string(),
                ],
            ),
        );

        // Experience System
        self.mechanics.insert(
            "Experience & Leveling".to_string(),
            MechanicEntry::new(
                "Experience & Leveling".to_string(),
                "Progression".to_string(),
                "Units gain experience from combat and other actions, leveling up to become stronger or evolve into new unit types.".to_string(),
                vec![
                    "XP required = Level² × 50 (e.g., Level 2 needs 200 XP)".to_string(),
                    "Leveling increases max HP, attack, and other stats".to_string(),
                    "Some units evolve into stronger forms at specific levels".to_string(),
                    "Evolution chains: Young → Base → Veteran/Elite".to_string(),
                ],
                vec![
                    "Dwarf Young Warrior (Level 1) → Dwarf Warrior (Level 2)".to_string(),
                    "Orc Swordsman gains +2 max HP and +1 attack per level".to_string(),
                ],
            ),
        );

        // Movement System
        self.mechanics.insert(
            "Movement".to_string(),
            MechanicEntry::new(
                "Movement".to_string(),
                "Core Mechanics".to_string(),
                "Units move across hexagonal tiles based on their movement speed and terrain costs.".to_string(),
                vec![
                    "Each unit has a movement speed stat".to_string(),
                    "Terrain affects movement cost (Mountains = 2, Grasslands = 1)".to_string(),
                    "Races have movement bonuses (Elves faster, Dwarves slower)".to_string(),
                    "Units can move multiple times per turn if movement allows".to_string(),
                ],
                vec![
                    "Elf Archer (5 movement) can cross 5 grassland tiles".to_string(),
                    "Dwarf Warrior (3 movement) crosses only 1.5 mountain tiles".to_string(),
                ],
            ),
        );

        // Damage Types
        self.mechanics.insert(
            "Damage Types".to_string(),
            MechanicEntry::new(
                "Damage Types".to_string(),
                "Combat".to_string(),
                "Different attacks deal different damage types, which interact with unit resistances.".to_string(),
                vec![
                    "Slash: Swords, blades (good vs light armor)".to_string(),
                    "Pierce: Arrows, spears (penetrates armor)".to_string(),
                    "Blunt: Hammers, clubs (good vs heavy armor)".to_string(),
                    "Crush: Overwhelming force attacks".to_string(),
                    "Fire: Magical fire damage".to_string(),
                    "Dark: Magical dark/shadow damage".to_string(),
                ],
                vec![
                    "Elf Archer's pierce damage vs heavily armored Dwarf".to_string(),
                    "Human Mage's fire spell vs fire-resistant unit".to_string(),
                ],
            ),
        );

        // Resistances
        self.mechanics.insert(
            "Resistances".to_string(),
            MechanicEntry::new(
                "Resistances".to_string(),
                "Defense".to_string(),
                "Units have resistance percentages that reduce incoming damage from specific damage types.".to_string(),
                vec![
                    "Resistance % reduces damage by that amount".to_string(),
                    "Heavy armor units have high slash/crush resistance".to_string(),
                    "Light armor units have lower physical resistance".to_string(),
                    "Some races naturally resist certain elements".to_string(),
                ],
                vec![
                    "Dwarf Warrior: 35% slash resistance reduces 15 damage to 9.75".to_string(),
                    "Goblin Grunt: Low resistance makes them vulnerable".to_string(),
                ],
            ),
        );

        // Terrain Effects
        self.mechanics.insert(
            "Terrain Effects".to_string(),
            MechanicEntry::new(
                "Terrain Effects".to_string(),
                "Environment".to_string(),
                "The terrain a unit stands on affects their combat effectiveness and movement."
                    .to_string(),
                vec![
                    "Each race has different defense values per terrain".to_string(),
                    "Elves excel in forests, Dwarves in mountains".to_string(),
                    "Defense value = base hit chance for enemies to hit you".to_string(),
                    "Lower defense values = harder to hit".to_string(),
                ],
                vec![
                    "Elf in Forest: 40% hit chance (very defensive)".to_string(),
                    "Human in Grasslands: 75% hit chance (neutral)".to_string(),
                ],
            ),
        );

        // Equipment
        self.mechanics.insert(
            "Equipment System".to_string(),
            MechanicEntry::new(
                "Equipment System".to_string(),
                "Items".to_string(),
                "Units can equip weapons, armor, and accessories to enhance their capabilities."
                    .to_string(),
                vec![
                    "Weapons provide attacks and increase attack strength".to_string(),
                    "Armor increases resistances and max health".to_string(),
                    "Accessories provide various bonuses".to_string(),
                    "Equipment bonuses are calculated on equip".to_string(),
                ],
                vec![
                    "Equipping a Steel Sword adds its attack bonus".to_string(),
                    "Heavy Plate Armor increases slash/crush resistance".to_string(),
                ],
            ),
        );
    }

    // ===== Query Methods =====

    /// Get a unit entry by name
    pub fn get_unit_entry(&self, name: &str) -> Option<&UnitEntry> {
        self.units.get(name)
    }

    /// Get a terrain entry by name
    pub fn get_terrain_entry(&self, name: &str) -> Option<&TerrainEntry> {
        self.terrain.get(name)
    }

    /// Get a mechanic entry by name
    pub fn get_mechanic_entry(&self, name: &str) -> Option<&MechanicEntry> {
        self.mechanics.get(name)
    }

    /// Get all unit entries
    pub fn all_units(&self) -> Vec<&UnitEntry> {
        self.units.values().collect()
    }

    /// Get all terrain entries
    pub fn all_terrain(&self) -> Vec<&TerrainEntry> {
        self.terrain.values().collect()
    }

    /// Get all mechanic entries
    pub fn all_mechanics(&self) -> Vec<&MechanicEntry> {
        self.mechanics.values().collect()
    }

    /// Search units by race
    pub fn units_by_race(&self, race: Race) -> Vec<&UnitEntry> {
        self.units.values().filter(|u| u.race == race).collect()
    }

    /// Search units by class
    pub fn units_by_class(&self, class: &str) -> Vec<&UnitEntry> {
        self.units
            .values()
            .filter(|u| u.class.eq_ignore_ascii_case(class))
            .collect()
    }

    // ===== Display Methods =====

    /// Display the main encyclopedia index
    pub fn display_index(&self) {
        println!("╔═══════════════════════════════════════════════════════════════════════╗");
        println!("║           📚 QUESTQUEST ENCYCLOPEDIA - TABLE OF CONTENTS              ║");
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║                                                                       ║");
        println!("║  1. UNITS                                                             ║");
        println!("║     Comprehensive information about all unit types                    ║");
        println!(
            "║     {} registered units                                              ║",
            self.units.len()
        );
        println!("║                                                                       ║");
        println!("║  2. TERRAIN                                                           ║");
        println!("║     Environmental effects and terrain mechanics                       ║");
        println!(
            "║     {} terrain types                                                 ║",
            self.terrain.len()
        );
        println!("║                                                                       ║");
        println!("║  3. GAME MECHANICS                                                    ║");
        println!("║     Combat, movement, progression, and core systems                   ║");
        println!(
            "║     {} mechanic entries                                              ║",
            self.mechanics.len()
        );
        println!("║                                                                       ║");
        println!("╚═══════════════════════════════════════════════════════════════════════╝");
    }

    /// Display the unit index
    pub fn display_unit_index(&self) {
        println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
        println!("║                        📖 UNIT ENCYCLOPEDIA                            ║");
        println!("╠═══════════════════════════════════════════════════════════════════════╣");

        // Group by race
        for race in [Race::Human, Race::Elf, Race::Dwarf, Race::Orc, Race::Goblin] {
            let race_units = self.units_by_race(race);
            if !race_units.is_empty() {
                println!(
                    "║                                                                       ║"
                );
                println!(
                    "║ {:?} UNITS                                                          ",
                    race
                );
                println!("║ {:<71} ║", "─".repeat(71));

                for unit in race_units {
                    println!(
                        "║   • {:<66} ║",
                        format!("{} (Level {})", unit.unit_type, unit.stats.level)
                    );
                }
            }
        }

        println!("║                                                                       ║");
        println!("╚═══════════════════════════════════════════════════════════════════════╝");
    }

    /// Display the terrain guide
    pub fn display_terrain_guide(&self) {
        println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
        println!("║                      🌍 TERRAIN GUIDE                                  ║");
        println!("╠═══════════════════════════════════════════════════════════════════════╣");

        let mut terrain_list: Vec<_> = self.terrain.values().collect();
        terrain_list.sort_by_key(|t| t.movement_cost);

        for terrain in terrain_list {
            println!(
                "║   • {:<50} (Movement: {})     ║",
                terrain.terrain_type.name(),
                terrain.movement_cost
            );
        }

        println!("╚═══════════════════════════════════════════════════════════════════════╝");
    }

    /// Display the mechanics index
    pub fn display_mechanics_index(&self) {
        println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
        println!("║                     ⚙️  GAME MECHANICS                                 ║");
        println!("╠═══════════════════════════════════════════════════════════════════════╣");

        let mut mechanics: Vec<_> = self.mechanics.values().collect();
        mechanics.sort_by_key(|m| &m.category);

        let mut current_category = String::new();
        for mechanic in mechanics {
            if mechanic.category != current_category {
                current_category = mechanic.category.clone();
                println!(
                    "║                                                                       ║"
                );
                println!(
                    "║ {}                                                              ",
                    current_category
                );
                println!("║ {:<71} ║", "─".repeat(71));
            }
            println!("║   • {:<66} ║", mechanic.title);
        }

        println!("║                                                                       ║");
        println!("╚═══════════════════════════════════════════════════════════════════════╝");
    }

    /// Search the encyclopedia
    pub fn search(&self, query: &str) -> Vec<EncyclopediaEntry> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        // Search units
        for unit in self.units.values() {
            if unit.name.to_lowercase().contains(&query_lower)
                || unit.unit_type.to_lowercase().contains(&query_lower)
                || unit.description.to_lowercase().contains(&query_lower)
            {
                results.push(EncyclopediaEntry::Unit(unit.clone()));
            }
        }

        // Search terrain
        for terrain in self.terrain.values() {
            if terrain
                .terrain_type
                .name()
                .to_lowercase()
                .contains(&query_lower)
                || terrain.description.to_lowercase().contains(&query_lower)
            {
                results.push(EncyclopediaEntry::Terrain(terrain.clone()));
            }
        }

        // Search mechanics
        for mechanic in self.mechanics.values() {
            if mechanic.title.to_lowercase().contains(&query_lower)
                || mechanic.description.to_lowercase().contains(&query_lower)
            {
                results.push(EncyclopediaEntry::Mechanic(mechanic.clone()));
            }
        }

        results
    }
}

impl Default for Encyclopedia {
    fn default() -> Self {
        Self::new()
    }
}
