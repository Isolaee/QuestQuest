//! Strategic goal system for multi-turn AI planning.
//!
//! This module provides a hierarchical goal framework for AI entities:
//!
//! - **ScenarioGoal**: Top-level objective derived from map/scenario definition.
//!   Represents what the AI team must accomplish to win or prevent player victory.
//!
//! - **LongTermGoal**: Multi-turn strategic goals that persist across turns.
//!   These are decomposed from ScenarioGoals based on current game state.
//!
//! - **Goal**: Single-turn tactical objectives used by the GOAP planner.
//!
//! ## Hierarchy
//! ```text
//! ScenarioGoal (e.g., "Prevent player from escorting VIP")
//!     ↓ decompose_to_strategy()
//! LongTermGoal (e.g., "Kill player units", "Block escape route")  
//!     ↓ decompose()
//! Goal (e.g., "Unit:orc1:InCombat = true")
//!     ↓ plan_for_team()
//! ActionInstance (e.g., Move, Attack)
//! ```

use crate::action::Goal;
use crate::world_state::{FactValue, HexCoord, WorldState};

// ============================================================================
// SCENARIO GOALS - Top-level objectives from map/scenario definition
// ============================================================================

/// Top-level scenario objectives that define what a team must achieve.
/// These are typically parsed from the scenario/map JSON and represent
/// the win/loss conditions for the scenario.
#[derive(Clone, Debug, PartialEq)]
pub enum ScenarioGoal {
    /// Defeat all units belonging to the target team
    DefeatAllEnemies,

    /// Prevent the player team from achieving their objective
    /// This is the typical AI goal - react to what the player is trying to do
    PreventPlayerVictory {
        /// What the player is trying to achieve (so AI can counter it)
        player_objective: Box<ScenarioGoal>,
    },

    /// Escort/protect a specific unit to a destination
    Escort {
        unit_id: String,
        destination: HexCoord,
    },

    /// Capture and hold specific locations
    CaptureObjectives { objectives: Vec<HexCoord> },

    /// Survive for a number of turns
    Survive { turns: u32 },

    /// Defend a location from attackers
    DefendLocation { location: HexCoord, radius: i32 },
}

impl ScenarioGoal {
    /// Parse a scenario goal from JSON-style type string
    pub fn from_type_string(type_str: &str) -> Option<Self> {
        match type_str {
            "DefeatAllEnemies" => Some(ScenarioGoal::DefeatAllEnemies),
            _ => None, // Extend as needed
        }
    }

    /// Decompose scenario goal into one or more long-term goals based on current state.
    /// This is where strategic AI decisions are made.
    pub fn decompose_to_strategy(&self, _state: &WorldState) -> Vec<LongTermGoal> {
        match self {
            ScenarioGoal::DefeatAllEnemies => {
                // Simple: just kill all enemies
                vec![LongTermGoal::KillAllEnemies {
                    search_radius: None,
                }]
            }

            ScenarioGoal::PreventPlayerVictory { player_objective } => {
                // React based on what the player is trying to do
                match player_objective.as_ref() {
                    ScenarioGoal::DefeatAllEnemies => {
                        // Player wants to kill us -> we kill them first
                        vec![LongTermGoal::KillPlayerUnits {
                            search_radius: None,
                        }]
                    }

                    ScenarioGoal::Escort { destination, .. } => {
                        // Player is escorting -> block the destination AND attack escort
                        vec![
                            LongTermGoal::Protect {
                                targets: vec![*destination],
                                reason: "block_escort_destination".to_string(),
                            },
                            LongTermGoal::KillPlayerUnits {
                                search_radius: None,
                            },
                        ]
                    }

                    ScenarioGoal::CaptureObjectives { objectives } => {
                        // Player wants objectives -> defend them AND attack player
                        vec![
                            LongTermGoal::Protect {
                                targets: objectives.clone(),
                                reason: "defend_objectives".to_string(),
                            },
                            LongTermGoal::KillPlayerUnits {
                                search_radius: Some(10),
                            },
                        ]
                    }

                    ScenarioGoal::Survive { .. } => {
                        // Player just needs to survive -> aggressive attack
                        vec![LongTermGoal::KillPlayerUnits {
                            search_radius: None,
                        }]
                    }

                    ScenarioGoal::DefendLocation { location, radius } => {
                        // Player is defending -> siege their position
                        vec![LongTermGoal::ReachArea {
                            area_centers: vec![*location],
                            reason: format!("attack_defended_area_radius_{}", radius),
                        }]
                    }

                    ScenarioGoal::PreventPlayerVictory { .. } => {
                        // Recursive prevention doesn't make sense, default to combat
                        vec![LongTermGoal::KillAllEnemies {
                            search_radius: None,
                        }]
                    }
                }
            }

            ScenarioGoal::Escort { destination, .. } => {
                // Move toward destination while protecting the escort target
                vec![LongTermGoal::ReachArea {
                    area_centers: vec![*destination],
                    reason: "escort_destination".to_string(),
                }]
            }

            ScenarioGoal::CaptureObjectives { objectives } => {
                vec![LongTermGoal::ReachArea {
                    area_centers: objectives.clone(),
                    reason: "capture_objectives".to_string(),
                }]
            }

            ScenarioGoal::Survive { .. } => {
                // Defensive posture - engage enemies that get close
                vec![LongTermGoal::KillAllEnemies {
                    search_radius: Some(5),
                }]
            }

            ScenarioGoal::DefendLocation { location, .. } => {
                vec![LongTermGoal::Protect {
                    targets: vec![*location],
                    reason: "defend_location".to_string(),
                }]
            }
        }
    }

    /// Check if the scenario goal has been achieved
    pub fn is_achieved(&self, state: &WorldState) -> bool {
        match self {
            ScenarioGoal::DefeatAllEnemies => {
                // Check if enemy unit count is 0
                if let Some(FactValue::Int(count)) = state.get("Global:EnemyUnitCount") {
                    return *count == 0;
                }
                false
            }

            ScenarioGoal::PreventPlayerVictory {
                player_objective: _,
            } => {
                // We win if player loses (their objective becomes impossible)
                // This is complex - for now check if player units are gone
                if let Some(FactValue::Int(count)) = state.get("Global:PlayerUnitCount") {
                    return *count == 0;
                }
                false
            }

            ScenarioGoal::Escort {
                unit_id,
                destination,
            } => {
                // Check if escort unit reached destination
                let pos_key = format!("Unit:{}:At", unit_id);
                if let Some(FactValue::Str(pos)) = state.get(&pos_key) {
                    if let Some((q, r)) = parse_position(pos) {
                        let current = HexCoord { q, r };
                        return current.distance(*destination) == 0;
                    }
                }
                false
            }

            ScenarioGoal::CaptureObjectives { objectives } => {
                // Check if all objectives are captured (simplified)
                objectives.iter().all(|obj| {
                    let key = format!("Objective:{}_{}: Captured", obj.q, obj.r);
                    state.get(&key) == Some(&FactValue::Bool(true))
                })
            }

            ScenarioGoal::Survive { turns } => {
                // Check if enough turns have passed
                if let Some(FactValue::Int(current_turn)) = state.get("Global:TurnNumber") {
                    return *current_turn >= *turns as i32;
                }
                false
            }

            ScenarioGoal::DefendLocation { location, radius } => {
                // Check if no enemies in the defended area
                // This would require spatial queries - simplified here
                let key = format!("Area:{}_{}_{}:EnemyCount", location.q, location.r, radius);
                if let Some(FactValue::Int(count)) = state.get(&key) {
                    return *count == 0;
                }
                true // Default to defended if no info
            }
        }
    }

    /// Priority for this scenario goal (for multi-objective scenarios)
    pub fn priority(&self) -> f32 {
        match self {
            ScenarioGoal::DefeatAllEnemies => 60.0,
            ScenarioGoal::PreventPlayerVictory { .. } => 80.0, // AI's main job
            ScenarioGoal::Escort { .. } => 70.0,
            ScenarioGoal::CaptureObjectives { .. } => 65.0,
            ScenarioGoal::Survive { .. } => 50.0,
            ScenarioGoal::DefendLocation { .. } => 55.0,
        }
    }
}

// ============================================================================
// STRATEGIES - Different approaches to achieving scenario goals
// ============================================================================

/// Strategic approach that determines HOW to pursue scenario goals.
/// Multiple strategies can achieve the same goal with different playstyles.
#[derive(Clone, Debug, PartialEq)]
pub enum Strategy {
    /// Aggressive: Attack enemies directly, prioritize damage
    /// Best when: Numerical advantage, high damage units, enemy is weak
    Aggressive {
        /// Focus fire on weakest targets first
        focus_weak: bool,
        /// Maximum distance to chase enemies
        pursuit_range: Option<i32>,
    },

    /// Defensive: Hold position, let enemies come to you
    /// Best when: Defensive terrain advantage, ranged units, outnumbered
    Defensive {
        /// Position to defend around
        anchor_point: HexCoord,
        /// Maximum distance to move from anchor
        hold_radius: i32,
    },

    /// Guerrilla: Hit and run tactics, avoid prolonged engagement
    /// Best when: Fast units, low HP, facing stronger enemies
    Guerrilla {
        /// Retreat after this many attacks
        disengage_after_attacks: i32,
        /// Minimum safe distance to maintain
        safe_distance: i32,
    },

    /// Flanking: Attack from multiple directions
    /// Best when: Multiple units, enemy has strong front line
    Flanking {
        /// Primary attack direction
        main_force_direction: HexCoord,
        /// Flanking positions to reach
        flank_targets: Vec<HexCoord>,
    },

    /// Focused: Concentrate all units on single objective
    /// Best when: Need to break through, time pressure
    Focused {
        /// The primary target (position or unit)
        target: HexCoord,
        /// All units converge here
        converge: bool,
    },

    /// Balanced: Mix of offense and defense
    /// Best when: Uncertain situation, need flexibility
    Balanced {
        /// Portion of units for attack (0.0-1.0)
        attack_ratio: f32,
        /// Portion of units for defense (0.0-1.0)
        defense_ratio: f32,
    },

    /// Attrition: Slowly wear down enemy while preserving own forces
    /// Best when: Stronger in long fights, enemy has limited resources
    Attrition {
        /// Only engage when advantage is clear
        min_advantage_ratio: f32,
        /// Retreat threshold (HP percentage)
        retreat_hp_threshold: f32,
    },

    /// Objective: Ignore enemies, rush to objective
    /// Best when: Time-limited scenario, objective is undefended
    ObjectiveRush {
        /// The objective to reach
        objective: HexCoord,
        /// Ignore enemies within this range (just run past)
        ignore_enemies_radius: i32,
    },
}

impl Strategy {
    /// Evaluate how suitable this strategy is for the current situation.
    /// Returns a score 0.0-1.0 where higher is better.
    pub fn evaluate_fitness(&self, state: &WorldState, team_id: &str) -> f32 {
        let our_units = get_team_unit_count(state, team_id);
        let enemy_units = get_enemy_unit_count(state, team_id);
        let our_avg_hp = get_team_average_hp(state, team_id);
        let enemy_avg_hp = get_enemy_average_hp(state, team_id);

        match self {
            Strategy::Aggressive { .. } => {
                // Good when we have advantage
                let unit_ratio = our_units as f32 / enemy_units.max(1) as f32;
                let hp_ratio = our_avg_hp / enemy_avg_hp.max(1.0);
                ((unit_ratio * 0.6 + hp_ratio * 0.4) / 2.0).min(1.0)
            }

            Strategy::Defensive { .. } => {
                // Good when outnumbered or have terrain advantage
                let unit_ratio = enemy_units as f32 / our_units.max(1) as f32;
                (unit_ratio * 0.5).min(1.0)
            }

            Strategy::Guerrilla { .. } => {
                // Good when we have fewer, faster units
                let disadvantage = enemy_units as f32 / our_units.max(1) as f32;
                if disadvantage > 1.5 {
                    0.8
                } else {
                    0.3
                }
            }

            Strategy::Flanking { .. } => {
                // Need at least 3 units to flank effectively
                if our_units >= 3 {
                    0.7
                } else {
                    0.2
                }
            }

            Strategy::Focused { .. } => {
                // Always somewhat viable
                0.5
            }

            Strategy::Balanced { .. } => {
                // Safe default, always moderately good
                0.6
            }

            Strategy::Attrition { .. } => {
                // Good when we have HP advantage
                let hp_ratio = our_avg_hp / enemy_avg_hp.max(1.0);
                (hp_ratio * 0.5).min(1.0)
            }

            Strategy::ObjectiveRush { .. } => {
                // Situational - depends on objective distance and blocking enemies
                0.4
            }
        }
    }

    /// Convert strategy into concrete LongTermGoals for execution.
    pub fn to_long_term_goals(&self, state: &WorldState, _team_id: &str) -> Vec<LongTermGoal> {
        match self {
            Strategy::Aggressive {
                focus_weak: _,
                pursuit_range,
            } => {
                vec![LongTermGoal::KillPlayerUnits {
                    search_radius: *pursuit_range,
                }]
            }

            Strategy::Defensive {
                anchor_point,
                hold_radius: _,
            } => {
                vec![
                    LongTermGoal::Protect {
                        targets: vec![*anchor_point],
                        reason: "defensive_anchor".to_string(),
                    },
                    LongTermGoal::KillAllEnemies {
                        search_radius: Some(5),
                    },
                ]
            }

            Strategy::Guerrilla { safe_distance, .. } => {
                // Attack but maintain distance
                vec![LongTermGoal::KillAllEnemies {
                    search_radius: Some(*safe_distance + 3),
                }]
            }

            Strategy::Flanking {
                flank_targets,
                main_force_direction: _,
            } => {
                let mut goals = vec![LongTermGoal::KillPlayerUnits {
                    search_radius: None,
                }];
                if !flank_targets.is_empty() {
                    goals.push(LongTermGoal::ReachArea {
                        area_centers: flank_targets.clone(),
                        reason: "flanking_position".to_string(),
                    });
                }
                goals
            }

            Strategy::Focused { target, .. } => {
                vec![LongTermGoal::ReachArea {
                    area_centers: vec![*target],
                    reason: "focused_assault".to_string(),
                }]
            }

            Strategy::Balanced { .. } => {
                vec![
                    LongTermGoal::KillAllEnemies {
                        search_radius: Some(8),
                    },
                    LongTermGoal::Protect {
                        targets: get_friendly_positions(state),
                        reason: "balanced_defense".to_string(),
                    },
                ]
            }

            Strategy::Attrition {
                retreat_hp_threshold: _,
                ..
            } => {
                // Careful aggression
                vec![LongTermGoal::KillAllEnemies {
                    search_radius: Some(6),
                }]
            }

            Strategy::ObjectiveRush { objective, .. } => {
                vec![LongTermGoal::ReachArea {
                    area_centers: vec![*objective],
                    reason: "objective_rush".to_string(),
                }]
            }
        }
    }

    /// Select the best strategy for the current situation.
    pub fn select_best(state: &WorldState, team_id: &str, candidates: &[Strategy]) -> Strategy {
        candidates
            .iter()
            .max_by(|a, b| {
                let score_a = a.evaluate_fitness(state, team_id);
                let score_b = b.evaluate_fitness(state, team_id);
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
            .unwrap_or(Strategy::Balanced {
                attack_ratio: 0.5,
                defense_ratio: 0.5,
            })
    }

    /// Generate default strategy candidates based on scenario goal.
    pub fn candidates_for_goal(goal: &ScenarioGoal) -> Vec<Strategy> {
        match goal {
            ScenarioGoal::DefeatAllEnemies => vec![
                Strategy::Aggressive {
                    focus_weak: true,
                    pursuit_range: None,
                },
                Strategy::Flanking {
                    main_force_direction: HexCoord { q: 0, r: 0 },
                    flank_targets: vec![],
                },
                Strategy::Balanced {
                    attack_ratio: 0.7,
                    defense_ratio: 0.3,
                },
            ],

            ScenarioGoal::PreventPlayerVictory { player_objective } => {
                match player_objective.as_ref() {
                    ScenarioGoal::DefeatAllEnemies => vec![
                        Strategy::Aggressive {
                            focus_weak: true,
                            pursuit_range: None,
                        },
                        Strategy::Defensive {
                            anchor_point: HexCoord { q: 0, r: 0 },
                            hold_radius: 5,
                        },
                        Strategy::Attrition {
                            min_advantage_ratio: 1.2,
                            retreat_hp_threshold: 0.3,
                        },
                    ],

                    ScenarioGoal::Escort { destination, .. } => vec![
                        Strategy::Focused {
                            target: *destination,
                            converge: true,
                        },
                        Strategy::Aggressive {
                            focus_weak: false, // Target the escort, not weak units
                            pursuit_range: Some(15),
                        },
                    ],

                    ScenarioGoal::CaptureObjectives { objectives } => vec![
                        Strategy::Defensive {
                            anchor_point: objectives
                                .first()
                                .copied()
                                .unwrap_or(HexCoord { q: 0, r: 0 }),
                            hold_radius: 3,
                        },
                        Strategy::Balanced {
                            attack_ratio: 0.4,
                            defense_ratio: 0.6,
                        },
                    ],

                    ScenarioGoal::Survive { .. } => vec![
                        Strategy::Aggressive {
                            focus_weak: true,
                            pursuit_range: None,
                        },
                        Strategy::Flanking {
                            main_force_direction: HexCoord { q: 0, r: 0 },
                            flank_targets: vec![],
                        },
                    ],

                    _ => vec![Strategy::Balanced {
                        attack_ratio: 0.5,
                        defense_ratio: 0.5,
                    }],
                }
            }

            ScenarioGoal::Escort { destination, .. } => vec![
                Strategy::ObjectiveRush {
                    objective: *destination,
                    ignore_enemies_radius: 2,
                },
                Strategy::Balanced {
                    attack_ratio: 0.3,
                    defense_ratio: 0.7,
                },
            ],

            ScenarioGoal::CaptureObjectives { objectives } => vec![
                Strategy::ObjectiveRush {
                    objective: objectives
                        .first()
                        .copied()
                        .unwrap_or(HexCoord { q: 0, r: 0 }),
                    ignore_enemies_radius: 3,
                },
                Strategy::Flanking {
                    main_force_direction: HexCoord { q: 0, r: 0 },
                    flank_targets: objectives.clone(),
                },
            ],

            ScenarioGoal::Survive { .. } => vec![
                Strategy::Defensive {
                    anchor_point: HexCoord { q: 0, r: 0 },
                    hold_radius: 4,
                },
                Strategy::Guerrilla {
                    disengage_after_attacks: 1,
                    safe_distance: 5,
                },
            ],

            ScenarioGoal::DefendLocation { location, radius } => vec![
                Strategy::Defensive {
                    anchor_point: *location,
                    hold_radius: *radius,
                },
                Strategy::Attrition {
                    min_advantage_ratio: 1.0,
                    retreat_hp_threshold: 0.2,
                },
            ],
        }
    }
}

// ============================================================================
// STRATEGY HELPER FUNCTIONS
// ============================================================================

fn get_team_unit_count(state: &WorldState, team_id: &str) -> i32 {
    let key = format!("Team:{}:UnitCount", team_id);
    if let Some(FactValue::Int(count)) = state.get(&key) {
        return *count;
    }
    // Fallback to checking individual units
    1
}

fn get_enemy_unit_count(state: &WorldState, team_id: &str) -> i32 {
    let key = format!("Team:{}:EnemyCount", team_id);
    if let Some(FactValue::Int(count)) = state.get(&key) {
        return *count;
    }
    if let Some(FactValue::Int(count)) = state.get("Global:PlayerUnitCount") {
        return *count;
    }
    1
}

fn get_team_average_hp(state: &WorldState, team_id: &str) -> f32 {
    let key = format!("Team:{}:AverageHP", team_id);
    if let Some(FactValue::Int(hp)) = state.get(&key) {
        return *hp as f32;
    }
    50.0
}

fn get_enemy_average_hp(state: &WorldState, team_id: &str) -> f32 {
    let key = format!("Team:{}:EnemyAverageHP", team_id);
    if let Some(FactValue::Int(hp)) = state.get(&key) {
        return *hp as f32;
    }
    50.0
}

fn get_friendly_positions(state: &WorldState) -> Vec<HexCoord> {
    // This would scan state for friendly unit positions
    // Simplified: return empty for now
    let _ = state;
    vec![]
}

// ============================================================================
// LONG-TERM GOALS - Multi-turn strategic goals
// ============================================================================

/// Long-term strategic goals that span multiple turns.
#[derive(Clone, Debug, PartialEq)]
pub enum LongTermGoal {
    /// Kill all enemies in the area
    KillAllEnemies {
        search_radius: Option<i32>, // None = unlimited range
    },

    /// Kill all player-controlled units (used by AI teams)
    KillPlayerUnits {
        search_radius: Option<i32>, // None = unlimited range
    },

    /// Protect area
    Protect {
        targets: Vec<HexCoord>,
        reason: String,
    },

    /// Reach area
    ReachArea {
        area_centers: Vec<HexCoord>,
        reason: String,
    },

    /// Siege castle
    SiegeCastle { castle_id: String },
}

impl std::fmt::Display for LongTermGoal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LongTermGoal::KillAllEnemies { search_radius } => {
                write!(
                    f,
                    "KillAllEnemies:{}",
                    search_radius
                        .map(|r| r.to_string())
                        .unwrap_or_else(|| "unlimited".to_string())
                )
            }
            LongTermGoal::KillPlayerUnits { search_radius } => {
                write!(
                    f,
                    "KillPlayerUnits:{}",
                    search_radius
                        .map(|r| r.to_string())
                        .unwrap_or_else(|| "unlimited".to_string())
                )
            }
            LongTermGoal::Protect { targets, reason } => {
                let targets_str = targets
                    .iter()
                    .map(|t| format!("{},{}", t.q, t.r))
                    .collect::<Vec<_>>()
                    .join(";");
                write!(f, "Protect:{}:{}", targets_str, reason)
            }
            LongTermGoal::ReachArea {
                area_centers,
                reason,
            } => {
                let centers_str = area_centers
                    .iter()
                    .map(|c| format!("{},{}", c.q, c.r))
                    .collect::<Vec<_>>()
                    .join(";");
                write!(f, "ReachArea:{}:{}", centers_str, reason)
            }
            LongTermGoal::SiegeCastle { castle_id } => {
                write!(f, "SiegeCastle:{}", castle_id)
            }
        }
    }
}

impl LongTermGoal {
    /// Parse string representation back to LongTermGoal.
    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() < 2 {
            return None;
        }

        match parts[0] {
            "KillAllEnemies" => {
                let search_radius = if parts[1] == "unlimited" {
                    None
                } else {
                    Some(parts[1].parse().ok()?)
                };
                Some(LongTermGoal::KillAllEnemies { search_radius })
            }
            "KillPlayerUnits" => {
                let search_radius = if parts[1] == "unlimited" {
                    None
                } else {
                    Some(parts[1].parse().ok()?)
                };
                Some(LongTermGoal::KillPlayerUnits { search_radius })
            }
            "Protect" => {
                let data: Vec<&str> = parts[1].splitn(2, ':').collect();
                if data.len() >= 2 {
                    let targets: Option<Vec<HexCoord>> = data[0]
                        .split(';')
                        .map(|t| {
                            let coords: Vec<&str> = t.split(',').collect();
                            if coords.len() == 2 {
                                Some(HexCoord {
                                    q: coords[0].parse().ok()?,
                                    r: coords[1].parse().ok()?,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    Some(LongTermGoal::Protect {
                        targets: targets?,
                        reason: data[1].to_string(),
                    })
                } else {
                    None
                }
            }
            "ReachArea" => {
                let data: Vec<&str> = parts[1].splitn(2, ':').collect();
                if data.len() >= 2 {
                    let area_centers: Option<Vec<HexCoord>> = data[0]
                        .split(';')
                        .map(|c| {
                            let coords: Vec<&str> = c.split(',').collect();
                            if coords.len() == 2 {
                                Some(HexCoord {
                                    q: coords[0].parse().ok()?,
                                    r: coords[1].parse().ok()?,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    Some(LongTermGoal::ReachArea {
                        area_centers: area_centers?,
                        reason: data[1].to_string(),
                    })
                } else {
                    None
                }
            }
            "SiegeCastle" => Some(LongTermGoal::SiegeCastle {
                castle_id: parts[1].to_string(),
            }),
            _ => None,
        }
    }

    /// Decompose long-term goal into a short-term goal for the current turn.
    pub fn decompose(&self, state: &WorldState, unit_id: &str) -> Option<Goal> {
        match self {
            LongTermGoal::KillAllEnemies { search_radius } => {
                decompose_kill_all_enemies(state, unit_id, *search_radius)
            }
            LongTermGoal::KillPlayerUnits { search_radius } => {
                decompose_kill_player_units(state, unit_id, *search_radius)
            }
            LongTermGoal::Protect { targets, .. } => decompose_protect(state, unit_id, targets),
            LongTermGoal::ReachArea { area_centers, .. } => {
                decompose_reach_area(state, unit_id, area_centers)
            }
            LongTermGoal::SiegeCastle { castle_id } => {
                decompose_siege_castle(state, unit_id, castle_id)
            }
        }
    }

    /// Check if the long-term goal has been achieved.
    pub fn is_achieved(&self, state: &WorldState, unit_id: &str) -> bool {
        match self {
            LongTermGoal::KillAllEnemies { search_radius } => {
                are_all_enemies_dead(state, unit_id, *search_radius)
            }
            LongTermGoal::KillPlayerUnits { search_radius } => {
                are_all_player_units_dead(state, *search_radius)
            }
            LongTermGoal::Protect { targets, .. } => is_protecting_area(state, unit_id, targets),
            LongTermGoal::ReachArea { area_centers, .. } => {
                is_in_area(state, unit_id, area_centers)
            }
            LongTermGoal::SiegeCastle { castle_id } => is_castle_captured(state, castle_id),
        }
    }

    /// Get a priority score for this goal (higher = more urgent).
    pub fn priority(&self) -> f32 {
        match self {
            LongTermGoal::KillAllEnemies { .. } => 65.0,
            LongTermGoal::KillPlayerUnits { .. } => 70.0, // Higher priority - targeting player
            LongTermGoal::SiegeCastle { .. } => 55.0,
            LongTermGoal::Protect { .. } => 50.0,
            LongTermGoal::ReachArea { .. } => 30.0,
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn decompose_reach_position(state: &WorldState, unit_id: &str, target: HexCoord) -> Option<Goal> {
    let pos_key = format!("Unit:{}:At", unit_id);
    if let Some(FactValue::Str(pos_str)) = state.get(&pos_key) {
        if let Some((q_str, r_str)) = pos_str.split_once(',') {
            if let (Ok(q), Ok(r)) = (q_str.parse::<i32>(), r_str.parse::<i32>()) {
                let current = HexCoord { q, r };
                let distance = current.distance(target);

                if distance <= 3 {
                    return Some(Goal {
                        key: pos_key,
                        value: FactValue::Str(format!("{},{}", target.q, target.r)),
                    });
                } else {
                    let intermediate = calculate_intermediate_position(current, target);
                    return Some(Goal {
                        key: pos_key,
                        value: FactValue::Str(format!("{},{}", intermediate.q, intermediate.r)),
                    });
                }
            }
        }
    }
    None
}

fn decompose_kill_all_enemies(
    state: &WorldState,
    unit_id: &str,
    _search_radius: Option<i32>,
) -> Option<Goal> {
    let enemies_key = format!("Unit:{}:NearbyEnemies", unit_id);
    if let Some(FactValue::Int(enemy_count)) = state.get(&enemies_key) {
        if *enemy_count > 0 {
            return Some(Goal {
                key: format!("Unit:{}:InCombat", unit_id),
                value: FactValue::Bool(true),
            });
        }
    }
    None
}

fn decompose_kill_player_units(
    state: &WorldState,
    unit_id: &str,
    _search_radius: Option<i32>,
) -> Option<Goal> {
    // Check for nearby player-controlled units
    let player_units_key = format!("Unit:{}:NearbyPlayerUnits", unit_id);
    if let Some(FactValue::Int(player_count)) = state.get(&player_units_key) {
        if *player_count > 0 {
            return Some(Goal {
                key: format!("Unit:{}:InCombat", unit_id),
                value: FactValue::Bool(true),
            });
        }
    }

    // Fallback: check for closest player unit position and move toward it
    let closest_player_key = format!("Unit:{}:ClosestPlayerUnit", unit_id);
    if let Some(FactValue::Str(player_pos)) = state.get(&closest_player_key) {
        if let Some((q, r)) = parse_position(player_pos) {
            let target = HexCoord { q, r };
            return decompose_reach_position(state, unit_id, target);
        }
    }

    None
}

fn decompose_protect(state: &WorldState, unit_id: &str, targets: &[HexCoord]) -> Option<Goal> {
    if targets.is_empty() {
        return None;
    }

    // Find closest target position to move toward
    let pos_key = format!("Unit:{}:At", unit_id);
    if let Some(FactValue::Str(pos_str)) = state.get(&pos_key) {
        if let Some((q, r)) = parse_position(pos_str) {
            let current = HexCoord { q, r };

            // Find closest target
            let closest = targets.iter().min_by_key(|t| current.distance(**t))?;

            let distance = current.distance(*closest);

            // If not at any target, move to closest
            if distance > 0 {
                return decompose_reach_position(state, unit_id, *closest);
            }

            // At a target - check for enemies and engage if present
            let enemies_key = format!("Unit:{}:NearbyEnemies", unit_id);
            if let Some(FactValue::Int(enemy_count)) = state.get(&enemies_key) {
                if *enemy_count > 0 {
                    return Some(Goal {
                        key: format!("Unit:{}:InCombat", unit_id),
                        value: FactValue::Bool(true),
                    });
                }
            }

            // Hold position
            return Some(Goal {
                key: pos_key,
                value: FactValue::Str(pos_str.to_string()),
            });
        }
    }
    None
}

fn are_all_enemies_dead(state: &WorldState, unit_id: &str, _search_radius: Option<i32>) -> bool {
    let enemies_key = format!("Unit:{}:NearbyEnemies", unit_id);
    if let Some(FactValue::Int(enemy_count)) = state.get(&enemies_key) {
        return *enemy_count == 0;
    }
    true
}

fn are_all_player_units_dead(state: &WorldState, _search_radius: Option<i32>) -> bool {
    // Check global player unit count
    if let Some(FactValue::Int(player_count)) = state.get("Global:PlayerUnitCount") {
        return *player_count == 0;
    }
    // If no count is available, assume goal is not achieved
    false
}

fn is_protecting_area(state: &WorldState, unit_id: &str, targets: &[HexCoord]) -> bool {
    if targets.is_empty() {
        return false;
    }

    let pos_key = format!("Unit:{}:At", unit_id);
    if let Some(FactValue::Str(pos_str)) = state.get(&pos_key) {
        if let Some((q, r)) = parse_position(pos_str) {
            let current = HexCoord { q, r };
            // Check if at any of the target positions
            return targets.iter().any(|t| current.distance(*t) == 0);
        }
    }
    false
}

fn decompose_reach_area(
    state: &WorldState,
    unit_id: &str,
    area_centers: &[HexCoord],
) -> Option<Goal> {
    if area_centers.is_empty() {
        return None;
    }

    // Find closest area center
    let pos_key = format!("Unit:{}:At", unit_id);
    if let Some(FactValue::Str(pos_str)) = state.get(&pos_key) {
        if let Some((q, r)) = parse_position(pos_str) {
            let current = HexCoord { q, r };

            let closest = area_centers
                .iter()
                .min_by_key(|center| current.distance(**center))?;

            // Move toward closest area center
            return decompose_reach_position(state, unit_id, *closest);
        }
    }
    None
}

fn decompose_siege_castle(state: &WorldState, unit_id: &str, castle_id: &str) -> Option<Goal> {
    // Get castle position
    let castle_pos_key = format!("Castle:{}:At", castle_id);
    if let Some(FactValue::Str(castle_pos)) = state.get(&castle_pos_key) {
        if let Some((cq, cr)) = parse_position(castle_pos) {
            let castle_coord = HexCoord { q: cq, r: cr };

            let pos_key = format!("Unit:{}:At", unit_id);
            if let Some(FactValue::Str(unit_pos)) = state.get(&pos_key) {
                if let Some((uq, ur)) = parse_position(unit_pos) {
                    let current = HexCoord { q: uq, r: ur };
                    let distance = current.distance(castle_coord);

                    // Check if castle is already under siege
                    let siege_key = format!("Castle:{}:UnderSiege", castle_id);
                    let under_siege = state.get(&siege_key) == Some(&FactValue::Bool(true));

                    if distance <= 2 && !under_siege {
                        // Adjacent to castle - begin siege
                        return Some(Goal {
                            key: siege_key,
                            value: FactValue::Bool(true),
                        });
                    } else if under_siege {
                        // Castle under siege - attack it
                        let castle_hp_key = format!("Castle:{}:HP", castle_id);
                        if let Some(FactValue::Int(hp)) = state.get(&castle_hp_key) {
                            if *hp > 0 {
                                return Some(Goal {
                                    key: castle_hp_key,
                                    value: FactValue::Int(0),
                                });
                            }
                        }
                    } else {
                        // Too far - move closer
                        return decompose_reach_position(state, unit_id, castle_coord);
                    }
                }
            }
        }
    }
    None
}

fn is_in_area(state: &WorldState, unit_id: &str, area_centers: &[HexCoord]) -> bool {
    if area_centers.is_empty() {
        return false;
    }

    let pos_key = format!("Unit:{}:At", unit_id);
    if let Some(FactValue::Str(pos_str)) = state.get(&pos_key) {
        if let Some((q, r)) = parse_position(pos_str) {
            let current = HexCoord { q, r };
            // Check if within reasonable distance of any area center (e.g., 3 hexes)
            return area_centers
                .iter()
                .any(|center| current.distance(*center) <= 3);
        }
    }
    false
}

fn is_castle_captured(state: &WorldState, castle_id: &str) -> bool {
    // Check if castle HP is 0 or if it's captured
    let hp_key = format!("Castle:{}:HP", castle_id);
    if let Some(FactValue::Int(hp)) = state.get(&hp_key) {
        if *hp <= 0 {
            return true;
        }
    }

    let captured_key = format!("Castle:{}:Captured", castle_id);
    state.get(&captured_key) == Some(&FactValue::Bool(true))
}

fn calculate_intermediate_position(current: HexCoord, target: HexCoord) -> HexCoord {
    let dq = target.q - current.q;
    let dr = target.r - current.r;

    let step_q = if dq.abs() > 3 { dq.signum() * 3 } else { dq };
    let step_r = if dr.abs() > 3 { dr.signum() * 3 } else { dr };

    HexCoord {
        q: current.q + step_q,
        r: current.r + step_r,
    }
}

fn parse_position(pos: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = pos.split(',').collect();
    if parts.len() == 2 {
        let q = parts[0].parse().ok()?;
        let r = parts[1].parse().ok()?;
        Some((q, r))
    } else {
        None
    }
}
