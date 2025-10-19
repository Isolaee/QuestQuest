use crate::core::{HexGrid, Hexagon};
use crate::rendering::{TextureManager, VertexBuffer};
use gl::types::*;
use std::ffi::CString;

pub struct Renderer {
    pub vao: GLuint,
    pub shader_program: GLuint,
    pub vertex_buffer: VertexBuffer,
    pub texture_manager: TextureManager,
}

impl Renderer {
    pub fn new(vao: GLuint, shader_program: GLuint, vbo: GLuint) -> Result<Self, String> {
        let mut texture_manager = TextureManager::new();

        unsafe {
            // Load all terrain textures
            texture_manager
                .load_terrain_sprites()
                .map_err(|e| format!("Failed to load textures: {}", e))?;

            // Load all item textures
            texture_manager
                .load_item_sprites()
                .map_err(|e| format!("Failed to load item sprites: {}", e))?;

            // Set up texture uniforms
            gl::UseProgram(shader_program);

            // Bind texture units to shader uniforms (0-6 for terrain, 7 for items)
            for i in 0..8 {
                let uniform_name = format!("textures[{}]", i);
                let uniform_name_c = CString::new(uniform_name).unwrap();
                let uniform_location =
                    gl::GetUniformLocation(shader_program, uniform_name_c.as_ptr());
                if uniform_location != -1 {
                    gl::Uniform1i(uniform_location, i);
                }
            }
        }

        Ok(Self {
            vao,
            shader_program,
            vertex_buffer: VertexBuffer::new(vbo),
            texture_manager,
        })
    }

    pub fn render(&self, hex_grid: &HexGrid) {
        let visible_hexagons = hex_grid.get_visible_hexagons();

        unsafe {
            // Clear screen and enable depth testing for proper layering
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);

            if !visible_hexagons.is_empty() {
                // Bind all textures
                self.texture_manager.bind_all_textures();
                gl::UseProgram(self.shader_program);
                gl::BindVertexArray(self.vao);

                // LAYER 1: Render terrain (bottom layer, z = 0.0)
                self.render_terrain_layer(&visible_hexagons, hex_grid);

                // LAYER 2: Render items and units (middle layer, z = -0.5)
                self.render_sprites_layer(&visible_hexagons, hex_grid);
            }
        }
    }

    unsafe fn render_terrain_layer(&self, visible_hexagons: &[&Hexagon], hex_grid: &HexGrid) {
        // Build vertices for terrain only
        let mut vertices = Vec::new();

        for hex in visible_hexagons {
            let terrain_sprite = hex.sprite;

            // Skip if no terrain
            if terrain_sprite == crate::core::hexagon::SpriteType::None {
                continue;
            }

            let center_x = hex.world_pos.x - hex_grid.camera.position.x;
            let center_y = hex.world_pos.y - hex_grid.camera.position.y;
            let texture_id = terrain_sprite.get_texture_id();
            let color = hex.get_display_color();

            // Center vertex (z = 0.0 for terrain layer)
            vertices.extend_from_slice(&[
                center_x, center_y, 0.0, // position with depth
                0.5, 0.5,        // uv
                texture_id, // texture id
                color[0], color[1], color[2], // RGB color
            ]);

            // Outer vertices
            for i in 0..=6 {
                let angle = (i as f32) * std::f32::consts::PI / 3.0;
                let x = center_x + hex_grid.hex_size * angle.cos();
                let y = center_y + hex_grid.hex_size * angle.sin();
                let u = 0.5 + 0.4 * angle.cos();
                let v = 1.0 - (0.5 + 0.4 * angle.sin());

                vertices.extend_from_slice(&[
                    x, y, 0.0, // position with depth
                    u, v,          // uv
                    texture_id, // texture id
                    color[0], color[1], color[2], // RGB color
                ]);
            }
        }

        // Upload and draw terrain
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        let hex_count = vertices.len() / (8 * 9); // 8 vertices, 9 floats each
        for i in 0..hex_count {
            let vertex_offset = (i * 8) as GLint;
            gl::DrawArrays(gl::TRIANGLE_FAN, vertex_offset, 8);
        }
    }

    unsafe fn render_sprites_layer(&self, visible_hexagons: &[&Hexagon], hex_grid: &HexGrid) {
        // Build vertices for items and units only
        let mut vertices = Vec::new();

        for hex in visible_hexagons {
            if let Some(unit_sprite) = hex.unit_sprite {
                let center_x = hex.world_pos.x - hex_grid.camera.position.x;
                let center_y = hex.world_pos.y - hex_grid.camera.position.y;
                let texture_id = unit_sprite.get_texture_id();
                let sprite_color = unit_sprite.get_color_tint();

                // Scale factor for items/units
                let scale_factor = if unit_sprite == crate::core::hexagon::SpriteType::Item {
                    0.5
                } else {
                    0.6
                };

                let small_hex_size = hex_grid.hex_size * scale_factor;

                // Center vertex (z = -0.5 to render on top of terrain)
                vertices.extend_from_slice(&[
                    center_x,
                    center_y,
                    -0.5, // position with depth
                    0.5,
                    0.5,        // uv
                    texture_id, // texture id
                    sprite_color[0],
                    sprite_color[1],
                    sprite_color[2], // RGB color
                ]);

                // Outer vertices
                for i in 0..=6 {
                    let angle = (i as f32) * std::f32::consts::PI / 3.0;
                    let x = center_x + small_hex_size * angle.cos();
                    let y = center_y + small_hex_size * angle.sin();
                    let u = 0.5 + 0.4 * angle.cos();
                    let v = 1.0 - (0.5 + 0.4 * angle.sin());

                    vertices.extend_from_slice(&[
                        x,
                        y,
                        -0.5, // position with depth
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

        if vertices.is_empty() {
            return;
        }

        // Upload and draw sprites
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        let hex_count = vertices.len() / (8 * 9); // 8 vertices, 9 floats each
        for i in 0..hex_count {
            let vertex_offset = (i * 8) as GLint;
            gl::DrawArrays(gl::TRIANGLE_FAN, vertex_offset, 8);
        }
    }
}
