//! Encyclopedia entry types for different content categories

use combat::{DamageType, RangeCategory, Resistances};
use units::{Race, Terrain, UnitFactory, UnitType};

/// Main encyclopedia entry type
#[derive(Debug, Clone)]
pub enum EncyclopediaEntry {
    /// Unit information entry
    Unit(UnitEntry),
    /// Terrain information entry
    Terrain(TerrainEntry),
    /// Game mechanic explanation entry
    Mechanic(MechanicEntry),
}

impl EncyclopediaEntry {
    /// Get the title of this entry
    pub fn title(&self) -> &str {
        match self {
            EncyclopediaEntry::Unit(entry) => &entry.name,
            EncyclopediaEntry::Terrain(entry) => entry.terrain_type.name(),
            EncyclopediaEntry::Mechanic(entry) => &entry.title,
        }
    }

    /// Get the category of this entry
    pub fn category(&self) -> &str {
        match self {
            EncyclopediaEntry::Unit(_) => "Units",
            EncyclopediaEntry::Terrain(_) => "Terrain",
            EncyclopediaEntry::Mechanic(_) => "Mechanics",
        }
    }

    /// Display this entry
    pub fn display(&self) {
        match self {
            EncyclopediaEntry::Unit(entry) => entry.display(),
            EncyclopediaEntry::Terrain(entry) => entry.display(),
            EncyclopediaEntry::Mechanic(entry) => entry.display(),
        }
    }
}

/// Detailed unit information entry
#[derive(Debug, Clone)]
pub struct UnitEntry {
    pub name: String,
    pub unit_type: String,
    pub description: String,
    pub race: Race,
    pub class: String,
    pub default_terrain: Terrain,
    pub stats: UnitStats,
    pub evolution: EvolutionInfo,
    pub attacks: Vec<AttackInfo>,
}

#[derive(Debug, Clone)]
pub struct UnitStats {
    pub level: i32,
    pub health: i32,
    pub max_health: i32,
    pub attack_strength: u32,
    pub movement_speed: i32,
    pub range_category: RangeCategory,
    pub resistances: Resistances,
    pub defense: u8,
}

#[derive(Debug, Clone)]
pub struct EvolutionInfo {
    pub previous_form: Option<UnitType>,
    /// Multiple possible evolution paths
    pub next_forms: Vec<UnitType>,
    pub evolution_level: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct AttackInfo {
    pub name: String,
    pub damage: u32,
    pub range: u32,
    pub damage_type: DamageType,
}

impl UnitEntry {
    /// Create a unit entry by dynamically loading a unit
    pub fn from_unit_type(type_name: &str) -> Result<Self, String> {
        // Create the unit to extract all its data
        let unit = UnitFactory::create(type_name, None, None)?;

        let stats = unit.combat_stats();
        let attacks = unit.get_attacks();

        // Verify the unit type is registered
        let _registry_info = units::UnitFactory::list_types()
            .iter()
            .find(|&t| *t == type_name)
            .ok_or_else(|| format!("Unit type {} not found in registry", type_name))?;

        // Get evolution information directly from the unit
        let evolutions = unit.evolution_next();
        let evolution_info = EvolutionInfo {
            previous_form: unit.evolution_previous(),
            next_forms: evolutions.clone(),
            evolution_level: if !evolutions.is_empty() {
                Some(unit.level() + 1)
            } else {
                None
            },
        };

        Ok(UnitEntry {
            name: unit.name().to_string(),
            unit_type: unit.unit_type().to_string(),
            description: unit.description().to_string(),
            race: unit.race(),
            class: Self::extract_class_from_type(type_name),
            default_terrain: units::Terrain::Grasslands, // Default terrain for display
            stats: UnitStats {
                level: unit.level(),
                health: stats.health,
                max_health: stats.max_health,
                attack_strength: stats.attack_strength,
                movement_speed: stats.movement_speed,
                range_category: stats.range_category,
                resistances: stats.resistances.clone(),
                defense: unit.get_defense(),
            },
            evolution: evolution_info,
            attacks: attacks
                .iter()
                .map(|a| AttackInfo {
                    name: a.name.clone(),
                    damage: a.damage,
                    range: a.range as u32,
                    damage_type: a.damage_type,
                })
                .collect(),
        })
    }

    fn extract_class_from_type(type_name: &str) -> String {
        // Extract class from names like "Human Warrior", "Elf Archer"
        let parts: Vec<&str> = type_name.split_whitespace().collect();
        if parts.len() > 1 {
            parts[parts.len() - 1].to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// Display this unit entry in a formatted way
    pub fn display(&self) {
        println!("╔═══════════════════════════════════════════════════════════════════════╗");
        println!(
            "║  📖 ENCYCLOPEDIA: {}                                              ",
            self.unit_type
        );
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ Type: {:<66} ║", self.unit_type);
        println!("║ Race: {:<66} ║", format!("{:?}", self.race));
        println!("║ Class: {:<65} ║", self.class);
        println!(
            "║ Default Terrain: {:<57} ║",
            format!("{:?}", self.default_terrain)
        );
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ DESCRIPTION                                                           ║");

        // Word wrap description
        Self::print_wrapped(&self.description, 69);

        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!(
            "║ BASE STATS (Level {})                                                  ║",
            self.stats.level
        );
        println!(
            "║   Health: {:<60} ║",
            format!("{}/{}", self.stats.health, self.stats.max_health)
        );
        println!("║   Attack Strength: {:<53} ║", self.stats.attack_strength);
        println!("║   Movement Speed: {:<54} ║", self.stats.movement_speed);
        println!("║   Defense: {:<61} ║", self.stats.defense);
        println!(
            "║   Range: {:<63} ║",
            format!("{:?}", self.stats.range_category)
        );
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ RESISTANCES                                                           ║");
        println!(
            "║   Blunt: {:>3}%  Pierce: {:>3}%  Slash: {:>3}%  Crush: {:>3}%           ║",
            self.stats.resistances.blunt,
            self.stats.resistances.pierce,
            self.stats.resistances.slash,
            self.stats.resistances.crush
        );
        println!(
            "║   Fire: {:>4}%  Dark: {:>6}%                                        ║",
            self.stats.resistances.fire, self.stats.resistances.dark
        );
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ ATTACKS                                                               ║");
        for attack in &self.attacks {
            println!(
                "║   • {} (Damage: {}, Range: {}, Type: {:?})                    ",
                attack.name, attack.damage, attack.range, attack.damage_type
            );
        }

        if self.evolution.previous_form.is_some() || !self.evolution.next_forms.is_empty() {
            println!("╠═══════════════════════════════════════════════════════════════════════╣");
            println!("║ EVOLUTION CHAIN                                                       ║");
            if let Some(prev) = &self.evolution.previous_form {
                println!("║   ← Previous: {:<58} ║", prev.as_str());
            }
            if !self.evolution.next_forms.is_empty() {
                println!("║   → Next Options:                                                  ║");
                for (idx, next) in self.evolution.next_forms.iter().enumerate() {
                    println!("║      {}. {:<60} ║", idx + 1, next.as_str());
                }
            }
        }

        println!("╚═══════════════════════════════════════════════════════════════════════╝");
    }

    fn print_wrapped(text: &str, width: usize) {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > width {
                println!("║ {:<71} ║", current_line);
                current_line = word.to_string();
            } else {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
        }

        if !current_line.is_empty() {
            println!("║ {:<71} ║", current_line);
        }
    }
}

/// Terrain information entry
#[derive(Debug, Clone)]
pub struct TerrainEntry {
    pub terrain_type: Terrain,
    pub description: String,
    pub movement_cost: i32,
    pub defense_modifiers: Vec<(Race, u8)>,
    pub special_effects: Vec<String>,
}

impl TerrainEntry {
    /// Create a terrain entry for the given terrain type
    pub fn new(terrain: Terrain) -> Self {
        let (description, movement_cost, special_effects) = match terrain {
            Terrain::Grasslands => (
                "Open grasslands provide balanced movement and moderate defensive positions. Most races perform adequately in this neutral terrain.",
                1,
                vec!["Standard movement".to_string(), "No special bonuses".to_string()],
            ),
            Terrain::Forest0 | Terrain::Forest1 => (
                "Dense forests provide excellent cover and concealment. Elves excel in forest terrain with natural bonuses, while others may find movement hampered.",
                1,
                vec!["Elves gain defense bonus".to_string(), "Good for ambushes".to_string()],
            ),
            Terrain::Mountain => (
                "Treacherous mountain terrain favors dwarves who are at home in rocky heights. Steep slopes slow movement but provide excellent defensive positions.",
                2,
                vec!["Dwarves excel here".to_string(), "High ground advantage".to_string(), "Slowed movement".to_string()],
            ),
            Terrain::Swamp => (
                "Fetid swamplands slow all movement and provide challenging combat conditions. Few races thrive here, making it treacherous territory.",
                2,
                vec!["Heavily reduced movement".to_string(), "Difficult terrain".to_string()],
            ),
            Terrain::Hills => (
                "Rolling hills provide moderate elevation advantages and partial cover. Good mobility while maintaining defensive benefits.",
                1,
                vec!["Moderate elevation bonus".to_string(), "Balanced terrain".to_string()],
            ),
            Terrain::HauntedWoods => (
                "Dark, cursed forests where unnatural forces dwell. The oppressive atmosphere affects morale and visibility, though some dark creatures feel at home.",
                1,
                vec!["Eerie atmosphere".to_string(), "Reduced visibility".to_string()],
            ),
        };

        // Calculate defense modifiers for each race
        let defense_modifiers = vec![
            (Race::Human, terrain.get_hit_chance(Race::Human)),
            (Race::Elf, terrain.get_hit_chance(Race::Elf)),
            (Race::Dwarf, terrain.get_hit_chance(Race::Dwarf)),
            (Race::Orc, terrain.get_hit_chance(Race::Orc)),
            (Race::Goblin, terrain.get_hit_chance(Race::Goblin)),
        ];

        TerrainEntry {
            terrain_type: terrain,
            description: description.to_string(),
            movement_cost,
            defense_modifiers,
            special_effects,
        }
    }

    /// Display this terrain entry
    pub fn display(&self) {
        println!("╔═══════════════════════════════════════════════════════════════════════╗");
        println!("║  🌍 TERRAIN: {:60} ║", self.terrain_type.name());
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ DESCRIPTION                                                           ║");

        // Word wrap description
        let words: Vec<&str> = self.description.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > 69 {
                println!("║ {:<71} ║", current_line);
                current_line = word.to_string();
            } else {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
        }

        if !current_line.is_empty() {
            println!("║ {:<71} ║", current_line);
        }

        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ PROPERTIES                                                            ║");
        println!("║   Movement Cost: {:<55} ║", self.movement_cost);
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ DEFENSE MODIFIERS (Hit Chance %)                                      ║");
        for (race, hit_chance) in &self.defense_modifiers {
            println!(
                "║   {:?}: {:<62} ║",
                race,
                format!("{}% base hit chance", hit_chance)
            );
        }
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ SPECIAL EFFECTS                                                       ║");
        for effect in &self.special_effects {
            println!("║   • {:<68} ║", effect);
        }
        println!("╚═══════════════════════════════════════════════════════════════════════╝");
    }
}

/// Game mechanic explanation entry
#[derive(Debug, Clone)]
pub struct MechanicEntry {
    pub title: String,
    pub category: String,
    pub description: String,
    pub details: Vec<String>,
    pub examples: Vec<String>,
}

impl MechanicEntry {
    /// Create a new mechanic entry
    pub fn new(
        title: String,
        category: String,
        description: String,
        details: Vec<String>,
        examples: Vec<String>,
    ) -> Self {
        MechanicEntry {
            title,
            category,
            description,
            details,
            examples,
        }
    }

    /// Display this mechanic entry
    pub fn display(&self) {
        println!("╔═══════════════════════════════════════════════════════════════════════╗");
        println!("║  ⚙️  MECHANIC: {:<59} ║", self.title);
        println!("║  Category: {:<61} ║", self.category);
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ DESCRIPTION                                                           ║");

        // Word wrap description
        let words: Vec<&str> = self.description.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > 69 {
                println!("║ {:<71} ║", current_line);
                current_line = word.to_string();
            } else {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
        }

        if !current_line.is_empty() {
            println!("║ {:<71} ║", current_line);
        }

        if !self.details.is_empty() {
            println!("╠═══════════════════════════════════════════════════════════════════════╣");
            println!("║ DETAILS                                                               ║");
            for detail in &self.details {
                println!("║   • {:<68} ║", detail);
            }
        }

        if !self.examples.is_empty() {
            println!("╠═══════════════════════════════════════════════════════════════════════╣");
            println!("║ EXAMPLES                                                              ║");
            for example in &self.examples {
                println!("║   {:<70} ║", example);
            }
        }

        println!("╚═══════════════════════════════════════════════════════════════════════╝");
    }
}
