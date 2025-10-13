use crate::core::{Camera, HexCoord, Hexagon, HighlightType, SpriteType};
use std::collections::HashMap;

pub struct HexGrid {
    pub hexagons: HashMap<HexCoord, Hexagon>,
    pub camera: Camera,
    pub hex_size: f32,
    #[allow(dead_code)]
    grid_radius: i32,
}

impl HexGrid {
    pub fn new() -> Self {
        let hex_size = 0.2; // Larger hexagons for better visibility
        let grid_radius = 15; // Grid extends 15 hexes in each direction
        let mut hexagons = HashMap::new();

        // Generate hexagonal grid using axial coordinates
        for q in -grid_radius..=grid_radius {
            let r1 = (-grid_radius).max(-q - grid_radius);
            let r2 = grid_radius.min(-q + grid_radius);

            for r in r1..=r2 {
                let coord = HexCoord::new(q, r);
                let hexagon = Hexagon::new(coord, hex_size);
                hexagons.insert(coord, hexagon);
            }
        }

        Self {
            hexagons,
            camera: Camera::new(),
            hex_size,
            grid_radius,
        }
    }
}

impl Default for HexGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl HexGrid {
    pub fn get_visible_hexagons(&self) -> Vec<&Hexagon> {
        // Get approximate camera hex coordinate for more efficient culling
        let cam_hex = self.camera.to_hex_coord(self.hex_size);
        let view_radius = (self.camera.view_distance / self.hex_size).ceil() as i32 + 1;

        self.hexagons
            .values()
            .filter(|hex| {
                // Quick hex-distance check first (cheaper than world distance)
                if cam_hex.distance(hex.coord) > view_radius {
                    return false;
                }
                // Then precise world distance check
                self.camera.can_see(hex.world_pos)
            })
            .collect()
    }

    pub fn move_camera(&mut self, dx: f32, dy: f32) {
        self.camera.move_by(dx, dy);
    }

    // Get hexagon at specific coordinate (useful for game logic)
    #[allow(dead_code)]
    pub fn get_hex_at(&self, coord: HexCoord) -> Option<&Hexagon> {
        self.hexagons.get(&coord)
    }

    // Get mutable hexagon at specific coordinate
    #[allow(dead_code)]
    pub fn get_hex_at_mut(&mut self, coord: HexCoord) -> Option<&mut Hexagon> {
        self.hexagons.get_mut(&coord)
    }

    // Set unit sprite at specific coordinate (preserving terrain)
    pub fn set_unit_at(&mut self, coord: HexCoord, unit_sprite: SpriteType) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_unit_sprite(Some(unit_sprite));
        }
    }

    // Remove unit sprite at specific coordinate (preserving terrain)
    pub fn remove_unit_at(&mut self, coord: HexCoord) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_unit_sprite(None);
        }
    }

    // Set sprite at specific coordinate (terrain layer)
    pub fn set_sprite_at(&mut self, coord: HexCoord, sprite: SpriteType) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_sprite(sprite);
        }
    }

    // Check if there's a unit at specific coordinate
    pub fn has_unit_at(&self, coord: HexCoord) -> bool {
        self.hexagons
            .get(&coord)
            .map(|hex| hex.has_unit())
            .unwrap_or(false)
    }

    // Clear all highlights
    pub fn clear_all_highlights(&mut self) {
        for hex in self.hexagons.values_mut() {
            hex.clear_highlight();
        }
    }

    // Highlight a specific hex
    pub fn highlight_hex(&mut self, coord: HexCoord, highlight_type: HighlightType) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_highlight(highlight_type);
        }
    }

    // Highlight multiple hexes
    pub fn highlight_hexes(&mut self, coords: &[HexCoord], highlight_type: HighlightType) {
        for coord in coords {
            self.highlight_hex(*coord, highlight_type);
        }
    }
}
