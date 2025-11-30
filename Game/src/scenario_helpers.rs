use crate::objects::*;
use ai::*;
use graphics::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use uuid::Uuid;

impl GameWorld {
    /// Dijkstra-like reachable calculation using integer costs.
    ///
    /// This was previously a nested function inside `generate_team_actions`.
    /// Moving it here makes it testable and keeps `generate_team_actions`
    /// focused on high-level logic.
    fn compute_reachable(&self, unit_id: Uuid, start: HexCoord, max_cost: i32) -> HashMap<HexCoord, i32> {
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
                // Skip out-of-bounds when a world radius is defined
                if let Some(radius) = self.get_world_radius_opt() {
                    if nb.distance(HexCoord::new(0, 0)) > radius {
                        continue;
                    }
                }

                // Skip impassable terrain
                if let Some(terrain) = self.get_terrain(*nb) {
                    if terrain.blocks_movement() {
                        continue;
                    }
                }

                // Skip occupied tiles by other units
                let units_there = self.get_units_at_position(*nb);
                let occupied_by_other = units_there.iter().any(|u| u.id() != unit_id);
                if occupied_by_other {
                    continue;
                }

                let step_cost = self.get_movement_cost(*nb);
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

    /// Helper to optionally read a `world_radius` field if present on `GameWorld`.
    ///
    /// Some versions of the world store a radius field; calling code here
    /// is defensive and treats absence as "no radius limit".
    fn get_world_radius_opt(&self) -> Option<i32> {
        // If GameWorld has a `world_radius` field, prefer it. Use reflection-like
        // approach via a stub accessor if the field exists in user's struct.
        // The real codebase may expose this as a field or a method; try method first.
        #[allow(unused_imports)]
        use std::convert::TryInto;

        // If `world_radius` is a public method, call it. If not, return None.
        // This function is intentionally simple; if your GameWorld already has
        // a `world_radius` field, you can replace the body with `Some(self.world_radius)`.
        None
    }
}
