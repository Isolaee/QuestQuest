use crate::item_properties::{ItemProperties, RangeType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for items
pub type ItemId = Uuid;

/// Different types of items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
}

/// Represents an item that can be equipped or used by a unit
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub properties: ItemProperties,
}

impl Item {
    /// Create a new item
    pub fn new(name: String, description: String, properties: ItemProperties) -> Self {
        let item_type = match properties {
            ItemProperties::Weapon { .. } => ItemType::Weapon,
            ItemProperties::Armor { .. } => ItemType::Armor,
            ItemProperties::Accessory { .. } => ItemType::Accessory,
            ItemProperties::Consumable { .. } => ItemType::Consumable,
        };

        Self {
            id: Uuid::new_v4(),
            name,
            description,
            item_type,
            properties,
        }
    }

    /// Get the attack bonus this item provides
    pub fn get_attack_bonus(&self) -> i32 {
        match &self.properties {
            ItemProperties::Weapon { attack_bonus, .. } => *attack_bonus,
            ItemProperties::Accessory { attack_bonus, .. } => *attack_bonus,
            _ => 0,
        }
    }

    /// Get the defense bonus this item provides
    pub fn get_defense_bonus(&self) -> i32 {
        match &self.properties {
            ItemProperties::Armor { defense_bonus, .. } => *defense_bonus,
            ItemProperties::Accessory { defense_bonus, .. } => *defense_bonus,
            _ => 0,
        }
    }

    /// Get the movement modifier this item provides (can be negative)
    pub fn get_movement_modifier(&self) -> i32 {
        match &self.properties {
            ItemProperties::Armor {
                movement_penalty, ..
            } => -*movement_penalty,
            ItemProperties::Accessory { movement_bonus, .. } => *movement_bonus,
            _ => 0,
        }
    }

    /// Get the health bonus this item provides
    pub fn get_health_bonus(&self) -> i32 {
        match &self.properties {
            ItemProperties::Accessory { health_bonus, .. } => *health_bonus,
            _ => 0,
        }
    }

    /// Get range type override if this item provides one
    pub fn get_range_type_override(&self) -> Option<RangeType> {
        match &self.properties {
            ItemProperties::Weapon {
                range_type_override,
                ..
            } => *range_type_override,
            _ => None,
        }
    }

    /// Get the range modifier this item provides
    pub fn get_range_modifier(&self) -> i32 {
        match &self.properties {
            ItemProperties::Weapon { range_modifier, .. } => *range_modifier,
            _ => 0,
        }
    }
}
