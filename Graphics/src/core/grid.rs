use crate::core::{Camera, HexCoord, Hexagon, HighlightType, SpriteType};
use crate::math::Vec2;
use std::collections::HashMap;

/// In-memory hex grid used by the renderer and game UI.
///
/// `HexGrid` stores a collection of `Hexagon` objects indexed by `HexCoord` and
/// provides helper methods for common operations like visibility culling,
/// setting sprites, and converting screen positions to hex coordinates.
pub struct HexGrid {
    pub hexagons: HashMap<HexCoord, Hexagon>,
    pub camera: Camera,
    pub hex_size: f32,
    #[allow(dead_code)]
    grid_radius: i32,
}

impl HexGrid {
    /// Create a new `HexGrid` with default sizing and a populated area around the origin.
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

    /// Create an empty `HexGrid` with no hexagons.
    ///
    /// Use `add_hex` or `add_hex_with_sprite` to populate the grid with only
    /// the tiles you need. This is useful when loading maps from JSON files
    /// where you only want to render the tiles defined in the file.
    pub fn empty() -> Self {
        Self {
            hexagons: HashMap::new(),
            camera: Camera::new(),
            hex_size: 0.2,
            grid_radius: 0,
        }
    }

    /// Create a `HexGrid` from a list of coordinates with their sprites.
    ///
    /// Only the specified coordinates will have hexagons created for them.
    /// This is the recommended way to create a grid from map data.
    ///
    /// # Arguments
    ///
    /// * `tiles` - Iterator of (HexCoord, SpriteType) pairs defining the map
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tiles = vec![
    ///     (HexCoord::new(0, 0), SpriteType::Grasslands),
    ///     (HexCoord::new(1, 0), SpriteType::Forest),
    /// ];
    /// let grid = HexGrid::from_tiles(tiles);
    /// ```
    pub fn from_tiles<I>(tiles: I) -> Self
    where
        I: IntoIterator<Item = (HexCoord, SpriteType)>,
    {
        let hex_size = 0.2;
        let mut hexagons = HashMap::new();

        for (coord, sprite) in tiles {
            let mut hexagon = Hexagon::new(coord, hex_size);
            hexagon.set_sprite(sprite);
            hexagons.insert(coord, hexagon);
        }

        Self {
            hexagons,
            camera: Camera::new(),
            hex_size,
            grid_radius: 0, // Not applicable for custom grids
        }
    }

    /// Add a single hexagon at the specified coordinate.
    ///
    /// If a hexagon already exists at this coordinate, it will be replaced.
    pub fn add_hex(&mut self, coord: HexCoord) {
        let hexagon = Hexagon::new(coord, self.hex_size);
        self.hexagons.insert(coord, hexagon);
    }

    /// Add a single hexagon with a specific terrain sprite.
    ///
    /// If a hexagon already exists at this coordinate, it will be replaced.
    pub fn add_hex_with_sprite(&mut self, coord: HexCoord, sprite: SpriteType) {
        let mut hexagon = Hexagon::new(coord, self.hex_size);
        hexagon.set_sprite(sprite);
        self.hexagons.insert(coord, hexagon);
    }

    /// Remove a hexagon from the grid.
    ///
    /// Returns the removed hexagon if it existed.
    pub fn remove_hex(&mut self, coord: HexCoord) -> Option<Hexagon> {
        self.hexagons.remove(&coord)
    }

    /// Clear all hexagons from the grid.
    pub fn clear(&mut self) {
        self.hexagons.clear();
    }
}

impl Default for HexGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl HexGrid {
    /// Return visible hexagons after coarse culling using camera position.
    ///
    /// This method performs a cheap hex-distance cull followed by a precise
    /// world-distance check to select hexes that should be rendered.
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

    /// Move the internal camera by the given delta.
    pub fn move_camera(&mut self, dx: f32, dy: f32) {
        self.camera.move_by(dx, dy);
    }

    /// Get hexagon at specific coordinate (useful for game logic).
    #[allow(dead_code)]
    pub fn get_hex_at(&self, coord: HexCoord) -> Option<&Hexagon> {
        self.hexagons.get(&coord)
    }

    /// Get mutable hexagon at specific coordinate.
    #[allow(dead_code)]
    pub fn get_hex_at_mut(&mut self, coord: HexCoord) -> Option<&mut Hexagon> {
        self.hexagons.get_mut(&coord)
    }

    /// Set a unit sprite at a coordinate (preserving the terrain sprite).
    pub fn set_unit_at(&mut self, coord: HexCoord, unit_sprite: SpriteType) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_unit_sprite(Some(unit_sprite));
        }
    }

    /// Set an item sprite at a coordinate (preserving terrain and unit sprites).
    pub fn set_item_at(&mut self, coord: HexCoord, item_sprite: SpriteType) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_item_sprite(Some(item_sprite));
        }
    }

    /// Set a structure sprite at a coordinate (preserving terrain sprite).
    pub fn set_structure_at(&mut self, coord: HexCoord, structure_sprite: SpriteType) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_structure_sprite(Some(structure_sprite));
        }
    }

    /// Remove unit sprite at a coordinate.
    pub fn remove_unit_at(&mut self, coord: HexCoord) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_unit_sprite(None);
        }
    }

    /// Remove item sprite at a coordinate.
    pub fn remove_item_at(&mut self, coord: HexCoord) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_item_sprite(None);
        }
    }

    /// Remove structure sprite at a coordinate.
    pub fn remove_structure_at(&mut self, coord: HexCoord) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_structure_sprite(None);
        }
    }

    /// Set terrain sprite at a coordinate.
    pub fn set_sprite_at(&mut self, coord: HexCoord, sprite: SpriteType) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_sprite(sprite);
        }
    }

    /// Check if there's a unit at the specified coordinate.
    pub fn has_unit_at(&self, coord: HexCoord) -> bool {
        self.hexagons
            .get(&coord)
            .map(|hex| hex.has_unit())
            .unwrap_or(false)
    }

    /// Check if there's a structure at the specified coordinate.
    pub fn has_structure_at(&self, coord: HexCoord) -> bool {
        self.hexagons
            .get(&coord)
            .map(|hex| hex.has_structure())
            .unwrap_or(false)
    }

    /// Clear visual highlights on all hexes.
    pub fn clear_all_highlights(&mut self) {
        for hex in self.hexagons.values_mut() {
            hex.clear_highlight();
        }
    }

    /// Highlight a specific hex with a given `HighlightType`.
    pub fn highlight_hex(&mut self, coord: HexCoord, highlight_type: HighlightType) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_highlight(highlight_type);
        }
    }

    /// Highlight multiple hexes.
    pub fn highlight_hexes(&mut self, coords: &[HexCoord], highlight_type: HighlightType) {
        for coord in coords {
            self.highlight_hex(*coord, highlight_type);
        }
    }

    /// Set text overlay on a specific hex.
    pub fn set_hex_text_overlay(&mut self, coord: HexCoord, text: Option<String>) {
        if let Some(hex) = self.hexagons.get_mut(&coord) {
            hex.set_text_overlay(text);
        }
    }

    /// Clear all text overlays from all hexes.
    pub fn clear_all_text_overlays(&mut self) {
        for hex in self.hexagons.values_mut() {
            hex.clear_text_overlay();
        }
    }

    /// Convert screen coordinates (mouse position) to hex coordinate.
    ///
    /// * `screen_pos`: Mouse position in screen pixels (origin at top-left)
    /// * `window_size`: Window dimensions (width, height in pixels)
    ///
    /// Returns the hex coordinate at that screen position, or `None` if out of the grid bounds.
    pub fn screen_to_hex_coord(&self, screen_pos: Vec2, window_size: Vec2) -> Option<HexCoord> {
        // Convert screen coordinates to world coordinates using camera
        let world_pos = self.camera.screen_to_world(screen_pos, window_size);

        // Convert world coordinates to hex coordinates using axial conversion (flat-top)
        let q = (2.0 / 3.0 * world_pos.x) / self.hex_size;
        let r = (-1.0 / 3.0 * world_pos.x + 3.0_f32.sqrt() / 3.0 * world_pos.y) / self.hex_size;

        // Round to nearest hex coordinate
        let hex_coord = HexCoord::axial_round(q, r);

        // Check if this hex exists in the grid
        if self.hexagons.contains_key(&hex_coord) {
            Some(hex_coord)
        } else {
            None
        }
    }
}
