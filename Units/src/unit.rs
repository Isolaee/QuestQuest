use crate::combat::CombatStats;
use crate::item::{Equipment, Item, ItemId};
use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use graphics::HexCoord;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for units
pub type UnitId = Uuid;

/// Represents a game unit with all its properties
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub id: UnitId,
    pub name: String,
    pub position: HexCoord,
    pub race: Race,
    pub class: UnitClass,
    pub experience: i32,
    pub level: i32,

    // Combat and stats
    pub combat_stats: CombatStats,
    pub equipment: Equipment,
    pub inventory: Vec<Item>,

    // Cached values (recalculated when equipment changes)
    cached_defense: i32,
    cached_attack: i32,
    cached_movement: i32,
    cached_max_health: i32,

    pub current_terrain: Terrain, // Add this field
}

impl Unit {
    /// Create a new unit
    pub fn new(
        name: String,
        position: HexCoord,
        race: Race,
        class: UnitClass,
        terrain: Terrain,
    ) -> Self {
        let base_health = class.get_base_health();
        let base_attack = class.get_attack_bonus() + race.get_attack_bonus();
        let base_defense = class.get_defense_bonus();
        let base_movement = class.get_movement_speed() + race.get_movement_bonus();
        let range_type = class.get_default_range();

        let combat_stats = CombatStats::new(
            base_health,
            base_attack,
            base_defense,
            base_movement,
            range_type,
        );

        let mut unit = Self {
            id: Uuid::new_v4(),
            name,
            position,
            race,
            class,
            experience: 0,
            level: 1,
            combat_stats,
            equipment: Equipment::new(),
            inventory: Vec::new(),
            cached_defense: 0,
            cached_attack: 0,
            cached_movement: 0,
            cached_max_health: 0,
            current_terrain: terrain,
        };

        // Calculate initial cached values
        unit.recalculate_stats();
        unit
    }

    /// Create a unit with specific level and experience
    pub fn new_with_level(
        name: String,
        position: HexCoord,
        race: Race,
        class: UnitClass,
        level: i32,
        experience: i32,
    ) -> Self {
        let mut unit = Self::new(name, position, race, class, Terrain::default());
        unit.level = level.max(1);
        unit.experience = experience.max(0);
        unit.recalculate_stats();
        unit
    }

    /// Recalculate all derived stats based on base stats, equipment, and level
    pub fn recalculate_stats(&mut self) {
        // Base stats from race and class
        let base_health = self.class.get_base_health();
        let base_attack = self.class.get_attack_bonus() + self.race.get_attack_bonus();
        let base_defense = self.class.get_defense_bonus();
        let base_movement = self.class.get_movement_speed() + self.race.get_movement_bonus();

        // Level bonuses (each level adds small bonuses)
        let level_health_bonus = (self.level - 1) * 5;
        let level_attack_bonus = (self.level - 1) / 2; // Every 2 levels
        let level_defense_bonus = (self.level - 1) / 3; // Every 3 levels

        // Equipment bonuses
        let equipment_attack = self.equipment.get_total_attack_bonus();
        let equipment_defense = self.equipment.get_total_defense_bonus();
        let equipment_movement = self.equipment.get_total_movement_modifier();
        let equipment_health = self.equipment.get_total_health_bonus();

        // Calculate final stats
        self.cached_attack = base_attack + level_attack_bonus + equipment_attack;
        self.cached_defense = base_defense + level_defense_bonus + equipment_defense;
        self.cached_movement = (base_movement + equipment_movement).max(1); // Minimum 1 movement
        self.cached_max_health = base_health + level_health_bonus + equipment_health;

        // Update combat stats
        let current_health_percentage = self.combat_stats.health_percentage();
        self.combat_stats.attack = self.cached_attack;
        self.combat_stats.defense = self.cached_defense;
        self.combat_stats.movement_speed = self.cached_movement;
        self.combat_stats.max_health = self.cached_max_health;

        // Maintain health percentage when max health changes
        self.combat_stats.health =
            (self.cached_max_health as f32 * current_health_percentage) as i32;

        // Update range type from equipment if overridden
        if let Some(range_override) = self.equipment.get_range_type_override() {
            self.combat_stats.range_type = range_override;
            self.combat_stats.attack_range =
                range_override.get_range_distance() + self.equipment.get_total_range_modifier();
        } else {
            let default_range = self.class.get_default_range();
            self.combat_stats.range_type = default_range;
            self.combat_stats.attack_range =
                default_range.get_range_distance() + self.equipment.get_total_range_modifier();
        }

        // Ensure minimum range of 1
        self.combat_stats.attack_range = self.combat_stats.attack_range.max(1);
    }

    /// Move unit to a new position
    pub fn move_to(&mut self, new_position: HexCoord) {
        self.position = new_position;
    }

    /// Check if unit can move to a target position (within movement range)
    pub fn can_move_to(&self, target: HexCoord) -> bool {
        let distance = self.position.distance(target);
        distance <= self.combat_stats.movement_speed
    }

    /// Get all hexagonal coordinates within movement range
    pub fn get_movement_range(&self) -> Vec<HexCoord> {
        let mut coords = Vec::new();
        let range = self.combat_stats.movement_speed;

        // Iterate through all hexes in a radius around the unit
        for q in -range..=range {
            for r in -range..=range {
                let coord = HexCoord::new(self.position.q + q, self.position.r + r);
                let distance = self.position.distance(coord);

                // Only include hexes within movement range (excluding current position)
                if distance > 0 && distance <= range {
                    coords.push(coord);
                }
            }
        }

        coords
    }

    /// Equip an item from inventory
    pub fn equip_item(&mut self, item_id: ItemId) -> Result<(), String> {
        if let Some(pos) = self.inventory.iter().position(|item| item.id == item_id) {
            let item = self.inventory.remove(pos);
            if let Some(old_item) = self.equipment.equip_item(item) {
                self.inventory.push(old_item);
            }
            self.recalculate_stats();
            Ok(())
        } else {
            Err("Item not found in inventory".to_string())
        }
    }

    /// Unequip an item to inventory
    pub fn unequip_item(&mut self, item_id: ItemId) -> Result<(), String> {
        if let Some(item) = self.equipment.unequip_item(item_id) {
            self.inventory.push(item);
            self.recalculate_stats();
            Ok(())
        } else {
            Err("Item not equipped".to_string())
        }
    }

    /// Add item to inventory
    pub fn add_item_to_inventory(&mut self, item: Item) {
        self.inventory.push(item);
    }

    /// Remove item from inventory
    pub fn remove_item_from_inventory(&mut self, item_id: ItemId) -> Option<Item> {
        if let Some(pos) = self.inventory.iter().position(|item| item.id == item_id) {
            Some(self.inventory.remove(pos))
        } else {
            None
        }
    }

    /// Add experience and handle leveling up
    pub fn add_experience(&mut self, exp: i32) -> bool {
        self.experience += exp.max(0);

        // Calculate required experience for next level (exponential growth)
        let required_exp = self.level * self.level * 100;

        if self.experience >= required_exp {
            self.level_up();
            true
        } else {
            false
        }
    }

    /// Level up the unit
    fn level_up(&mut self) {
        self.level += 1;
        self.recalculate_stats();

        // Heal to full health on level up
        self.combat_stats.health = self.combat_stats.max_health;
    }

    /// Check if unit can attack target position
    pub fn can_attack(&self, target_position: HexCoord) -> bool {
        let distance = self.position.distance(target_position);
        distance <= self.combat_stats.attack_range && distance > 0
    }

    /// Calculate damage this unit would deal to a target
    pub fn calculate_damage_to(&self, target: &Unit) -> i32 {
        self.combat_stats.calculate_damage(&target.combat_stats)
    }

    /// Get the unit's attack power
    pub fn get_attack_power(&self) -> u32 {
        self.cached_attack.max(0) as u32
    }

    /// Take damage
    pub fn take_damage(&mut self, damage: u32) {
        self.combat_stats.take_damage(damage as i32);
    }

    /// Heal the unit
    pub fn heal(&mut self, amount: i32) {
        self.combat_stats.heal(amount);
    }

    /// Check if unit is alive
    pub fn is_alive(&self) -> bool {
        self.combat_stats.health > 0
    }

    /// Get unit's display color based on race
    pub fn get_display_color(&self) -> [f32; 3] {
        self.race.get_display_color()
    }

    /// Get experience required for next level
    pub fn experience_for_next_level(&self) -> i32 {
        let required = self.level * self.level * 100;
        (required - self.experience).max(0)
    }

    /// Get level progress as percentage (0.0 to 1.0)
    pub fn level_progress(&self) -> f32 {
        let current_level_exp = (self.level - 1) * (self.level - 1) * 100;
        let next_level_exp = self.level * self.level * 100;
        let progress_exp = self.experience - current_level_exp;
        let level_exp_range = next_level_exp - current_level_exp;

        if level_exp_range > 0 {
            (progress_exp as f32 / level_exp_range as f32).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    /// Get detailed unit information for display
    pub fn get_info(&self) -> String {
        format!(
            "{} (Level {} {} {})\nPos: {:?}\nHP: {}/{}\nAtk: {} | Def: {} | Mov: {}\nExp: {} ({:.1}% to next)\nItems: {} equipped, {} in inventory",
            self.name,
            self.level,
            self.race.get_name(),
            self.class.get_name(),
            self.position,
            self.combat_stats.health,
            self.combat_stats.max_health,
            self.combat_stats.attack,
            self.combat_stats.defense,
            self.combat_stats.movement_speed,
            self.experience,
            self.level_progress() * 100.0,
            self.equipment.get_all_equipped().len(),
            self.inventory.len()
        )
    }

    /// Get a short summary of the unit
    pub fn get_summary(&self) -> String {
        format!(
            "{} (L{} {} {})",
            self.name,
            self.level,
            self.race.get_name(),
            self.class.get_name()
        )
    }

    /// Handle click event - displays detailed unit information
    pub fn on_click(&self) {
        self.display_unit_info();
    }

    /// Display comprehensive unit information on screen
    pub fn display_unit_info(&self) {
        println!("\n┌─────────────────────────────────────────────────────┐");
        println!("│                  UNIT DETAILS                       │");
        println!("├─────────────────────────────────────────────────────┤");

        // Basic info
        println!("│ Name: {:<45} │", self.name);
        println!(
            "│ Level {:<2} {:<8} {:<30} │",
            self.level,
            self.race.get_name(),
            self.class.get_name()
        );
        println!("│ Position: {:?}{:<32} │", self.position, "");
        println!("│ Experience: {:<37} │", self.experience);

        println!("├─────────────────────────────────────────────────────┤");

        // Combat stats
        let current_hp = self.combat_stats.health;
        let max_hp = self.combat_stats.max_health;
        let health_bar = self.create_health_bar(current_hp, max_hp, 20);

        println!("│ Health: {}/{} {:<25} │", current_hp, max_hp, health_bar);
        println!("│ Attack: {:<41} │", self.cached_attack);
        println!("│ Defense: {:<40} │", self.cached_defense);
        println!("│ Movement: {:<39} │", self.cached_movement);
        println!(
            "│ Range: {} ({}){:<32} │",
            self.combat_stats.attack_range,
            self.combat_stats.range_type.get_name(),
            ""
        );

        println!("├─────────────────────────────────────────────────────┤");

        // Equipment
        println!("│ EQUIPMENT:{:<39} │", "");
        if let Some(weapon) = &self.equipment.weapon {
            println!("│  🗡️  Weapon: {:<35} │", weapon.name);
        } else {
            println!("│  🗡️  Weapon: None{:<31} │", "");
        }

        if let Some(armor) = &self.equipment.armor {
            println!("│  🛡️  Armor: {:<36} │", armor.name);
        } else {
            println!("│  🛡️  Armor: None{:<32} │", "");
        }

        // Show first accessory if any
        if !self.equipment.accessories.is_empty() {
            println!(
                "│  💍  Accessories: {:<27} │",
                self.equipment.accessories[0].name
            );
            if self.equipment.accessories.len() > 1 {
                println!(
                    "│      (+{} more){:<32} │",
                    self.equipment.accessories.len() - 1,
                    ""
                );
            }
        } else {
            println!("│  💍  Accessories: None{:<24} │", "");
        }

        println!("├─────────────────────────────────────────────────────┤");

        // Inventory
        println!("│ INVENTORY ({} items):{:<27} │", self.inventory.len(), "");
        if self.inventory.is_empty() {
            println!("│  (Empty){:<41} │", "");
        } else {
            for (i, item) in self.inventory.iter().take(3).enumerate() {
                println!("│  {}. {:<43} │", i + 1, item.name);
            }
            if self.inventory.len() > 3 {
                println!(
                    "│  ... and {} more item(s){:<22} │",
                    self.inventory.len() - 3,
                    ""
                );
            }
        }

        println!("└─────────────────────────────────────────────────────┘");

        // Additional status info
        if current_hp < max_hp / 4 {
            println!("⚠️  CRITICAL HEALTH - Unit needs healing!");
        } else if current_hp < max_hp / 2 {
            println!("🔸 Low health - Consider healing soon");
        }

        // Check if unit has enough experience to level up (simple check)
        let exp_needed_for_next_level = self.level * 100; // Simple progression
        if self.experience >= exp_needed_for_next_level {
            println!("🌟 Ready to level up!");
        }

        println!(); // Extra spacing
    }

    /// Create a visual health bar
    fn create_health_bar(&self, current: i32, max: i32, width: usize) -> String {
        if max == 0 {
            return "░".repeat(width);
        }

        let filled = ((current as f32 / max as f32) * width as f32) as usize;
        let filled = filled.min(width);

        let bar_char = if current as f32 / max as f32 > 0.75 {
            "█" // Full health - green
        } else if current as f32 / max as f32 > 0.5 {
            "▓" // Medium health - yellow
        } else if current as f32 / max as f32 > 0.25 {
            "▒" // Low health - orange
        } else {
            "░" // Critical health - red
        };

        let filled_part = bar_char.repeat(filled);
        let empty_part = "░".repeat(width - filled);

        format!("[{}{}]", filled_part, empty_part)
    }

    /// Display a compact unit summary for quick reference
    pub fn display_quick_info(&self) {
        println!(
            "📋 {} | Lv.{} | HP:{}/{} | ATK:{} | DEF:{} | POS:{:?}",
            self.name,
            self.level,
            self.combat_stats.health,
            self.combat_stats.max_health,
            self.cached_attack,
            self.cached_defense,
            self.position
        );
    }

    /// Update the unit's current terrain
    pub fn set_terrain(&mut self, terrain: Terrain) {
        self.current_terrain = terrain;
    }

    /// Get the unit's current terrain
    pub fn get_terrain(&self) -> Terrain {
        self.current_terrain
    }

    /// Get the unit's defense value based on current terrain
    pub fn get_defense(&self) -> u8 {
        self.race.get_base_defense(self.current_terrain)
    }
}

/// Unit creation builder for easier unit construction
pub struct UnitBuilder {
    name: String,
    position: HexCoord,
    race: Race,
    class: UnitClass,
    level: Option<i32>,
    experience: Option<i32>,
    equipment: Vec<Item>,
    inventory: Vec<Item>,
}

impl UnitBuilder {
    /// Create a new unit builder
    pub fn new(name: impl Into<String>, position: HexCoord, race: Race, class: UnitClass) -> Self {
        Self {
            name: name.into(),
            position,
            race,
            class,
            level: None,
            experience: None,
            equipment: Vec::new(),
            inventory: Vec::new(),
        }
    }

    /// Set the unit's level
    pub fn with_level(mut self, level: i32) -> Self {
        self.level = Some(level);
        self
    }

    /// Set the unit's experience
    pub fn with_experience(mut self, experience: i32) -> Self {
        self.experience = Some(experience);
        self
    }

    /// Add equipment to the unit
    pub fn with_equipment(mut self, item: Item) -> Self {
        self.equipment.push(item);
        self
    }

    /// Add inventory items to the unit
    pub fn with_inventory_item(mut self, item: Item) -> Self {
        self.inventory.push(item);
        self
    }

    /// Build the unit
    pub fn build(self) -> Unit {
        let mut unit = if let (Some(level), Some(exp)) = (self.level, self.experience) {
            Unit::new_with_level(self.name, self.position, self.race, self.class, level, exp)
        } else {
            Unit::new(
                self.name,
                self.position,
                self.race,
                self.class,
                Terrain::default(),
            )
        };

        // Add equipment and inventory
        for item in self.equipment {
            let item_id = item.id;
            unit.add_item_to_inventory(item);
            let _ = unit.equip_item(item_id); // Ignore errors for builder
        }

        for item in self.inventory {
            unit.add_item_to_inventory(item);
        }

        unit
    }
}
