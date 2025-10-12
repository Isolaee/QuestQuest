use crate::math::Vec2;
use serde::{Deserialize, Serialize};

// Axial coordinates for hexagonal grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HexCoord {
    pub q: i32, // column
    pub r: i32, // row
}

impl HexCoord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    // Convert axial coordinates to world position (FLAT-TOP hexagons)
    pub fn to_world_pos(self, hex_size: f32) -> Vec2 {
        let x = hex_size * (3.0 / 2.0 * self.q as f32);
        let y = hex_size * (3.0_f32.sqrt() * (self.r as f32 + self.q as f32 / 2.0));
        Vec2::new(x, y)
    }

    // Get distance between two hex coordinates
    #[allow(dead_code)]
    pub fn distance(self, other: HexCoord) -> i32 {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r).abs())
            / 2
    }

    // Get neighboring coordinates (corrected for FLAT-TOP hexagons)
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

// Sprite data for hexagons
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
}

impl SpriteType {
    // Get texture coordinates for sprite (UV mapping)
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
        }
    }

    // Get texture file path for sprite
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
        }
    }

    // Get color tint for sprite (fallback when textures aren't loaded)
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
        }
    }

    // Get all terrain sprite types (excluding None)
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

    // Get a random terrain sprite (excluding None)
    pub fn random_terrain(seed: i32) -> SpriteType {
        let all = Self::all_terrain();
        all[(seed.abs() % 7) as usize]
    }

    // Get texture array index for OpenGL shader
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
        }
    }
}

#[derive(Clone)]
pub struct Hexagon {
    pub coord: HexCoord,
    pub world_pos: Vec2,
    pub color: [f32; 3],
    pub sprite: SpriteType,              // Base terrain sprite
    pub unit_sprite: Option<SpriteType>, // Optional unit sprite on top
}

impl Hexagon {
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
        }
    }

    // Set unit sprite (rendered on top of terrain)
    pub fn set_unit_sprite(&mut self, unit_sprite: Option<SpriteType>) {
        self.unit_sprite = unit_sprite;
    }

    // Set terrain sprite (base layer)
    pub fn set_sprite(&mut self, sprite: SpriteType) {
        self.sprite = sprite;
    }

    // Get the display sprite (unit takes priority if present)
    pub fn get_display_sprite(&self) -> SpriteType {
        self.unit_sprite.unwrap_or(self.sprite)
    }

    // Check if hex has a unit
    pub fn has_unit(&self) -> bool {
        self.unit_sprite.is_some()
    }

    // Get the final display color
    pub fn get_display_color(&self) -> [f32; 3] {
        let display_sprite = self.get_display_sprite();

        if display_sprite == SpriteType::None {
            self.color
        } else {
            let sprite_color = display_sprite.get_color_tint();
            // Blend base color with sprite color
            [
                self.color[0] * 0.3 + sprite_color[0] * 0.7,
                self.color[1] * 0.3 + sprite_color[1] * 0.7,
                self.color[2] * 0.3 + sprite_color[2] * 0.7,
            ]
        }
    }

    // Check if hex has a sprite (terrain or unit)
    pub fn has_sprite(&self) -> bool {
        self.sprite != SpriteType::None || self.unit_sprite.is_some()
    }
}
