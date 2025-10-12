use crate::core::HexGrid;
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

            // Set up texture uniforms
            gl::UseProgram(shader_program);

            // Bind texture units to shader uniforms
            for i in 0..7 {
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
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if !visible_hexagons.is_empty() {
                // Bind all terrain textures to their respective texture units
                self.texture_manager.bind_all_textures();

                self.vertex_buffer.update(
                    &visible_hexagons,
                    hex_grid.camera.position.x,
                    hex_grid.camera.position.y,
                    hex_grid.hex_size,
                );

                gl::UseProgram(self.shader_program);
                gl::BindVertexArray(self.vao);

                // Draw all visible hexagons
                for (i, _) in visible_hexagons.iter().enumerate() {
                    let vertex_offset = (i * 8) as GLint; // 8 vertices per hexagon
                    gl::DrawArrays(gl::TRIANGLE_FAN, vertex_offset, 8);
                }
            }
        }
    }
}
