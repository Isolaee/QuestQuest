//! Encyclopedia Panel UI Component
//!
//! Displays the in-game encyclopedia with scrollable text content showing
//! units, terrain, and game mechanics information.

use super::text_renderer::TextRenderer;
use gl::types::*;

/// Width of the encyclopedia panel as percentage of screen width
const ENCYCLOPEDIA_WIDTH_RATIO: f32 = 0.70;

/// Height of the encyclopedia panel as percentage of screen height
const ENCYCLOPEDIA_HEIGHT_RATIO: f32 = 0.85;

/// Line height for text rendering in pixels
const LINE_HEIGHT: f32 = 16.0;

/// Text size for encyclopedia content
const TEXT_SIZE: f32 = 10.0;

/// Text size for titles
const TITLE_TEXT_SIZE: f32 = 14.0;

/// Categories available in the encyclopedia
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaCategory {
    /// Unit information
    Units,
    /// Terrain information
    Terrain,
    /// Game mechanics
    Mechanics,
}

/// Encyclopedia panel that displays wiki-style game information
pub struct EncyclopediaPanel {
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
    /// Text renderer for all text in the encyclopedia
    text_renderer: TextRenderer,
    /// Current scroll offset (in lines)
    pub scroll_offset: i32,
    /// Current category being displayed
    pub current_category: EncyclopediaCategory,
    /// Cached content lines for current view
    content_lines: Vec<String>,
    /// Maximum scroll offset based on content
    max_scroll: i32,
}

impl EncyclopediaPanel {
    /// Creates a new encyclopedia panel centered on the screen
    ///
    /// # Arguments
    ///
    /// * `screen_width` - Width of the screen in pixels
    /// * `screen_height` - Height of the screen in pixels
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `EncyclopediaPanel` or an error message
    pub fn new(screen_width: f32, screen_height: f32) -> Result<Self, String> {
        let width = screen_width * ENCYCLOPEDIA_WIDTH_RATIO;
        let height = screen_height * ENCYCLOPEDIA_HEIGHT_RATIO;
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
            scroll_offset: 0,
            current_category: EncyclopediaCategory::Units,
            content_lines: Vec::new(),
            max_scroll: 0,
        })
    }

    /// Sets up OpenGL resources for encyclopedia rendering
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

        Ok((shader_program, vao, vbo))
    }

    /// Update the content to display based on current category
    pub fn update_content(&mut self, content: Vec<String>) {
        self.content_lines = content;
        self.max_scroll = (self.content_lines.len() as i32)
            .saturating_sub((self.height / LINE_HEIGHT) as i32)
            .max(0);
        self.scroll_offset = self.scroll_offset.clamp(0, self.max_scroll);
    }

    /// Scroll the content up by the specified number of lines
    pub fn scroll_up(&mut self, lines: i32) {
        self.scroll_offset = (self.scroll_offset - lines).max(0);
    }

    /// Scroll the content down by the specified number of lines
    pub fn scroll_down(&mut self, lines: i32) {
        self.scroll_offset = (self.scroll_offset + lines).min(self.max_scroll);
    }

    /// Switch to a different category
    pub fn set_category(&mut self, category: EncyclopediaCategory) {
        if self.current_category != category {
            self.current_category = category;
            self.scroll_offset = 0;
        }
    }

    /// Render the encyclopedia panel
    pub fn render(&mut self, screen_width: f32, screen_height: f32) {
        unsafe {
            // Enable blending for transparency
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            // Render semi-transparent background overlay
            self.render_background_overlay(screen_width, screen_height);

            // Render panel background (light brown - opaque)
            self.render_box(
                self.x,
                self.y,
                self.width,
                self.height,
                [0.45, 0.38, 0.30, 1.0],
            );

            // Render border (worn book edge - darker brown, opaque)
            self.render_border(
                self.x,
                self.y,
                self.width,
                self.height,
                [0.4, 0.3, 0.2, 1.0],
            );

            // Unbind VAO before text rendering
            gl::BindVertexArray(0);

            // Render title bar
            self.render_title_bar(screen_width, screen_height);

            // Render content
            self.render_content(screen_width, screen_height);

            // Render scroll indicator
            self.render_scroll_indicator(screen_width, screen_height);

            gl::Disable(gl::BLEND);
        }
    }

    /// Render a fully opaque overlay over the entire screen
    unsafe fn render_background_overlay(&self, screen_width: f32, screen_height: f32) {
        // Solid dark brown background to completely block game
        self.render_box(
            0.0,
            0.0,
            screen_width,
            screen_height,
            [0.20, 0.16, 0.12, 1.0],
        );
    }

    /// Render the title bar with category tabs
    unsafe fn render_title_bar(&mut self, screen_width: f32, screen_height: f32) {
        let title_height = 40.0;
        let margin = 10.0;

        // Title bar background (medium brown, opaque)
        self.render_box(
            self.x,
            self.y,
            self.width,
            title_height,
            [0.40, 0.33, 0.26, 1.0],
        );

        // Title bar bottom accent line (decorative book line - golden brown)
        self.render_box(
            self.x,
            self.y + title_height - 2.0,
            self.width,
            2.0,
            [0.5, 0.4, 0.25, 1.0],
        );

        // Title text (white)
        let title = "📚 QUESTQUEST ENCYCLOPEDIA";
        self.text_renderer.render_text(
            title,
            self.x + margin,
            self.y + 12.0,
            TITLE_TEXT_SIZE,
            [1.0, 1.0, 1.0, 1.0],
            screen_width,
            screen_height,
        );

        // Category tabs (dynamically numbered 1..n)
        let tab_y = self.y + title_height + 5.0;
        let categories = [
            ("Units", EncyclopediaCategory::Units),
            ("Terrain", EncyclopediaCategory::Terrain),
            ("Mechanics", EncyclopediaCategory::Mechanics),
        ];

        let mut tab_x = self.x + margin;
        for (i, (label, category)) in categories.iter().enumerate() {
            let is_active = *category == self.current_category;
            let color = if is_active {
                [1.0, 1.0, 1.0, 1.0] // Active tab: pure white
            } else {
                [0.8, 0.8, 0.8, 1.0] // Inactive tab: light gray
            };
            let tab_label = format!("{}: {}", i + 1, label);
            self.text_renderer.render_text(
                &tab_label,
                tab_x,
                tab_y,
                TEXT_SIZE,
                color,
                screen_width,
                screen_height,
            );
            tab_x += 120.0;
        }
    }

    /// Render the scrollable content
    unsafe fn render_content(&mut self, screen_width: f32, screen_height: f32) {
        let content_start_y = self.y + 60.0;
        let margin = 15.0;
        let max_visible_lines = ((self.height - 80.0) / LINE_HEIGHT) as usize;

        // Render content area background (light beige/cream - opaque)
        let content_height = self.height - 80.0;
        self.render_box(
            self.x + 5.0,
            content_start_y - 5.0,
            self.width - 10.0,
            content_height + 10.0,
            [0.55, 0.48, 0.40, 1.0],
        );

        let start_line = self.scroll_offset as usize;
        let end_line = (start_line + max_visible_lines).min(self.content_lines.len());

        for (i, line) in self.content_lines[start_line..end_line].iter().enumerate() {
            let y = content_start_y + (i as f32 * LINE_HEIGHT);

            // Determine color based on line content (white text on brown)
            let color = if line.starts_with("╔")
                || line.starts_with("╠")
                || line.starts_with("╚")
                || line.starts_with("║")
                || line.starts_with("═")
            {
                [0.9, 0.85, 0.75, 1.0] // Border color (light beige)
            } else if line.contains("•") {
                [1.0, 1.0, 1.0, 1.0] // List item color (pure white for emphasis)
            } else {
                [0.95, 0.95, 0.95, 1.0] // Default text color (off-white)
            };

            self.text_renderer.render_text(
                line,
                self.x + margin,
                y,
                TEXT_SIZE - 1.0,
                color,
                screen_width,
                screen_height,
            );
        }
    }

    /// Render scroll indicator showing position in content
    unsafe fn render_scroll_indicator(&mut self, screen_width: f32, screen_height: f32) {
        let footer_height = 35.0;
        let footer_y = self.y + self.height - footer_height;

        // Render footer background (medium brown - opaque)
        self.render_box(
            self.x,
            footer_y,
            self.width,
            footer_height,
            [0.42, 0.35, 0.28, 1.0],
        );

        // Footer top accent line (decorative book line - golden brown)
        self.render_box(self.x, footer_y, self.width, 2.0, [0.5, 0.4, 0.25, 1.0]);

        if self.max_scroll > 0 {
            let indicator_text = format!(
                "↑/↓ to scroll | Line {}/{}",
                self.scroll_offset + 1,
                self.content_lines.len()
            );
            let indicator_y = footer_y + 12.0;

            self.text_renderer.render_text(
                &indicator_text,
                self.x + 15.0,
                indicator_y,
                TEXT_SIZE - 2.0,
                [1.0, 1.0, 1.0, 1.0], // White text
                screen_width,
                screen_height,
            );
        }

        // Show ESC hint
        let hint_text = "Press E or ESC to close";
        self.text_renderer.render_text(
            hint_text,
            self.x + self.width - 180.0,
            footer_y + 12.0,
            TEXT_SIZE - 2.0,
            [1.0, 1.0, 1.0, 1.0], // White text
            screen_width,
            screen_height,
        );
    }

    /// Render a filled box
    unsafe fn render_box(&self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
        gl::UseProgram(self.shader_program);

        let screen_size_loc = gl::GetUniformLocation(self.shader_program, c"screenSize".as_ptr());
        let color_loc = gl::GetUniformLocation(self.shader_program, c"color".as_ptr());

        gl::Uniform2f(screen_size_loc, 1920.0, 1080.0);
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
    unsafe fn render_border(&self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
        let border_width = 2.0;

        // Top
        self.render_box(x, y, width, border_width, color);
        // Bottom
        self.render_box(x, y + height - border_width, width, border_width, color);
        // Left
        self.render_box(x, y, border_width, height, color);
        // Right
        self.render_box(x + width - border_width, y, border_width, height, color);
    }
}

impl Drop for EncyclopediaPanel {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteProgram(self.shader_program);
        }
    }
}
