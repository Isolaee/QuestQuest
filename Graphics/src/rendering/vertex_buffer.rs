use crate::core::Hexagon;
use gl::types::*;
use std::mem;

pub struct VertexBuffer {
    pub vbo: GLuint,
}

impl VertexBuffer {
    pub fn new(vbo: GLuint) -> Self {
        Self { vbo }
    }

    pub fn update(
        &self,
        visible_hexagons: &[&Hexagon],
        camera_x: f32,
        camera_y: f32,
        hex_size: f32,
    ) {
        let mut vertices = Vec::new();

        for hex in visible_hexagons {
            // Generate hexagon vertices relative to camera
            let center_x = hex.world_pos.x - camera_x;
            let center_y = hex.world_pos.y - camera_y;

            // Get texture ID for this hexagon's sprite
            let texture_id = hex.sprite.get_texture_id();

            // Center vertex with texture coordinates (center of texture)
            vertices.extend_from_slice(&[
                center_x, center_y, 0.0, 0.5, // u
                0.5, // v (center is fine as-is)
                texture_id,
            ]);

            // Outer vertices (6 points of FLAT-TOP hexagon with equal sides)
            // For flat-top hexagons, vertices should start at 0 degrees (rightmost point)
            // and go counter-clockwise to create flat edges on top and bottom
            for i in 0..=6 {
                let angle = (i as f32) * std::f32::consts::PI / 3.0; // 60-degree steps
                let x = center_x + hex_size * angle.cos();
                let y = center_y + hex_size * angle.sin();

                // Calculate texture coordinates for hexagon vertices
                // Map the hexagon vertices to a square texture (0,0) to (1,1)
                // Fix upside-down sprites by inverting V coordinate
                let u = 0.5 + 0.4 * angle.cos(); // Slightly smaller to keep texture within bounds
                let v = 1.0 - (0.5 + 0.4 * angle.sin()); // Inverted V coordinate

                vertices.extend_from_slice(&[x, y, 0.0, u, v, texture_id]);
            }
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
        }
    }
}
