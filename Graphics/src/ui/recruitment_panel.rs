//! Recruitment Panel UI Component
//!
//! Displays a panel for recruiting units with a list of available units.

use super::text_renderer::TextRenderer;
use gl::types::*;

/// Text size for recruitment panel
const TEXT_SIZE: f32 = 12.0;

/// Title text size
const TITLE_SIZE: f32 = 16.0;

/// Recruitment panel that displays available units for recruitment
pub struct RecruitmentPanel {
    /// X coordinate of the panel's top-left corner
    pub x: f32,
    /// Y coordinate of the panel's top-left corner
    pub y: f32,
    /// Width of the panel in pixels
    pub width: f32,
    /// Height of the panel in pixels
    pub height: f32,
    /// OpenGL Vertex Array Object for rendering
    vao: GLuint,
    /// OpenGL Vertex Buffer Object for rendering
    vbo: GLuint,
    /// Compiled shader program for UI rendering
    shader_program: GLuint,
    /// Text renderer for panel text
    text_renderer: TextRenderer,
}

impl RecruitmentPanel {
    /// Creates a new recruitment panel centered on screen
    pub fn new(screen_width: f32, screen_height: f32) -> Result<Self, String> {
        let width = 500.0;
        let height = 450.0;
        let x = (screen_width - width) / 2.0;
        let y = (screen_height - height) / 2.0;

        let (vao, vbo, shader_program) = unsafe { Self::setup_rendering()? };
        let text_renderer = TextRenderer::new()?;

        Ok(Self {
            x,
            y,
            width,
            height,
            vao,
            vbo,
            shader_program,
            text_renderer,
        })
    }

    /// Sets up OpenGL resources for panel rendering
    unsafe fn setup_rendering() -> Result<(GLuint, GLuint, GLuint), String> {
        let vertex_src = r#"
            #version 330 core
            layout (location = 0) in vec2 position;
            uniform vec2 screenSize;
            
            void main() {
                vec2 ndc = (position / screenSize) * 2.0 - 1.0;
                ndc.y = -ndc.y;
                gl_Position = vec4(ndc, 0.0, 1.0);
            }
        "#;

        let fragment_src = r#"
            #version 330 core
            out vec4 FragColor;
            uniform vec4 color;
            
            void main() {
                FragColor = color;
            }
        "#;

        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str = std::ffi::CString::new(vertex_src).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str = std::ffi::CString::new(fragment_src).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let mut vao = 0;
        let mut vbo = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        Ok((vao, vbo, shader_program))
    }

    /// Renders the recruitment panel with unit list
    pub fn render(
        &mut self,
        screen_width: f32,
        screen_height: f32,
        unit_names: &[(&str, usize)], // (name, index)
    ) {
        unsafe {
            // Enable blending for transparency
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            // Disable depth testing so panel appears on top
            gl::Disable(gl::DEPTH_TEST);
            gl::DepthMask(gl::FALSE);

            // Render semi-transparent overlay
            self.render_box(
                0.0,
                0.0,
                screen_width,
                screen_height,
                [0.0, 0.0, 0.0, 0.5],
                screen_width,
                screen_height,
            );

            // Render panel background (light brown - opaque)
            self.render_box(
                self.x,
                self.y,
                self.width,
                self.height,
                [0.45, 0.38, 0.30, 1.0],
                screen_width,
                screen_height,
            );

            // Render border (darker brown)
            self.render_border(
                self.x,
                self.y,
                self.width,
                self.height,
                [0.25, 0.18, 0.10, 1.0],
                screen_width,
                screen_height,
            );

            // Render title bar background
            self.render_box(
                self.x,
                self.y,
                self.width,
                50.0,
                [0.38, 0.28, 0.16, 1.0],
                screen_width,
                screen_height,
            );

            // Render title bar bottom line
            self.render_box(
                self.x,
                self.y + 48.0,
                self.width,
                2.0,
                [0.5, 0.4, 0.25, 1.0],
                screen_width,
                screen_height,
            );

            // Unbind VAO before text rendering
            gl::BindVertexArray(0);

            // Render title text
            self.text_renderer.render_text(
                "Recruit Human Units",
                self.x + 20.0,
                self.y + 15.0,
                TITLE_SIZE,
                [1.0, 1.0, 1.0, 1.0],
                screen_width,
                screen_height,
            );

            // Render instruction text
            self.text_renderer.render_text(
                "Click on a unit to recruit:",
                self.x + 20.0,
                self.y + 65.0,
                TEXT_SIZE,
                [0.9, 0.9, 0.9, 1.0],
                screen_width,
                screen_height,
            );

            // Render unit list
            let start_y = self.y + 95.0;
            for (name, index) in unit_names {
                let y = start_y + (*index as f32 * 40.0);

                // Render item background on hover (optional - can be added later)
                self.text_renderer.render_text(
                    &format!("{}. {}", index + 1, name),
                    self.x + 40.0,
                    y,
                    TEXT_SIZE + 2.0,
                    [1.0, 1.0, 0.8, 1.0],
                    screen_width,
                    screen_height,
                );
            }

            // Render footer
            let footer_y = self.y + self.height - 35.0;
            self.render_box(
                self.x,
                footer_y,
                self.width,
                35.0,
                [0.42, 0.35, 0.28, 1.0],
                screen_width,
                screen_height,
            );

            self.text_renderer.render_text(
                "Press ESC to cancel",
                self.x + 20.0,
                footer_y + 10.0,
                TEXT_SIZE - 2.0,
                [0.8, 0.8, 0.8, 1.0],
                screen_width,
                screen_height,
            );

            // Restore depth state
            gl::DepthMask(gl::TRUE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Disable(gl::BLEND);
        }
    }

    /// Gets the unit index at the given screen position, if any
    pub fn get_unit_at_position(&self, x: f32, y: f32, unit_count: usize) -> Option<usize> {
        let start_y = self.y + 95.0;

        if x >= self.x
            && x <= self.x + self.width
            && y >= start_y
            && y <= start_y + (unit_count as f32 * 40.0)
        {
            let index = ((y - start_y) / 40.0) as usize;
            if index < unit_count {
                return Some(index);
            }
        }
        None
    }

    /// Render a filled box
    #[allow(clippy::too_many_arguments)]
    unsafe fn render_box(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [f32; 4],
        screen_width: f32,
        screen_height: f32,
    ) {
        gl::UseProgram(self.shader_program);

        let screen_name = std::ffi::CString::new("screenSize").unwrap();
        let color_name = std::ffi::CString::new("color").unwrap();

        let screen_size_loc = gl::GetUniformLocation(self.shader_program, screen_name.as_ptr());
        let color_loc = gl::GetUniformLocation(self.shader_program, color_name.as_ptr());

        gl::Uniform2f(screen_size_loc, screen_width, screen_height);
        gl::Uniform4f(color_loc, color[0], color[1], color[2], color[3]);

        let vertices: [f32; 12] = [
            x,
            y,
            x + width,
            y,
            x + width,
            y + height,
            x,
            y,
            x + width,
            y + height,
            x,
            y + height,
        ];

        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            2 * std::mem::size_of::<f32>() as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }

    /// Render a border around a box
    #[allow(clippy::too_many_arguments)]
    unsafe fn render_border(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [f32; 4],
        screen_width: f32,
        screen_height: f32,
    ) {
        let border_width = 3.0;
        // Top
        self.render_box(
            x,
            y,
            width,
            border_width,
            color,
            screen_width,
            screen_height,
        );
        // Bottom
        self.render_box(
            x,
            y + height - border_width,
            width,
            border_width,
            color,
            screen_width,
            screen_height,
        );
        // Left
        self.render_box(
            x,
            y,
            border_width,
            height,
            color,
            screen_width,
            screen_height,
        );
        // Right
        self.render_box(
            x + width - border_width,
            y,
            border_width,
            height,
            color,
            screen_width,
            screen_height,
        );
    }
}

impl Drop for RecruitmentPanel {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteProgram(self.shader_program);
        }
    }
}
