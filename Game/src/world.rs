use crate::objects::*;
use graphics::{HexCoord, SpriteType};
use std::collections::HashMap;
use uuid::Uuid;

/// Attack info for display in combat dialog
#[derive(Clone, Debug)]
pub struct AttackInfo {
    pub name: String,
    pub damage: u32,
    pub range: i32,
}

/// Pending combat confirmation data
#[derive(Clone, Debug)]
pub struct PendingCombat {
    pub attacker_id: Uuid,
    pub defender_id: Uuid,
    pub attacker_name: String,
    pub attacker_hp: u32,
    pub attacker_max_hp: u32,
    pub attacker_attack: u32,
    pub attacker_defense: u32,
    pub attacker_attacks_per_round: u32,
    pub attacker_attacks: Vec<AttackInfo>,
    pub defender_name: String,
    pub defender_hp: u32,
    pub defender_max_hp: u32,
    pub defender_attack: u32,
    pub defender_defense: u32,
    pub defender_attacks_per_round: u32,
    pub defender_attacks: Vec<AttackInfo>,
    pub selected_attack_index: usize, // Which attack the player selected
}

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

    /// Pending combat confirmation
    pub pending_combat: Option<PendingCombat>,
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
            pending_combat: None,
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

        if let Some(unit) = self.units.get_mut(&unit_id) {
            unit.set_position(new_position);
            Ok(())
        } else {
            Err("Unit not found".to_string())
        }
    }

    /// Request combat confirmation between two units
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

    /// Execute the pending combat (called after player confirms)
    pub fn execute_pending_combat(&mut self) -> Result<(), String> {
        let pending = self.pending_combat.take().ok_or("No pending combat")?;
        let selected_attack_idx = pending.selected_attack_index;
        self.initiate_combat(
            pending.attacker_id,
            pending.defender_id,
            selected_attack_idx,
        )
    }

    /// Cancel the pending combat
    pub fn cancel_pending_combat(&mut self) {
        self.pending_combat = None;
    }

    /// Initiate combat between two units
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
