use crate::math::Vec2;
use serde::{Deserialize, Serialize};

/// Simple, robust hexagonal coordinate system utilities.
///
/// `SimpleHexCoord` mirrors `HexCoord` but offers a standalone set of helpers
/// (conversion to/from pixel coordinates, neighbor iteration and distance).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimpleHexCoord {
    pub q: i32, // column
    pub r: i32, // row
}

impl SimpleHexCoord {
    /// Create a new `SimpleHexCoord(q, r)`.
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// Convert hex coordinates to pixel position (flat-top orientation).
    ///
    /// Returns the world/pixel coordinates for the center of this hex.
    pub fn to_pixel(self, hex_size: f32) -> Vec2 {
        let x = hex_size * (3.0 / 2.0) * self.q as f32;
        let y = hex_size * 3.0_f32.sqrt() * (self.r as f32 + 0.5 * self.q as f32);
        Vec2::new(x, y)
    }

    /// Convert pixel position to hex coordinates (flat-top orientation).
    ///
    /// Performs the inverse transformation of `to_pixel` and returns the nearest
    /// hex coordinate using a robust rounding method.
    pub fn from_pixel(pixel_pos: Vec2, hex_size: f32) -> Self {
        // Convert to fractional hex coordinates
        let q_frac = (2.0 / 3.0) * pixel_pos.x / hex_size;
        let r_frac = (-1.0 / 3.0) * pixel_pos.x / hex_size
            + (1.0 / 3.0) * 3.0_f32.sqrt() * pixel_pos.y / hex_size;

        // Use axial_round for accurate conversion
        Self::axial_round(q_frac, r_frac)
    }

    /// Robust rounding function for axial coordinates.
    fn axial_round(q: f32, r: f32) -> Self {
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

        // Suppress the unused assignment warning by using ry
        let _ = ry;

        // Convert back to axial coordinates
        Self::new(rx as i32, rz as i32)
    }

    /// Get the 6 neighboring hexagons (flat-top orientation).
    pub fn neighbors(self) -> [SimpleHexCoord; 6] {
        [
            SimpleHexCoord::new(self.q + 1, self.r),
            SimpleHexCoord::new(self.q + 1, self.r - 1),
            SimpleHexCoord::new(self.q, self.r - 1),
            SimpleHexCoord::new(self.q - 1, self.r),
            SimpleHexCoord::new(self.q - 1, self.r + 1),
            SimpleHexCoord::new(self.q, self.r + 1),
        ]
    }

    /// Calculate distance between two hex coordinates.
    pub fn distance(self, other: Self) -> i32 {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r).abs())
            / 2
    }

    /// Check if this coordinate is within a given radius of another coordinate.
    pub fn is_within_radius(self, center: Self, radius: i32) -> bool {
        self.distance(center) <= radius
    }
}

/// Simple camera for 2D hexagon world (pixel-space).
#[derive(Clone, Debug)]
pub struct SimpleCamera {
    pub position: Vec2,
    pub zoom: f32,
}

impl Default for SimpleCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleCamera {
    /// Create a new default `SimpleCamera` centered at the origin.
    pub fn new() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            zoom: 1.0,
        }
    }

    /// Convert screen coordinates to world coordinates (pixel-space).
    pub fn screen_to_world(&self, screen_pos: Vec2, window_size: Vec2) -> Vec2 {
        let center = Vec2::new(window_size.x / 2.0, window_size.y / 2.0);
        let offset = screen_pos - center;
        // Flip Y axis for screen coordinate system
        let world_offset = Vec2::new(offset.x, -offset.y) / self.zoom;
        self.position + world_offset
    }

    /// Convert world coordinates to screen coordinates (pixel-space).
    pub fn world_to_screen(&self, world_pos: Vec2, window_size: Vec2) -> Vec2 {
        let world_offset = world_pos - self.position;
        let screen_offset = world_offset * self.zoom;
        let center = Vec2::new(window_size.x / 2.0, window_size.y / 2.0);
        // Flip Y axis back to screen coordinate system
        center + Vec2::new(screen_offset.x, -screen_offset.y)
    }

    /// Move camera by a given offset.
    pub fn move_by(&mut self, offset: Vec2) {
        self.position += offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_coordinate_conversion() {
        let hex_size = 50.0;
        let origin = SimpleHexCoord::new(0, 0);
        let pixel = origin.to_pixel(hex_size);
        let back_to_hex = SimpleHexCoord::from_pixel(pixel, hex_size);
        assert_eq!(origin, back_to_hex);
    }

    #[test]
    fn test_camera_conversion() {
        let mut camera = SimpleCamera::new();
        let window_size = Vec2::new(800.0, 600.0);
        let screen_center = Vec2::new(400.0, 300.0);
        let world_pos = camera.screen_to_world(screen_center, window_size);
        assert_eq!(world_pos, Vec2::new(0.0, 0.0));
    }
}
