use crate::objects::*;
use graphics::{HexCoord, SpriteType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Game world that manages all game objects
#[derive(Debug, Serialize, Deserialize)]
pub struct GameWorld {
    /// All terrain tiles in the world
    pub terrain: HashMap<HexCoord, TerrainTile>,

    /// All units in the world
    pub units: HashMap<Uuid, GameUnit>,

    /// All interactive objects in the world
    pub interactive_objects: HashMap<Uuid, InteractiveObject>,

    /// World size (radius from center)
    world_radius: i32,

    /// Current game time
    pub game_time: f32,
    // Removed objects field: trait objects cannot be serialized or debugged directly
}

impl GameWorld {
    /// Create a new empty game world
    pub fn new(world_radius: i32) -> Self {
        Self {
            terrain: HashMap::new(),
            units: HashMap::new(),
            interactive_objects: HashMap::new(),
            world_radius,
            game_time: 0.0,
        }
    }

    /// Generate terrain for the world using the sprite system
    pub fn generate_terrain(&mut self) {
        for q in -self.world_radius..=self.world_radius {
            for r in -self.world_radius..=self.world_radius {
                // Skip hexes that are too far from center
                let coord = HexCoord::new(q, r);
                if coord.distance(HexCoord::new(0, 0)) <= self.world_radius {
                    // Generate terrain based on coordinate
                    let sprite_type = self.generate_terrain_type(coord);
                    let terrain = TerrainTile::new(coord, sprite_type);
                    self.terrain.insert(coord, terrain);
                }
            }
        }
    }

    /// Generate terrain type based on position
    fn generate_terrain_type(&self, coord: HexCoord) -> SpriteType {
        // Use coordinate-based seeding for consistent terrain generation
        let seed = coord.q * 73 + coord.r * 37 + coord.q * coord.r * 17;
        SpriteType::random_terrain(seed)
    }

    /// Add a unit to the world
    pub fn add_unit(&mut self, unit: GameUnit) -> Uuid {
        let id = unit.id();
        self.units.insert(id, unit);
        id
    }

    /// Remove a unit from the world
    pub fn remove_unit(&mut self, id: Uuid) -> Option<GameUnit> {
        self.units.remove(&id)
    }

    /// Get a unit by ID
    pub fn get_unit(&self, id: Uuid) -> Option<&GameUnit> {
        self.units.get(&id)
    }

    /// Get a mutable reference to a unit by ID
    pub fn get_unit_mut(&mut self, id: Uuid) -> Option<&mut GameUnit> {
        self.units.get_mut(&id)
    }

    /// Get all units
    pub fn units(&self) -> &HashMap<Uuid, GameUnit> {
        &self.units
    }

    /// Get all units at a specific position
    pub fn get_units_at_position(&self, position: HexCoord) -> Vec<&GameUnit> {
        self.units
            .values()
            .filter(|unit| unit.position() == position)
            .collect()
    }

    /// Add an interactive object to the world
    pub fn add_interactive_object(&mut self, object: InteractiveObject) -> Uuid {
        let id = object.id();
        self.interactive_objects.insert(id, object);
        id
    }

    /// Remove an interactive object from the world
    pub fn remove_interactive_object(&mut self, id: Uuid) -> Option<InteractiveObject> {
        self.interactive_objects.remove(&id)
    }

    /// Get an interactive object by ID
    pub fn get_interactive_object(&self, id: Uuid) -> Option<&InteractiveObject> {
        self.interactive_objects.get(&id)
    }

    /// Get a mutable reference to an interactive object by ID
    pub fn get_interactive_object_mut(&mut self, id: Uuid) -> Option<&mut InteractiveObject> {
        self.interactive_objects.get_mut(&id)
    }

    /// Get all interactive objects
    pub fn interactive_objects(&self) -> &HashMap<Uuid, InteractiveObject> {
        &self.interactive_objects
    }

    /// Get terrain at a specific position
    pub fn get_terrain(&self, position: HexCoord) -> Option<&TerrainTile> {
        self.terrain.get(&position)
    }

    /// Get mutable terrain at a specific position
    pub fn get_terrain_mut(&mut self, position: HexCoord) -> Option<&mut TerrainTile> {
        self.terrain.get_mut(&position)
    }

    /// Get all terrain tiles
    pub fn terrain(&self) -> &HashMap<HexCoord, TerrainTile> {
        &self.terrain
    }

    /// Check if a position is valid for movement
    pub fn is_position_valid_for_movement(
        &self,
        position: HexCoord,
        unit_id: Option<Uuid>,
    ) -> bool {
        // Check if position is within world bounds
        if position.distance(HexCoord::new(0, 0)) > self.world_radius {
            return false;
        }

        // Check terrain blocking
        if let Some(terrain) = self.get_terrain(position) {
            if terrain.blocks_movement() {
                return false;
            }
        }

        // Check if another unit is blocking (except the moving unit itself)
        for unit in self.units.values() {
            if unit.position() == position {
                if let Some(moving_unit_id) = unit_id {
                    if unit.id() != moving_unit_id {
                        return false; // Another unit is blocking
                    }
                } else {
                    return false; // A unit is blocking and no exception
                }
            }
        }

        // Check interactive objects that block movement
        for object in self.interactive_objects.values() {
            if object.position() == position && object.blocks_movement() {
                return false;
            }
        }

        true
    }

    /// Move a unit to a new position (with validation)
    pub fn move_unit(&mut self, unit_id: Uuid, new_position: HexCoord) -> Result<(), String> {
        if !self.is_position_valid_for_movement(new_position, Some(unit_id)) {
            return Err("Position is blocked or invalid".to_string());
        }

        if let Some(unit) = self.units.get_mut(&unit_id) {
            unit.set_position(new_position);
            Ok(())
        } else {
            Err("Unit not found".to_string())
        }
    }

    /// Get movement cost for a position
    pub fn get_movement_cost(&self, position: HexCoord) -> f32 {
        if let Some(terrain) = self.get_terrain(position) {
            terrain.movement_cost()
        } else {
            f32::INFINITY // Invalid terrain has infinite cost
        }
    }

    /// Update the world (called each frame)
    pub fn update(&mut self, delta_time: f32) {
        self.game_time += delta_time;

        // Update all units
        for unit in self.units.values_mut() {
            unit.update(delta_time);
        }

        // Handle interactions between objects at the same position
        self.process_interactions();
    }

    /// Process interactions between objects at the same positions
    fn process_interactions(&mut self) {
        let mut interactions_to_process = Vec::new();

        // Find all positions with multiple objects
        for unit in self.units.values() {
            let position = unit.position();

            // Check for interactive objects at the same position
            for (obj_id, object) in &self.interactive_objects {
                if object.position() == position {
                    interactions_to_process.push((unit.id(), *obj_id));
                }
            }
        }

        // Process the interactions
        for (unit_id, obj_id) in interactions_to_process {
            if let (Some(unit), Some(object)) = (
                self.units.get_mut(&unit_id),
                self.interactive_objects.get_mut(&obj_id),
            ) {
                // Try to interact
                let interacted = object.interact(unit);
                if interacted && !object.can_interact() {
                    // Remove the object if it can no longer be interacted with
                    self.interactive_objects.remove(&obj_id);
                }
            }
        }
    }

    /// Get current game time
    pub fn game_time(&self) -> f32 {
        self.game_time
    }

    /// Get world radius
    pub fn world_radius(&self) -> i32 {
        self.world_radius
    }

    // Removed add_object and get_objects_at_position methods: trait objects cannot be serialized or debugged directly
}

impl Default for GameWorld {
    fn default() -> Self {
        Self::new(10) // Default world radius of 10
    }
}
