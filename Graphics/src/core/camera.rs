use crate::core::HexCoord;
use crate::math::Vec2;

pub struct Camera {
    pub position: Vec2,
    pub view_distance: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            view_distance: 3.0,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn can_see(&self, world_pos: Vec2) -> bool {
        let distance = self.position.distance(&world_pos);
        distance <= self.view_distance
    }

    // Convert camera position to hex coordinate (CORRECTED for pointy-top)
    #[allow(dead_code)]
    pub fn to_hex_coord(&self, hex_size: f32) -> HexCoord {
        let q = (2.0 / 3.0 * self.position.x) / (hex_size * 3.0_f32.sqrt());
        let r = (-1.0 / 3.0 * self.position.x + 3.0_f32.sqrt() / 3.0 * self.position.y)
            / (hex_size * 3.0 / 2.0);
        HexCoord::new(q.round() as i32, r.round() as i32)
    }

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }
}
