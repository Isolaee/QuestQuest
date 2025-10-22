//! # Game World Module
//!
//! This module provides the `GameWorld` structure that manages all game entities,
//! terrain, and combat in a hex-based grid system. It serves as the central state
//! management system for the game.
//!
//! ## Core Features
//!
//! - **Terrain Management**: Procedural terrain generation and querying
//! - **Unit Management**: Add, remove, move, and query units across the world
//! - **Combat System**: Combat initiation, confirmation dialogs, and resolution
//! - **Interactive Objects**: Manage pickups, quest objects, and NPCs
//! - **Collision Detection**: Movement validation considering terrain and units
//!
//! ## Combat Flow
//!
//! 1. Player attempts to move onto enemy unit's position
//! 2. Combat confirmation dialog is created (`PendingCombat`)
//! 3. Player selects attack and confirms
//! 4. Combat is executed with damage calculations and counter-attacks
//! 5. Defeated units are removed from the world

use crate::objects::*;
use graphics::{HexCoord, SpriteType};
use std::collections::HashMap;
use uuid::Uuid;

/// Information about a unit's attack for display in combat dialogs.
///
/// Used to present attack options to the player before combat begins.
#[derive(Clone, Debug)]
pub struct AttackInfo {
    /// Display name of the attack
    pub name: String,
    /// Base damage value
    pub damage: u32,
    /// Attack range (1 for melee, higher for ranged)
    pub range: i32,
}

/// Contains all data needed for combat confirmation dialog.
///
/// When a player unit moves onto an enemy unit, combat is not immediately
/// executed. Instead, a `PendingCombat` is created to allow the player to
/// review both combatants' stats and select which attack to use.
///
/// # Examples
///
/// ```ignore
/// // After world.move_unit() detects enemy collision:
/// if let Some(pending) = &world.pending_combat {
///     println!("Attack with: {}", pending.attacker_name);
///     println!("Defend: {}", pending.defender_name);
///     // Player selects attack, then calls world.execute_pending_combat()
/// }
/// ```
#[derive(Clone, Debug)]
pub struct PendingCombat {
    /// UUID of the attacking unit
    pub attacker_id: Uuid,
    /// UUID of the defending unit
    pub defender_id: Uuid,
    /// Display name of the attacker
    pub attacker_name: String,
    /// Current hit points of the attacker
    pub attacker_hp: u32,
    /// Maximum hit points of the attacker
    pub attacker_max_hp: u32,
    /// Attack stat of the attacker
    pub attacker_attack: u32,
    /// Defense stat of the attacker
    pub attacker_defense: u32,
    /// Number of attacks the attacker makes per combat round
    pub attacker_attacks_per_round: u32,
    /// Available attacks for the attacker to choose from
    pub attacker_attacks: Vec<AttackInfo>,
    /// Display name of the defender
    pub defender_name: String,
    /// Current hit points of the defender
    pub defender_hp: u32,
    /// Maximum hit points of the defender
    pub defender_max_hp: u32,
    /// Attack stat of the defender
    pub defender_attack: u32,
    /// Defense stat of the defender
    pub defender_defense: u32,
    /// Number of attacks the defender makes per combat round
    pub defender_attacks_per_round: u32,
    /// Available attacks for the defender
    pub defender_attacks: Vec<AttackInfo>,
    /// Index of the attack selected by the player (0-based)
    pub selected_attack_index: usize,
}

/// The game world that manages all game entities and their interactions.
///
/// `GameWorld` is the central state container for the game, managing terrain,
/// units, interactive objects, and combat on a hex-based grid. It provides
/// methods for querying positions, validating movement, and executing game logic.
///
/// # Note
///
/// Cannot derive `Serialize`/`Deserialize` because `GameUnit` contains trait objects.
/// Save/load functionality must be implemented through custom serialization or
/// by reconstructing units from saved data.
///
/// # Examples
///
/// ```
/// use game::GameWorld;
/// use graphics::HexCoord;
///
/// // Create a world with radius 10 (covers coordinates from -10 to +10)
/// let mut world = GameWorld::new(10);
///
/// // Generate procedural terrain
/// world.generate_terrain();
///
/// // Query terrain at a position
/// if let Some(terrain) = world.get_terrain(HexCoord::new(0, 0)) {
///     println!("Movement cost: {}", terrain.movement_cost());
/// }
/// ```
pub struct GameWorld {
    /// All terrain tiles in the world, indexed by hex coordinate
    pub terrain: HashMap<HexCoord, TerrainTile>,

    /// All units in the world, indexed by UUID
    pub units: HashMap<Uuid, GameUnit>,

    /// All interactive objects in the world, indexed by UUID
    pub interactive_objects: HashMap<Uuid, InteractiveObject>,

    /// World size as radius from center (e.g., 10 means coordinates from -10 to +10)
    world_radius: i32,

    /// Current game time in seconds, used for cooldowns and time-based mechanics
    pub game_time: f32,

    /// Pending combat awaiting player confirmation
    pub pending_combat: Option<PendingCombat>,

    /// Turn-based gameplay system
    pub turn_system: crate::turn_system::TurnSystem,
}

impl GameWorld {
    /// Creates a new empty game world with the specified radius.
    ///
    /// The world is initially empty and must be populated with terrain using
    /// `generate_terrain()` or by manually adding terrain tiles.
    ///
    /// # Arguments
    ///
    /// * `world_radius` - Maximum distance from center (0,0) for valid coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// use game::GameWorld;
    ///
    /// let world = GameWorld::new(15); // Creates a world with radius 15
    /// assert_eq!(world.world_radius(), 15);
    /// ```
    pub fn new(world_radius: i32) -> Self {
        let mut turn_system = crate::turn_system::TurnSystem::new();
        // By default, only Player team is player-controlled
        turn_system.set_team_control(Team::Player, true);
        turn_system.set_team_control(Team::Enemy, false);
        turn_system.set_team_control(Team::Neutral, false);

        Self {
            terrain: HashMap::new(),
            units: HashMap::new(),
            interactive_objects: HashMap::new(),
            world_radius,
            game_time: 0.0,
            pending_combat: None,
            turn_system,
        }
    }

    /// Generates procedural terrain for the entire world.
    ///
    /// Uses coordinate-based seeding to generate consistent, deterministic terrain.
    /// Each hex within the world radius is assigned a terrain type based on its
    /// position, creating varied landscapes of grasslands, forests, hills, etc.
    ///
    /// This method should be called once after creating a new world.
    ///
    /// # Examples
    ///
    /// ```
    /// use game::GameWorld;
    ///
    /// let mut world = GameWorld::new(10);
    /// world.generate_terrain();
    ///
    /// // World now has terrain tiles at all valid coordinates
    /// assert!(world.terrain().len() > 0);
    /// ```
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

    /// Generates terrain type based on hex coordinate position.
    ///
    /// Uses coordinate-based pseudo-random generation to ensure consistent
    /// terrain across multiple calls.
    ///
    /// # Arguments
    ///
    /// * `coord` - Hex coordinate to generate terrain for
    ///
    /// # Returns
    ///
    /// A sprite type representing the terrain at this position
    fn generate_terrain_type(&self, coord: HexCoord) -> SpriteType {
        // Use coordinate-based seeding for consistent terrain generation
        let seed = coord.q * 73 + coord.r * 37 + coord.q * coord.r * 17;
        SpriteType::random_terrain(seed)
    }

    /// Adds a unit to the world and returns its UUID.
    ///
    /// # Arguments
    ///
    /// * `unit` - The GameUnit to add
    ///
    /// # Returns
    ///
    /// The UUID of the added unit, which can be used for later queries
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use game::{GameWorld, GameUnit};
    ///
    /// let mut world = GameWorld::new(10);
    /// let unit_id = world.add_unit(some_game_unit);
    /// ```
    pub fn add_unit(&mut self, unit: GameUnit) -> Uuid {
        let id = unit.id();
        self.units.insert(id, unit);
        id
    }

    /// Removes a unit from the world.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the unit to remove
    ///
    /// # Returns
    ///
    /// `Some(GameUnit)` if the unit existed, `None` otherwise
    pub fn remove_unit(&mut self, id: Uuid) -> Option<GameUnit> {
        self.units.remove(&id)
    }

    /// Returns a reference to a unit by its UUID.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the unit to retrieve
    ///
    /// # Returns
    ///
    /// `Some(&GameUnit)` if found, `None` otherwise
    pub fn get_unit(&self, id: Uuid) -> Option<&GameUnit> {
        self.units.get(&id)
    }

    /// Returns a mutable reference to a unit by its UUID.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the unit to retrieve
    ///
    /// # Returns
    ///
    /// `Some(&mut GameUnit)` if found, `None` otherwise
    pub fn get_unit_mut(&mut self, id: Uuid) -> Option<&mut GameUnit> {
        self.units.get_mut(&id)
    }

    /// Returns a reference to all units in the world.
    pub fn units(&self) -> &HashMap<Uuid, GameUnit> {
        &self.units
    }

    /// Returns all units at a specific hex coordinate.
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate to query
    ///
    /// # Returns
    ///
    /// Vector of references to units at this position
    pub fn get_units_at_position(&self, position: HexCoord) -> Vec<&GameUnit> {
        self.units
            .values()
            .filter(|unit| unit.position() == position)
            .collect()
    }

    /// Adds an interactive object to the world and returns its UUID.
    ///
    /// # Arguments
    ///
    /// * `object` - The InteractiveObject to add
    ///
    /// # Returns
    ///
    /// The UUID of the added object
    pub fn add_interactive_object(&mut self, object: InteractiveObject) -> Uuid {
        let id = object.id();
        self.interactive_objects.insert(id, object);
        id
    }

    /// Removes an interactive object from the world.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the object to remove
    ///
    /// # Returns
    ///
    /// `Some(InteractiveObject)` if the object existed, `None` otherwise
    pub fn remove_interactive_object(&mut self, id: Uuid) -> Option<InteractiveObject> {
        self.interactive_objects.remove(&id)
    }

    /// Returns a reference to an interactive object by its UUID.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the object to retrieve
    ///
    /// # Returns
    ///
    /// `Some(&InteractiveObject)` if found, `None` otherwise
    pub fn get_interactive_object(&self, id: Uuid) -> Option<&InteractiveObject> {
        self.interactive_objects.get(&id)
    }

    /// Returns a mutable reference to an interactive object by its UUID.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the object to retrieve
    ///
    /// # Returns
    ///
    /// `Some(&mut InteractiveObject)` if found, `None` otherwise
    pub fn get_interactive_object_mut(&mut self, id: Uuid) -> Option<&mut InteractiveObject> {
        self.interactive_objects.get_mut(&id)
    }

    /// Returns a reference to all interactive objects in the world.
    pub fn interactive_objects(&self) -> &HashMap<Uuid, InteractiveObject> {
        &self.interactive_objects
    }

    /// Returns a reference to terrain at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate to query
    ///
    /// # Returns
    ///
    /// `Some(&TerrainTile)` if terrain exists, `None` otherwise
    pub fn get_terrain(&self, position: HexCoord) -> Option<&TerrainTile> {
        self.terrain.get(&position)
    }

    /// Returns a mutable reference to terrain at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate to query
    ///
    /// # Returns
    ///
    /// `Some(&mut TerrainTile)` if terrain exists, `None` otherwise
    pub fn get_terrain_mut(&mut self, position: HexCoord) -> Option<&mut TerrainTile> {
        self.terrain.get_mut(&position)
    }

    /// Returns a reference to all terrain tiles in the world.
    pub fn terrain(&self) -> &HashMap<HexCoord, TerrainTile> {
        &self.terrain
    }

    /// Checks if a position is valid for movement.
    ///
    /// Validates movement based on:
    /// - World boundaries
    /// - Terrain blocking (e.g., mountains)
    /// - Unit blocking (allied units block, enemy units allow combat)
    /// - Interactive object blocking
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate to validate
    /// * `unit_id` - Optional UUID of the moving unit (for team checking)
    ///
    /// # Returns
    ///
    /// `true` if the position is valid for movement, `false` otherwise
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

    /// Moves a unit to a new position with validation and combat detection.
    ///
    /// If the target position contains an enemy unit, combat confirmation is
    /// initiated instead of moving. If the position is blocked or invalid,
    /// an error is returned.
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to move
    /// * `new_position` - Target hex coordinate
    ///
    /// # Returns
    ///
    /// - `Ok(())` for successful movement
    /// - `Err(String)` with error message for blocked movement or combat initiation
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use game::GameWorld;
    /// use graphics::HexCoord;
    ///
    /// let mut world = GameWorld::new(10);
    /// // ... add units ...
    ///
    /// match world.move_unit(unit_id, HexCoord::new(1, 0)) {
    ///     Ok(()) => println!("Unit moved successfully"),
    ///     Err(msg) => println!("Movement failed: {}", msg),
    /// }
    /// ```
    pub fn move_unit(&mut self, unit_id: Uuid, new_position: HexCoord) -> Result<(), String> {
        // Check if game has started
        if !self.turn_system.is_game_started() {
            return Err("Game has not started yet".to_string());
        }

        // Check if it's this unit's team's turn
        if let Some(unit) = self.units.get(&unit_id) {
            let unit_team = unit.team();
            if !self.turn_system.is_team_turn(unit_team) {
                return Err(format!("Not {:?}'s turn", unit_team));
            }
        } else {
            return Err("Unit not found".to_string());
        }

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
                    // Enemy units - request combat confirmation!
                    return self.request_combat(unit_id, target_id);
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

        // Determine movement cost for the target tile
        let movement_cost = self.get_movement_cost(new_position);

        if let Some(unit) = self.units.get_mut(&unit_id) {
            // Ensure the unit has enough movement points left
            if !unit.consume_moves(movement_cost) {
                return Err("Not enough movement points".to_string());
            }

            // Perform move
            unit.set_position(new_position);
            // Mark unit as having acted (moved) this turn
            self.turn_system.mark_unit_acted(unit_id);
            Ok(())
        } else {
            Err("Unit not found".to_string())
        }
    }

    /// Initiates combat confirmation between two units.
    ///
    /// Creates a `PendingCombat` structure containing all necessary information
    /// for the combat confirmation dialog. The player can then review stats,
    /// select an attack, and either execute or cancel the combat.
    ///
    /// # Arguments
    ///
    /// * `attacker_id` - UUID of the attacking unit
    /// * `defender_id` - UUID of the defending unit
    ///
    /// # Returns
    ///
    /// `Ok(())` if combat request was created, `Err(String)` if either unit not found
    pub fn request_combat(&mut self, attacker_id: Uuid, defender_id: Uuid) -> Result<(), String> {
        // Get unit info for confirmation dialog
        let attacker = self.units.get(&attacker_id).ok_or("Attacker not found")?;
        let defender = self.units.get(&defender_id).ok_or("Defender not found")?;

        let attacker_stats = attacker.unit().combat_stats();
        let defender_stats = defender.unit().combat_stats();

        // Get attacker's available attacks
        let attacker_attacks = attacker
            .unit()
            .get_attacks()
            .iter()
            .map(|attack| AttackInfo {
                name: attack.name.clone(),
                damage: attack.damage,
                range: attack.range,
            })
            .collect();

        // Get defender attacks
        let defender_attacks = defender
            .unit()
            .get_attacks()
            .iter()
            .map(|attack| AttackInfo {
                name: attack.name.clone(),
                damage: attack.damage,
                range: attack.range,
            })
            .collect();

        let pending = PendingCombat {
            attacker_id,
            defender_id,
            attacker_name: attacker.name(),
            attacker_hp: attacker_stats.health as u32,
            attacker_max_hp: attacker_stats.max_health as u32,
            attacker_attack: attacker_stats.get_total_attack(),
            attacker_defense: attacker_stats.resistances.slash as u32, // Use slash resistance as defense for display
            attacker_attacks_per_round: attacker_stats.attacks_per_round,
            attacker_attacks,
            defender_name: defender.name(),
            defender_hp: defender_stats.health as u32,
            defender_max_hp: defender_stats.max_health as u32,
            defender_attack: defender_stats.get_total_attack(),
            defender_defense: defender_stats.resistances.slash as u32,
            defender_attacks_per_round: defender_stats.attacks_per_round,
            defender_attacks,
            selected_attack_index: 0, // Default to first attack, will be updated by UI
        };

        self.pending_combat = Some(pending);
        Ok(())
    }

    /// Executes the pending combat after player confirmation.
    ///
    /// Retrieves the pending combat data, extracts the selected attack,
    /// and initiates the full combat sequence. The pending combat is
    /// consumed by this operation.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if combat was executed successfully
    /// - `Err(String)` if no pending combat exists or combat execution fails
    pub fn execute_pending_combat(&mut self) -> Result<(), String> {
        let pending = self.pending_combat.take().ok_or("No pending combat")?;
        let selected_attack_idx = pending.selected_attack_index;
        self.initiate_combat(
            pending.attacker_id,
            pending.defender_id,
            selected_attack_idx,
        )
    }

    /// Cancels the pending combat without executing it.
    ///
    /// Clears the pending combat state, allowing the player to cancel
    /// an attack and take a different action.
    pub fn cancel_pending_combat(&mut self) {
        self.pending_combat = None;
    }

    /// Executes combat between two units with the selected attack.
    ///
    /// This is the core combat resolution method. It:
    /// 1. Applies the attacker's selected attack multiple times based on attacks_per_round
    /// 2. Calculates damage with resistance modifiers
    /// 3. Allows defender to counter-attack if in melee range
    /// 4. Removes defeated units and moves attacker to defender's position if victorious
    ///
    /// Combat results are printed to console with formatted output.
    ///
    /// # Arguments
    ///
    /// * `attacker_id` - UUID of the attacking unit
    /// * `defender_id` - UUID of the defending unit
    /// * `selected_attack_idx` - Index of the attack chosen by the player
    ///
    /// # Returns
    ///
    /// `Ok(())` if combat executed successfully, `Err(String)` on error
    fn initiate_combat(
        &mut self,
        attacker_id: Uuid,
        defender_id: Uuid,
        selected_attack_idx: usize,
    ) -> Result<(), String> {
        // Get unit info and selected attack before combat
        let (attacker_name, defender_name, attacker_pos, defender_pos, selected_attack) = {
            let attacker = self.units.get(&attacker_id).ok_or("Attacker not found")?;
            let defender = self.units.get(&defender_id).ok_or("Defender not found")?;

            // Debug: print all attacks
            let attacks = attacker.unit().get_attacks();
            println!("🔍 Unit has {} attacks:", attacks.len());
            for (i, atk) in attacks.iter().enumerate() {
                println!("  [{}] {} - {} damage", i, atk.name, atk.damage);
            }
            println!("🎯 Selected attack index: {}", selected_attack_idx);

            // Get the selected attack
            let attack = attacker
                .unit()
                .get_attacks()
                .get(selected_attack_idx)
                .ok_or("Selected attack not found")?
                .clone();

            (
                attacker.name(),
                defender.name(),
                attacker.position(),
                defender.position(),
                attack,
            )
        };

        println!("\n╔════════════════════════════════════════╗");
        println!("║          ⚔️  COMBAT INITIATED  ⚔️          ║");
        println!("╠════════════════════════════════════════╣");
        println!("║  Attacker: {:<28} ║", attacker_name);
        println!("║  Defender: {:<28} ║", defender_name);
        println!("║  Attack:   {:<28} ║", selected_attack.name);
        println!("╚════════════════════════════════════════╝\n");

        // Get combat stats for calculations
        let (attacker_stats, defender_stats) = {
            let attacker = self.units.get(&attacker_id).ok_or("Attacker not found")?;
            let defender = self.units.get(&defender_id).ok_or("Defender not found")?;

            (
                attacker.unit().combat_stats().clone(),
                defender.unit().combat_stats().clone(),
            )
        };

        // Calculate how many times the attacker can use this attack
        let attacker_attacks_per_round = attacker_stats.attacks_per_round;

        // Attacker performs all their attacks with the selected attack
        let mut total_attacker_damage = 0;
        println!(
            "⚔️  {} attacks {} times with {}:",
            attacker_name, attacker_attacks_per_round, selected_attack.name
        );

        for i in 1..=attacker_attacks_per_round {
            if let Some(defender) = self.units.get(&defender_id) {
                if defender.unit().combat_stats().health == 0 {
                    break; // Defender already dead
                }
            }

            // Calculate damage with resistance
            let defender_resistance = defender_stats
                .resistances
                .get_resistance(selected_attack.damage_type);
            let resistance_multiplier = 1.0 - (defender_resistance as f32 / 100.0);
            let damage = ((selected_attack.damage as f32 * resistance_multiplier) as u32).max(1);

            // Apply damage to defender
            if let Some(defender) = self.units.get_mut(&defender_id) {
                defender.unit_mut().take_damage(damage);
                total_attacker_damage += damage;

                let damage_type_str = match selected_attack.damage_type {
                    combat::DamageType::Slash => "⚔️ ",
                    combat::DamageType::Pierce => "🗡️ ",
                    combat::DamageType::Blunt => "🔨",
                    combat::DamageType::Crush => "🔨",
                    combat::DamageType::Fire => "🔥",
                    combat::DamageType::Dark => "🌑",
                };

                println!(
                    "  {}Attack {}/{}: {} damage",
                    damage_type_str, i, attacker_attacks_per_round, damage
                );
            }
        }

        println!("\n📊 Total damage dealt: {}", total_attacker_damage);

        // Check if defender is still alive for counter-attack
        let defender_alive = self
            .units
            .get(&defender_id)
            .map(|u| u.unit().combat_stats().health > 0)
            .unwrap_or(false);

        if defender_alive {
            // Calculate actual distance between units
            let combat_distance = attacker_pos.distance(defender_pos);

            // Defender can counter-attack if combat happened at melee range (distance 1)
            if combat_distance == 1 {
                // Get defender's melee attack (range 1)
                let defender_melee_attack = {
                    let defender = self.units.get(&defender_id).ok_or("Defender not found")?;
                    defender
                        .unit()
                        .get_attacks()
                        .iter()
                        .find(|a| a.range == 1) // Find melee attack
                        .cloned()
                };

                if let Some(defender_attack) = defender_melee_attack {
                    let defender_attacks_per_round = defender_stats.attacks_per_round;

                    println!(
                        "\n🛡️  {} counter-attacks {} times with {}:",
                        defender_name, defender_attacks_per_round, defender_attack.name
                    );

                    let mut total_defender_damage = 0;
                    for i in 1..=defender_attacks_per_round {
                        if let Some(attacker) = self.units.get(&attacker_id) {
                            if attacker.unit().combat_stats().health == 0 {
                                break; // Attacker already dead
                            }
                        }

                        // Calculate damage with resistance
                        let attacker_resistance = attacker_stats
                            .resistances
                            .get_resistance(defender_attack.damage_type);
                        let resistance_multiplier = 1.0 - (attacker_resistance as f32 / 100.0);
                        let damage =
                            ((defender_attack.damage as f32 * resistance_multiplier) as u32).max(1);

                        // Apply damage to attacker
                        if let Some(attacker) = self.units.get_mut(&attacker_id) {
                            attacker.unit_mut().take_damage(damage);
                            total_defender_damage += damage;

                            let damage_type_str = match defender_attack.damage_type {
                                combat::DamageType::Slash => "⚔️ ",
                                combat::DamageType::Pierce => "🗡️ ",
                                combat::DamageType::Blunt => "🔨",
                                combat::DamageType::Crush => "🔨",
                                combat::DamageType::Fire => "🔥",
                                combat::DamageType::Dark => "🌑",
                            };

                            println!(
                                "  {}Attack {}/{}: {} damage",
                                damage_type_str, i, defender_attacks_per_round, damage
                            );
                        }
                    }

                    println!(
                        "\n📊 Total counter-attack damage: {}",
                        total_defender_damage
                    );
                } else {
                    println!(
                        "\n🛡️  {} has no melee weapon to counter-attack!",
                        defender_name
                    );
                }
            } else {
                println!(
                    "\n🏹 Combat at range {} - no counter-attack possible",
                    combat_distance
                );
            }
        }

        // Check final status and remove defeated units
        let defender_still_alive = self
            .units
            .get(&defender_id)
            .map(|u| u.unit().combat_stats().health > 0)
            .unwrap_or(false);

        if !defender_still_alive {
            println!("\n💀 {} has been defeated!", defender_name);
            self.units.remove(&defender_id);

            // Move attacker to defender's position
            if let Some(attacker) = self.units.get_mut(&attacker_id) {
                attacker.set_position(defender_pos);
                println!(
                    "📍 {} moves to ({}, {})",
                    attacker_name, defender_pos.q, defender_pos.r
                );
            }
        } else {
            // Show remaining HP
            if let Some(defender) = self.units.get(&defender_id) {
                let hp = defender.unit().combat_stats().health;
                let max_hp = defender.unit().combat_stats().max_health;
                println!("\n❤️  {} HP: {}/{}", defender_name, hp, max_hp);
            }
        }

        // Check if attacker survived
        let attacker_alive = self
            .units
            .get(&attacker_id)
            .map(|u| u.unit().combat_stats().health > 0)
            .unwrap_or(false);

        if !attacker_alive {
            println!("💀 {} has been defeated!", attacker_name);
            self.units.remove(&attacker_id);
        } else {
            // Show remaining HP
            if let Some(attacker) = self.units.get(&attacker_id) {
                let hp = attacker.unit().combat_stats().health;
                let max_hp = attacker.unit().combat_stats().max_health;
                println!("❤️  {} HP: {}/{}", attacker_name, hp, max_hp);
            }
        }

        println!("\n╚════════════════════════════════════════╝\n");
        Ok(())
    }

    /// Returns the movement cost for a position.
    ///
    /// Used for pathfinding and action point calculations.
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate to query
    ///
    /// # Returns
    ///
    /// Movement cost as f32, or `f32::INFINITY` for invalid/missing terrain
    pub fn get_movement_cost(&self, position: HexCoord) -> f32 {
        if let Some(terrain) = self.get_terrain(position) {
            terrain.movement_cost()
        } else {
            f32::INFINITY // Invalid terrain has infinite cost
        }
    }

    /// Updates the world state (called each frame).
    ///
    /// Advances game time, updates all units, and processes interactions
    /// between objects at the same positions.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - Time elapsed since last update in seconds
    pub fn update(&mut self, delta_time: f32) {
        self.game_time += delta_time;

        // Update turn system (handles AI turn auto-pass)
        self.turn_system.update(delta_time);

        // Update all units
        for unit in self.units.values_mut() {
            unit.update(delta_time);
        }

        // Handle interactions between objects at the same position
        self.process_interactions();
    }

    /// Processes interactions between units and interactive objects.
    ///
    /// Checks for units occupying the same position as interactive objects
    /// and triggers interaction logic (e.g., picking up items). Objects with
    /// no remaining interactions are automatically removed.
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

    /// Returns the current game time in seconds.
    ///
    /// Game time is used for cooldowns, time-based events, and other
    /// temporal mechanics.
    pub fn game_time(&self) -> f32 {
        self.game_time
    }

    /// Returns the world radius.
    ///
    /// The radius defines the maximum distance from the center (0,0) that
    /// is considered part of the game world.
    pub fn world_radius(&self) -> i32 {
        self.world_radius
    }

    // ===== Turn System Methods =====

    /// Starts the game and begins turn-based gameplay
    pub fn start_turn_based_game(&mut self) {
        self.turn_system.start_game();
        // Reset movement points for units on the starting team
        let current_team = self.turn_system.current_team();
        self.reset_moves_for_team(current_team);
    }

    /// Ends the current turn and advances to the next team
    pub fn end_current_turn(&mut self) {
        // End the turn in the turn system (advances to next team)
        self.turn_system.end_turn();

        // Reset movement points for units on the new current team
        let current_team = self.turn_system.current_team();
        self.reset_moves_for_team(current_team);
    }

    /// Resets movement points for all units belonging to a given team.
    fn reset_moves_for_team(&mut self, team: Team) {
        for unit in self.units.values_mut() {
            if unit.team() == team {
                unit.reset_moves_to_max();
            }
        }
    }

    /// Returns the team whose turn it currently is
    pub fn current_turn_team(&self) -> Team {
        self.turn_system.current_team()
    }

    /// Checks if it's the specified team's turn
    pub fn is_team_turn(&self, team: Team) -> bool {
        self.turn_system.is_team_turn(team)
    }

    /// Checks if the current team is player-controlled
    pub fn is_current_team_player_controlled(&self) -> bool {
        self.turn_system.is_current_team_player_controlled()
    }

    /// Returns the current turn number
    pub fn turn_number(&self) -> u32 {
        self.turn_system.turn_number()
    }

    /// Returns time remaining in AI turn (0 if player turn)
    pub fn ai_turn_time_remaining(&self) -> f32 {
        self.turn_system.ai_turn_time_remaining()
    }

    /// Checks if a unit can act (not already acted and correct team turn)
    pub fn can_unit_act(&self, unit_id: Uuid) -> bool {
        if let Some(unit) = self.units.get(&unit_id) {
            let is_team_turn = self.turn_system.is_team_turn(unit.team());
            let has_not_acted = !self.turn_system.has_unit_acted(unit_id);
            is_team_turn && has_not_acted
        } else {
            false
        }
    }

    /// Sets whether a team is player-controlled
    pub fn set_team_control(&mut self, team: Team, is_player_controlled: bool) {
        self.turn_system
            .set_team_control(team, is_player_controlled);
    }

    /// Sets the AI turn delay
    pub fn set_ai_turn_delay(&mut self, delay: f32) {
        self.turn_system.set_ai_turn_delay(delay);
    }
}

impl Default for GameWorld {
    /// Creates a default game world with radius 10.
    ///
    /// The world is empty and must be populated with terrain and entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use game::GameWorld;
    ///
    /// let world = GameWorld::default();
    /// assert_eq!(world.world_radius(), 10);
    /// ```
    fn default() -> Self {
        Self::new(10) // Default world radius of 10
    }
}
