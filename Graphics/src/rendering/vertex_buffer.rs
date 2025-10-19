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
            // LAYER 1: Render terrain (base layer)
            let terrain_sprite = hex.sprite;

            // Skip rendering if no terrain
            if terrain_sprite != crate::core::hexagon::SpriteType::None {
                // Generate hexagon vertices relative to camera
                let center_x = hex.world_pos.x - camera_x;
                let center_y = hex.world_pos.y - camera_y;

                // Get texture ID for this hexagon's terrain sprite
                let texture_id = terrain_sprite.get_texture_id();

                // Get the display color (includes highlight tinting)
                let color = hex.get_display_color();

                // Center vertex with texture coordinates and color
                vertices.extend_from_slice(&[
                    center_x, center_y, 0.0, // position
                    0.5, 0.5,        // uv (center)
                    texture_id, // texture id
                    color[0], color[1], color[2], // RGB color
                ]);

                // Outer vertices (6 points of FLAT-TOP hexagon with equal sides)
                for i in 0..=6 {
                    let angle = (i as f32) * std::f32::consts::PI / 3.0; // 60-degree steps
                    let x = center_x + hex_size * angle.cos();
                    let y = center_y + hex_size * angle.sin();

                    // Calculate texture coordinates for hexagon vertices
                    let u = 0.5 + 0.4 * angle.cos();
                    let v = 1.0 - (0.5 + 0.4 * angle.sin()); // Inverted V coordinate

                    vertices.extend_from_slice(&[
                        x, y, 0.0, // position
                        u, v,          // uv
                        texture_id, // texture id
                        color[0], color[1], color[2], // RGB color
                    ]);
                }
            }

            // LAYER 2: Render unit/item sprite on top (smaller)
            if let Some(unit_sprite) = hex.unit_sprite {
                let center_x = hex.world_pos.x - camera_x;
                let center_y = hex.world_pos.y - camera_y;

                // Get texture ID for unit/item sprite
                let texture_id = unit_sprite.get_texture_id();

                // Use sprite's own color instead of terrain color
                let sprite_color = unit_sprite.get_color_tint();

                // Scale factor for items/units (make them smaller than terrain)
                let scale_factor = if unit_sprite == crate::core::hexagon::SpriteType::Item {
                    0.5 // Items are 50% the size of the hex
                } else {
                    0.6 // Units are 60% the size of the hex
                };

                let small_hex_size = hex_size * scale_factor;

                // Center vertex
                vertices.extend_from_slice(&[
                    center_x,
                    center_y,
                    0.0, // position
                    0.5,
                    0.5,        // uv (center)
                    texture_id, // texture id
                    sprite_color[0],
                    sprite_color[1],
                    sprite_color[2], // RGB color
                ]);

                // Outer vertices (smaller hexagon for items/units)
                for i in 0..=6 {
                    let angle = (i as f32) * std::f32::consts::PI / 3.0;
                    let x = center_x + small_hex_size * angle.cos();
                    let y = center_y + small_hex_size * angle.sin();

                    let u = 0.5 + 0.4 * angle.cos();
                    let v = 1.0 - (0.5 + 0.4 * angle.sin());

                    vertices.extend_from_slice(&[
                        x,
                        y,
                        0.0, // position
                        u,
                        v,          // uv
                        texture_id, // texture id
                        sprite_color[0],
                        sprite_color[1],
                        sprite_color[2], // RGB color
                    ]);
                }
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
