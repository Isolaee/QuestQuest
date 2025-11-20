//! Movement animation system for smooth unit movement along hex paths
//!
//! This module provides pathfinding and animation state management for units
//! moving across the hex grid. Units animate smoothly from hex to hex rather
//! than teleporting instantly.

use crate::HexCoord;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

/// Animation state for a moving unit
#[derive(Clone, Debug)]
pub struct UnitAnimation {
    unit_id: uuid::Uuid,
    path: Vec<HexCoord>,
    current_step: usize,
    progress: f32, // 0.0 to 1.0 progress between current and next hex
    speed: f32,    // Steps per second
}

impl UnitAnimation {
    /// Create a new movement animation
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to animate
    /// * `path` - Complete path from start to destination (inclusive)
    /// * `speed` - Movement speed in hexes per second
    pub fn new(unit_id: uuid::Uuid, path: Vec<HexCoord>, speed: f32) -> Self {
        Self {
            unit_id,
            path,
            current_step: 0,
            progress: 0.0,
            speed,
        }
    }

    /// Get the unit ID being animated
    pub fn unit_id(&self) -> uuid::Uuid {
        self.unit_id
    }

    /// Get the current hex coordinate in the animation
    pub fn current_hex(&self) -> HexCoord {
        self.path[self.current_step]
    }

    /// Get the next hex coordinate in the animation (if any)
    pub fn next_hex(&self) -> Option<HexCoord> {
        if self.current_step + 1 < self.path.len() {
            Some(self.path[self.current_step + 1])
        } else {
            None
        }
    }

    /// Get the final destination hex
    pub fn destination(&self) -> HexCoord {
        *self.path.last().unwrap()
    }

    /// Get the interpolation progress between current and next hex
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Set the animation speed (hexes per second)
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    /// Get the current animation speed (hexes per second)
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Check if the animation is complete
    pub fn is_complete(&self) -> bool {
        self.current_step + 1 >= self.path.len()
    }

    /// Update animation state by advancing along the path
    ///
    /// # Arguments
    ///
    /// * `delta_time` - Time elapsed since last update (in seconds)
    ///
    /// # Returns
    ///
    /// Vector of hex coordinates that were stepped through this frame
    pub fn update(&mut self, delta_time: f32) -> Vec<HexCoord> {
        let mut stepped_hexes = Vec::new();

        // Advance animation
        self.progress += self.speed * delta_time;

        // Step through hexes as progress crosses 1.0
        while self.progress >= 1.0 && self.current_step + 1 < self.path.len() {
            self.progress -= 1.0;
            self.current_step += 1;
            stepped_hexes.push(self.path[self.current_step]);
        }

        stepped_hexes
    }
}

/// Find a path between two hex coordinates using BFS (Breadth-First Search)
///
/// This function finds the shortest path between two hexes on the grid,
/// treating all hexes as equally traversable (no terrain costs).
///
/// # Arguments
///
/// * `start` - Starting hex coordinate
/// * `end` - Destination hex coordinate
///
/// # Returns
///
/// `Some(Vec<HexCoord>)` containing the complete path from start to end (inclusive),
/// or `None` if no path exists
pub fn find_path(start: HexCoord, end: HexCoord) -> Option<Vec<HexCoord>> {
    if start == end {
        return Some(vec![start]);
    }

    let mut queue = VecDeque::new();
    let mut came_from: HashMap<HexCoord, HexCoord> = HashMap::new();

    queue.push_back(start);
    came_from.insert(start, start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            // Reconstruct path by backtracking from end to start
            let mut path = Vec::new();
            let mut pos = end;

            while pos != start {
                path.push(pos);
                pos = came_from[&pos];
            }
            path.push(start);
            path.reverse();

            return Some(path);
        }

        // Explore all 6 neighbors
        for neighbor in current.neighbors() {
            if let Entry::Vacant(e) = came_from.entry(neighbor) {
                // Valid hex (we don't check terrain/units here, just connectivity)
                queue.push_back(neighbor);
                e.insert(current);
            }
        }
    }

    None // No path found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_path_same_hex() {
        let start = HexCoord::new(0, 0);
        let path = find_path(start, start);
        assert_eq!(path, Some(vec![start]));
    }

    #[test]
    fn test_find_path_adjacent() {
        let start = HexCoord::new(0, 0);
        let end = HexCoord::new(1, 0);
        let path = find_path(start, end);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], start);
        assert_eq!(path[1], end);
    }

    #[test]
    fn test_find_path_multiple_steps() {
        let start = HexCoord::new(0, 0);
        let end = HexCoord::new(3, 0);
        let path = find_path(start, end);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 4); // Distance of 3 means 4 hexes total
        assert_eq!(path[0], start);
        assert_eq!(*path.last().unwrap(), end);
    }

    #[test]
    fn test_animation_creation() {
        let unit_id = uuid::Uuid::new_v4();
        let path = vec![
            HexCoord::new(0, 0),
            HexCoord::new(1, 0),
            HexCoord::new(2, 0),
        ];
        let anim = UnitAnimation::new(unit_id, path.clone(), 5.0);

        assert_eq!(anim.unit_id(), unit_id);
        assert_eq!(anim.current_hex(), HexCoord::new(0, 0));
        assert_eq!(anim.destination(), HexCoord::new(2, 0));
        assert!(!anim.is_complete());
    }

    #[test]
    fn test_animation_update() {
        let unit_id = uuid::Uuid::new_v4();
        let path = vec![
            HexCoord::new(0, 0),
            HexCoord::new(1, 0),
            HexCoord::new(2, 0),
        ];
        let mut anim = UnitAnimation::new(unit_id, path, 5.0);

        // Update with 0.25 seconds - should move 1.25 hexes
        let stepped = anim.update(0.25);
        assert_eq!(stepped.len(), 1);
        assert_eq!(stepped[0], HexCoord::new(1, 0));
        assert_eq!(anim.current_hex(), HexCoord::new(1, 0));
        assert!(!anim.is_complete());

        // Update with another 0.25 seconds - should complete
        let stepped = anim.update(0.25);
        assert_eq!(stepped.len(), 1);
        assert_eq!(stepped[0], HexCoord::new(2, 0));
        assert!(anim.is_complete());
    }
}
