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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
/// ```
/// use game::{TerrainTile, GameObject};
/// use graphics::HexCoord;
/// use graphics::SpriteType;
///
/// let tile = TerrainTile::new(HexCoord::new(0, 0), SpriteType::Grasslands);
/// assert_eq!(tile.position(), HexCoord::new(0, 0));
/// let c = tile.movement_cost();
/// assert!(c >= 1);
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
    pub fn new(position: HexCoord, sprite_type: SpriteType) -> Self {
        // Use canonical movement cost from SpriteType so costs are centralized
        let movement_cost = sprite_type.movement_cost() as f32;

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
    pub fn movement_cost(&self) -> i32 {
        self.movement_cost as i32
    }

    /// Sets the movement cost for this terrain.
    ///
    /// # Arguments
    ///
    /// * `cost` - New movement cost (typically 1.0-4.0 for standard terrain)
    pub fn set_movement_cost(&mut self, cost: i32) {
        self.movement_cost = cost as f32;
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

pub struct GameUnit {
    id: Uuid,
    unit: Box<dyn units::Unit>,
    team: Team,
    last_action_time: f32,
    action_cooldown: f32,
    /// Remaining movement points for the current turn (integer tiles).
    moves_left: i32,
    /// Optional per-unit AI executor used during AI-controlled turns
    pub ai_executor: Option<ai::ActionExecutor>,
    /// Planned AI actions (grounded ActionInstance sequence)
    pub ai_plan: Vec<ai::ActionInstance>,
    /// Long-term strategic goal this unit is pursuing (serialized goal description)
    pub ai_long_term_goal: Option<String>,
    /// How many turns ahead to plan (0 = current turn only, higher = more strategic)
    pub ai_plan_horizon: usize,
}

impl GameUnit {
    pub fn new(unit: Box<dyn units::Unit>) -> Self {
        // Initialize moves_left from the underlying unit's combat stats
        let movement = unit.combat_stats().movement_speed;
        Self {
            id: Uuid::new_v4(),
            unit,
            team: Team::Player, // Default to Player team
            last_action_time: 0.0,
            action_cooldown: 1.0, // 1 second default cooldown
            moves_left: movement,
            ai_executor: None,
            ai_plan: Vec::new(),
            ai_long_term_goal: None,
            ai_plan_horizon: 1, // Default to single-turn planning
        }
    }

    pub fn new_with_team(unit: Box<dyn units::Unit>, team: Team) -> Self {
        let movement = unit.combat_stats().movement_speed;
        Self {
            id: Uuid::new_v4(),
            unit,
            team,
            last_action_time: 0.0,
            action_cooldown: 1.0,
            moves_left: movement,
            ai_executor: None,
            ai_plan: Vec::new(),
            ai_long_term_goal: None,
            ai_plan_horizon: 1, // Default to single-turn planning
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

    /// Sets the UUID of this game unit.
    ///
    /// This is used when replacing a unit with its evolved form to preserve
    /// the same ID throughout the evolution.
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID to assign to this unit
    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
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

    /// Returns the remaining movement points for this unit this turn.
    pub fn moves_left(&self) -> i32 {
        self.moves_left
    }

    /// Sets the remaining movement points for this unit.
    pub fn set_moves_left(&mut self, val: i32) {
        self.moves_left = val.max(0);
    }

    /// Resets the unit's movement points to its maximum movement for the
    /// current stats (usually called at the start of the unit's team's turn).
    pub fn reset_moves_to_max(&mut self) {
        self.moves_left = self.unit.combat_stats().movement_speed;
    }

    /// Attempts to consume movement points. Returns true if the unit had
    /// enough moves and the cost was subtracted, false otherwise.
    pub fn consume_moves(&mut self, cost: i32) -> bool {
        if cost <= 0 {
            return true;
        }
        if self.moves_left >= cost {
            self.moves_left -= cost;
            true
        } else {
            false
        }
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

    /// Sets a long-term goal for this unit.
    ///
    /// # Arguments
    ///
    /// * `goal` - Description of the strategic goal (e.g., "ReachPosition:5,3", "EliminateTarget:unit_id")
    pub fn set_long_term_goal(&mut self, goal: Option<String>) {
        self.ai_long_term_goal = goal;
    }

    /// Gets the current long-term goal for this unit.
    pub fn long_term_goal(&self) -> Option<&String> {
        self.ai_long_term_goal.as_ref()
    }

    /// Sets how many turns ahead this unit should plan.
    ///
    /// # Arguments
    ///
    /// * `horizon` - Number of turns to plan (1 = current turn only, higher = more strategic)
    pub fn set_plan_horizon(&mut self, horizon: usize) {
        self.ai_plan_horizon = horizon;
    }

    /// Gets the planning horizon for this unit.
    pub fn plan_horizon(&self) -> usize {
        self.ai_plan_horizon
    }

    /// Clears the long-term goal and resets to single-turn planning.
    pub fn clear_long_term_goal(&mut self) {
        self.ai_long_term_goal = None;
        self.ai_plan_horizon = 1;
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
    /// ```
    /// use game::InteractiveObject;
    /// use graphics::HexCoord;
    /// use items::item_definitions::create_iron_sword;
    ///
    /// // Use helper from item_definitions to create a concrete Item
    /// let sword = create_iron_sword();
    /// let pickup = InteractiveObject::new_item_pickup(HexCoord::new(5, 3), sword);
    /// assert!(pickup.has_item());
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

    /// Returns true if this interactive object contains an item to pick up.
    pub fn has_item(&self) -> bool {
        self.item.is_some()
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
