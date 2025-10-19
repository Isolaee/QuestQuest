use crate::base_unit::BaseUnit;
use crate::unit_class::UnitClass;
use crate::unit_race::Race;
use crate::unit_race::Terrain;
use crate::unit_trait::Unit;
use crate::units::*;
use graphics::HexCoord;

/// Factory for creating unit instances based on race and class combinations
pub struct UnitFactory;

impl UnitFactory {
    /// Create a unit based on race and class combination
    /// Returns a boxed trait object that implements Unit
    pub fn create_unit(
        name: String,
        position: HexCoord,
        race: Race,
        class: UnitClass,
        terrain: Terrain,
    ) -> Box<dyn Unit> {
        match (race, class) {
            // Human units
            (Race::Human, UnitClass::Warrior) => {
                Box::new(HumanWarrior::new(name, position, terrain))
            }
            (Race::Human, UnitClass::Archer) => Box::new(HumanArcher::new(name, position, terrain)),
            (Race::Human, UnitClass::Mage) => Box::new(HumanMage::new(name, position, terrain)),

            // Elf units
            (Race::Elf, UnitClass::Warrior) => Box::new(ElfWarrior::new(name, position, terrain)),
            (Race::Elf, UnitClass::Archer) => Box::new(ElfArcher::new(name, position, terrain)),
            (Race::Elf, UnitClass::Mage) => Box::new(ElfMage::new(name, position, terrain)),

            // Dwarf units
            (Race::Dwarf, UnitClass::Warrior) => {
                Box::new(DwarfWarrior::new(name, position, terrain))
            }
            (Race::Dwarf, UnitClass::Archer) => Box::new(DwarfArcher::new(name, position, terrain)),
            (Race::Dwarf, UnitClass::Mage) => Box::new(DwarfMage::new(name, position, terrain)),

            // For other race/class combinations, create a generic unit for now
            _ => Box::new(GenericUnit::new(name, position, race, class, terrain)),
        }
    }
}

/// Temporary generic unit implementation for unsupported race/class combinations
/// This will be replaced by concrete implementations as they are developed
struct GenericUnit {
    base: BaseUnit,
}

impl GenericUnit {
    fn new(
        name: String,
        position: HexCoord,
        race: Race,
        class: UnitClass,
        terrain: Terrain,
    ) -> Self {
        let mut base = BaseUnit::new(name, position, race, class, terrain);
        base.recalculate_stats();
        Self { base }
    }
}

impl Unit for GenericUnit {
    fn id(&self) -> crate::unit_trait::UnitId {
        self.base.id
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn position(&self) -> HexCoord {
        self.base.position
    }

    fn race(&self) -> Race {
        self.base.race
    }

    fn class(&self) -> UnitClass {
        self.base.class
    }

    fn move_to(&mut self, position: HexCoord) -> bool {
        if self.can_move_to(position) {
            self.base.position = position;
            true
        } else {
            false
        }
    }

    fn combat_stats(&self) -> &combat::CombatStats {
        &self.base.combat_stats
    }

    fn combat_stats_mut(&mut self) -> &mut combat::CombatStats {
        &mut self.base.combat_stats
    }

    fn equipment(&self) -> &items::Equipment {
        &self.base.equipment
    }

    fn equipment_mut(&mut self) -> &mut items::Equipment {
        &mut self.base.equipment
    }

    fn inventory(&self) -> &[items::Item] {
        &self.base.inventory
    }

    fn inventory_mut(&mut self) -> &mut Vec<items::Item> {
        &mut self.base.inventory
    }

    fn equip_item(&mut self, item_id: items::ItemId) -> Result<(), String> {
        if let Some(pos) = self
            .base
            .inventory
            .iter()
            .position(|item| item.id == item_id)
        {
            let item = self.base.inventory.remove(pos);
            if let Some(old_item) = self.base.equipment.equip_item(item) {
                self.base.inventory.push(old_item);
            }
            self.recalculate_stats();
            Ok(())
        } else {
            Err("Item not found in inventory".to_string())
        }
    }

    fn unequip_item(&mut self, item_id: items::ItemId) -> Result<(), String> {
        if let Some(item) = self.base.equipment.unequip_item(item_id) {
            self.base.inventory.push(item);
            self.recalculate_stats();
            Ok(())
        } else {
            Err("Item not equipped".to_string())
        }
    }

    fn add_item_to_inventory(&mut self, item: items::Item) {
        self.base.inventory.push(item);
    }

    fn remove_item_from_inventory(&mut self, item_id: items::ItemId) -> Option<items::Item> {
        if let Some(pos) = self
            .base
            .inventory
            .iter()
            .position(|item| item.id == item_id)
        {
            Some(self.base.inventory.remove(pos))
        } else {
            None
        }
    }

    fn level(&self) -> i32 {
        self.base.level
    }

    fn experience(&self) -> i32 {
        self.base.experience
    }

    fn add_experience(&mut self, exp: i32) -> bool {
        self.base.experience += exp.max(0);
        let required_exp = self.base.level * self.base.level * 100;

        if self.base.experience >= required_exp {
            self.base.level += 1;
            self.recalculate_stats();
            self.base.combat_stats.health = self.base.combat_stats.max_health;
            true
        } else {
            false
        }
    }

    fn experience_for_next_level(&self) -> i32 {
        let required = self.base.level * self.base.level * 100;
        (required - self.base.experience).max(0)
    }

    fn level_progress(&self) -> f32 {
        let current_level_exp = (self.base.level - 1) * (self.base.level - 1) * 100;
        let next_level_exp = self.base.level * self.base.level * 100;
        let progress_exp = self.base.experience - current_level_exp;
        let level_exp_range = next_level_exp - current_level_exp;

        if level_exp_range > 0 {
            (progress_exp as f32 / level_exp_range as f32).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    fn current_terrain(&self) -> Terrain {
        self.base.current_terrain
    }

    fn set_terrain(&mut self, terrain: Terrain) {
        self.base.current_terrain = terrain;
    }

    fn is_alive(&self) -> bool {
        self.base.combat_stats.health > 0
    }

    fn can_attack(&self, target_position: HexCoord) -> bool {
        let distance = self.base.position.distance(target_position);
        distance <= self.base.combat_stats.attack_range && distance > 0
    }

    fn can_move_to(&self, target: HexCoord) -> bool {
        let distance = self.base.position.distance(target);
        distance <= self.base.combat_stats.movement_speed
    }

    fn get_movement_range(&self) -> Vec<HexCoord> {
        self.base.get_movement_range()
    }

    fn take_damage(&mut self, damage: u32) {
        self.base.combat_stats.take_damage(damage as i32);
    }

    fn heal(&mut self, amount: i32) {
        self.base.combat_stats.heal(amount);
    }

    fn recalculate_stats(&mut self) {
        self.base.recalculate_stats();
    }

    fn get_display_color(&self) -> [f32; 3] {
        self.base.race.get_display_color()
    }

    fn get_info(&self) -> String {
        format!(
            "{} (Level {} {} {})\nPos: {:?}\nHP: {}/{}\nAtk: {} | Mov: {}\nExp: {} ({:.1}% to next)\nItems: {} equipped, {} in inventory",
            self.base.name,
            self.base.level,
            self.base.race.get_name(),
            self.base.class.get_name(),
            self.base.position,
            self.base.combat_stats.health,
            self.base.combat_stats.max_health,
            self.base.combat_stats.get_total_attack(),
            self.base.combat_stats.movement_speed,
            self.base.experience,
            self.level_progress() * 100.0,
            self.base.equipment.get_all_equipped().len(),
            self.base.inventory.len()
        )
    }

    fn get_summary(&self) -> String {
        format!(
            "{} (L{} {} {})",
            self.base.name,
            self.base.level,
            self.base.race.get_name(),
            self.base.class.get_name()
        )
    }

    fn display_unit_info(&self) {
        println!("\n┌─────────────────────────────────────────────────────┐");
        println!("│                  UNIT DETAILS                       │");
        println!("├─────────────────────────────────────────────────────┤");
        println!("│ Name: {:<45} │", self.base.name);
        println!(
            "│ Level {:<2} {:<8} {:<30} │",
            self.base.level,
            self.base.race.get_name(),
            self.base.class.get_name()
        );
        println!("│ Position: {:?}{:<32} │", self.base.position, "");
        println!("│ Experience: {:<37} │", self.base.experience);
        println!("├─────────────────────────────────────────────────────┤");

        let current_hp = self.base.combat_stats.health;
        let max_hp = self.base.combat_stats.max_health;
        let health_bar = self.base.create_health_bar(current_hp, max_hp, 20);

        println!("│ Health: {}/{} {:<25} │", current_hp, max_hp, health_bar);
        println!("│ Attack: {:<41} │", self.base.cached_attack);
        println!("│ Movement: {:<39} │", self.base.cached_movement);
        println!(
            "│ Range: {} ({}){:<32} │",
            self.base.combat_stats.attack_range,
            match self.base.combat_stats.range_category {
                combat::RangeCategory::Melee => "Melee",
                combat::RangeCategory::Range => "Range",
                combat::RangeCategory::Siege => "Siege",
            },
            ""
        );

        println!("└─────────────────────────────────────────────────────┘\n");
    }

    fn display_quick_info(&self) {
        println!(
            "📋 {} | Lv.{} | HP:{}/{} | ATK:{} | POS:{:?}",
            self.base.name,
            self.base.level,
            self.base.combat_stats.health,
            self.base.combat_stats.max_health,
            self.base.cached_attack,
            self.base.position
        );
    }

    fn on_click(&self) {
        self.display_unit_info();
    }
}
