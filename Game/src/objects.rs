//! # Game Objects Module
//!
//! This module defines the core object system for the game world, including the base
//! `GameObject` trait and concrete implementations for terrain, units, and interactive objects.
//!
//! ## Object Types
//!
//! - **TerrainTile**: Represents terrain hexes with movement costs and properties
//! - **GameUnit**: Wrapper around unit trait objects with team affiliation and game logic
//! - **InteractiveObject**: Items, pickups, and other interactable world objects
//!
//! ## Design Philosophy
//!
//! The `GameObject` trait provides a unified interface for all entities in the game world,
//! enabling polymorphic behavior and consistent position/rendering management.

use graphics::{HexCoord, SpriteType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Team affiliation for units in the game world.
///
/// Determines friend-or-foe relationships for combat and movement validation.
/// Units of the same team cannot move onto each other's positions, while
/// units of different teams can engage in combat.
///
/// # Examples
///
/// ```
/// use game::Team;
///
/// let player_team = Team::Player;
/// let enemy_team = Team::Enemy;
/// assert_ne!(player_team, enemy_team);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Team {
    /// Player-controlled units
    Player,
    /// Enemy-controlled units that can be attacked
    Enemy,
    /// Neutral units that don't participate in combat
    Neutral,
}

/// Base trait for all game objects in the world.
///
/// `GameObject` provides a unified interface for all entities that exist in the game world,
/// including terrain, units, and interactive objects. This enables polymorphic storage and
/// consistent handling of position, rendering, and interaction logic.
///
/// # Required Methods
///
/// Implementors must provide:
/// - Unique identification via UUID
/// - Position management on the hex grid
/// - Sprite type for rendering
/// - Type name for debugging and type checking
///
/// # Optional Methods
///
/// Default implementations are provided for:
/// - `update()`: Per-frame update logic
/// - `interact()`: Object-to-object interaction handling
/// - `blocks_movement()`: Whether this object prevents movement
/// - `show_details()`: Display detailed information to the player
///
/// # Examples
///
/// ```ignore
/// use game::{GameObject, TerrainTile};
/// use graphics::{HexCoord, SpriteType};
///
/// let tile = TerrainTile::new(HexCoord::new(0, 0), SpriteType::Grasslands);
/// println!("Tile at: {:?}", tile.position());
/// println!("Movement cost: {}", tile.movement_cost());
/// ```
pub trait GameObject {
    /// Returns the unique identifier for this object.
    ///
    /// Each game object has a UUID that remains constant throughout its lifetime.
    fn id(&self) -> Uuid;

    /// Returns the display name of this object.
    fn name(&self) -> String;

    /// Returns the current position on the hex grid.
    fn position(&self) -> HexCoord;

    /// Sets the position on the hex grid.
    ///
    /// # Arguments
    ///
    /// * `position` - The new hex coordinate for this object
    fn set_position(&mut self, position: HexCoord);

    /// Returns the sprite type for rendering this object.
    fn sprite_type(&self) -> SpriteType;

    /// Updates the object state (called each game tick).
    ///
    /// # Arguments
    ///
    /// * `_delta_time` - Time elapsed since last update in seconds
    fn update(&mut self, _delta_time: f32) {}

    /// Handles interaction with another game object.
    ///
    /// Called when two objects occupy the same position or attempt to interact.
    ///
    /// # Arguments
    ///
    /// * `_other` - The other game object to interact with
    ///
    /// # Returns
    ///
    /// `true` if the interaction was successful and should be processed, `false` otherwise
    fn interact(&mut self, _other: &mut dyn GameObject) -> bool {
        false
    }

    /// Returns whether this object blocks movement.
    ///
    /// Objects that block movement prevent other objects from occupying the same position.
    fn blocks_movement(&self) -> bool {
        false
    }

    /// Returns the type name for debugging and runtime type checking.
    fn type_name(&self) -> &'static str;

    /// Displays detailed information about this object.
    ///
    /// Primarily used for GameUnit objects to show stats and equipment.
    /// Default implementation does nothing.
    fn show_details(&self) {
        // Default implementation does nothing
        println!("This object has no detailed information.");
    }
}

/// Represents a terrain tile on the hex grid.
///
/// Terrain tiles define the base layer of the game world, providing movement costs,
/// blocking behavior, and visual appearance for each hex. Terrain properties affect
/// pathfinding and tactical positioning during gameplay.
///
/// # Examples
///
/// ```
/// use game::TerrainTile;
/// use graphics::{HexCoord, SpriteType};
///
/// let position = HexCoord::new(5, -3);
/// let mut tile = TerrainTile::new(position, SpriteType::Forest);
///
/// // Check movement cost
/// assert_eq!(tile.movement_cost(), 2.0);
///
/// // Add metadata
/// tile.set_metadata("description".to_string(), "Dense forest".to_string());
/// ```
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
    /// Creates a new terrain tile at the specified position.
    ///
    /// Movement cost and blocking behavior are automatically determined based on
    /// the sprite type:
    /// - Grasslands: 1.0 movement cost
    /// - Forest: 2.0 movement cost
    /// - Hills: 2.5 movement cost
    /// - Mountain: 4.0 movement cost (blocks movement)
    /// - Swamp: 3.0 movement cost
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate for this tile
    /// * `sprite_type` - Visual appearance and terrain type
    ///
    /// # Examples
    ///
    /// ```
    /// use game::TerrainTile;
    /// use graphics::{HexCoord, SpriteType};
    ///
    /// let tile = TerrainTile::new(HexCoord::new(0, 0), SpriteType::Grasslands);
    /// ```
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

    /// Returns the movement cost for this terrain.
    ///
    /// Movement cost affects pathfinding and determines how many action points
    /// are required to move through this tile.
    pub fn movement_cost(&self) -> f32 {
        self.movement_cost
    }

    /// Sets the movement cost for this terrain.
    ///
    /// # Arguments
    ///
    /// * `cost` - New movement cost (typically 1.0-4.0 for standard terrain)
    pub fn set_movement_cost(&mut self, cost: f32) {
        self.movement_cost = cost;
    }

    /// Adds or updates metadata for this terrain tile.
    ///
    /// Metadata can store arbitrary key-value pairs for game-specific data like
    /// resource availability, special properties, or quest-related information.
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Retrieves metadata from this terrain tile.
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key to look up
    ///
    /// # Returns
    ///
    /// `Some(&String)` if the key exists, `None` otherwise
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

/// Wrapper around a Unit trait object that integrates it into the game world.
///
/// `GameUnit` provides game-level functionality on top of the core Unit trait,
/// including team affiliation, action cooldowns, and GameObject trait implementation.
/// It cannot be serialized due to containing trait objects.
///
/// # Examples
///
/// ```ignore
/// use game::{GameUnit, Team};
/// use units::warrior::Warrior;
/// use graphics::HexCoord;
///
/// let warrior = Warrior::new("Ragnar".to_string(), HexCoord::new(0, 0));
/// let mut game_unit = GameUnit::new(Box::new(warrior));
/// game_unit.set_team(Team::Player);
///
/// println!("Unit: {}", game_unit.name());
/// println!("Team: {:?}", game_unit.team());
/// ```
pub struct GameUnit {
    id: Uuid,
    unit: Box<dyn units::Unit>,
    team: Team,
    last_action_time: f32,
    action_cooldown: f32,
}

impl GameUnit {
    /// Creates a new game unit from a boxed Unit trait object.
    ///
    /// The unit is assigned to the Player team by default and given a unique UUID.
    ///
    /// # Arguments
    ///
    /// * `unit` - Boxed trait object implementing the Unit trait
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use game::GameUnit;
    /// use units::warrior::Warrior;
    /// use graphics::HexCoord;
    ///
    /// let warrior = Warrior::new("Bjorn".to_string(), HexCoord::new(0, 0));
    /// let game_unit = GameUnit::new(Box::new(warrior));
    /// ```
    pub fn new(unit: Box<dyn units::Unit>) -> Self {
        Self {
            id: Uuid::new_v4(),
            unit,
            team: Team::Player, // Default to Player team
            last_action_time: 0.0,
            action_cooldown: 1.0, // 1 second default cooldown
        }
    }

    /// Creates a new game unit with the specified team affiliation.
    ///
    /// # Arguments
    ///
    /// * `unit` - Boxed trait object implementing the Unit trait
    /// * `team` - Team affiliation (Player, Enemy, or Neutral)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use game::{GameUnit, Team};
    /// use units::warrior::Warrior;
    /// use graphics::HexCoord;
    ///
    /// let warrior = Warrior::new("Enemy Guard".to_string(), HexCoord::new(5, 5));
    /// let enemy_unit = GameUnit::new_with_team(Box::new(warrior), Team::Enemy);
    /// ```
    pub fn new_with_team(unit: Box<dyn units::Unit>, team: Team) -> Self {
        Self {
            id: Uuid::new_v4(),
            unit,
            team,
            last_action_time: 0.0,
            action_cooldown: 1.0,
        }
    }

    /// Returns the team affiliation of this unit.
    pub fn team(&self) -> Team {
        self.team
    }

    /// Sets the team affiliation of this unit.
    ///
    /// # Arguments
    ///
    /// * `team` - New team affiliation
    pub fn set_team(&mut self, team: Team) {
        self.team = team;
    }

    /// Returns a reference to the underlying Unit trait object.
    ///
    /// This allows access to unit-specific methods and properties without
    /// exposing the concrete type.
    pub fn unit(&self) -> &dyn units::Unit {
        &*self.unit
    }

    /// Returns a mutable reference to the underlying Unit trait object.
    ///
    /// Allows modification of unit state, including taking damage, gaining
    /// experience, and equipment changes.
    pub fn unit_mut(&mut self) -> &mut dyn units::Unit {
        &mut *self.unit
    }

    /// Checks if the unit can perform an action based on cooldown.
    ///
    /// # Arguments
    ///
    /// * `current_time` - Current game time in seconds
    ///
    /// # Returns
    ///
    /// `true` if enough time has elapsed since the last action, `false` otherwise
    pub fn can_act(&self, current_time: f32) -> bool {
        current_time - self.last_action_time >= self.action_cooldown
    }

    /// Records the time of the last action for cooldown tracking.
    ///
    /// # Arguments
    ///
    /// * `time` - Game time when the action occurred
    pub fn set_last_action_time(&mut self, time: f32) {
        self.last_action_time = time;
    }

    /// Displays detailed unit information to the console.
    ///
    /// Calls the underlying unit's `on_click` method to show stats,
    /// equipment, and other relevant information.
    pub fn show_unit_details(&self) {
        self.unit.on_click();
    }

    /// Returns unit details as a formatted string.
    ///
    /// Provides an alternative to console output for displaying unit information
    /// in UI elements or logs.
    ///
    /// # Returns
    ///
    /// A formatted string containing unit name, level, position, health, and experience
    pub fn get_unit_info_string(&self) -> String {
        let stats = self.unit.combat_stats();
        format!(
            "Unit: {} (Level {})\nPosition: {:?}\nHealth: {}/{}\nExperience: {}",
            self.unit.name(),
            self.unit.level(),
            self.unit.position(),
            stats.health,
            stats.max_health,
            self.unit.experience()
        )
    }

    /// Returns the sprite type for rendering this unit.
    ///
    /// Currently returns a placeholder (Grasslands). Future implementations
    /// will return unit-specific sprites based on class, race, and equipment.
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
        self.unit.name().to_string()
    }

    fn position(&self) -> HexCoord {
        self.unit.position()
    }

    fn set_position(&mut self, position: HexCoord) {
        self.unit.move_to(position);
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

/// Represents an interactive object in the game world.
///
/// Interactive objects include item pickups, quest objects, NPCs, and other
/// entities that players can interact with. They can be configured to provide
/// items, have limited interactions, and optionally block movement.
///
/// # Examples
///
/// ```
/// use game::InteractiveObject;
/// use graphics::{HexCoord, SpriteType};
///
/// // Create a generic interactive object
/// let chest = InteractiveObject::new(
///     HexCoord::new(3, 2),
///     "Treasure Chest".to_string(),
///     "A wooden chest with rusty hinges".to_string(),
///     SpriteType::Grasslands,
/// );
/// ```
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
    /// Creates a new item pickup at the specified position.
    ///
    /// Item pickups are single-use interactive objects that provide an item
    /// when picked up. They don't block movement and are automatically removed
    /// after interaction.
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate for the pickup
    /// * `item` - The item to be picked up
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use game::InteractiveObject;
    /// use graphics::HexCoord;
    /// use units::Item;
    ///
    /// let sword = Item { /* ... */ };
    /// let pickup = InteractiveObject::new_item_pickup(HexCoord::new(5, 3), sword);
    /// ```
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

    /// Creates a new interactive object with custom properties.
    ///
    /// This constructor provides full control over the object's properties,
    /// allowing creation of quest objects, NPCs, or other custom interactables
    /// that don't necessarily provide items.
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate for the object
    /// * `name` - Display name
    /// * `description` - Detailed description shown on interaction
    /// * `sprite_type` - Visual appearance
    ///
    /// # Examples
    ///
    /// ```
    /// use game::InteractiveObject;
    /// use graphics::{HexCoord, SpriteType};
    ///
    /// let shrine = InteractiveObject::new(
    ///     HexCoord::new(0, 0),
    ///     "Ancient Shrine".to_string(),
    ///     "A mysterious shrine radiating power".to_string(),
    ///     SpriteType::Grasslands,
    /// );
    /// ```
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

    /// Returns the name of this object.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description of this object.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Attempts to take the item from this object.
    ///
    /// If the object has remaining interactions and contains an item, the item
    /// is returned and the interaction count is decremented. For objects with
    /// unlimited interactions, the item is cloned each time.
    ///
    /// # Returns
    ///
    /// `Some(Item)` if an item was successfully taken, `None` otherwise
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

    /// Checks if this object can still be interacted with.
    ///
    /// Objects with limited interactions become unavailable once all interactions
    /// are used. Objects with unlimited interactions (None) are always available.
    ///
    /// # Returns
    ///
    /// `true` if the object can be interacted with, `false` otherwise
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
