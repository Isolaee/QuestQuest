//! Long-term strategic goals for multi-turn planning.
//!
//! This module defines goal types that span multiple turns and provides
//! utilities for decomposing them into achievable short-term objectives.

use crate::action::Goal;
use crate::world_state::{FactValue, HexCoord, WorldState};

/// Long-term strategic goals that can span multiple turns.
///
/// These goals represent high-level objectives that may require several turns
/// to complete, enabling AI units to plan strategically rather than reactively.
#[derive(Clone, Debug, PartialEq)]
pub enum LongTermGoal {
    /// Move to a specific position over multiple turns
    ReachPosition { target: HexCoord, reason: String },

    /// Eliminate a specific enemy unit
    EliminateTarget { target_id: String },

    /// Maintain formation with nearby allies
    StayInFormation { max_distance_from_allies: i32 },

    /// Control a specific area of the map
    ControlZone { center: HexCoord, radius: i32 },

    /// Protect a specific ally unit
    ProtectAlly { ally_id: String, max_distance: i32 },

    /// Custom goal with description and fact-based objective
    Custom { description: String, goal: Goal },
}

impl std::fmt::Display for LongTermGoal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LongTermGoal::ReachPosition { target, reason } => {
                write!(f, "ReachPosition:{},{}:{}", target.q, target.r, reason)
            }
            LongTermGoal::EliminateTarget { target_id } => {
                write!(f, "EliminateTarget:{}", target_id)
            }
            LongTermGoal::StayInFormation {
                max_distance_from_allies,
            } => {
                write!(f, "StayInFormation:{}", max_distance_from_allies)
            }
            LongTermGoal::ControlZone { center, radius } => {
                write!(f, "ControlZone:{},{}:{}", center.q, center.r, radius)
            }
            LongTermGoal::ProtectAlly {
                ally_id,
                max_distance,
            } => {
                write!(f, "ProtectAlly:{}:{}", ally_id, max_distance)
            }
            LongTermGoal::Custom {
                description,
                goal: _,
            } => {
                write!(f, "Custom:{}", description)
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
            "ReachPosition" => {
                let data: Vec<&str> = parts[1].splitn(3, [',', ':']).collect();
                if data.len() >= 3 {
                    let q = data[0].parse().ok()?;
                    let r = data[1].parse().ok()?;
                    let reason = data.get(2).unwrap_or(&"").to_string();
                    Some(LongTermGoal::ReachPosition {
                        target: HexCoord { q, r },
                        reason,
                    })
                } else {
                    None
                }
            }
            "EliminateTarget" => Some(LongTermGoal::EliminateTarget {
                target_id: parts[1].to_string(),
            }),
            "StayInFormation" => {
                let max_distance = parts[1].parse().ok()?;
                Some(LongTermGoal::StayInFormation {
                    max_distance_from_allies: max_distance,
                })
            }
            "ControlZone" => {
                let data: Vec<&str> = parts[1].splitn(3, [',', ':']).collect();
                if data.len() >= 3 {
                    let q = data[0].parse().ok()?;
                    let r = data[1].parse().ok()?;
                    let radius = data[2].parse().ok()?;
                    Some(LongTermGoal::ControlZone {
                        center: HexCoord { q, r },
                        radius,
                    })
                } else {
                    None
                }
            }
            "ProtectAlly" => {
                let data: Vec<&str> = parts[1].splitn(2, ':').collect();
                if data.len() >= 2 {
                    let ally_id = data[0].to_string();
                    let max_distance = data[1].parse().ok()?;
                    Some(LongTermGoal::ProtectAlly {
                        ally_id,
                        max_distance,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Decompose long-term goal into a short-term goal for the current turn.
    ///
    /// # Arguments
    ///
    /// * `state` - Current world state
    /// * `unit_id` - ID of the unit pursuing this goal
    ///
    /// # Returns
    ///
    /// A short-term Goal that moves the unit toward the long-term objective
    pub fn decompose(&self, state: &WorldState, unit_id: &str) -> Option<Goal> {
        match self {
            LongTermGoal::ReachPosition { target, .. } => {
                // Get current position
                let pos_key = format!("Unit:{}:At", unit_id);
                if let Some(FactValue::Str(pos_str)) = state.get(&pos_key) {
                    if let Some((q_str, r_str)) = pos_str.split_once(',') {
                        if let (Ok(q), Ok(r)) = (q_str.parse::<i32>(), r_str.parse::<i32>()) {
                            let current = HexCoord { q, r };
                            let distance = current.distance(*target);

                            if distance <= 3 {
                                // Close enough - go for final position
                                return Some(Goal {
                                    key: pos_key,
                                    value: FactValue::Str(format!("{},{}", target.q, target.r)),
                                });
                            } else {
                                // Move closer - create intermediate goal
                                let intermediate =
                                    calculate_intermediate_position(current, *target);
                                return Some(Goal {
                                    key: pos_key,
                                    value: FactValue::Str(format!(
                                        "{},{}",
                                        intermediate.q, intermediate.r
                                    )),
                                });
                            }
                        }
                    }
                }
                None
            }

            LongTermGoal::EliminateTarget { target_id } => {
                // Check if in attack range
                let unit_range_key = format!("Unit:{}:AttackRange", unit_id);
                let target_pos_key = format!("Unit:{}:At", target_id);

                if let (Some(FactValue::Int(range)), Some(FactValue::Str(target_pos))) =
                    (state.get(&unit_range_key), state.get(&target_pos_key))
                {
                    let pos_key = format!("Unit:{}:At", unit_id);
                    if let Some(FactValue::Str(unit_pos)) = state.get(&pos_key) {
                        let dist = calculate_distance_from_strings(unit_pos, target_pos);
                        if dist <= *range {
                            // In range - attack goal
                            return Some(Goal {
                                key: format!("Unit:{}:Alive", target_id),
                                value: FactValue::Bool(false),
                            });
                        } else {
                            // Not in range - move closer
                            if let Some((tq_str, tr_str)) = target_pos.split_once(',') {
                                if let (Ok(tq), Ok(tr)) =
                                    (tq_str.parse::<i32>(), tr_str.parse::<i32>())
                                {
                                    return Some(Goal {
                                        key: pos_key,
                                        value: FactValue::Str(format!("{},{}", tq, tr)),
                                    });
                                }
                            }
                        }
                    }
                }
                None
            }

            LongTermGoal::StayInFormation {
                max_distance_from_allies: _,
            } => {
                // Check nearby allies
                let nearby_key = format!("Unit:{}:NearbyAllies", unit_id);
                if let Some(FactValue::Int(nearby)) = state.get(&nearby_key) {
                    if *nearby >= 1 {
                        // Already in formation - maintain position
                        let pos_key = format!("Unit:{}:At", unit_id);
                        if let Some(pos_val) = state.get(&pos_key) {
                            return Some(Goal {
                                key: pos_key,
                                value: pos_val.clone(),
                            });
                        }
                    }
                }
                // Not in formation - try to regroup (simplified: move toward center)
                None
            }

            LongTermGoal::ProtectAlly {
                ally_id,
                max_distance,
            } => {
                // Get ally position
                let ally_pos_key = format!("Unit:{}:At", ally_id);
                if let Some(FactValue::Str(ally_pos)) = state.get(&ally_pos_key) {
                    let unit_pos_key = format!("Unit:{}:At", unit_id);
                    if let Some(FactValue::Str(unit_pos)) = state.get(&unit_pos_key) {
                        let dist = calculate_distance_from_strings(unit_pos, ally_pos);
                        if dist > *max_distance {
                            // Too far - move closer
                            return Some(Goal {
                                key: unit_pos_key,
                                value: FactValue::Str(ally_pos.clone()),
                            });
                        }
                    }
                }
                None
            }

            LongTermGoal::Custom { goal, .. } => Some(goal.clone()),

            _ => None,
        }
    }

    /// Check if the long-term goal has been achieved.
    pub fn is_achieved(&self, state: &WorldState, unit_id: &str) -> bool {
        match self {
            LongTermGoal::ReachPosition { target, .. } => {
                let pos_key = format!("Unit:{}:At", unit_id);
                if let Some(FactValue::Str(pos_str)) = state.get(&pos_key) {
                    if let Some((q_str, r_str)) = pos_str.split_once(',') {
                        if let (Ok(q), Ok(r)) = (q_str.parse::<i32>(), r_str.parse::<i32>()) {
                            return q == target.q && r == target.r;
                        }
                    }
                }
                false
            }

            LongTermGoal::EliminateTarget { target_id } => {
                let alive_key = format!("Unit:{}:Alive", target_id);
                if let Some(FactValue::Bool(alive)) = state.get(&alive_key) {
                    return !alive;
                }
                // If target doesn't exist in state, consider it eliminated
                true
            }

            LongTermGoal::StayInFormation {
                max_distance_from_allies,
            } => {
                let nearby_key = format!("Unit:{}:NearbyAllies", unit_id);
                if let Some(FactValue::Int(nearby)) = state.get(&nearby_key) {
                    return *nearby >= 1 && *max_distance_from_allies >= 2; // Simplified check
                }
                false
            }

            LongTermGoal::ProtectAlly {
                ally_id,
                max_distance,
            } => {
                let ally_pos_key = format!("Unit:{}:At", ally_id);
                let unit_pos_key = format!("Unit:{}:At", unit_id);
                if let (Some(FactValue::Str(ally_pos)), Some(FactValue::Str(unit_pos))) =
                    (state.get(&ally_pos_key), state.get(&unit_pos_key))
                {
                    let dist = calculate_distance_from_strings(unit_pos, ally_pos);
                    return dist <= *max_distance;
                }
                false
            }

            LongTermGoal::Custom { goal, .. } => state.satisfies(&goal.key, &goal.value),

            _ => false,
        }
    }
}

/// Calculate intermediate position between current and target.
fn calculate_intermediate_position(current: HexCoord, target: HexCoord) -> HexCoord {
    // Simple approach: move roughly 1/3 of the way toward target
    let dq = target.q - current.q;
    let dr = target.r - current.r;

    // Clamp to reasonable step
    let step_q = if dq.abs() > 3 { dq.signum() * 3 } else { dq };
    let step_r = if dr.abs() > 3 { dr.signum() * 3 } else { dr };

    HexCoord {
        q: current.q + step_q,
        r: current.r + step_r,
    }
}

/// Calculate distance between two positions given as "q,r" strings.
fn calculate_distance_from_strings(pos1: &str, pos2: &str) -> i32 {
    let parse_pos = |s: &str| -> Option<(i32, i32)> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 2 {
            let q = parts[0].parse().ok()?;
            let r = parts[1].parse().ok()?;
            Some((q, r))
        } else {
            None
        }
    };

    if let (Some((q1, r1)), Some((q2, r2))) = (parse_pos(pos1), parse_pos(pos2)) {
        let coord1 = HexCoord { q: q1, r: r1 };
        let coord2 = HexCoord { q: q2, r: r2 };
        coord1.distance(coord2)
    } else {
        999 // Large distance if parsing fails
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_term_goal_serialization() {
        let goal = LongTermGoal::ReachPosition {
            target: HexCoord { q: 5, r: 3 },
            reason: "high_ground".to_string(),
        };

        let serialized = goal.to_string();
        assert_eq!(serialized, "ReachPosition:5,3:high_ground");

        let deserialized = LongTermGoal::from_string(&serialized);
        assert_eq!(deserialized, Some(goal));
    }

    #[test]
    fn test_eliminate_target_goal() {
        let goal = LongTermGoal::EliminateTarget {
            target_id: "enemy_123".to_string(),
        };

        let serialized = goal.to_string();
        assert_eq!(serialized, "EliminateTarget:enemy_123");

        let deserialized = LongTermGoal::from_string(&serialized);
        assert_eq!(deserialized, Some(goal));
    }

    #[test]
    fn test_goal_achievement() {
        let mut state = WorldState::new();
        state.insert(
            "Unit:player1:At".to_string(),
            FactValue::Str("5,3".to_string()),
        );

        let goal = LongTermGoal::ReachPosition {
            target: HexCoord { q: 5, r: 3 },
            reason: "test".to_string(),
        };

        assert!(goal.is_achieved(&state, "player1"));
    }
}
