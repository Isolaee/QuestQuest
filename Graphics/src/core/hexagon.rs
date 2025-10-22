use crate::math::Vec2;
use serde::{Deserialize, Serialize};

/// Axial coordinates for hexagonal grid (flat-top orientation).
///
/// `HexCoord` uses axial coordinates (q, r) where q is the column and r is the row.
/// This type is used across the graphics and game logic to index hexagons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HexCoord {
    pub q: i32, // column
    pub r: i32, // row
}

impl HexCoord {
    /// Construct a new `HexCoord`.
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// Convert axial coordinates to world position (flat-top hexagons).
    ///
    /// The returned `Vec2` is the pixel/world position that corresponds to the
    /// hexagon center for rendering or spatial lookup calculations.
    pub fn to_world_pos(self, hex_size: f32) -> Vec2 {
        let x = hex_size * (3.0 / 2.0 * self.q as f32);
        let y = hex_size * (3.0_f32.sqrt() * (self.r as f32 + self.q as f32 / 2.0));
        Vec2::new(x, y)
    }

    /// Robust rounding function for axial coordinates.
    ///
    /// Converts fractional axial coordinates to the nearest integer `HexCoord`.
    pub fn axial_round(q: f32, r: f32) -> Self {
        // Convert to cube coordinates for easier rounding
        let x = q;
        let z = r;
        let y = -x - z;

        // Round each coordinate
        let mut rx = x.round();
        let mut ry = y.round();
        let mut rz = z.round();

        // Calculate rounding errors
        let x_diff = (rx - x).abs();
        let y_diff = (ry - y).abs();
        let z_diff = (rz - z).abs();

        // Correct the coordinate with the largest error to maintain x + y + z = 0
        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry - rz;
        } else if y_diff > z_diff {
            ry = -rx - rz;
        } else {
            rz = -rx - ry;
        }

        // Suppress unused assignment warning for `ry` when it's only used in corrections
        let _ = ry;

        // Convert back to axial coordinates
        Self::new(rx as i32, rz as i32)
    }

    /// Get distance between two hex coordinates.
    #[allow(dead_code)]
    pub fn distance(self, other: HexCoord) -> i32 {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r).abs())
            / 2
    }

    /// Get neighboring coordinates (flat-top orientation).
    #[allow(dead_code)]
    pub fn neighbors(self) -> [HexCoord; 6] {
        [
            HexCoord::new(self.q, self.r - 1),     // North
            HexCoord::new(self.q + 1, self.r - 1), // Northeast
            HexCoord::new(self.q + 1, self.r),     // Southeast
            HexCoord::new(self.q, self.r + 1),     // South
            HexCoord::new(self.q - 1, self.r + 1), // Southwest
            HexCoord::new(self.q - 1, self.r),     // Northwest
        ]
    }
}

/// Sprite data for hexagons (terrain, units and items).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SpriteType {
    None,
    Forest,       // forest.png
    Forest2,      // forest2.png
    Grasslands,   // grasslands.png
    HauntedWoods, // haunted_woods.png
    Hills,        // hills.png
    Mountain,     // mountain.png
    Swamp,        // swamp.png
    Unit,         // Red circle for units
    Item,         // Gold/yellow circle for items
}

impl SpriteType {
    /// Get texture coordinates for sprite (UV mapping).
    #[allow(dead_code)]
    pub fn get_texture_coords(self) -> [f32; 8] {
        match self {
            SpriteType::None => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], // No texture
            SpriteType::Forest => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0], // Full texture for now
            SpriteType::Forest2 => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            SpriteType::Grasslands => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            SpriteType::HauntedWoods => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            SpriteType::Hills => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            SpriteType::Mountain => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            SpriteType::Swamp => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            SpriteType::Unit => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], // No texture --- IGNORE ---
            SpriteType::Item => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], // No texture --- IGNORE ---
        }
    }

    /// Get texture file path for sprite if available.
    pub fn get_texture_path(self) -> Option<&'static str> {
        match self {
            SpriteType::None => None,
            SpriteType::Forest => Some("terrain_sprites/forest.png"),
            SpriteType::Forest2 => Some("terrain_sprites/forest2.png"),
            SpriteType::Grasslands => Some("terrain_sprites/grasslands.png"),
            SpriteType::HauntedWoods => Some("terrain_sprites/haunted_woods.png"),
            SpriteType::Hills => Some("terrain_sprites/hills.png"),
            SpriteType::Mountain => Some("terrain_sprites/mountain.png"),
            SpriteType::Swamp => Some("terrain_sprites/swamp.png"),
            SpriteType::Unit => None, // We'll render this as a colored circle
            SpriteType::Item => Some("item_sprites/sword.png"), // Sword sprite for items
        }
    }

    /// Get color tint for sprite (fallback when textures aren't loaded).
    pub fn get_color_tint(self) -> [f32; 3] {
        match self {
            SpriteType::None => [1.0, 1.0, 1.0],         // White (no tint)
            SpriteType::Forest => [0.2, 0.7, 0.2],       // Dark green
            SpriteType::Forest2 => [0.3, 0.8, 0.3],      // Medium green
            SpriteType::Grasslands => [0.4, 0.9, 0.3],   // Light green
            SpriteType::HauntedWoods => [0.4, 0.2, 0.6], // Dark purple
            SpriteType::Hills => [0.7, 0.6, 0.4],        // Brown
            SpriteType::Mountain => [0.6, 0.6, 0.7],     // Gray-blue
            SpriteType::Swamp => [0.3, 0.5, 0.2],        // Dark green-brown
            SpriteType::Unit => [0.9, 0.2, 0.2],         // Bright red for units
            SpriteType::Item => [1.0, 0.84, 0.0],        // Gold/yellow for items
        }
    }

    /// Get all terrain sprite types (excluding None).
    pub fn all_terrain() -> [SpriteType; 7] {
        [
            SpriteType::Forest,
            SpriteType::Forest2,
            SpriteType::Grasslands,
            SpriteType::HauntedWoods,
            SpriteType::Hills,
            SpriteType::Mountain,
            SpriteType::Swamp,
        ]
    }

    /// Get a deterministic pseudo-random terrain sprite given a seed.
    pub fn random_terrain(seed: i32) -> SpriteType {
        let all = Self::all_terrain();
        all[(seed.abs() % 7) as usize]
    }

    /// Get texture array index for OpenGL shader.
    pub fn get_texture_id(&self) -> f32 {
        match self {
            SpriteType::None => -1.0, // No texture
            SpriteType::Forest => 0.0,
            SpriteType::Forest2 => 1.0,
            SpriteType::Grasslands => 2.0,
            SpriteType::HauntedWoods => 3.0,
            SpriteType::Hills => 4.0,
            SpriteType::Mountain => 5.0,
            SpriteType::Swamp => 6.0,
            SpriteType::Unit => -1.0, // Use color rendering, not texture
            SpriteType::Item => 7.0,  // Item texture at unit 7
        }
    }
}

/// Types of highlighting applied to hexagons for UI feedback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighlightType {
    None,
    Selected,      // Yellow highlight for selected unit
    MovementRange, // Blue highlight for movement range
}

/// A renderable hexagon used by the `HexGrid` and renderer.
///
/// Contains rendering state such as terrain `sprite`, optional `unit_sprite`/
/// `item_sprite` overlays, the hex center `world_pos` and visual highlight state.
#[derive(Clone)]
pub struct Hexagon {
    pub coord: HexCoord,
    pub world_pos: Vec2,
    pub color: [f32; 3],
    pub sprite: SpriteType,              // Base terrain sprite
    pub unit_sprite: Option<SpriteType>, // Optional unit sprite on top
    pub item_sprite: Option<SpriteType>, // Optional item sprite on top
    pub highlight: HighlightType,        // Highlight state
}

impl Hexagon {
    /// Create a new `Hexagon` with sensible defaults and a deterministic seed-based terrain.
    pub fn new(coord: HexCoord, hex_size: f32) -> Self {
        let world_pos = coord.to_world_pos(hex_size);

        // Generate base color based on coordinate for visual debugging
        let base_color = [
            0.3 + 0.4 * ((coord.q + coord.r) % 3) as f32 / 3.0,
            0.4 + 0.3 * (coord.q % 4) as f32 / 4.0,
            0.5 + 0.3 * (coord.r % 5) as f32 / 5.0,
        ];

        // Randomly assign terrain sprites for demonstration - NO EMPTY HEXES
        let seed = coord.q * 17 + coord.r * 23 + coord.q * coord.r;
        let sprite = SpriteType::random_terrain(seed); // 100% terrain coverage

        Self {
            coord,
            world_pos,
            color: base_color,
            sprite,
            unit_sprite: None, // No unit by default
            item_sprite: None, // No item by default
            highlight: HighlightType::None,
        }
    }

    /// Set unit sprite (rendered on top of terrain)
    pub fn set_unit_sprite(&mut self, unit_sprite: Option<SpriteType>) {
        self.unit_sprite = unit_sprite;
    }

    /// Set item sprite (rendered on top of terrain and units)
    pub fn set_item_sprite(&mut self, item_sprite: Option<SpriteType>) {
        self.item_sprite = item_sprite;
    }

    /// Set terrain sprite (base layer)
    pub fn set_sprite(&mut self, sprite: SpriteType) {
        self.sprite = sprite;
    }

    /// Set highlight type
    pub fn set_highlight(&mut self, highlight: HighlightType) {
        self.highlight = highlight;
    }

    /// Clear highlight
    pub fn clear_highlight(&mut self) {
        self.highlight = HighlightType::None;
    }

    /// Get the display sprite (unit takes priority if present)
    pub fn get_display_sprite(&self) -> SpriteType {
        self.unit_sprite.unwrap_or(self.sprite)
    }

    /// Check if hex has a unit
    pub fn has_unit(&self) -> bool {
        self.unit_sprite.is_some()
    }

    /// Get the final display color (blends terrain tint and highlights)
    pub fn get_display_color(&self) -> [f32; 3] {
        let display_sprite = self.get_display_sprite();

        let base_color = if display_sprite == SpriteType::None {
            self.color
        } else {
            let sprite_color = display_sprite.get_color_tint();
            // Blend base color with sprite color
            [
                self.color[0] * 0.3 + sprite_color[0] * 0.7,
                self.color[1] * 0.3 + sprite_color[1] * 0.7,
                self.color[2] * 0.3 + sprite_color[2] * 0.7,
            ]
        };

        // Apply highlight tinting
        match self.highlight {
            HighlightType::None => base_color,
            HighlightType::Selected => {
                // Yellow tint for selected unit
                [
                    (base_color[0] * 0.5 + 1.0 * 0.5).min(1.0),
                    (base_color[1] * 0.5 + 0.9 * 0.5).min(1.0),
                    (base_color[2] * 0.5 + 0.2 * 0.5).min(1.0),
                ]
            }
            HighlightType::MovementRange => {
                // Blue tint for movement range
                [
                    (base_color[0] * 0.6 + 0.2 * 0.4).min(1.0),
                    (base_color[1] * 0.6 + 0.5 * 0.4).min(1.0),
                    (base_color[2] * 0.6 + 1.0 * 0.4).min(1.0),
                ]
            }
        }
    }

    /// Check if hex has a sprite (terrain or unit)
    pub fn has_sprite(&self) -> bool {
        self.sprite != SpriteType::None || self.unit_sprite.is_some()
    }
}
