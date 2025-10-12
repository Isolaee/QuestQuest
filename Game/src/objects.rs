use graphics::{HexCoord, SpriteType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Base trait for all game objects
pub trait GameObject {
    /// Get the unique identifier for this object
    fn id(&self) -> Uuid;

    fn name(&self) -> String;

    /// Get the current position on the hex grid
    fn position(&self) -> HexCoord;

    /// Set the position on the hex grid
    fn set_position(&mut self, position: HexCoord);

    /// Get the sprite type for rendering
    fn sprite_type(&self) -> SpriteType;

    /// Update the object (called each game tick)
    fn update(&mut self, _delta_time: f32) {}

    /// Handle interaction with another game object
    fn interact(&mut self, _other: &mut dyn GameObject) -> bool {
        false
    }

    /// Check if this object blocks movement
    fn blocks_movement(&self) -> bool {
        false
    }

    /// Get object type name for debugging
    fn type_name(&self) -> &'static str;

    /// Call unit-specific details (only works for GameUnit objects)
    fn show_details(&self) {
        // Default implementation does nothing
        println!("This object has no detailed information.");
    }
}

/// Terrain tile object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainTile {
    id: Uuid,
    position: HexCoord,
    sprite_type: SpriteType,
    movement_cost: f32,
    can_block: bool,
    metadata: HashMap<String, String>,
}

impl TerrainTile {
    /// Create a new terrain tile
    pub fn new(position: HexCoord, sprite_type: SpriteType) -> Self {
        let movement_cost = match sprite_type {
            SpriteType::None => 1.0,
            SpriteType::Grasslands => 1.0,
            SpriteType::Forest => 2.0,
            SpriteType::Forest2 => 1.5,
            SpriteType::Hills => 2.5,
            SpriteType::Mountain => 4.0,
            SpriteType::Swamp => 3.0,
            SpriteType::HauntedWoods => 2.0,
            _ => 1.0, // Default cost
        };

        let can_block = matches!(sprite_type, SpriteType::Mountain);

        Self {
            id: Uuid::new_v4(),
            position,
            sprite_type,
            movement_cost,
            can_block,
            metadata: HashMap::new(),
        }
    }

    /// Get movement cost for this terrain
    pub fn movement_cost(&self) -> f32 {
        self.movement_cost
    }

    /// Set movement cost
    pub fn set_movement_cost(&mut self, cost: f32) {
        self.movement_cost = cost;
    }

    /// Add metadata to this terrain tile
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata from this terrain tile
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

impl GameObject for TerrainTile {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> String {
        format!("{:?}", self.sprite_type)
    }

    fn position(&self) -> HexCoord {
        self.position
    }

    fn set_position(&mut self, position: HexCoord) {
        self.position = position;
    }

    fn sprite_type(&self) -> SpriteType {
        self.sprite_type
    }

    fn blocks_movement(&self) -> bool {
        self.can_block
    }

    fn type_name(&self) -> &'static str {
        "TerrainTile"
    }
}

/// Game unit wrapper that implements GameObject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameUnit {
    id: Uuid,
    unit: units::Unit,
    last_action_time: f32,
    action_cooldown: f32,
}

impl GameUnit {
    /// Create a new game unit from a units::Unit
    pub fn new(unit: units::Unit) -> Self {
        Self {
            id: Uuid::new_v4(),
            unit,
            last_action_time: 0.0,
            action_cooldown: 1.0, // 1 second default cooldown
        }
    }

    /// Get reference to the underlying unit
    pub fn unit(&self) -> &units::Unit {
        &self.unit
    }

    /// Get mutable reference to the underlying unit
    pub fn unit_mut(&mut self) -> &mut units::Unit {
        &mut self.unit
    }

    /// Check if the unit can perform an action
    pub fn can_act(&self, current_time: f32) -> bool {
        current_time - self.last_action_time >= self.action_cooldown
    }

    /// Set the last action time
    pub fn set_last_action_time(&mut self, time: f32) {
        self.last_action_time = time;
    }

    /// Call the underlying unit's on_click method
    pub fn show_unit_details(&self) {
        self.unit.on_click();
    }

    /// Get unit details as string (alternative to console output)
    pub fn get_unit_info_string(&self) -> String {
        // This would return the same info that on_click() prints, but as a string
        format!(
            "Unit: {} (Level {})\nPosition: {:?}\nHealth: {}/{}\nExperience: {}",
            self.unit.name,
            self.unit.level,
            self.unit.position,
            self.unit.combat_stats.health,
            self.unit.combat_stats.max_health,
            self.unit.experience
        )
    }

    /// Get sprite type based on unit class
    fn get_unit_sprite(&self) -> SpriteType {
        // For now, units appear on grasslands
        // TODO: Add unit-specific sprites
        SpriteType::Grasslands
    }
}

impl GameObject for GameUnit {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> String {
        self.unit.name.clone()
    }

    fn position(&self) -> HexCoord {
        self.unit.position
    }

    fn set_position(&mut self, position: HexCoord) {
        self.unit.position = position;
    }

    fn sprite_type(&self) -> SpriteType {
        self.get_unit_sprite()
    }

    fn update(&mut self, delta_time: f32) {
        // Update any time-based mechanics
        self.last_action_time += delta_time;
    }

    fn interact(&mut self, other: &mut dyn GameObject) -> bool {
        // Handle unit-to-unit interactions (combat, etc.)
        if other.type_name() == "GameUnit" {
            // Combat logic could go here
            return true;
        }
        false
    }

    fn blocks_movement(&self) -> bool {
        true // Units block movement
    }

    fn type_name(&self) -> &'static str {
        "GameUnit"
    }

    fn show_details(&self) {
        self.unit.on_click(); // Call the actual Unit's on_click method!
    }
}

/// Interactive game object (items, NPCs, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveObject {
    id: Uuid,
    position: HexCoord,
    sprite_type: SpriteType,
    name: String,
    description: String,
    item: Option<units::Item>,
    blocks_movement: bool,
    interactions_remaining: Option<u32>,
}

impl InteractiveObject {
    /// Create a new item pickup
    pub fn new_item_pickup(position: HexCoord, item: units::Item) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            sprite_type: SpriteType::Grasslands, // Items appear on grasslands for now
            name: item.name.clone(),
            description: item.description.clone(),
            item: Some(item),
            blocks_movement: false,
            interactions_remaining: Some(1), // Single use
        }
    }

    /// Create a new interactive object
    pub fn new(
        position: HexCoord,
        name: String,
        description: String,
        sprite_type: SpriteType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            sprite_type,
            name,
            description,
            item: None,
            blocks_movement: false,
            interactions_remaining: None, // Unlimited interactions
        }
    }

    /// Get the name of this object
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the description of this object
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Take the item from this object (if it has one)
    pub fn take_item(&mut self) -> Option<units::Item> {
        if let Some(ref mut remaining) = self.interactions_remaining {
            if *remaining > 0 {
                *remaining -= 1;
                return self.item.take();
            }
        } else {
            return self.item.clone();
        }
        None
    }

    /// Check if this object can still be interacted with
    pub fn can_interact(&self) -> bool {
        if let Some(remaining) = self.interactions_remaining {
            remaining > 0
        } else {
            true
        }
    }
}

impl GameObject for InteractiveObject {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn position(&self) -> HexCoord {
        self.position
    }

    fn set_position(&mut self, position: HexCoord) {
        self.position = position;
    }

    fn sprite_type(&self) -> SpriteType {
        self.sprite_type
    }

    fn interact(&mut self, other: &mut dyn GameObject) -> bool {
        // Handle interaction with units
        if other.type_name() == "GameUnit" && self.can_interact() {
            if let Some(_item) = self.take_item() {
                // Item was taken
                return true;
            }
        }
        false
    }

    fn blocks_movement(&self) -> bool {
        self.blocks_movement
    }

    fn type_name(&self) -> &'static str {
        "InteractiveObject"
    }
}
