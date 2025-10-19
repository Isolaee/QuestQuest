use crate::base_unit::BaseUnit;
use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use crate::unit_trait::{Unit, UnitId};
use combat::CombatStats;
use graphics::HexCoord;
use items::{Equipment, Item, ItemId};

pub struct ElfWarrior {
    base: BaseUnit,
    dodge_bonus: u32,
}

impl ElfWarrior {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let base = BaseUnit::new(name, position, Race::Elf, UnitClass::Warrior, terrain);

        Self {
            base,
            dodge_bonus: 10, // Base 10% dodge
        }
    }
}

impl Unit for ElfWarrior {
    fn id(&self) -> UnitId {
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

    fn combat_stats(&self) -> &CombatStats {
        &self.base.combat_stats
    }

    fn combat_stats_mut(&mut self) -> &mut CombatStats {
        &mut self.base.combat_stats
    }

    fn equipment(&self) -> &Equipment {
        &self.base.equipment
    }

    fn equipment_mut(&mut self) -> &mut Equipment {
        &mut self.base.equipment
    }

    fn inventory(&self) -> &[Item] {
        &self.base.inventory
    }

    fn inventory_mut(&mut self) -> &mut Vec<Item> {
        &mut self.base.inventory
    }

    fn equip_item(&mut self, item_id: ItemId) -> Result<(), String> {
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

    fn unequip_item(&mut self, item_id: ItemId) -> Result<(), String> {
        if let Some(item) = self.base.equipment.unequip_item(item_id) {
            self.base.inventory.push(item);
            self.recalculate_stats();
            Ok(())
        } else {
            Err("Item not equipped".to_string())
        }
    }

    fn add_item_to_inventory(&mut self, item: Item) {
        self.base.inventory.push(item);
    }

    fn remove_item_from_inventory(&mut self, item_id: ItemId) -> Option<Item> {
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
            self.dodge_bonus += 1;
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
            "{} (Lv.{} Elf Warrior)\nHP: {}/{}\nATK: {}\nExp: {}/{}",
            self.name(),
            self.level(),
            self.base.combat_stats.health,
            self.base.combat_stats.max_health,
            self.dodge_bonus,
            self.experience(),
            self.experience_for_next_level()
        )
    }

    fn get_summary(&self) -> String {
        format!(
            "{} L{} HP:{}/{}",
            self.name(),
            self.level(),
            self.base.combat_stats.health,
            self.base.combat_stats.max_health
        )
    }

    fn display_unit_info(&self) {
        println!("{}", self.get_info());
    }

    fn display_quick_info(&self) {
        println!("{}", self.get_summary());
    }

    fn on_click(&self) {
        println!("Elf Warrior {}", self.name());
    }
}
