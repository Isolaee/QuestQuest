use crate::core::HexGrid;
use crate::rendering::VertexBuffer;
use gl::types::*;

pub struct Renderer {
    pub vao: GLuint,
    pub shader_program: GLuint,
    pub vertex_buffer: VertexBuffer,
}

impl Renderer {
    pub fn new(vao: GLuint, shader_program: GLuint, vbo: GLuint) -> Self {
        Self {
            vao,
            shader_program,
            vertex_buffer: VertexBuffer::new(vbo),
        }
    }

    pub fn render(&self, hex_grid: &HexGrid) {
        let visible_hexagons = hex_grid.get_visible_hexagons();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if !visible_hexagons.is_empty() {
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
