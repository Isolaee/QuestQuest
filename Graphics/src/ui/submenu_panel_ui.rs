//! Submenu Panel UI Component
//!
//! Displays a small contextual submenu with a list of action items.
//! Appears when right-clicking with no unit selected.

use super::text_renderer::TextRenderer;
use gl::types::*;

/// Width of the submenu panel in pixels
const SUBMENU_WIDTH: f32 = 180.0;

/// Height per menu item in pixels
const ITEM_HEIGHT: f32 = 35.0;

/// Text size for menu items
const TEXT_SIZE: f32 = 11.0;

/// Padding inside the submenu
const PADDING: f32 = 12.0;

/// Represents a menu item in the submenu
#[derive(Debug, Clone)]
pub struct SubmenuItem {
    pub label: String,
    pub selected: bool,
}

/// Small submenu panel that displays a list of action items
pub struct SubmenuPanel {
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
    /// Text renderer for menu items
    text_renderer: TextRenderer,
    /// Menu items to display
    items: Vec<SubmenuItem>,
}

impl SubmenuPanel {
    /// Creates a new submenu panel at the specified position
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate for panel position
    /// * `y` - Y coordinate for panel position
    /// * `items` - List of menu item labels
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `SubmenuPanel` or an error message
    pub fn new(x: f32, y: f32, items: Vec<String>) -> Result<Self, String> {
        let (vao, vbo, shader_program) = unsafe { Self::setup_rendering()? };
        let text_renderer = TextRenderer::new()?;

        let menu_items: Vec<SubmenuItem> = items
            .into_iter()
            .map(|label| SubmenuItem {
                label,
                selected: false,
            })
            .collect();

        let height = (menu_items.len() as f32) * ITEM_HEIGHT + PADDING * 2.0;

        Ok(Self {
            x,
            y,
            width: SUBMENU_WIDTH,
            height,
            vao,
            vbo,
            shader_program,
            text_renderer,
            items: menu_items,
        })
    }

    /// Sets up OpenGL resources for submenu rendering
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

    /// Updates the menu items
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items
            .into_iter()
            .map(|label| SubmenuItem {
                label,
                selected: false,
            })
            .collect();
        self.height = (self.items.len() as f32) * ITEM_HEIGHT + PADDING * 2.0;
    }

    /// Sets the selected state of an item by index
    pub fn set_selected(&mut self, index: usize, selected: bool) {
        if let Some(item) = self.items.get_mut(index) {
            item.selected = selected;
        }
    }

    /// Checks if a screen position is inside the submenu bounds
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Gets the item index at the given screen position, if any
    pub fn get_item_at_position(&self, x: f32, y: f32) -> Option<usize> {
        if !self.contains_point(x, y) {
            return None;
        }

        let relative_y = y - self.y - PADDING;
        let index = (relative_y / ITEM_HEIGHT) as usize;

        if index < self.items.len() {
            Some(index)
        } else {
            None
        }
    }

    /// Render the submenu panel
    pub fn render(&mut self, screen_width: f32, screen_height: f32) {
        unsafe {
            // Enable blending for transparency
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            // Disable depth testing so menu appears on top
            gl::Disable(gl::DEPTH_TEST);
            gl::DepthMask(gl::FALSE);

            // Render panel background (dark semi-transparent)
            self.render_box(
                self.x,
                self.y,
                self.width,
                self.height,
                [0.15, 0.15, 0.15, 0.95],
                screen_width,
                screen_height,
            );

            // Render border (lighter gray)
            self.render_border(
                self.x,
                self.y,
                self.width,
                self.height,
                [0.4, 0.4, 0.4, 1.0],
                screen_width,
                screen_height,
            );

            // Unbind VAO before text rendering
            gl::BindVertexArray(0);

            // Render menu items
            for (i, item) in self.items.iter().enumerate() {
                let item_y = self.y + PADDING + (i as f32 * ITEM_HEIGHT);

                // Highlight selected item
                if item.selected {
                    self.render_box(
                        self.x + 2.0,
                        item_y,
                        self.width - 4.0,
                        ITEM_HEIGHT,
                        [0.3, 0.4, 0.5, 0.8],
                        screen_width,
                        screen_height,
                    );
                }

                // Render item text
                let text_color = if item.selected {
                    [1.0, 1.0, 1.0, 1.0] // White when selected
                } else {
                    [0.9, 0.9, 0.9, 1.0] // Light gray otherwise
                };

                self.text_renderer.render_text(
                    &item.label,
                    self.x + PADDING,
                    item_y + (ITEM_HEIGHT - TEXT_SIZE) / 2.0,
                    TEXT_SIZE,
                    text_color,
                    screen_width,
                    screen_height,
                );
            }

            // Restore depth state
            gl::DepthMask(gl::TRUE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Disable(gl::BLEND);
        }
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
        let border_width = 1.5;
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

impl Drop for SubmenuPanel {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteProgram(self.shader_program);
        }
    }
}
