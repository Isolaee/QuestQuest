use crate::objects::*;
use graphics::{HexCoord, SpriteType};
use std::collections::HashMap;
use uuid::Uuid;

/// Game world that manages all game objects
/// Note: Cannot derive Serialize/Deserialize because GameUnit contains trait objects
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

        // Check if another unit is blocking
        // Allow movement onto enemy units (for combat), but not allied units
        for unit in self.units.values() {
            if unit.position() == position {
                if let Some(moving_unit_id) = unit_id {
                    if unit.id() != moving_unit_id {
                        // There's another unit at this position
                        // Check if it's an enemy (allow) or ally (block)
                        if let Some(moving_unit) = self.units.get(&moving_unit_id) {
                            if moving_unit.team() == unit.team() {
                                // Same team - block movement
                                return false;
                            }
                            // Different team - allow movement (combat will be initiated)
                        } else {
                            // Moving unit not found - block to be safe
                            return false;
                        }
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
    /// Returns Ok(()) for normal movement, or a combat result message
    pub fn move_unit(&mut self, unit_id: Uuid, new_position: HexCoord) -> Result<(), String> {
        // Check if there's an enemy unit at the target position
        let units_at_target: Vec<Uuid> = self
            .get_units_at_position(new_position)
            .iter()
            .map(|u| u.id())
            .collect();

        let target_unit_id = units_at_target.first().copied();

        if let Some(target_id) = target_unit_id {
            // There's a unit at the target position - check if it's an enemy
            let moving_unit_team = self.units.get(&unit_id).map(|u| u.team());
            let target_unit_team = self.units.get(&target_id).map(|u| u.team());

            if let (Some(mover_team), Some(target_team)) = (moving_unit_team, target_unit_team) {
                if mover_team != target_team {
                    // Enemy units - initiate combat!
                    return self.initiate_combat(unit_id, target_id);
                } else {
                    // Same team - can't move there
                    return Err("Cannot move onto allied unit".to_string());
                }
            }
        }

        // No enemy at target - check normal movement validation
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

    /// Initiate combat between two units
    fn initiate_combat(&mut self, attacker_id: Uuid, defender_id: Uuid) -> Result<(), String> {
        // Get unit info before combat for reporting
        let (attacker_name, defender_name, defender_pos) = {
            let attacker = self.units.get(&attacker_id).ok_or("Attacker not found")?;
            let defender = self.units.get(&defender_id).ok_or("Defender not found")?;
            (attacker.name(), defender.name(), defender.position())
        };

        println!("⚔️  COMBAT INITIATED!");
        println!("⚔️  {} attacks {}!", attacker_name, defender_name);

        // Get combat stats (clone to get owned copies)
        let (mut attacker_stats, mut defender_stats, attacker_damage_type, defender_damage_type) = {
            let attacker = self.units.get(&attacker_id).ok_or("Attacker not found")?;
            let defender = self.units.get(&defender_id).ok_or("Defender not found")?;
            (
                attacker.unit().combat_stats().clone(),
                defender.unit().combat_stats().clone(),
                attacker.unit().class().get_default_damage_type(),
                defender.unit().class().get_default_damage_type(),
            )
        };

        // Resolve attacker's attack
        let attack_result = combat::resolve_combat(
            &mut attacker_stats,
            &mut defender_stats,
            attacker_damage_type,
        );
        let attacker_damage = attack_result.attacker_damage_dealt;

        // Apply damage to defender
        if let Some(defender) = self.units.get_mut(&defender_id) {
            defender.unit_mut().take_damage(attacker_damage);
        }

        println!("⚔️  {} dealt {} damage", attacker_name, attacker_damage);

        // Check if defender is still alive for counter-attack
        let defender_alive = self
            .units
            .get(&defender_id)
            .map(|u| u.unit().combat_stats().health > 0)
            .unwrap_or(false);

        if defender_alive {
            // Defender counter-attacks
            let counter_result = combat::resolve_combat(
                &mut defender_stats,
                &mut attacker_stats,
                defender_damage_type,
            );
            let damage = counter_result.attacker_damage_dealt;

            // Apply counter-attack damage to attacker
            if let Some(attacker) = self.units.get_mut(&attacker_id) {
                attacker.unit_mut().take_damage(damage);
            }

            println!(
                "🛡️  {} dealt {} damage (counter-attack)",
                defender_name, damage
            );
        }

        // Check final status and remove defeated units
        let defender_still_alive = self
            .units
            .get(&defender_id)
            .map(|u| u.unit().combat_stats().health > 0)
            .unwrap_or(false);

        if !defender_still_alive {
            println!("💀 {} has been defeated!", defender_name);
            self.units.remove(&defender_id);

            // Move attacker to defender's position
            if let Some(attacker) = self.units.get_mut(&attacker_id) {
                attacker.set_position(defender_pos);
                println!(
                    "⚔️  {} moves to ({}, {})",
                    attacker_name, defender_pos.q, defender_pos.r
                );
            }
        }

        let attacker_alive = self
            .units
            .get(&attacker_id)
            .map(|u| u.unit().combat_stats().health > 0)
            .unwrap_or(false);

        if !attacker_alive {
            println!("💀 {} has been defeated by counter-attack!", attacker_name);
            self.units.remove(&attacker_id);
        }

        Ok(())
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
