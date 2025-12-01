//! # ScenarioWorld - Game State Manager
//!
//! `ScenarioWorld` is the single source of truth for game state and the coordinator
//! between different game systems. It maintains all world state (units, terrain,
//! objects) and delegates specialized operations to appropriate crates:
//!
//! ## Architecture Responsibilities
//!
//! ### Core State Management
//! - Stores all units, terrain, and interactive objects
//! - Manages turn-based gameplay system
//! - Tracks pending combat for player confirmation
//! - Maintains movement and action state per unit
//!
//! ### System Coordination
//! - **Combat Resolution**: Delegates to `Combat` crate via `execute_pending_combat()`
//! - **AI Planning**: Delegates to `AI` crate via `run_ai_for_current_team()`
//! - **Presentation Layer**: Provides query methods for rendering (QuestApp)
//!
//! ### Key Methods
//! - `move_unit()`: Handles unit movement and initiates combat requests
//! - `execute_pending_combat()`: Delegates combat resolution to Combat crate
//! - `run_ai_for_current_team()`: Delegates AI planning to AI crate
//! - `all_legal_moves()`: Queries legal moves for UI display
//! - `extract_detailed_world_state()`: Exports state for AI planning
//!
//! ## Design Pattern
//!
//! ScenarioWorld acts as a **Facade** and **Mediator**:
//! - Provides a unified interface to complex subsystems (Combat, AI)
//! - Coordinates interactions between presentation layer and game logic
//! - Ensures single source of truth for game state

use crate::objects::*;
use crate::world::PendingCombat;
use ai::{
    ActionInstance as AiActionInstance, FactValue as AiFactValue, Goal as AiGoal,
    WorldState as AiWorldState,
};
use graphics::HexCoord;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// Local constants used by AI action cost calculations (match world.rs defaults)
const ATTACK_EXPECTED_UTILITY_WEIGHT: f32 = 1.0;
const MIN_ACTION_COST: f32 = 0.01;

/// Game event emitted by AI executors for tracking actions
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

/// ScenarioWorld: Single source of truth for game state
///
/// Coordinates between Combat crate (combat resolution), AI crate (enemy AI),
/// and QuestApp (presentation/controls). All game state mutations go through
/// this structure to maintain consistency.
pub struct ScenarioWorld {
    /// All terrain tiles in the world, indexed by hex coordinate
    pub terrain: HashMap<HexCoord, TerrainTile>,

    /// All units in the world, indexed by UUID
    pub units: HashMap<Uuid, GameUnit>,

    /// All interactive objects in the world, indexed by UUID
    pub interactive_objects: HashMap<Uuid, InteractiveObject>,

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

impl ScenarioWorld {
    pub fn new(map_json: String) -> Self {
        let terrain: HashMap<HexCoord, TerrainTile>;
        let units: HashMap<Uuid, GameUnit>;
        let interactive_objects: HashMap<Uuid, InteractiveObject>;

        // Parse the map JSON
        match ScenarioWorld::parse_map_json(&map_json) {
            Ok(parsed) => {
                terrain = parsed.terrain;
                units = ScenarioWorld::populate_units(parsed.units);

                // Combine items and structures into interactive_objects
                let mut items = ScenarioWorld::populate_items(parsed.items);
                let structures = ScenarioWorld::populate_structures(parsed.structures);
                items.extend(structures);
                interactive_objects = items;
            }
            Err(e) => {
                eprintln!("Failed to parse map JSON: {}", e);
                terrain = HashMap::new();
                units = HashMap::new();
                interactive_objects = HashMap::new();
            }
        }

        // Create a standard turn system for all scenarios
        let mut turn_system = crate::turn_system::TurnSystem::new();
        turn_system.set_team_control(Team::Player, true);
        turn_system.set_team_control(Team::Enemy, false);
        turn_system.set_team_control(Team::Neutral, false);

        Self {
            terrain,
            units,
            interactive_objects,
            pending_combat: None,
            ai_event_queue: Arc::new(Mutex::new(Vec::new())),
            turn_system,
            last_known_team: None,
        }
    }
    /// Extract detailed world state with comprehensive tactical information.
    ///
    /// This enhanced version includes health states, movement capabilities, attack ranges,
    /// terrain information, team clustering metrics, and threat assessments. Enables
    /// sophisticated AI decision-making and strategic planning.
    ///
    /// # Arguments
    ///
    /// * `team` - The team perspective for which to extract state (affects friendly/enemy categorization)
    ///
    /// # Returns
    ///
    /// AiWorldState containing detailed tactical information for all units and relevant terrain
    pub fn extract_detailed_world_state(&self, team: Team) -> AiWorldState {
        let mut ws = AiWorldState::new();

        // === TEAM METADATA ===
        ws.insert(
            "CurrentTeam".to_string(),
            AiFactValue::Str(format!("{:?}", team)),
        );

        let mut friendly_positions: Vec<(Uuid, HexCoord)> = Vec::new();
        let mut enemy_positions: Vec<(Uuid, HexCoord)> = Vec::new();

        // === UNIT INFORMATION ===
        for (id, unit) in &self.units {
            let id_str = id.to_string();
            let pos = unit.position();
            let stats = unit.unit().combat_stats();
            let is_friendly = unit.team() == team;

            // Basic position and status
            ws.insert(
                format!("Unit:{}:At", id_str),
                AiFactValue::Str(format!("{},{}", pos.q, pos.r)),
            );
            ws.insert(
                format!("Unit:{}:Alive", id_str),
                AiFactValue::Bool(unit.unit().is_alive()),
            );
            ws.insert(
                format!("Unit:{}:Team", id_str),
                AiFactValue::Str(format!("{:?}", unit.team())),
            );
            ws.insert(
                format!("Unit:{}:IsFriendly", id_str),
                AiFactValue::Bool(is_friendly),
            );

            // Health information
            ws.insert(
                format!("Unit:{}:Health", id_str),
                AiFactValue::Int(stats.health),
            );
            ws.insert(
                format!("Unit:{}:MaxHealth", id_str),
                AiFactValue::Int(stats.max_health),
            );
            let health_pct = (stats.health * 100) / stats.max_health.max(1);
            ws.insert(
                format!("Unit:{}:HealthPercent", id_str),
                AiFactValue::Int(health_pct),
            );
            ws.insert(
                format!("Unit:{}:IsWounded", id_str),
                AiFactValue::Bool(health_pct < 50),
            );

            // Movement information
            ws.insert(
                format!("Unit:{}:MovesLeft", id_str),
                AiFactValue::Int(unit.moves_left()),
            );
            ws.insert(
                format!("Unit:{}:MovementSpeed", id_str),
                AiFactValue::Int(stats.movement_speed),
            );

            // Combat information
            ws.insert(
                format!("Unit:{}:AttackPower", id_str),
                AiFactValue::Int(stats.base_attack as i32),
            );
            ws.insert(
                format!("Unit:{}:AttackedThisTurn", id_str),
                AiFactValue::Bool(stats.attacked_this_turn),
            );

            // Attack range
            let attacks = unit.unit().get_attacks();
            let max_range = attacks.iter().map(|a| a.range).max().unwrap_or(1);
            ws.insert(
                format!("Unit:{}:AttackRange", id_str),
                AiFactValue::Int(max_range),
            );

            // Threat level calculation
            let threat = self.calculate_threat_level(unit);
            ws.insert(
                format!("Unit:{}:ThreatLevel", id_str),
                AiFactValue::Int(threat),
            );

            // Terrain defense
            let defense = unit.unit().get_defense();
            ws.insert(
                format!("Unit:{}:Defense", id_str),
                AiFactValue::Int(defense as i32),
            );

            // Store positions for clustering calculations
            if is_friendly {
                friendly_positions.push((*id, pos));
            } else {
                enemy_positions.push((*id, pos));
            }
        }

        // === TERRAIN INFORMATION ===
        // Include terrain for all unit positions
        for (_, pos) in friendly_positions.iter().chain(enemy_positions.iter()) {
            if let Some(terrain_tile) = self.get_terrain(*pos) {
                let pos_key = format!("{},{}", pos.q, pos.r);
                let terrain_type = terrain_tile.sprite_type();

                ws.insert(
                    format!("Terrain:{}:Type", pos_key),
                    AiFactValue::Str(format!("{:?}", terrain_type)),
                );
                ws.insert(
                    format!("Terrain:{}:MoveCost", pos_key),
                    AiFactValue::Int(terrain_tile.movement_cost()),
                );
            }
        }

        // === CLUSTERING & DISTANCES ===
        for (id, pos) in &friendly_positions {
            let id_str = id.to_string();

            // Count nearby allies
            let nearby_allies = friendly_positions
                .iter()
                .filter(|(other_id, other_pos)| other_id != id && pos.distance(*other_pos) <= 2)
                .count();

            ws.insert(
                format!("Unit:{}:NearbyAllies", id_str),
                AiFactValue::Int(nearby_allies as i32),
            );
            ws.insert(
                format!("Unit:{}:IsIsolated", id_str),
                AiFactValue::Bool(nearby_allies == 0),
            );

            // Find nearest enemy
            if let Some(nearest_dist) = enemy_positions
                .iter()
                .map(|(_, epos)| pos.distance(*epos))
                .min()
            {
                ws.insert(
                    format!("Unit:{}:NearestEnemyDist", id_str),
                    AiFactValue::Int(nearest_dist),
                );
            }

            // Count enemies in attack range
            if let Some(unit) = self.units.get(id) {
                let attacks = unit.unit().get_attacks();
                let max_range = attacks.iter().map(|a| a.range).max().unwrap_or(1);

                let enemies_in_range = enemy_positions
                    .iter()
                    .filter(|(_, epos)| pos.distance(*epos) <= max_range)
                    .count();

                ws.insert(
                    format!("Unit:{}:EnemiesInRange", id_str),
                    AiFactValue::Int(enemies_in_range as i32),
                );
            }
        }

        // === TEAM-LEVEL METRICS ===
        ws.insert(
            "Team:AllyCount".to_string(),
            AiFactValue::Int(friendly_positions.len() as i32),
        );
        ws.insert(
            "Team:EnemyCount".to_string(),
            AiFactValue::Int(enemy_positions.len() as i32),
        );

        // Calculate average team health
        let total_health: i32 = self
            .units
            .iter()
            .filter(|(_, u)| u.team() == team)
            .map(|(_, u)| u.unit().combat_stats().health)
            .sum();
        let avg_health = if !friendly_positions.is_empty() {
            total_health / friendly_positions.len() as i32
        } else {
            0
        };
        ws.insert(
            "Team:AverageHealth".to_string(),
            AiFactValue::Int(avg_health),
        );

        ws
    }

    /// Calculate threat level of a unit based on combat capabilities and health.
    ///
    /// Threat level combines attack power, number of attacks, range, and current health
    /// to provide a single metric for prioritizing targets or assessing danger.
    ///
    /// # Arguments
    ///
    /// * `unit` - The unit to assess
    ///
    /// # Returns
    ///
    /// Integer threat level (higher = more dangerous)
    fn calculate_threat_level(&self, unit: &GameUnit) -> i32 {
        let stats = unit.unit().combat_stats();
        let attacks = unit.unit().get_attacks();

        // Base threat = attack power
        let mut threat = stats.base_attack as i32;

        // Factor in number of attacks
        threat *= stats.attacks_per_round as i32;

        // Factor in attack range (ranged units more threatening)
        let max_range = attacks.iter().map(|a| a.range).max().unwrap_or(1);
        if max_range > 1 {
            threat = (threat as f32 * 1.5) as i32;
        }

        // Factor in remaining health (wounded units less threatening)
        let health_factor = stats.health as f32 / stats.max_health.max(1) as f32;
        threat = (threat as f32 * health_factor) as i32;

        threat
    }
    pub fn generate_team_actions(&self, team: Team) -> Vec<AiActionInstance> {
        let mut out: Vec<AiActionInstance> = Vec::new();

        // Reachable calculation moved to `scenario_helpers.rs` as
        // a private `GameWorld::compute_reachable` helper.

        for (id, unit) in &self.units {
            if unit.team() != team {
                continue;
            }

            let uid_str = id.to_string();
            let pos = unit.position();
            let moves_left = unit.moves_left();

            let reachable = self.compute_reachable(*id, pos, moves_left);

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
                        // final_hit_chance = (defender.get_defense() as i32).clamp(10,95)
                        let defender_def = other_unit.unit().get_defense() as i32;
                        let final_hit_chance = defender_def.clamp(10, 95) as u8;
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
    /// Runs AI planning for the current team.
    ///
    /// # Architecture
    ///
    /// This method coordinates AI planning by:
    /// 1. Extracting world state for AI crate via `extract_detailed_world_state()`
    /// 2. Generating possible actions via `generate_team_actions()`
    /// 3. Delegating planning to AI crate via `plan_for_team()`
    /// 4. Executing planned actions (movement, combat) via ScenarioWorld methods
    ///
    /// The AI crate provides planning logic, while ScenarioWorld handles execution.
    ///
    /// # Only runs for AI-controlled teams
    pub fn run_ai_for_current_team(&mut self) {
        let current_team = self.turn_system.current_team();
        println!(
            "🤖 [AI DEBUG] run_ai_for_current_team called for team {:?}",
            current_team
        );

        if self.turn_system.is_current_team_player_controlled() {
            println!("🤖 [AI DEBUG] Team is player-controlled, skipping AI");
            return; // Player team handled by UI
        }

        // Count units on this team
        let team_units: Vec<_> = self
            .units
            .iter()
            .filter(|(_, u)| u.team() == current_team)
            .collect();
        println!("🤖 [AI DEBUG] Team has {} units", team_units.len());
        for (_id, unit) in &team_units {
            println!("🤖 [AI DEBUG]   - {} at {:?}", unit.name(), unit.position());
        }

        // Prepare AI world state and actions
        let ws = self.extract_detailed_world_state(current_team);
        println!("🤖 [AI DEBUG] World state extracted");

        let actions = self.generate_team_actions(current_team);
        println!("🤖 [AI DEBUG] Generated {} possible actions", actions.len());

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

            // Goals for agent: find closest enemy and set goal to kill them
            let mut goals: Vec<AiGoal> = Vec::new();

            // Find all enemy units and their distances
            let mut enemies_with_distance: Vec<(Uuid, &GameUnit, i32)> = Vec::new();
            for (enemy_id, enemy_unit) in &self.units {
                if enemy_unit.team() != current_team {
                    let distance = unit.position().distance(enemy_unit.position());
                    enemies_with_distance.push((*enemy_id, enemy_unit, distance));
                }
            }

            // Sort by distance (closest first)
            enemies_with_distance.sort_by_key(|(_, _, dist)| *dist);

            // Set goal to kill the closest enemy
            if let Some((enemy_id, enemy_unit, distance)) = enemies_with_distance.first() {
                goals.push(AiGoal {
                    key: format!("Unit:{}:Alive", enemy_id),
                    value: AiFactValue::Bool(false),
                });
                println!(
                    "🤖 [AI DEBUG] Unit {} targeting closest enemy {} at distance {}",
                    unit.name(),
                    enemy_unit.name(),
                    distance
                );
            }

            println!(
                "🤖 [AI DEBUG] Unit {} has {} goals",
                unit.name(),
                goals.len()
            );
            goals_per_agent.insert(aid, goals);
        }

        // Call team planner (bounded search per agent)
        println!(
            "🤖 [AI DEBUG] Calling planner for {} agents...",
            agent_order.len()
        );

        // Debug: Count action types
        let move_count = actions
            .iter()
            .filter(|a| a.name.starts_with("Move-"))
            .count();
        let attack_count = actions
            .iter()
            .filter(|a| a.name.starts_with("Attack-"))
            .count();
        println!(
            "🤖 [AI DEBUG] Action breakdown: {} moves, {} attacks",
            move_count, attack_count
        );

        // Debug: Print first few actions to see what's available
        println!("🤖 [AI DEBUG] Sample actions (first 10):");
        for (i, action) in actions.iter().take(10).enumerate() {
            println!(
                "🤖 [AI DEBUG]   {}. {} (cost: {})",
                i, action.name, action.cost
            );
        }

        // Debug: Show any attack actions if they exist
        let sample_attacks: Vec<_> = actions
            .iter()
            .filter(|a| a.name.starts_with("Attack-"))
            .take(3)
            .collect();
        if !sample_attacks.is_empty() {
            println!("🤖 [AI DEBUG] Sample attack actions:");
            for action in sample_attacks {
                println!("🤖 [AI DEBUG]   - {} (cost: {})", action.name, action.cost);
                println!(
                    "🤖 [AI DEBUG]      Preconditions: {:?}",
                    action.preconditions
                );
                println!("🤖 [AI DEBUG]      Effects: {:?}", action.effects);
            }
        } else {
            println!("🤖 [AI DEBUG] ⚠️ NO ATTACK ACTIONS GENERATED! Units may be out of range.");
        }

        // Debug: Print goals
        println!("🤖 [AI DEBUG] Goals per agent:");
        for (agent, goals) in &goals_per_agent {
            println!("🤖 [AI DEBUG]   Agent {}: {} goals", agent, goals.len());
            for goal in goals {
                println!("🤖 [AI DEBUG]      {} = {:?}", goal.key, goal.value);
            }
        }

        // Increase planner depth limit significantly
        let plans = ai::plan_for_team(&ws, &actions, &goals_per_agent, &agent_order, 5000);
        println!(
            "🤖 [AI DEBUG] Planner returned plans for {} agents",
            plans.len()
        );

        // Execute plans per agent
        // executed_actions_count removed — we end AI turn immediately after executing plans
        let mut total_actions_executed = 0;
        for (agent, plan) in plans {
            println!(
                "🤖 [AI DEBUG] Agent {} has plan with {} steps",
                agent,
                plan.len()
            );

            // find unit uuid
            if let Ok(uuid) = Uuid::parse_str(&agent) {
                if plan.is_empty() {
                    println!("🤖 [AI DEBUG] Agent {} has empty plan, using fallback: move toward nearest enemy", agent);

                    // FALLBACK: Move toward nearest enemy
                    if let Some(unit) = self.units.get(&uuid) {
                        let unit_pos = unit.position();
                        let unit_team = unit.team();

                        // Find nearest enemy
                        let mut nearest_enemy: Option<(Uuid, &GameUnit, i32)> = None;
                        for (enemy_id, enemy_unit) in &self.units {
                            if enemy_unit.team() != unit_team {
                                let distance = unit_pos.distance(enemy_unit.position());
                                if nearest_enemy.is_none() || distance < nearest_enemy.unwrap().2 {
                                    nearest_enemy = Some((*enemy_id, enemy_unit, distance));
                                }
                            }
                        }

                        if let Some((_, enemy, distance)) = nearest_enemy {
                            println!(
                                "🤖 [AI DEBUG] Fallback: Moving toward enemy at distance {}",
                                distance
                            );
                            let enemy_pos = enemy.position();

                            // Find the best move action that gets us closer
                            let agent_actions: Vec<AiActionInstance> = actions
                                .iter()
                                .filter(|a| a.agent.as_ref().map(|s| s == &agent).unwrap_or(false))
                                .cloned()
                                .collect();

                            let mut best_move: Option<(usize, i32)> = None; // (action_index, new_distance)
                            for (idx, action) in agent_actions.iter().enumerate() {
                                if action.name.starts_with("Move-") {
                                    // Extract destination from effects
                                    if let Some((_, AiFactValue::Str(dest))) =
                                        action.effects.first()
                                    {
                                        let parts: Vec<&str> = dest.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(q), Ok(r)) =
                                                (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                                            {
                                                let dest_coord = HexCoord::new(q, r);
                                                let new_distance = dest_coord.distance(enemy_pos);

                                                if best_move.is_none()
                                                    || new_distance < best_move.unwrap().1
                                                {
                                                    best_move = Some((idx, new_distance));
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Execute best move
                            if let Some((idx, new_dist)) = best_move {
                                if let Some(action) = agent_actions.get(idx) {
                                    println!("🤖 [AI DEBUG] Executing fallback move: {} (new distance: {})", action.name, new_dist);
                                    if let Some((_, AiFactValue::Str(dest))) =
                                        action.effects.first()
                                    {
                                        let parts: Vec<&str> = dest.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(q), Ok(r)) =
                                                (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                                            {
                                                let dest_coord = HexCoord::new(q, r);
                                                match self.move_unit(uuid, dest_coord) {
                                                    Ok(()) => {
                                                        println!("🤖 [AI DEBUG] Fallback move successful!");
                                                        total_actions_executed += 1;
                                                    }
                                                    Err(e) => println!(
                                                        "🤖 [AI DEBUG] Fallback move failed: {}",
                                                        e
                                                    ),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }
                // For simplicity, perform actions in plan for this agent
                let agent_actions: Vec<AiActionInstance> = actions
                    .iter()
                    .filter(|a| a.agent.as_ref().map(|s| s == &agent).unwrap_or(false))
                    .cloned()
                    .collect();
                println!(
                    "🤖 [AI DEBUG] Agent {} has {} available actions",
                    agent,
                    agent_actions.len()
                );

                for &idx in &plan {
                    if let Some(a) = agent_actions.get(idx) {
                        println!("🤖 [AI DEBUG] Executing action: {}", a.name);
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
                                        println!("🤖 [AI DEBUG] Moving to ({}, {})", q, r);
                                        // Use move_unit; ignore errors for prototype
                                        match self.move_unit(uuid, dest_coord) {
                                            Ok(()) => {
                                                println!("🤖 [AI DEBUG] Move successful!");
                                                total_actions_executed += 1;
                                            }
                                            Err(e) => println!("🤖 [AI DEBUG] Move failed: {}", e),
                                        }
                                    }
                                }
                            }
                        } else if a.name.starts_with("Attack-") {
                            // effects contain Unit:{other}:Alive=false; extract other id
                            if let Some((k, _)) = a.effects.first() {
                                if k.starts_with("Unit:") && k.ends_with(":Alive") {
                                    let mid = &k[5..k.len() - 6];
                                    if let Ok(target_uuid) = Uuid::parse_str(mid) {
                                        println!("🤖 [AI DEBUG] Attacking target {}", target_uuid);
                                        // Request combat (this will set pending_combat). If the
                                        // request fails (e.g., attacker already attacked), skip
                                        // executing combat.
                                        // request_combat now returns Ok(()) even when it silently
                                        // skips creating a pending combat (attacker already
                                        // attacked). Only execute if a pending combat was
                                        // actually created.
                                        let _ = self.request_combat(uuid, target_uuid);
                                        if self.pending_combat.is_some() {
                                            println!("🤖 [AI DEBUG] Executing combat...");
                                            // execute_pending_combat may set state; count it as an executed action
                                            let _ = self.execute_pending_combat();
                                            total_actions_executed += 1;
                                        } else {
                                            println!("🤖 [AI DEBUG] Combat request failed (unit may have already attacked)");
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        println!("🤖 [AI DEBUG] Invalid action index {} in plan", idx);
                    }
                }
            }
        }

        println!(
            "🤖 [AI DEBUG] AI executed {} total actions",
            total_actions_executed
        );
        // End AI turn after actions (use GameWorld API so unit moves are reset)
        self.end_current_turn();
    }

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

    /// Updates the world state (called each frame).
    ///
    /// Advances game time, updates all units, and processes interactions
    /// between objects at the same positions.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - Time elapsed since last update in seconds (currently unused)
    pub fn update(&mut self, _delta_time: f32) {
        // Update turn system (handles AI turn auto-pass)
        // Remember previous team so we can detect changes made by TurnSystem::update
        let prev_team = self.last_known_team;

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
            unit.update(); // TODO: Consider using delta_time parameter if needed
        }

        // NOTE: AI logic has been moved to run_ai_for_current_team() which is called
        // explicitly from the main game loop. This prevents duplicate/conflicting AI systems.

        // Old AI integration code REMOVED - now handled by run_ai_for_current_team()
        // The main application (QuestApp/main.rs) calls run_ai_for_current_team() when
        // appropriate based on turn timing.

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

    /// Returns all legal move positions for a unit.
    ///
    /// Calculates all hexes the unit can reach this turn based on:
    /// - Unit's remaining movement points
    /// - Terrain passability and costs
    /// - Other unit positions (blocking)
    ///
    /// This method first computes reachable positions, then validates each
    /// through `can_move_to` to ensure consistency with movement rules.
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to query
    ///
    /// # Returns
    ///
    /// Vector of `(HexCoord, i32)` pairs where each entry is a legal destination
    /// and its movement cost. Returns empty vector if unit not found.
    pub fn all_legal_moves(&self, unit_id: Uuid) -> Vec<(HexCoord, i32)> {
        // Get the unit
        let Some(unit) = self.units.get(&unit_id) else {
            return Vec::new();
        };

        let current_position = unit.position();
        let moves_left = unit.moves_left();

        // Calculate all reachable tiles using Dijkstra pathfinding
        let reachable = self.compute_reachable(unit_id, current_position, moves_left);

        // Validate each position through can_move_to to ensure consistency
        let mut legal_moves: Vec<(HexCoord, i32)> = Vec::new();

        for (coord, _cost) in reachable {
            // Use can_move_to to validate the move
            if let Ok(validated_cost) = self.can_move_to(unit_id, coord) {
                legal_moves.push((coord, validated_cost));
            }
        }

        // Sort by cost for consistent ordering (optional but helpful for debugging/UI)
        legal_moves.sort_by_key(|(_, cost)| *cost);

        legal_moves
    }

    /// Checks if a unit can move to the specified position.
    ///
    /// Validates movement based on:
    /// - Terrain passability and movement cost
    /// - Unit's remaining movement points
    /// - Position occupancy by other units
    /// - Distance from unit's current position (must be reachable via pathfinding)
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit attempting to move
    /// * `target_position` - Destination hex coordinate
    ///
    /// # Returns
    ///
    /// `Ok(cost)` with the movement cost if the move is valid, `Err(String)` otherwise
    pub fn can_move_to(&self, unit_id: Uuid, target_position: HexCoord) -> Result<i32, String> {
        // Get the unit
        let unit = self.units.get(&unit_id).ok_or("Unit not found")?;

        let current_position = unit.position();

        // Can't move to current position (although it's technically valid, return 0 cost)
        if current_position == target_position {
            return Ok(0);
        }

        // Check if target terrain exists and is passable
        let target_terrain = self
            .get_terrain(target_position)
            .ok_or("Target position out of bounds")?;

        if target_terrain.blocks_movement() {
            return Err("Target position is impassable".to_string());
        }

        // Check if target position is occupied by a friendly unit
        // (Enemy units are allowed - combat will be initiated)
        let moving_unit_team = unit.team();
        let units_at_target = self.get_units_at_position(target_position);
        for target_unit in units_at_target.iter() {
            if target_unit.id() != unit_id && target_unit.team() == moving_unit_team {
                return Err("Target position is occupied by friendly unit".to_string());
            }
        }

        // Calculate reachable tiles using Dijkstra pathfinding
        let moves_left = unit.moves_left();
        let reachable = self.compute_reachable(unit_id, current_position, moves_left);

        // Check if target is reachable within movement budget
        match reachable.get(&target_position) {
            Some(&cost) => Ok(cost),
            None => Err(format!(
                "Target position not reachable (requires more than {} movement)",
                moves_left
            )),
        }
    }

    /// Moves a unit to a new position.
    ///
    /// Validates the move using `can_move_to`, then updates the unit's position
    /// and consumes the appropriate movement points based on terrain costs.
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to move
    /// * `new_position` - Destination hex coordinate
    ///
    /// # Returns
    ///
    /// `Ok(())` if move succeeded, `Err(String)` with reason if it failed
    pub fn move_unit(&mut self, unit_id: Uuid, new_position: HexCoord) -> Result<(), String> {
        // Check if there's an enemy unit at the target position
        let moving_unit_team = self.units.get(&unit_id).ok_or("Unit not found")?.team();

        let units_at_target = self.get_units_at_position(new_position);
        if let Some(target_unit) = units_at_target.first() {
            // There's a unit at the target position
            if target_unit.team() != moving_unit_team {
                // It's an enemy - initiate combat instead of moving
                return self.request_combat(unit_id, target_unit.id());
            } else {
                // It's a friendly unit - can't move there
                return Err("Target position is occupied by friendly unit".to_string());
            }
        }

        // No unit at target, proceed with normal movement validation
        let movement_cost = self.can_move_to(unit_id, new_position)?;

        // Get mutable reference to the unit
        let unit = self.units.get_mut(&unit_id).ok_or("Unit not found")?;

        // Consume movement points
        if !unit.consume_moves(movement_cost) {
            return Err("Not enough movement points".to_string());
        }

        // Update position
        unit.set_position(new_position);

        Ok(())
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
            return Ok(());
        }

        let attacker_stats = attacker.unit().combat_stats();
        let defender_stats = defender.unit().combat_stats();

        // Get attacker's available attacks
        let attacker_attacks = attacker
            .unit()
            .get_attacks()
            .iter()
            .map(|attack| crate::world::AttackInfo {
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
            .map(|attack| crate::world::AttackInfo {
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
            attacker_defense: attacker_stats.resistances.slash as u32,
            attacker_attacks_per_round: attacker_stats.attacks_per_round,
            attacker_attacks,
            defender_name: defender.name(),
            defender_hp: defender_stats.health as u32,
            defender_max_hp: defender_stats.max_health as u32,
            defender_attack: defender_stats.get_total_attack(),
            defender_defense: defender_stats.resistances.slash as u32,
            defender_attacks_per_round: defender_stats.attacks_per_round,
            defender_attacks,
            selected_attack_index: 0,
        };

        self.pending_combat = Some(pending);
        Ok(())
    }

    /// Executes the pending combat after player confirmation.
    ///
    /// Retrieves the pending combat data, extracts the selected attack,
    /// and initiates the full combat sequence using the combat crate's
    /// resolve_combat function. The pending combat is consumed by this operation.
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

    /// Cancels the pending combat confirmation.
    ///
    /// Clears the pending combat state, allowing the player to take other actions.
    pub fn cancel_pending_combat(&mut self) {
        self.pending_combat = None;
    }

    /// Executes combat between two units with the selected attack.
    ///
    /// ScenarioWorld detects when two units want to fight and passes them
    /// to the Combat crate's resolve_combat function which handles all combat logic.
    ///
    /// This method:
    /// 1. Extracts the two units who want to fight
    /// 2. Passes them to combat::resolve_combat with the selected attack's damage type
    /// 3. Combat crate handles hit rolls, damage calculation, resistances, and counter-attacks
    /// 4. Processes the combat result (removes defeated units, moves winner to defender's position)
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
        let (attacker_name, defender_name, defender_pos, selected_attack) = {
            let attacker = self.units.get(&attacker_id).ok_or("Attacker not found")?;
            let defender = self.units.get(&defender_id).ok_or("Defender not found")?;

            // Prevent starting combat if the attacker already attacked this game turn
            if attacker.unit().combat_stats().attacked_this_turn {
                println!("⛔ Combat canceled: attacker has already attacked this turn");
                return Err("Attacker has already attacked this turn".to_string());
            }

            let attack = attacker
                .unit()
                .get_attacks()
                .get(selected_attack_idx)
                .ok_or("Selected attack not found")?
                .clone();

            (
                attacker.name(),
                defender.name(),
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

        // Execute combat: ScenarioWorld detects two units want to fight
        // and executes the combat logic
        let mut attacker_damage = 0;
        let mut defender_damage = 0;
        let mut attacker_hit = false;
        let mut defender_hit = false;

        // Get combat stats before battle
        let (attacker_hit_chance, defender_hit_chance, defender_counter_attack) = {
            let attacker = self.units.get(&attacker_id).ok_or("Attacker not found")?;
            let defender = self.units.get(&defender_id).ok_or("Defender not found")?;

            let attacker_hit_chance = attacker.unit().combat_stats().terrain_hit_chance;
            let defender_hit_chance = defender.unit().combat_stats().terrain_hit_chance;
            let counter = if selected_attack.range == 1 && !defender.unit().get_attacks().is_empty()
            {
                Some(defender.unit().get_attacks()[0].clone())
            } else {
                None
            };

            (attacker_hit_chance, defender_hit_chance, counter)
        };

        // Attacker's turn
        {
            let hit_roll: u8 = rand::random::<u8>() % 100;

            if hit_roll < attacker_hit_chance {
                attacker_hit = true;

                let defender = self.units.get(&defender_id).ok_or("Defender not found")?;
                let resistance = defender
                    .unit()
                    .combat_stats()
                    .resistances
                    .get_resistance(selected_attack.damage_type);
                let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
                let final_damage =
                    ((selected_attack.damage as f32 * resistance_multiplier) as i32).max(1);

                let defender = self
                    .units
                    .get_mut(&defender_id)
                    .ok_or("Defender not found")?;
                defender.unit_mut().take_damage(final_damage as u32);
                attacker_damage = final_damage as u32;
            }

            // Mark attacker as having attacked
            let attacker = self
                .units
                .get_mut(&attacker_id)
                .ok_or("Attacker not found")?;
            attacker.unit_mut().combat_stats_mut().attacked_this_turn = true;
        }

        // Defender counter-attack (only if melee range and defender is alive)
        if let Some(counter_attack) = defender_counter_attack {
            let defender = self.units.get(&defender_id).ok_or("Defender not found")?;
            if defender.unit().is_alive() {
                let hit_roll: u8 = rand::random::<u8>() % 100;

                if hit_roll < defender_hit_chance {
                    defender_hit = true;

                    let attacker = self.units.get(&attacker_id).ok_or("Attacker not found")?;
                    let resistance = attacker
                        .unit()
                        .combat_stats()
                        .resistances
                        .get_resistance(counter_attack.damage_type);
                    let resistance_multiplier = 1.0 - (resistance as f32 / 100.0);
                    let final_damage =
                        ((counter_attack.damage as f32 * resistance_multiplier) as i32).max(1);

                    let attacker = self
                        .units
                        .get_mut(&attacker_id)
                        .ok_or("Attacker not found")?;
                    attacker.unit_mut().take_damage(final_damage as u32);
                    defender_damage = final_damage as u32;
                }
            }
        }

        // Process combat results
        println!("⚔️  Combat Results:");
        if attacker_hit {
            println!("   ✓ {} hit for {} damage", attacker_name, attacker_damage);
        } else {
            println!("   ✗ {} missed", attacker_name);
        }

        if selected_attack.range > 1 {
            println!("   ⚠ {} could not counter-attack (ranged)", defender_name);
        } else if defender_hit {
            println!(
                "   ✓ {} counter-attacked for {} damage",
                defender_name, defender_damage
            );
        } else {
            println!("   ✗ {} missed counter-attack", defender_name);
        }

        // Check if defender was defeated
        let defender_defeated = {
            let defender = self.units.get(&defender_id).ok_or("Defender not found")?;
            !defender.unit().is_alive()
        };

        if defender_defeated {
            println!("💀 {} was defeated!", defender_name);
            self.remove_unit(defender_id);

            // Move attacker to defender's position
            if let Some(attacker) = self.units.get_mut(&attacker_id) {
                attacker.set_position(defender_pos);
                println!("➡️  {} moves to {:?}", attacker_name, defender_pos);
            }
        }

        // Check if attacker was defeated (rare but possible via counter-attack)
        let attacker_defeated = {
            self.units
                .get(&attacker_id)
                .is_some_and(|a| !a.unit().is_alive())
        };
        if attacker_defeated {
            println!("💀 {} was defeated by counter-attack!", attacker_name);
            self.remove_unit(attacker_id);
        }

        println!("╚════════════════════════════════════════╝\n");

        Ok(())
    }

    /// Resets movement points for all units belonging to a given team.
    fn reset_moves_for_team(&mut self, team: Team) {
        for unit in self.units.values_mut() {
            if unit.team() == team {
                unit.reset_moves_to_max();
            }
        }
    }

    /// Return the movement cost for a given hex coordinate.
    pub fn get_movement_cost(&self, position: HexCoord) -> i32 {
        if let Some(terrain) = self.get_terrain(position) {
            terrain.movement_cost()
        } else {
            i32::MAX
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
