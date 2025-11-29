// === Encyclopedia Content Helpers (moved from main.rs) ===
use encyclopedia::{Encyclopedia, EncyclopediaEntry, MechanicEntry};

pub struct EncyclopediaBuilder {
    title: String,
    description: Vec<String>,
    stats: Vec<(String, String)>,
    tips: Vec<String>,
}

impl EncyclopediaBuilder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: Vec::new(),
            stats: Vec::new(),
            tips: Vec::new(),
        }
    }
    pub fn description(mut self, line: impl Into<String>) -> Self {
        self.description.push(line.into());
        self
    }
    pub fn stat(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.stats.push((key.into(), value.into()));
        self
    }
    pub fn tip(mut self, tip: impl Into<String>) -> Self {
        self.tips.push(tip.into());
        self
    }
    pub fn build(self) -> EncyclopediaEntry {
        // Compose description and details for MechanicEntry
        let mut details = Vec::new();
        if !self.stats.is_empty() {
            details.push("Stats:".to_string());
            for (k, v) in &self.stats {
                details.push(format!("  • {}: {}", k, v));
            }
        }
        if !self.tips.is_empty() {
            details.push("Tips:".to_string());
            for tip in &self.tips {
                details.push(format!("  • {}", tip));
            }
        }
        EncyclopediaEntry::Mechanic(MechanicEntry {
            title: self.title,
            category: "Guide".to_string(),
            description: self.description.join("\n"),
            details,
            examples: Vec::new(),
        })
    }
}

pub struct EncyclopediaLibrary;

impl EncyclopediaLibrary {
    pub fn combat_system() -> EncyclopediaEntry {
        EncyclopediaBuilder::new("Combat System")
            .description("QuestQuest uses a tactical turn-based combat system.")
            .description("Combat is resolved using attack, defense, and damage types.")
            .description("")
            .description("Damage Types:")
            .description("• Slash - Swords and bladed weapons")
            .description("• Pierce - Arrows, spears, daggers")
            .description("• Blunt - Hammers and clubs")
            .description("• Crush - Heavy weapons and siege")
            .description("• Fire - Magical fire damage")
            .description("• Dark - Shadow and curse magic")
            .stat("Hit Chance", "Base + Terrain Bonus - Enemy Evasion")
            .stat("Damage", "Attack - (Defense * Resistance)")
            .tip("Different units have different resistances to damage types")
            .tip("Terrain provides bonuses to hit chance for certain races")
            .tip("Equipment can modify your attack range and damage type")
            .build()
    }
    pub fn movement_system() -> EncyclopediaEntry {
        EncyclopediaBuilder::new("Movement System")
            .description("Units move across a hexagonal grid.")
            .description("Each unit has a movement range based on their class.")
            .description("")
            .description("Movement Categories:")
            .description("• Infantry: 2-3 hexes per turn")
            .description("• Cavalry: 4-5 hexes per turn")
            .description("• Scouts: 5-6 hexes per turn")
            .stat("Movement Cost", "1 hex = 1 movement point")
            .stat("Terrain Effects", "Some terrain may cost extra")
            .tip("Plan your movement to stay within attack range")
            .tip("Use scouts for reconnaissance")
            .tip("Positioning is key in tactical combat")
            .build()
    }
    pub fn character_classes() -> EncyclopediaEntry {
        EncyclopediaBuilder::new("Character Classes")
            .description("Each unit belongs to one of four main classes:")
            .description("")
            .description("Warrior - Melee combat specialist")
            .description("• High health and defense")
            .description("• Deals Slash or Blunt damage")
            .description("• Melee range only")
            .description("")
            .description("Archer - Ranged attacker")
            .description("• Medium health, low defense")
            .description("• Deals Pierce damage")
            .description("• Long attack range")
            .description("")
            .description("Mage - Magic user")
            .description("• Low health and defense")
            .description("• Deals Fire or Dark damage")
            .description("• Medium range, high damage")
            .description("")
            .description("Paladin - Holy knight")
            .description("• High health and defense")
            .description("• Deals Fire damage")
            .description("• Can heal allies")
            .stat("Total Classes", "4 per race")
            .tip("Each race has unique bonuses for each class")
            .tip("Mix different classes for tactical advantage")
            .build()
    }
    pub fn character_races() -> EncyclopediaEntry {
        EncyclopediaBuilder::new("Character Races")
            .description("Four playable races, each with unique traits:")
            .description("")
            .description("Human - Balanced and versatile")
            .description("• No terrain penalties")
            .description("• Bonus to all stats")
            .description("")
            .description("Elf - Swift and precise")
            .description("• Bonus in forests")
            .description("• Extra movement and accuracy")
            .description("")
            .description("Dwarf - Sturdy and resilient")
            .description("• Bonus in mountains")
            .description("• Extra health and defense")
            .description("")
            .description("Orc - Strong and aggressive")
            .description("• Bonus in all terrains")
            .description("• Extra attack power")
            .stat("Playable Races", "4")
            .tip("Choose race based on your playstyle")
            .tip("Terrain bonuses are significant")
            .build()
    }
    pub fn equipment_system() -> EncyclopediaEntry {
        EncyclopediaBuilder::new("Equipment System")
            .description("Units can equip weapons, armor, and accessories.")
            .description("Equipment provides stat bonuses and special effects.")
            .description("")
            .description("Equipment Slots:")
            .description("• Weapon - Increases attack and may extend range")
            .description("• Armor - Increases defense and resistances")
            .description("• Accessory - Various special effects")
            .stat("Max Items", "3 equipped + inventory")
            .stat("Weight Limit", "Based on strength stat")
            .tip("Better equipment = stronger units")
            .tip("Some items are class-specific")
            .tip("Legendary items provide unique abilities")
            .build()
    }
    pub fn terrain_types() -> EncyclopediaEntry {
        EncyclopediaBuilder::new("Terrain Types")
            .description("Different terrains affect combat and movement:")
            .description("")
            .description("• Grasslands - Open, no bonuses")
            .description("• Forest - Cover, bonus for Elves")
            .description("• Mountain - High ground, bonus for Dwarves")
            .description("• Hills - Elevated, minor defense bonus")
            .description("• Swamp - Difficult, movement penalty")
            .description("• Haunted Woods - Cursed, magical effects")
            .stat("Total Terrain Types", "7")
            .tip("Use terrain to your advantage")
            .tip("High ground provides defensive bonuses")
            .tip("Some races excel in specific terrains")
            .build()
    }
    pub fn unit_class_entry(class_name: &str) -> EncyclopediaEntry {
        match class_name.to_lowercase().as_str() {
            "warrior" => EncyclopediaBuilder::new("Warrior")
                .description("Frontline melee combat specialist")
                .description("Warriors excel at close-quarters combat.")
                .stat("Primary Role", "Tank / Melee DPS")
                .stat("Attack Type", "Slash / Blunt")
                .stat("Range", "Melee (1 hex)")
                .stat("Health", "High")
                .stat("Defense", "High")
                .tip("Position warriors at the front")
                .tip("Use terrain for defensive bonuses")
                .tip("Equip heavy armor for survivability")
                .build(),
            "archer" => EncyclopediaBuilder::new("Archer")
                .description("Long-range precision attacker")
                .description("Archers strike from a distance with deadly accuracy.")
                .stat("Primary Role", "Ranged DPS")
                .stat("Attack Type", "Pierce")
                .stat("Range", "Ranged (3-5 hexes)")
                .stat("Health", "Medium")
                .stat("Defense", "Low")
                .tip("Keep archers behind your frontline")
                .tip("Use high ground for range bonuses")
                .tip("Focus on eliminating enemy mages first")
                .build(),
            "mage" => EncyclopediaBuilder::new("Mage")
                .description("Powerful magic user with area effects")
                .description("Mages deal devastating magical damage.")
                .stat("Primary Role", "Magic DPS")
                .stat("Attack Type", "Fire / Dark")
                .stat("Range", "Medium (2-4 hexes)")
                .stat("Health", "Low")
                .stat("Defense", "Very Low")
                .tip("Protect mages from melee attackers")
                .tip("Mages ignore armor with magic damage")
                .tip("Save mana for critical moments")
                .build(),
            "paladin" => EncyclopediaBuilder::new("Paladin")
                .description("Holy warrior with healing abilities")
                .description("Paladins combine combat prowess with divine magic.")
                .stat("Primary Role", "Tank / Support")
                .stat("Attack Type", "Fire (Holy)")
                .stat("Range", "Melee to Short")
                .stat("Health", "Very High")
                .stat("Defense", "High")
                .tip("Paladins can heal allies")
                .tip("Use holy damage against dark enemies")
                .tip("Position near allies to provide support")
                .build(),
            _ => EncyclopediaBuilder::new("Unknown Class")
                .description("No information available for this class.")
                .build(),
        }
    }
}

pub fn format_entry(entry: &EncyclopediaEntry) -> Vec<String> {
    let mut lines = Vec::new();
    match entry {
        EncyclopediaEntry::Mechanic(mech) => {
            lines.push(format!("╔════════ {} ════════╗", mech.title));
            lines.push("".to_string());
            for desc in mech.description.split('\n') {
                lines.push(desc.to_string());
            }
            if !mech.details.is_empty() {
                lines.push("".to_string());
                for d in &mech.details {
                    lines.push(d.clone());
                }
            }
            lines.push("".to_string());
            lines.push(
                "╚═══════════════════════════════════════════════════════════════════════╝"
                    .to_string(),
            );
            lines.push("".to_string());
        }
        _ => {
            lines.push("[Non-guide entry formatting not implemented]".to_string());
        }
    }
    lines
}

pub fn get_units_content_comprehensive(encyclopedia: &Encyclopedia) -> Vec<String> {
    let mut lines = vec![
        "╔═══════════════════════════════════════════════════════════════════════╗".to_string(),
        "║                        📖 UNIT ENCYCLOPEDIA                            ║".to_string(),
        "╠═══════════════════════════════════════════════════════════════════════╣".to_string(),
        "".to_string(),
    ];
    lines.extend(format_entry(&EncyclopediaLibrary::character_classes()));
    lines.extend(format_entry(&EncyclopediaLibrary::character_races()));
    for class in ["Warrior", "Archer", "Mage", "Paladin"] {
        lines.extend(format_entry(&EncyclopediaLibrary::unit_class_entry(class)));
    }
    lines.push(
        "════════════════════════════════════════════════════════════════════════".to_string(),
    );
    lines.push("ALL REGISTERED UNITS:".to_string());
    for unit in encyclopedia.all_units() {
        lines.push(format!("• {} [{} - {}]", unit.name, unit.race, unit.class));
        lines.push(format!("  {}", unit.description));
        lines.push(format!(
            "  Stats: HP {}/{} | ATK {} | DEF {} | MOV {} | Range {:?}",
            unit.stats.health,
            unit.stats.max_health,
            unit.stats.attack_strength,
            unit.stats.defense,
            unit.stats.movement_speed,
            unit.stats.range_category
        ));
        if !unit.attacks.is_empty() {
            lines.push("  Attacks:".to_string());
            for atk in &unit.attacks {
                lines.push(format!(
                    "    - {} ({} dmg, {} range, {:?})",
                    atk.name, atk.damage, atk.range, atk.damage_type
                ));
            }
        }
        if unit.evolution.previous_form.is_some() || !unit.evolution.next_forms.is_empty() {
            lines.push("  Evolution:".to_string());
            if let Some(prev) = &unit.evolution.previous_form {
                lines.push(format!("    ← Previous: {}", prev.as_str()));
            }
            for next in &unit.evolution.next_forms {
                lines.push(format!("    → Next: {}", next.as_str()));
            }
        }
        lines.push("".to_string());
    }
    lines
}

pub fn get_terrain_content_comprehensive(encyclopedia: &Encyclopedia) -> Vec<String> {
    let mut lines = vec![
        "╔═══════════════════════════════════════════════════════════════════════╗".to_string(),
        "║                       🗺️  TERRAIN GUIDE                                ║".to_string(),
        "╠═══════════════════════════════════════════════════════════════════════╣".to_string(),
        "".to_string(),
    ];
    lines.extend(format_entry(&EncyclopediaLibrary::terrain_types()));
    lines.push(
        "════════════════════════════════════════════════════════════════════════".to_string(),
    );
    lines.push("ALL REGISTERED TERRAINS:".to_string());
    for terrain in encyclopedia.all_terrain() {
        lines.push(format!(
            "• {} (Cost: {})",
            terrain.terrain_type.name(),
            terrain.movement_cost
        ));
        lines.push(format!("  {}", terrain.description));
        lines.push("".to_string());
    }
    lines
}

pub fn get_mechanics_content_comprehensive(encyclopedia: &Encyclopedia) -> Vec<String> {
    let mut lines = vec![
        "╔═══════════════════════════════════════════════════════════════════════╗".to_string(),
        "║                      ⚙️  GAME MECHANICS                                ║".to_string(),
        "╠═══════════════════════════════════════════════════════════════════════╣".to_string(),
        "".to_string(),
    ];
    lines.extend(format_entry(&EncyclopediaLibrary::combat_system()));
    lines.extend(format_entry(&EncyclopediaLibrary::movement_system()));
    lines.extend(format_entry(&EncyclopediaLibrary::equipment_system()));
    lines.push(
        "════════════════════════════════════════════════════════════════════════".to_string(),
    );
    lines.push("ALL REGISTERED MECHANICS:".to_string());
    for mech in encyclopedia.all_mechanics() {
        lines.push(format!("• {} [{}]", mech.title, mech.category));
        lines.push(format!("  {}", mech.description));
        if !mech.details.is_empty() {
            lines.push("  Details:".to_string());
            for d in &mech.details {
                lines.push(format!("    - {}", d));
            }
        }
        if !mech.examples.is_empty() {
            lines.push("  Examples:".to_string());
            for ex in &mech.examples {
                lines.push(format!("    - {}", ex));
            }
        }
        lines.push("".to_string());
    }
    lines
}
