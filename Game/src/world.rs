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
use ai::{
    ActionInstance as AiActionInstance, FactValue as AiFactValue, Goal as AiGoal,
    WorldState as AiWorldState,
};
use graphics::{HexCoord, SpriteType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// Weight applied to expected damage when converting to a negative cost (higher -> more aggressive)
const ATTACK_EXPECTED_UTILITY_WEIGHT: f32 = 1.0;
// Minimum action cost to avoid non-positive costs which may confuse planner ordering
const MIN_ACTION_COST: f32 = 0.01;

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
/// ```
/// use game::GameWorld;
/// use graphics::HexCoord;
/// use units::unit_factory::UnitFactory;
/// use game::{GameUnit, Team};
///
/// // Create world and start turn system so movement is allowed
/// let mut world = GameWorld::new(4);
/// world.start_turn_based_game();
///
/// // Create player unit at (0,0)
/// let boxed_p = UnitFactory::create_goblin_grunt(
///     "Player".to_string(),
///     HexCoord::new(0, 0),
///     units::unit_race::Terrain::Grasslands,
/// );
/// let mut pu = GameUnit::new(boxed_p);
/// pu.set_team(Team::Player);
/// let pid = world.add_unit(pu);
///
/// // Create enemy at (1,0)
/// let boxed_e = UnitFactory::create_goblin_grunt(
///     "Enemy".to_string(),
///     HexCoord::new(1, 0),
///     units::unit_race::Terrain::Grasslands,
/// );
/// let mut eu = GameUnit::new(boxed_e);
/// eu.set_team(Team::Enemy);
/// let _eid = world.add_unit(eu);
///
/// // Move player into enemy tile which should create a pending combat
/// let _ = world.move_unit(pid, HexCoord::new(1, 0));
/// assert!(world.pending_combat.is_some());
/// let pending = world.pending_combat.as_ref().unwrap();
/// assert_eq!(pending.attacker_name, "Player");
/// assert_eq!(pending.defender_name, "Enemy");
/// assert!(pending.attacker_attacks.len() > 0);
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

/// Events emitted by the AI executor for the game to react to.
#[derive(Clone, Debug)]
pub enum GameEvent {
    ActionStarted {
        unit_id: Uuid,
        action: ai::ActionInstance,
    },
    ActionCompleted {
        unit_id: Uuid,
        action: ai::ActionInstance,
    },
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
    /// Queue of AI events emitted by executors (start/complete). Protected by mutex
    /// so executor callbacks can push events from closures.
    pub ai_event_queue: Arc<Mutex<Vec<GameEvent>>>,

    /// Turn-based gameplay system
    pub turn_system: crate::turn_system::TurnSystem,
    /// Last known active team (used to detect auto-advanced turns so we can
    /// reset per-team movement points when TurnSystem advances the turn)
    last_known_team: Option<Team>,
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

        let mut world = Self {
            terrain: HashMap::new(),
            units: HashMap::new(),
            interactive_objects: HashMap::new(),
            world_radius,
            game_time: 0.0,
            pending_combat: None,
            ai_event_queue: Arc::new(Mutex::new(Vec::new())),
            turn_system,
            last_known_team: None,
        };

        // Populate terrain so movement costs are available by default
        world.generate_terrain();

        world
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

    /// Extract a minimal AI world state for the given team.
    ///
    /// Prototype: encode unit positions and alive flags as simple string facts.
    pub fn extract_world_state_for_team(&self, team: Team) -> AiWorldState {
        let mut ws = AiWorldState::new();

        for (id, unit) in &self.units {
            // fact keys: "Unit:{id}:At" => "q,r" string
            let pos = unit.position();
            let key_pos = format!("Unit:{}:At", id);
            ws.insert(
                key_pos.clone(),
                AiFactValue::Str(format!("{},{}", pos.q, pos.r)),
            );

            // alive flag
            let alive_key = format!("Unit:{}:Alive", id);
            ws.insert(alive_key, AiFactValue::Bool(true));
        }

        // Additionally include a team marker
        ws.insert(
            "CurrentTeam".to_string(),
            AiFactValue::Str(format!("{:?}", team)),
        );

        ws
    }

    /// Generate simple grounded actions for all units of `team`.
    ///
    /// Prototype actions:
    /// - Move to adjacent hex (precond: At=id_pos)
    /// - Attack adjacent enemy (precond: At=id_pos, EnemyAt=other_pos)
    pub fn generate_team_actions(&self, team: Team) -> Vec<AiActionInstance> {
        let mut out: Vec<AiActionInstance> = Vec::new();

        // Dijkstra-like reachable calculation using integer costs
        fn compute_reachable(
            world: &GameWorld,
            unit_id: Uuid,
            start: HexCoord,
            max_cost: i32,
        ) -> HashMap<HexCoord, i32> {
            use std::cmp::Reverse;
            use std::collections::BinaryHeap;

            let mut dist: HashMap<HexCoord, i32> = HashMap::new();
            // Use primitive tuple in heap so ordering is defined
            let mut heap: BinaryHeap<(Reverse<i32>, (i32, i32))> = BinaryHeap::new();

            dist.insert(start, 0);
            heap.push((Reverse(0), (start.q, start.r)));

            while let Some((Reverse(cost), (cq, cr))) = heap.pop() {
                let coord = HexCoord::new(cq, cr);
                if let Some(&best) = dist.get(&coord) {
                    if cost > best {
                        continue;
                    }
                }

                for nb in coord.neighbors().iter() {
                    // Skip out-of-bounds
                    if nb.distance(HexCoord::new(0, 0)) > world.world_radius {
                        continue;
                    }
                    // Skip impassable terrain
                    if let Some(terrain) = world.get_terrain(*nb) {
                        if terrain.blocks_movement() {
                            continue;
                        }
                    }
                    // Skip occupied tiles by other units
                    let units_there = world.get_units_at_position(*nb);
                    let occupied_by_other = units_there.iter().any(|u| u.id() != unit_id);
                    if occupied_by_other {
                        continue;
                    }

                    let step_cost = world.get_movement_cost(*nb);
                    let new_cost = cost + step_cost;
                    if new_cost > max_cost {
                        continue;
                    }

                    match dist.get(nb) {
                        None => {
                            dist.insert(*nb, new_cost);
                            heap.push((Reverse(new_cost), (nb.q, nb.r)));
                        }
                        Some(&c) => {
                            if new_cost < c {
                                dist.insert(*nb, new_cost);
                                heap.push((Reverse(new_cost), (nb.q, nb.r)));
                            }
                        }
                    }
                }
            }

            dist
        }

        for (id, unit) in &self.units {
            if unit.team() != team {
                continue;
            }

            let uid_str = id.to_string();
            let pos = unit.position();
            let moves_left = unit.moves_left();

            let reachable = compute_reachable(self, *id, pos, moves_left);

            // Ground Move actions for each reachable tile (excluding start)
            for (tile, cost) in &reachable {
                if *tile == pos {
                    continue;
                }

                let preconds = vec![(
                    format!("Unit:{}:At", id),
                    AiFactValue::Str(format!("{},{}", pos.q, pos.r)),
                )];
                let effects = vec![(
                    format!("Unit:{}:At", id),
                    AiFactValue::Str(format!("{},{}", tile.q, tile.r)),
                )];

                out.push(AiActionInstance {
                    name: format!("Move-{}->{},{}", uid_str, tile.q, tile.r),
                    preconditions: preconds,
                    effects,
                    cost: *cost as f32,
                    agent: Some(uid_str.clone()),
                });
            }

            // Ground Attack actions for reachable attack positions based on unit's available attacks and ranges
            for (other_id, other_unit) in &self.units {
                if other_unit.team() == team {
                    continue;
                }
                let enemy_pos = other_unit.position();

                // Candidate attacker positions: current position + reachable tiles
                let mut candidate_positions: Vec<HexCoord> = reachable.keys().cloned().collect();
                if !candidate_positions.contains(&pos) {
                    candidate_positions.push(pos);
                }

                // Query the unit's available attacks (includes equipped weapons)
                let attacks = unit.unit().get_attacks();

                for attack in attacks.iter() {
                    for from in &candidate_positions {
                        let dist = from.distance(enemy_pos);
                        // Use attack's own range check
                        if !attack.can_reach(dist) {
                            continue;
                        }

                        // Ensure the unit can actually be at `from` this turn (either already there or reachable)
                        let can_from_here = *from == pos || reachable.contains_key(from);
                        if !can_from_here {
                            continue;
                        }

                        let preconds = vec![
                            (
                                format!("Unit:{}:At", id),
                                AiFactValue::Str(format!("{},{}", from.q, from.r)),
                            ),
                            (format!("Unit:{}:Alive", other_id), AiFactValue::Bool(true)),
                        ];
                        let effects =
                            vec![(format!("Unit:{}:Alive", other_id), AiFactValue::Bool(false))];

                        // Estimate expected damage and hit chance to convert into an expected-utility cost.
                        // Hit chance is approximated using the same formula as combat resolver:
                        // final_hit_chance = (defender.get_defense() as i32 - attacker_race_bonus*2).clamp(10,95)
                        let defender_def = other_unit.unit().get_defense() as i32;
                        let attacker_bonus = unit.unit().race().get_attack_bonus();
                        let final_hit_chance =
                            (defender_def - attacker_bonus * 2).clamp(10, 95) as u8;
                        let hit_prob = final_hit_chance as f32 / 100.0;

                        // Compute expected damage on hit using attack.damage and defender resistances
                        let def_stats = other_unit.unit().combat_stats();
                        let resistance =
                            def_stats.resistances.get_resistance(attack.damage_type) as f32;
                        let resist_mul = 1.0 - (resistance / 100.0);
                        let damage_on_hit = (attack.damage as f32 * resist_mul).max(1.0);

                        let expected_damage = hit_prob * damage_on_hit;

                        // Movement cost to get to `from` (0 if already at pos)
                        let movement_cost = if *from == pos {
                            0
                        } else {
                            *reachable.get(from).unwrap_or(&0)
                        };

                        // Convert to planner cost: movement cost + base action cost - expected utility
                        // Planner minimizes cost, so higher expected_damage should lower cost.
                        let mut computed_cost = movement_cost as f32 + 1.0
                            - (ATTACK_EXPECTED_UTILITY_WEIGHT * expected_damage);
                        if computed_cost < MIN_ACTION_COST {
                            computed_cost = MIN_ACTION_COST;
                        }

                        out.push(AiActionInstance {
                            name: format!(
                                "Attack-{}-{}-{}-from-{},{}",
                                uid_str,
                                other_id,
                                attack.name.replace(' ', "_"),
                                from.q,
                                from.r
                            ),
                            preconditions: preconds,
                            effects,
                            cost: computed_cost,
                            agent: Some(uid_str.clone()),
                        });
                    }
                }
            }
        }

        out
    }

    /// Run the AI planner for the current team if the current team is AI-controlled.
    ///
    /// This prototype: when it's an AI team's turn, plan sequentially for each unit,
    /// execute move actions (via `move_unit`) or request combat (via `request_combat`),
    /// then end the turn.
    pub fn run_ai_for_current_team(&mut self) {
        let current_team = self.turn_system.current_team();
        if self.turn_system.is_current_team_player_controlled() {
            return; // Player team handled by UI
        }

        // Prepare AI world state and actions
        let ws = self.extract_world_state_for_team(current_team);
        let actions = self.generate_team_actions(current_team);

        // Build goals: naive goal is to kill any enemy unit found (per agent we add goals later)
        use std::collections::HashMap as StdHashMap;
        let mut goals_per_agent: StdHashMap<String, Vec<AiGoal>> = StdHashMap::new();
        let mut agent_order: Vec<String> = Vec::new();

        for (id, unit) in &self.units {
            if unit.team() != current_team {
                continue;
            }
            let aid = id.to_string();
            agent_order.push(aid.clone());

            // Goals for agent: prefer to kill adjacent enemy if any; otherwise move (no explicit goal)
            let mut goals: Vec<AiGoal> = Vec::new();
            // find any adjacent enemies
            for nb in unit.position().neighbors().iter() {
                let units_here = self.get_units_at_position(*nb);
                for u in units_here {
                    if u.team() != current_team {
                        // goal: that unit is not alive
                        goals.push(AiGoal {
                            key: format!("Unit:{}:Alive", u.id()),
                            value: AiFactValue::Bool(false),
                        });
                    }
                }
            }
            goals_per_agent.insert(aid, goals);
        }

        // Call team planner (bounded search per agent)
        let plans = ai::plan_for_team(&ws, &actions, &goals_per_agent, &agent_order, 500);

        // Execute plans per agent
        // executed_actions_count removed — we end AI turn immediately after executing plans
        for (agent, plan) in plans {
            if plan.is_empty() {
                continue;
            }
            // find unit uuid
            if let Ok(uuid) = Uuid::parse_str(&agent) {
                // For simplicity, perform actions in plan for this agent
                let agent_actions: Vec<AiActionInstance> = actions
                    .iter()
                    .filter(|a| a.agent.as_ref().map(|s| s == &agent).unwrap_or(false))
                    .cloned()
                    .collect();
                for &idx in &plan {
                    if let Some(a) = agent_actions.get(idx) {
                        // If action is Move (name starts with Move-), parse target and call move_unit
                        if a.name.starts_with("Move-") {
                            // effects contain Unit:{id}:At -> "q,r"
                            if let Some((_, AiFactValue::Str(dest))) = a.effects.first() {
                                let parts: Vec<&str> = dest.split(',').collect();
                                if parts.len() == 2 {
                                    if let (Ok(q), Ok(r)) =
                                        (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                                    {
                                        let dest_coord = HexCoord::new(q, r);
                                        // Use move_unit; ignore errors for prototype
                                        let _ = self.move_unit(uuid, dest_coord);
                                    }
                                }
                            }
                        } else if a.name.starts_with("Attack-") {
                            // effects contain Unit:{other}:Alive=false; extract other id
                            if let Some((k, _)) = a.effects.first() {
                                if k.starts_with("Unit:") && k.ends_with(":Alive") {
                                    let mid = &k[5..k.len() - 6];
                                    if let Ok(target_uuid) = Uuid::parse_str(mid) {
                                        // Request combat (this will set pending_combat). If the
                                        // request fails (e.g., attacker already attacked), skip
                                        // executing combat.
                                        // request_combat now returns Ok(()) even when it silently
                                        // skips creating a pending combat (attacker already
                                        // attacked). Only execute if a pending combat was
                                        // actually created.
                                        let _ = self.request_combat(uuid, target_uuid);
                                        if self.pending_combat.is_some() {
                                            // execute_pending_combat may set state; count it as an executed action
                                            let _ = self.execute_pending_combat();
                                        } else {
                                            // Silent skip - nothing to do
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // End AI turn after actions (use GameWorld API so unit moves are reset)
        self.end_current_turn();
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
    /// ```
    /// use game::{GameWorld, GameUnit};
    /// use graphics::HexCoord;
    /// use units::unit_factory::UnitFactory;
    ///
    /// let mut world = GameWorld::new(4);
    /// let boxed = UnitFactory::create_goblin_grunt(
    ///     "Gob1".to_string(),
    ///     HexCoord::new(0, 0),
    ///     units::unit_race::Terrain::Grasslands,
    /// );
    /// let u = GameUnit::new(boxed);
    /// let id = world.add_unit(u);
    /// assert!(world.get_unit(id).is_some());
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
    /// ```
    /// use game::GameWorld;
    /// use graphics::HexCoord;
    /// use units::unit_factory::UnitFactory;
    /// use game::{GameUnit, Team};
    ///
    /// let mut world = GameWorld::new(4);
    /// world.start_turn_based_game();
    ///
    /// // Create player unit and add to world
    /// let boxed_p = UnitFactory::create_goblin_grunt(
    ///     "Player".to_string(),
    ///     HexCoord::new(0,0),
    ///     units::unit_race::Terrain::Grasslands,
    /// );
    /// let mut pu = GameUnit::new(boxed_p);
    /// pu.set_team(Team::Player);
    /// let pid = world.add_unit(pu);
    ///
    /// // Create enemy next to player
    /// let boxed_e = UnitFactory::create_goblin_grunt(
    ///     "Enemy".to_string(),
    ///     HexCoord::new(1,0),
    ///     units::unit_race::Terrain::Grasslands,
    /// );
    /// let mut eu = GameUnit::new(boxed_e);
    /// eu.set_team(Team::Enemy);
    /// let _eid = world.add_unit(eu);
    ///
    /// // Moving into the enemy tile should request combat (returns Err or sets pending_combat)
    /// let res = world.move_unit(pid, HexCoord::new(1,0));
    /// assert!(res.is_err() || world.pending_combat.is_some());
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

        // Determine movement cost for the target tile (integer)
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

        // If the attacker has already attacked this game turn, silently do
        // nothing: don't create a pending combat confirmation. Returning Ok
        // without setting `pending_combat` keeps the call-site behavior simple
        // (UI/AI won't see a pending combat and will not open the dialog).
        if attacker.unit().combat_stats().attacked_this_turn {
            // Intentionally silent: no pending combat is created and callers
            // should observe that `pending_combat` is still None.
            return Ok(());
        }

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

        // Prevent starting combat if the attacker already attacked this game turn
        if let Some(attacker_unit) = self.units.get(&attacker_id) {
            if attacker_unit.unit().combat_stats().attacked_this_turn {
                println!("⛔ Combat canceled: attacker has already attacked this turn");
                return Err("Attacker has already attacked this turn".to_string());
            }
        }

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

        // Mark attacker as having attacked this turn (after performing all attack instances)
        if let Some(attacker_unit) = self.units.get_mut(&attacker_id) {
            attacker_unit
                .unit_mut()
                .combat_stats_mut()
                .attacked_this_turn = true;
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
                    // Mark defender as having attacked this turn (after performing counters)
                    if let Some(defender_unit) = self.units.get_mut(&defender_id) {
                        defender_unit
                            .unit_mut()
                            .combat_stats_mut()
                            .attacked_this_turn = true;
                    }
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
    pub fn get_movement_cost(&self, position: HexCoord) -> i32 {
        if let Some(terrain) = self.get_terrain(position) {
            terrain.movement_cost()
        } else {
            // Invalid terrain treated as very high cost to block movement
            i32::MAX
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
        // Remember previous team so we can detect changes made by TurnSystem::update
        let prev_team = self.last_known_team;
        self.turn_system.update(delta_time);

        // If the turn system auto-advanced the team (AI timeout or similar), reset moves for the new team
        if self.turn_system.is_game_started() {
            let current_team = self.turn_system.current_team();
            if prev_team != Some(current_team) {
                self.reset_moves_for_team(current_team);
                self.last_known_team = Some(current_team);
            }
        }

        // Update all units
        for unit in self.units.values_mut() {
            unit.update(delta_time);
        }

        // Simple AI integration: if it's an AI-controlled team's turn, let each AI unit
        // plan and execute a short plan. We'll take a snapshot of enemy info first to avoid
        // borrow conflicts, then iterate units and use their executor/plan.
        if !self.is_current_team_player_controlled() {
            let current_team = self.turn_system.current_team();

            // Snapshot enemies: list of (id, pos, health, alive)
            let enemies: Vec<(Uuid, graphics::HexCoord, u32, bool)> = self
                .units
                .iter()
                .filter_map(|(eid, u)| {
                    if u.team() != current_team {
                        Some((
                            *eid,
                            u.position(),
                            u.unit().combat_stats().health as u32,
                            u.unit().is_alive(),
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            let unit_ids: Vec<Uuid> = self.units.keys().cloned().collect();

            use std::collections::HashMap as Map;
            let mut planned_plans: Map<Uuid, Vec<ai::ActionInstance>> = Map::new();

            // Planning pass (immutable borrows)
            for uid in &unit_ids {
                if let Some(unit_ref) = self.units.get(uid) {
                    if unit_ref.team() != current_team {
                        continue;
                    }

                    if unit_ref.ai_plan.is_empty() {
                        if enemies.is_empty() {
                            continue;
                        }

                        let mut state = ai::WorldState::new();
                        let pos = unit_ref.position();
                        state.insert(
                            "At".to_string(),
                            ai::FactValue::Str(format!("{}:{}", pos.q, pos.r)),
                        );

                        let (_enemy_id, opos, ohealth, oalive) = enemies[0];
                        state.insert(
                            "EnemyAt".to_string(),
                            ai::FactValue::Str(format!("{}:{}", opos.q, opos.r)),
                        );
                        state.insert(
                            "EnemyHealth".to_string(),
                            ai::FactValue::Int(ohealth as i32),
                        );
                        state.insert("EnemyAlive".to_string(), ai::FactValue::Bool(oalive));

                        let move_t = ai::ActionTemplate {
                            name: "MoveToEnemy".to_string(),
                            preconditions: vec![(
                                "At".to_string(),
                                ai::FactValue::Str(format!("{}:{}", pos.q, pos.r)),
                            )],
                            effects: vec![(
                                "At".to_string(),
                                ai::FactValue::Str(format!("{}:{}", opos.q, opos.r)),
                            )],
                            cost: 1.0,
                        };

                        let attack_template = ai::AttackTemplate {
                            name_base: "Attack".to_string(),
                            damage: 5,
                            cost: 1.0,
                            range: 1,
                        };

                        let mut instances: Vec<ai::ActionInstance> = Vec::new();
                        instances.push(ai::ground_action_from_template(
                            &move_t,
                            Some(unit_ref.name()),
                        ));
                        let mut att =
                            attack_template.ground_for_state(&state, Some(unit_ref.name()));
                        instances.append(&mut att);

                        let goal = ai::Goal {
                            key: "EnemyAlive".to_string(),
                            value: ai::FactValue::Bool(false),
                        };
                        if let Some(plan_idx) = ai::plan_instances(&state, &instances, &goal, 500) {
                            let plan_vec: Vec<ai::ActionInstance> =
                                plan_idx.iter().map(|&i| instances[i].clone()).collect();
                            planned_plans.insert(*uid, plan_vec);
                        }
                    }
                }
            }

            // Execution pass (mutable borrows). Collect pending updates to apply after loop.
            let mut unit_pos_updates: Vec<(Uuid, graphics::HexCoord)> = Vec::new();
            let mut enemy_health_updates: Vec<(Uuid, i32)> = Vec::new();

            for uid in unit_ids {
                // Mutable borrow per unit
                if let Some(unit_mut) = self.units.get_mut(&uid) {
                    if unit_mut.team() != current_team {
                        continue;
                    }

                    if unit_mut.ai_executor.is_none() {
                        unit_mut.ai_executor = Some(ai::ActionExecutor::new());
                    }

                    // Ensure callbacks are set on the executor to push events to the world's queue
                    if let Some(ex) = &mut unit_mut.ai_executor {
                        // Capture unit id and queue clone
                        let q = self.ai_event_queue.clone();
                        let uid_clone = uid;
                        ex.set_on_start(move |ai_inst| {
                            let ev = GameEvent::ActionStarted {
                                unit_id: uid_clone,
                                action: ai_inst.clone(),
                            };
                            if let Ok(mut vec) = q.lock() {
                                vec.push(ev);
                            }
                        });

                        let q2 = self.ai_event_queue.clone();
                        let uid_clone2 = uid;
                        ex.set_on_complete(move |ai_inst| {
                            let ev = GameEvent::ActionCompleted {
                                unit_id: uid_clone2,
                                action: ai_inst.clone(),
                            };
                            if let Ok(mut vec) = q2.lock() {
                                vec.push(ev);
                            }
                        });
                    }

                    // Assign planned plan if present
                    if unit_mut.ai_plan.is_empty() {
                        if let Some(p) = planned_plans.remove(&uid) {
                            unit_mut.ai_plan = p;
                        }
                    }

                    // Read position before taking a mutable borrow of the executor
                    let p = unit_mut.position();
                    let mut ws = ai::WorldState::new();
                    ws.insert(
                        "At".to_string(),
                        ai::FactValue::Str(format!("{}:{}", p.q, p.r)),
                    );
                    if !enemies.is_empty() {
                        let (_, op, oh, oa) = enemies[0];
                        ws.insert(
                            "EnemyAt".to_string(),
                            ai::FactValue::Str(format!("{}:{}", op.q, op.r)),
                        );
                        ws.insert("EnemyHealth".to_string(), ai::FactValue::Int(oh as i32));
                        ws.insert("EnemyAlive".to_string(), ai::FactValue::Bool(oa));
                    }

                    if let Some(ex) = &mut unit_mut.ai_executor {
                        if ex.current.is_none() && !unit_mut.ai_plan.is_empty() {
                            let next = unit_mut.ai_plan.remove(0);
                            let runtime = if next.name.starts_with("Move") {
                                ai::RuntimeAction::Timed {
                                    instance: next,
                                    duration: 1.0,
                                    elapsed: 0.0,
                                }
                            } else {
                                ai::RuntimeAction::Instant(next)
                            };
                            ex.start(runtime);
                        }

                        let mut applied_ws = ws.clone();
                        let completed = ex.update(delta_time, &mut applied_ws);

                        // Record position update if changed
                        if let Some(ai::FactValue::Str(newpos)) = applied_ws.get("At") {
                            if let Some((qstr, rstr)) = newpos.split_once(':') {
                                if let (Ok(q), Ok(r)) = (qstr.parse::<i32>(), rstr.parse::<i32>()) {
                                    unit_pos_updates.push((uid, graphics::HexCoord::new(q, r)));
                                }
                            }
                        }

                        // Record enemy health update for first enemy
                        if !enemies.is_empty() {
                            let (enemy_id, _, _, _) = enemies[0];
                            if let Some(ai::FactValue::Int(h)) = applied_ws.get("EnemyHealth") {
                                enemy_health_updates.push((enemy_id, *h));
                            }
                        }

                        if completed {
                            self.turn_system.mark_unit_acted(unit_mut.id());
                        }
                    }
                }
            }

            // Apply recorded updates
            for (uid, newpos) in unit_pos_updates {
                if let Some(u) = self.units.get_mut(&uid) {
                    u.set_position(newpos);
                }
            }
            for (eid, h) in enemy_health_updates {
                if let Some(enemy_unit) = self.units.get_mut(&eid) {
                    let stats = enemy_unit.unit_mut().combat_stats_mut();
                    stats.health = h;
                }
            }

            // Drain AI event queue and apply game-side reactions (animations, movement)
            if let Ok(mut q) = self.ai_event_queue.lock() {
                let events: Vec<GameEvent> = q.drain(..).collect();
                drop(q);

                for ev in events {
                    match ev {
                        GameEvent::ActionStarted { unit_id, action } => {
                            if action.name.starts_with("Move") {
                                if let Some(u) = self.units.get_mut(&unit_id) {
                                    for (k, v) in &action.effects {
                                        if k == "At" {
                                            match v {
                                                ai::FactValue::Hex(h) => {
                                                    let _ = u
                                                        .unit_mut()
                                                        .move_to(graphics::HexCoord::new(h.q, h.r));
                                                }
                                                ai::FactValue::Str(s) => {
                                                    if let Some((qstr, rstr)) = s.split_once(':') {
                                                        if let (Ok(qv), Ok(rv)) = (
                                                            qstr.parse::<i32>(),
                                                            rstr.parse::<i32>(),
                                                        ) {
                                                            let _ = u.unit_mut().move_to(
                                                                graphics::HexCoord::new(qv, rv),
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        GameEvent::ActionCompleted { unit_id, action } => {
                            if action.name.starts_with("Move") {
                                if let Some(u) = self.units.get_mut(&unit_id) {
                                    for (k, v) in &action.effects {
                                        if k == "At" {
                                            match v {
                                                ai::FactValue::Hex(h) => {
                                                    u.set_position(graphics::HexCoord::new(
                                                        h.q, h.r,
                                                    ));
                                                }
                                                ai::FactValue::Str(s) => {
                                                    if let Some((qstr, rstr)) = s.split_once(':') {
                                                        if let (Ok(qv), Ok(rv)) = (
                                                            qstr.parse::<i32>(),
                                                            rstr.parse::<i32>(),
                                                        ) {
                                                            u.set_position(
                                                                graphics::HexCoord::new(qv, rv),
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                // Completed move handling
                            }
                        }
                    }
                }

                // After processing events and releasing the borrow, nothing else to do here
            }
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
                // If this interactive object is an item pickup (contains an item),
                // do NOT auto-interact here. The intended UX is that items on the
                // ground remain until the player explicitly picks them up via the
                // UI (or a pickup action). Rendering already supports showing a
                // smaller item icon next to a unit when both are on the same hex.
                if object.has_item() {
                    // Skip automatic pickup - leave the object in the world so the
                    // UI can prompt the player to pick it up.
                    continue;
                }

                // Try to interact for non-item interactive objects
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
        // Track the active team so that future auto-advances can be detected
        self.last_known_team = Some(current_team);
    }

    /// Ends the current turn and advances to the next team
    pub fn end_current_turn(&mut self) {
        // End the turn in the turn system (advances to next team)
        self.turn_system.end_turn();

        // Reset per-turn combat flags for all units since the team has advanced.
        // This ensures `attacked_this_turn` is cleared and units may act again.
        for unit in self.units.values_mut() {
            unit.unit_mut().combat_stats_mut().reset_turn_flags();
        }

        // Reset movement points for units on the new current team
        let current_team = self.turn_system.current_team();
        self.reset_moves_for_team(current_team);
        // Update last known team to avoid duplicate resets
        self.last_known_team = Some(current_team);
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
