use crate::core::HexCoord;
use crate::math::Vec2;

/// Camera for the HexGrid rendering system
///
/// NOTE: This camera works in OpenGL clip space coordinates (-1.0 to 1.0).
/// The renderer uses these coordinates directly without a projection matrix.
/// For pixel-based world coordinates (like lookup tables), use separate conversion.
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

    /// Convert screen coordinates to world coordinates (clip space)
    /// screen_pos: Mouse position in screen coordinates (origin at top-left)
    /// window_size: Window dimensions (width, height)
    ///
    /// Note: This converts to clip space coordinates (-1 to 1) which is what the renderer uses
    pub fn screen_to_world(&self, screen_pos: Vec2, window_size: Vec2) -> Vec2 {
        // Convert screen coordinates to normalized device coordinates (NDC)
        // Screen: (0,0) at top-left, (width, height) at bottom-right
        // NDC: (-1,-1) at bottom-left, (1,1) at top-right
        let ndc_x = (screen_pos.x / window_size.x) * 2.0 - 1.0;
        let ndc_y = -((screen_pos.y / window_size.y) * 2.0 - 1.0); // Flip Y

        // In the renderer, world coordinates are used directly as clip space coordinates
        // So we just add the camera position (which is also in clip space)
        Vec2::new(ndc_x + self.position.x, ndc_y + self.position.y)
    }

    /// Convert world coordinates (clip space) to screen coordinates
    /// world_pos: Position in world/clip space coordinates
    /// window_size: Window dimensions (width, height)
    pub fn world_to_screen(&self, world_pos: Vec2, window_size: Vec2) -> Vec2 {
        // Convert from world/clip space to camera-relative coordinates
        let camera_relative = world_pos - self.position;

        // Convert from NDC (-1 to 1) to screen coordinates
        let screen_x = (camera_relative.x + 1.0) * 0.5 * window_size.x;
        let screen_y = (-camera_relative.y + 1.0) * 0.5 * window_size.y; // Flip Y

        Vec2::new(screen_x, screen_y)
    }

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }
}
