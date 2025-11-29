// EncyclopediaEntry: unified for all encyclopedia/guide data
pub struct EncyclopediaEntry {
    pub title: String,
    pub description: Vec<String>,
    pub stats: Vec<(String, String)>,
    pub tips: Vec<String>,
}

/// Builder for creating encyclopedia entries with convenience methods
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
        EncyclopediaEntry {
            title: self.title,
            description: self.description,
            stats: self.stats,
            tips: self.tips,
        }
    }
}

/// Pre-built encyclopedia entries for common game elements
pub struct EncyclopediaLibrary;

impl EncyclopediaLibrary {
    /// Encyclopedia entry for the combat system
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

    /// Encyclopedia entry for movement mechanics
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

    /// Encyclopedia entry for character classes
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

    /// Encyclopedia entry for character races
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

    /// Encyclopedia entry for the equipment system
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

    /// Encyclopedia entry for terrain types
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

    /// Get an encyclopedia entry for a specific unit class
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
