//! Strategic goal system for multi-turn AI planning.
//!
//! This module provides a goal framework for AI entities to pursue
//! multi-turn objectives focused on combat and positioning.

use crate::action::Goal;
use crate::world_state::{FactValue, HexCoord, WorldState};

/// Long-term strategic goals that span multiple turns.
#[derive(Clone, Debug, PartialEq)]
pub enum LongTermGoal {
    /// Kill all enemies in the area
    KillAllEnemies {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kill_all_enemies_serialization() {
        let goal = LongTermGoal::KillAllEnemies {
            search_radius: Some(10),
        };

        let serialized = goal.to_string();
        assert_eq!(serialized, "KillAllEnemies:10");

        let deserialized = LongTermGoal::from_string(&serialized);
        assert_eq!(deserialized, Some(goal));
    }

    #[test]
    fn test_kill_all_enemies_unlimited() {
        let goal = LongTermGoal::KillAllEnemies {
            search_radius: None,
        };

        let serialized = goal.to_string();
        assert_eq!(serialized, "KillAllEnemies:unlimited");

        let deserialized = LongTermGoal::from_string(&serialized);
        assert_eq!(deserialized, Some(goal));
    }

    #[test]
    fn test_kill_all_enemies_achievement() {
        let mut state = WorldState::new();
        state.insert("Unit:player1:NearbyEnemies".to_string(), FactValue::Int(0));

        let goal = LongTermGoal::KillAllEnemies {
            search_radius: Some(10),
        };

        assert!(goal.is_achieved(&state, "player1"));
    }

    #[test]
    fn test_protect_serialization() {
        let goal = LongTermGoal::Protect {
            targets: vec![
                HexCoord { q: 5, r: 3 },
                HexCoord { q: 6, r: 4 },
                HexCoord { q: 7, r: 5 },
            ],
            reason: "defend_base".to_string(),
        };

        let serialized = goal.to_string();
        assert_eq!(serialized, "Protect:5,3;6,4;7,5:defend_base");

        let deserialized = LongTermGoal::from_string(&serialized);
        assert_eq!(deserialized, Some(goal));
    }

    #[test]
    fn test_protect_achievement() {
        let mut state = WorldState::new();
        state.insert(
            "Unit:player1:At".to_string(),
            FactValue::Str("5,3".to_string()),
        );

        let goal = LongTermGoal::Protect {
            targets: vec![HexCoord { q: 5, r: 3 }, HexCoord { q: 6, r: 4 }],
            reason: "test".to_string(),
        };

        assert!(goal.is_achieved(&state, "player1"));
    }

    #[test]
    fn test_reach_area_serialization() {
        let goal = LongTermGoal::ReachArea {
            area_centers: vec![HexCoord { q: 10, r: 10 }, HexCoord { q: 15, r: 15 }],
            reason: "capture_zone".to_string(),
        };

        let serialized = goal.to_string();
        assert_eq!(serialized, "ReachArea:10,10;15,15:capture_zone");

        let deserialized = LongTermGoal::from_string(&serialized);
        assert_eq!(deserialized, Some(goal));
    }

    #[test]
    fn test_reach_area_achievement() {
        let mut state = WorldState::new();
        state.insert(
            "Unit:player1:At".to_string(),
            FactValue::Str("10,10".to_string()),
        );

        let goal = LongTermGoal::ReachArea {
            area_centers: vec![HexCoord { q: 10, r: 10 }, HexCoord { q: 20, r: 20 }],
            reason: "test".to_string(),
        };

        assert!(goal.is_achieved(&state, "player1"));
    }

    #[test]
    fn test_siege_castle_serialization() {
        let goal = LongTermGoal::SiegeCastle {
            castle_id: "fortress_01".to_string(),
        };

        let serialized = goal.to_string();
        assert_eq!(serialized, "SiegeCastle:fortress_01");

        let deserialized = LongTermGoal::from_string(&serialized);
        assert_eq!(deserialized, Some(goal));
    }

    #[test]
    fn test_siege_castle_achievement() {
        let mut state = WorldState::new();
        state.insert("Castle:fortress_01:HP".to_string(), FactValue::Int(0));

        let goal = LongTermGoal::SiegeCastle {
            castle_id: "fortress_01".to_string(),
        };

        assert!(goal.is_achieved(&state, "player1"));
    }
}
