use crate::item_properties::RangeType;
use crate::{Item, ItemId, ItemType};
use serde::{Deserialize, Serialize};

/// Equipment slots for a unit
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub accessories: Vec<Item>, // Multiple accessories allowed
}

impl Equipment {
    /// Create new empty equipment
    pub fn new() -> Self {
        Self {
            weapon: None,
            armor: None,
            accessories: Vec::new(),
        }
    }

    /// Equip an item in the appropriate slot
    pub fn equip_item(&mut self, item: Item) -> Option<Item> {
        match item.item_type {
            ItemType::Weapon => {
                let old_weapon = self.weapon.take();
                self.weapon = Some(item);
                old_weapon
            }
            ItemType::Armor => {
                let old_armor = self.armor.take();
                self.armor = Some(item);
                old_armor
            }
            ItemType::Accessory => {
                self.accessories.push(item);
                None
            }
            ItemType::Consumable => None, // Consumables aren't equipped
        }
    }

    /// Unequip an item by ID
    pub fn unequip_item(&mut self, item_id: ItemId) -> Option<Item> {
        if let Some(weapon) = &self.weapon {
            if weapon.id == item_id {
                return self.weapon.take();
            }
        }

        if let Some(armor) = &self.armor {
            if armor.id == item_id {
                return self.armor.take();
            }
        }

        if let Some(pos) = self.accessories.iter().position(|item| item.id == item_id) {
            return Some(self.accessories.remove(pos));
        }

        None
    }

    /// Get total attack bonus from all equipped items
    pub fn get_total_attack_bonus(&self) -> i32 {
        let mut total = 0;

        if let Some(weapon) = &self.weapon {
            total += weapon.get_attack_bonus();
        }

        for accessory in &self.accessories {
            total += accessory.get_attack_bonus();
        }

        total
    }

    /// Get total defense bonus from all equipped items
    pub fn get_total_defense_bonus(&self) -> i32 {
        let mut total = 0;

        if let Some(armor) = &self.armor {
            total += armor.get_defense_bonus();
        }

        for accessory in &self.accessories {
            total += accessory.get_defense_bonus();
        }

        total
    }

    /// Get total movement modifier from all equipped items
    pub fn get_total_movement_modifier(&self) -> i32 {
        let mut total = 0;

        if let Some(armor) = &self.armor {
            total += armor.get_movement_modifier();
        }

        for accessory in &self.accessories {
            total += accessory.get_movement_modifier();
        }

        total
    }

    /// Get total health bonus from all equipped items
    pub fn get_total_health_bonus(&self) -> i32 {
        let mut total = 0;

        for accessory in &self.accessories {
            total += accessory.get_health_bonus();
        }

        total
    }

    /// Get range type override from weapon
    pub fn get_range_type_override(&self) -> Option<RangeType> {
        self.weapon
            .as_ref()
            .and_then(|w| w.get_range_type_override())
    }

    /// Get total range modifier from all equipped items
    pub fn get_total_range_modifier(&self) -> i32 {
        let mut total = 0;

        if let Some(weapon) = &self.weapon {
            total += weapon.get_range_modifier();
        }

        for accessory in &self.accessories {
            total += accessory.get_range_modifier();
        }

        total
    }

    /// Get all equipped items as a vector
    pub fn get_all_equipped(&self) -> Vec<&Item> {
        let mut items = Vec::new();

        if let Some(weapon) = &self.weapon {
            items.push(weapon);
        }

        if let Some(armor) = &self.armor {
            items.push(armor);
        }

        for accessory in &self.accessories {
            items.push(accessory);
        }

        items
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self::new()
    }
}
