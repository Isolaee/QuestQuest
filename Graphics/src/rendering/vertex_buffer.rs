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

            // Get display color (base color blended with sprite color)
            let display_color = hex.get_display_color();

            // Center vertex with color
            vertices.extend_from_slice(&[
                center_x,
                center_y,
                0.0,
                display_color[0],
                display_color[1],
                display_color[2],
            ]);

            // Outer vertices (6 points of POINTY-TOP hexagon)
            // Start at 30 degrees (π/6) to make pointy-top orientation
            for i in 0..=6 {
                let angle = (i as f32) * std::f32::consts::PI / 3.0 + std::f32::consts::PI / 6.0;
                let x = center_x + hex_size * angle.cos();
                let y = center_y + hex_size * angle.sin();
                vertices.extend_from_slice(&[
                    x,
                    y,
                    0.0,
                    display_color[0],
                    display_color[1],
                    display_color[2],
                ]);
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
