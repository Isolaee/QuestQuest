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

    // Convert axial coordinates to world position (POINTY-TOP hexagons)
    pub fn to_world_pos(self, hex_size: f32) -> Vec2 {
        let x = hex_size * (3.0_f32.sqrt() * (self.q as f32 + self.r as f32 / 2.0));
        let y = hex_size * (3.0 / 2.0 * self.r as f32);
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

    // Get neighboring coordinates (corrected for POINTY-TOP hexagons)
    #[allow(dead_code)]
    pub fn neighbors(self) -> [HexCoord; 6] {
        [
            HexCoord::new(self.q + 1, self.r - 1), // Northeast
            HexCoord::new(self.q + 1, self.r),     // East
            HexCoord::new(self.q, self.r + 1),     // Southeast
            HexCoord::new(self.q - 1, self.r + 1), // Southwest
            HexCoord::new(self.q - 1, self.r),     // West
            HexCoord::new(self.q, self.r - 1),     // Northwest
        ]
    }
}

// Sprite data for hexagons
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpriteType {
    None,
    Tree,
    Rock,
    Water,
    Grass,
    Sand,
}

impl SpriteType {
    // Get texture coordinates for sprite (UV mapping)
    #[allow(dead_code)]
    pub fn get_texture_coords(self) -> [f32; 8] {
        match self {
            SpriteType::None => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], // No texture
            SpriteType::Tree => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0], // Full texture for now
            SpriteType::Rock => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            SpriteType::Water => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            SpriteType::Grass => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            SpriteType::Sand => [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
        }
    }

    // Get color tint for sprite (for now, until we have actual textures)
    pub fn get_color_tint(self) -> [f32; 3] {
        match self {
            SpriteType::None => [1.0, 1.0, 1.0],  // White (no tint)
            SpriteType::Tree => [0.2, 0.8, 0.2],  // Green
            SpriteType::Rock => [0.6, 0.6, 0.6],  // Gray
            SpriteType::Water => [0.2, 0.4, 0.8], // Blue
            SpriteType::Grass => [0.4, 0.9, 0.3], // Light green
            SpriteType::Sand => [0.9, 0.8, 0.5],  // Sandy yellow
        }
    }
}

#[derive(Clone)]
pub struct Hexagon {
    pub coord: HexCoord,
    pub world_pos: Vec2,
    pub color: [f32; 3],
    pub sprite: SpriteType,
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

        // Randomly assign sprites for demonstration
        let sprite = match (coord.q + coord.r * 3) % 6 {
            0 => SpriteType::None,
            1 => SpriteType::Tree,
            2 => SpriteType::Rock,
            3 => SpriteType::Water,
            4 => SpriteType::Grass,
            _ => SpriteType::Sand,
        };

        Self {
            coord,
            world_pos,
            color: base_color,
            sprite,
        }
    }

    // Set sprite for this hexagon
    #[allow(dead_code)]
    pub fn set_sprite(&mut self, sprite: SpriteType) {
        self.sprite = sprite;
    }

    // Get the final display color (base color mixed with sprite tint)
    pub fn get_display_color(&self) -> [f32; 3] {
        if self.sprite == SpriteType::None {
            self.color
        } else {
            let sprite_color = self.sprite.get_color_tint();
            // Blend base color with sprite color
            [
                self.color[0] * 0.3 + sprite_color[0] * 0.7,
                self.color[1] * 0.3 + sprite_color[1] * 0.7,
                self.color[2] * 0.3 + sprite_color[2] * 0.7,
            ]
        }
    }

    // Check if hex has a sprite
    #[allow(dead_code)]
    pub fn has_sprite(&self) -> bool {
        self.sprite != SpriteType::None
    }
}
