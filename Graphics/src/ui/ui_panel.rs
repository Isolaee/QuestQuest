//! UI Panel implementation for the game interface.
//!
//! This module provides the main UI panel that displays unit information, stats,
//! equipment slots, and interactive elements like buttons. The panel is rendered
//! on the right side of the screen using OpenGL.

use super::text_renderer::TextRenderer;
use crate::core::hexagon::SpriteType;
use gl::types::*;
use std::ffi::CString;

/// Width of the UI panel in pixels.
const UI_PANEL_WIDTH: f32 = 350.0;

/// A clickable button with rectangular bounds.
///
/// Represents a UI button that can detect mouse clicks within its boundaries.
#[derive(Debug, Clone, Copy)]
pub struct Button {
    /// X coordinate of the button's top-left corner.
    pub x: f32,
    /// Y coordinate of the button's top-left corner.
    pub y: f32,
    /// Width of the button in pixels.
    pub width: f32,
    /// Height of the button in pixels.
    pub height: f32,
}

impl Button {
    /// Creates a new button with the specified position and size.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the button's top-left corner
    /// * `y` - Y coordinate of the button's top-left corner
    /// * `width` - Width of the button in pixels
    /// * `height` - Height of the button in pixels
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Checks if a point is contained within the button's boundaries.
    ///
    /// # Arguments
    ///
    /// * `px` - X coordinate of the point to test
    /// * `py` - Y coordinate of the point to test
    ///
    /// # Returns
    ///
    /// `true` if the point is within the button, `false` otherwise.
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}

/// Information about a unit to be displayed in the UI panel.
///
/// Contains all the necessary data to render unit stats, health bars,
/// experience progress, and positioning information.
#[derive(Debug, Clone)]
pub struct UnitDisplayInfo {
    /// Name of the unit.
    pub name: String,
    /// Race of the unit (e.g., "Human", "Elf").
    pub race: String,
    /// Class of the unit (e.g., "Warrior", "Mage").
    pub class: String,
    /// Current level of the unit.
    pub level: i32,
    /// Current experience points.
    pub experience: i32,
    /// Current health points.
    pub health: u32,
    /// Maximum health points.
    pub max_health: u32,
    /// Type of terrain the unit is standing on.
    pub terrain: String,
    /// Q coordinate in hexagonal grid system.
    pub position_q: i32,
    /// R coordinate in hexagonal grid system.
    pub position_r: i32,
    /// Remaining movement points for this turn.
    pub moves_left: u32,
    /// Maximum movement points per turn.
    pub max_moves: u32,
    /// Sprite type for visual representation of the unit.
    pub sprite_type: SpriteType,
}

/// Main UI panel that displays game information and interactive elements.
///
/// The UI panel is rendered on the right side of the screen and contains:
/// - Day/time information
/// - Terrain information
/// - Unit sprite and stats
/// - HP and EXP bars with visual fill indicators
/// - Equipment slots layout
/// - Item pickup prompts
/// - End turn button
///
/// The panel uses OpenGL for rendering boxes and borders, and a text renderer
/// for all text elements.
pub struct UiPanel {
    /// X coordinate of the panel's top-left corner.
    pub x: f32,
    /// Y coordinate of the panel's top-left corner.
    pub y: f32,
    /// Width of the panel in pixels.
    pub width: f32,
    /// Height of the panel in pixels.
    pub height: f32,
    /// OpenGL Vertex Array Object for rendering.
    vao: GLuint,
    /// OpenGL Vertex Buffer Object for rendering.
    vbo: GLuint,
    /// Compiled shader program for UI rendering.
    shader_program: GLuint,
    /// Currently displayed unit information, if any.
    unit_info: Option<UnitDisplayInfo>,
    /// Text renderer for all text in the UI.
    text_renderer: TextRenderer,
    /// Name of item for pickup prompt, if active.
    pickup_prompt: Option<String>,
    /// "Pick Up" button for item pickup.
    pickup_yes_button: Option<Button>,
    /// "Leave" button for item pickup.
    pickup_no_button: Option<Button>,
    /// Clickable area around the item name text.
    pickup_text_button: Option<Button>,
    /// End turn button at the bottom of the panel.
    pub end_turn_button: Button,
}

impl UiPanel {
    /// Creates a new UI panel positioned on the right side of the screen.
    ///
    /// # Arguments
    ///
    /// * `screen_width` - Width of the screen in pixels
    /// * `screen_height` - Height of the screen in pixels
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `UiPanel` or an error message if
    /// OpenGL initialization fails.
    ///
    /// # Errors
    ///
    /// Returns an error if shader compilation or OpenGL resource creation fails.
    pub fn new(screen_width: f32, screen_height: f32) -> Result<Self, String> {
        let width = UI_PANEL_WIDTH;
        let height = screen_height;
        let x = screen_width - width;
        let y = 0.0;

        let (vao, vbo, shader_program) = unsafe { Self::setup_ui_rendering()? };
        let text_renderer = TextRenderer::new()?;

        // Create end turn button at bottom of panel
        // y=0 is at TOP of screen, so bottom is at y=height
        let button_width = 280.0;
        let button_height = 50.0;
        let button_x = x + (width - button_width) / 2.0;
        // Position 20px from bottom of screen (y + height - button_height - margin)
        let button_y = y + height - button_height - 20.0;
        let end_turn_button = Button::new(button_x, button_y, button_width, button_height);

        Ok(Self {
            x,
            y,
            width,
            height,
            vao,
            vbo,
            shader_program,
            unit_info: None,
            text_renderer,
            pickup_prompt: None,
            pickup_yes_button: None,
            pickup_no_button: None,
            pickup_text_button: None,
            end_turn_button,
        })
    }

    /// Sets up OpenGL resources for UI rendering.
    ///
    /// Creates and configures the Vertex Array Object (VAO), Vertex Buffer Object (VBO),
    /// and compiles the shader program for rendering UI elements.
    ///
    /// # Safety
    ///
    /// This function makes raw OpenGL calls and must be called with a valid OpenGL context.
    ///
    /// # Returns
    ///
    /// A tuple containing (VAO, VBO, shader_program) or an error message.
    unsafe fn setup_ui_rendering() -> Result<(GLuint, GLuint, GLuint), String> {
        // Create VAO
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create VBO
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Position attribute (2 floats)
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (2 * std::mem::size_of::<f32>()) as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // Create simple shader program
        let shader_program = Self::create_ui_shader()?;

        gl::BindVertexArray(0);

        Ok((vao, vbo, shader_program))
    }

    /// Creates and compiles the shader program for UI rendering.
    ///
    /// Compiles vertex and fragment shaders, links them into a program,
    /// and validates the compilation and linking process.
    ///
    /// # Safety
    ///
    /// This function makes raw OpenGL calls and must be called with a valid OpenGL context.
    ///
    /// # Returns
    ///
    /// The compiled shader program ID or an error message if compilation/linking fails.
    unsafe fn create_ui_shader() -> Result<GLuint, String> {
        let vertex_shader_src = CString::new(
            r#"
            #version 330 core
            layout (location = 0) in vec2 aPos;
            
            uniform vec2 screenSize;
            
            void main() {
                // Convert from screen coordinates to NDC
                vec2 ndc = (aPos / screenSize) * 2.0 - 1.0;
                ndc.y = -ndc.y; // Flip Y
                gl_Position = vec4(ndc, 0.0, 1.0);
            }
            "#,
        )
        .unwrap();

        let fragment_shader_src = CString::new(
            r#"
            #version 330 core
            out vec4 FragColor;
            uniform vec4 color;
            
            void main() {
                FragColor = color;
            }
            "#,
        )
        .unwrap();

        // Compile vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_src.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        // Check compilation
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                vertex_shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );
            return Err(format!(
                "Vertex shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            ));
        }

        // Compile fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_src.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        // Check compilation
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                fragment_shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );
            return Err(format!(
                "Fragment shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            ));
        }

        // Link shaders
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check linking
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetProgramInfoLog(
                shader_program,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );
            return Err(format!(
                "Shader linking failed: {}",
                String::from_utf8_lossy(&buffer)
            ));
        }

        // Clean up shaders (no longer needed after linking)
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Ok(shader_program)
    }

    /// Set the unit information to display
    pub fn set_unit_info(&mut self, info: UnitDisplayInfo) {
        self.unit_info = Some(info);
    }

    /// Clear the unit information
    pub fn clear_unit_info(&mut self) {
        self.unit_info = None;
    }

    /// Set the pickup prompt with item name
    pub fn set_pickup_prompt(&mut self, item_name: String) {
        self.pickup_prompt = Some(item_name);
        // Buttons will be created during rendering based on screen size
    }

    /// Clear the pickup prompt
    pub fn clear_pickup_prompt(&mut self) {
        self.pickup_prompt = None;
        self.pickup_yes_button = None;
        self.pickup_no_button = None;
        self.pickup_text_button = None;
    }

    /// Get a mutable reference to the text renderer
    pub fn text_renderer_mut(&mut self) -> &mut TextRenderer {
        &mut self.text_renderer
    }

    /// Get a reference to the text renderer
    pub fn text_renderer(&self) -> &TextRenderer {
        &self.text_renderer
    }

    /// Render text using the UI panel's text renderer
    #[allow(clippy::too_many_arguments)]
    pub fn render_text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        color: [f32; 4],
        screen_width: f32,
        screen_height: f32,
    ) {
        self.text_renderer
            .render_text(text, x, y, size, color, screen_width, screen_height);
    }

    /// Checks if a click position hits the "Yes" (Pick Up) button.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the click
    /// * `y` - Y coordinate of the click
    ///
    /// # Returns
    ///
    /// `true` if the click is within the Yes button bounds, `false` otherwise.
    pub fn check_yes_button_click(&self, x: f32, y: f32) -> bool {
        if let Some(button) = &self.pickup_yes_button {
            button.contains(x, y)
        } else {
            false
        }
    }

    /// Checks if a click position hits the "No" (Leave) button.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the click
    /// * `y` - Y coordinate of the click
    ///
    /// # Returns
    ///
    /// `true` if the click is within the No button bounds, `false` otherwise.
    pub fn check_no_button_click(&self, x: f32, y: f32) -> bool {
        if let Some(button) = &self.pickup_no_button {
            button.contains(x, y)
        } else {
            false
        }
    }

    /// Checks if a click position hits the "End Turn" button.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the click
    /// * `y` - Y coordinate of the click
    ///
    /// # Returns
    ///
    /// `true` if the click is within the End Turn button bounds, `false` otherwise.
    pub fn check_end_turn_button_click(&self, x: f32, y: f32) -> bool {
        self.end_turn_button.contains(x, y)
    }

    /// Renders the entire UI panel including all sub-components.
    ///
    /// This is the main rendering method that orchestrates rendering of:
    /// - Background panel
    /// - Day/time boxes
    /// - Terrain information
    /// - Unit sprite placeholder
    /// - Equipment slots
    /// - Unit stat boxes and bars (HP/EXP)
    /// - All text overlays
    /// - Item pickup prompts (if active)
    /// - End turn button
    ///
    /// # Arguments
    ///
    /// * `screen_width` - Current screen width in pixels
    /// * `screen_height` - Current screen height in pixels
    /// * `texture_manager` - Reference to the texture manager for rendering sprites
    ///
    /// # Safety
    ///
    /// This method contains unsafe OpenGL calls and should be called within
    /// a valid OpenGL rendering context.
    pub fn render(
        &mut self,
        screen_width: f32,
        screen_height: f32,
        renderer: &crate::rendering::renderer::Renderer,
    ) {
        unsafe {
            // Disable depth testing for UI rendering
            gl::Disable(gl::DEPTH_TEST);
            // Enable blending for transparency
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::UseProgram(self.shader_program);

            // Set screen size uniform
            let screen_size_loc =
                gl::GetUniformLocation(self.shader_program, c"screenSize".as_ptr());
            gl::Uniform2f(screen_size_loc, screen_width, screen_height);

            gl::BindVertexArray(self.vao);

            // Render panel background
            self.render_background(screen_width, screen_height);

            // Render clock placeholder
            self.render_clock_placeholder(screen_width, screen_height);

            // Render terrain sprite placeholder
            self.render_terrain_sprite_placeholder(screen_width, screen_height);

            // Render unit sprite placeholder
            self.render_unit_sprite_placeholder(screen_width, screen_height, renderer);

            // Render equipment slots placeholders
            self.render_equipment_slots(screen_width, screen_height);

            // Render unit stat boxes and bars (before unbinding VAO)
            self.render_unit_boxes_and_bars(screen_width, screen_height);

            // Render pickup prompt if active (clone to avoid borrow issues)
            if let Some(item_name) = self.pickup_prompt.clone() {
                self.render_pickup_prompt(screen_width, screen_height, &item_name);
            }

            gl::BindVertexArray(0);

            // Render text on top of everything else
            self.render_time_and_terrain_labels(screen_width, screen_height);
            self.render_unit_text(screen_width, screen_height);

            // Render end turn button last so it remains visible above any prompts
            self.render_end_turn_button(screen_width, screen_height);
        }
    }

    /// Renders unit stat boxes and visual bars (HP and EXP).
    ///
    /// This method renders all the filled boxes and bars for unit stats while the
    /// OpenGL VAO is still bound. It must be called before unbinding the VAO.
    ///
    /// Renders:
    /// - Character name box
    /// - HP/EXP bars container
    /// - HP bar background and colored fill (green/yellow/red based on health)
    /// - EXP bar background and colored fill (blue)
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context with VAO bound.
    unsafe fn render_unit_boxes_and_bars(&mut self, _screen_width: f32, _screen_height: f32) {
        if let Some(info) = &self.unit_info {
            let margin = 10.0;
            let left_box_width = 60.0;
            let sprite_size = self.width - left_box_width - margin * 3.0;
            let stats_section_y = self.y + margin + sprite_size + margin;

            // Character name box
            let name_box_height = 25.0;
            self.render_box(
                self.x + margin,
                stats_section_y,
                self.width - margin * 2.0,
                name_box_height,
                [0.15, 0.15, 0.2, 0.95],
            );
            self.render_border(
                self.x + margin,
                stats_section_y,
                self.width - margin * 2.0,
                name_box_height,
                [0.7, 0.7, 0.7, 1.0],
            );

            // HP and EXP bars section
            let bars_section_y = stats_section_y + name_box_height + 5.0;
            let bars_height = 80.0;
            self.render_box(
                self.x + margin,
                bars_section_y,
                self.width - margin * 2.0,
                bars_height,
                [0.15, 0.15, 0.2, 0.95],
            );
            self.render_border(
                self.x + margin,
                bars_section_y,
                self.width - margin * 2.0,
                bars_height,
                [0.7, 0.7, 0.7, 1.0],
            );

            // Health bar
            let health_bar_y = bars_section_y + 17.0;
            let bar_width = self.width - margin * 4.0;
            let bar_height = 22.0;
            let health_percentage = info.health as f32 / info.max_health as f32;

            // Background bar (darker)
            self.render_box(
                self.x + margin * 2.0,
                health_bar_y,
                bar_width,
                bar_height,
                [0.15, 0.15, 0.15, 1.0],
            );
            self.render_border(
                self.x + margin * 2.0,
                health_bar_y,
                bar_width,
                bar_height,
                [0.3, 0.3, 0.3, 1.0],
            );

            // Health fill with gradient based on percentage
            let health_color = if health_percentage > 0.6 {
                [0.2, 0.8, 0.2, 1.0] // Green
            } else if health_percentage > 0.3 {
                [0.9, 0.8, 0.2, 1.0] // Yellow
            } else {
                [0.9, 0.2, 0.2, 1.0] // Red
            };

            self.render_box(
                self.x + margin * 2.0,
                health_bar_y,
                bar_width * health_percentage,
                bar_height,
                health_color,
            );

            // Experience bar
            let exp_bar_y = bars_section_y + 57.0;
            let exp_bar_height = 18.0;
            let exp_needed = 1000;
            let current_exp = info.experience % exp_needed;
            let exp_percentage = current_exp as f32 / exp_needed as f32;

            // Background bar (dark)
            self.render_box(
                self.x + margin * 2.0,
                exp_bar_y,
                bar_width,
                exp_bar_height,
                [0.15, 0.15, 0.2, 1.0],
            );
            self.render_border(
                self.x + margin * 2.0,
                exp_bar_y,
                bar_width,
                exp_bar_height,
                [0.3, 0.3, 0.35, 1.0],
            );

            // Experience fill (blue/cyan)
            self.render_box(
                self.x + margin * 2.0,
                exp_bar_y,
                bar_width * exp_percentage,
                exp_bar_height,
                [0.3, 0.6, 0.9, 1.0],
            );
        }
    }

    /// Renders all text overlays for unit information.
    ///
    /// This method renders text after the VAO is unbound, including:
    /// - Unit name
    /// - Race and class
    /// - HP bar label and numeric values (current/max)
    /// - EXP bar label and numeric values (current/needed)
    ///
    /// Text is rendered with shadows for better visibility.
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context after VAO is unbound.
    unsafe fn render_unit_text(&mut self, screen_width: f32, screen_height: f32) {
        if let Some(info) = &self.unit_info {
            let margin = 10.0;
            let left_box_width = 60.0;
            let sprite_size = self.width - left_box_width - margin * 3.0;
            let stats_section_y = self.y + margin + sprite_size + margin;
            let text_x = self.x + margin;
            let text_size = 10.0;
            let white = [1.0, 1.0, 1.0, 1.0];

            let name_box_height = 25.0;

            // Render unit name and race on the left side of the box
            let name_y = stats_section_y + 4.0;
            self.text_renderer.render_text(
                &info.name,
                text_x + 5.0,
                name_y,
                text_size + 2.0,
                white,
                screen_width,
                screen_height,
            );

            // Render race and class below name in smaller text
            let race_class_y = name_y + 13.0;
            let race_class = format!("{} {}", info.race, info.class);
            self.text_renderer.render_text(
                &race_class,
                text_x + 5.0,
                race_class_y,
                text_size - 1.0,
                [0.8, 0.8, 0.8, 1.0],
                screen_width,
                screen_height,
            );

            // HP and EXP bars section (boxes rendered separately)
            let bars_section_y = stats_section_y + name_box_height + 5.0;

            // HP bar label
            let hp_label_y = bars_section_y + 5.0;
            self.text_renderer.render_text(
                "HP",
                text_x + 5.0,
                hp_label_y,
                text_size - 2.0,
                [0.7, 0.7, 0.7, 1.0],
                screen_width,
                screen_height,
            );

            // Health bar text (bar rendered separately)
            let health_bar_y = bars_section_y + 17.0;
            let bar_width = self.width - margin * 4.0;
            let bar_height = 22.0;

            // HP text centered on the bar with shadow for visibility
            let hp_text = format!("{}/{}", info.health, info.max_health);
            let hp_text_x = self.x + margin * 2.0 + bar_width / 2.0
                - (hp_text.len() as f32 * (text_size - 1.0) * 0.3);
            let hp_text_y = health_bar_y + bar_height / 2.0 - (text_size - 1.0) / 2.0;

            // Text shadow for better readability
            self.text_renderer.render_text(
                &hp_text,
                hp_text_x + 1.0,
                hp_text_y + 1.0,
                text_size - 1.0,
                [0.0, 0.0, 0.0, 0.8],
                screen_width,
                screen_height,
            );

            // Main text
            self.text_renderer.render_text(
                &hp_text,
                hp_text_x,
                hp_text_y,
                text_size - 1.0,
                white,
                screen_width,
                screen_height,
            );

            // Exp bar label
            let exp_label_y = bars_section_y + 45.0;
            self.text_renderer.render_text(
                "Experience",
                text_x + 5.0,
                exp_label_y,
                text_size - 2.0,
                [0.7, 0.7, 0.7, 1.0],
                screen_width,
                screen_height,
            );

            // Experience bar text (bar rendered separately)
            let exp_bar_y = bars_section_y + 57.0;
            let exp_bar_height = 18.0;
            let exp_needed = 1000;
            let current_exp = info.experience % exp_needed;

            // EXP text centered on the bar with shadow
            let exp_text = format!("{}/{}", current_exp, exp_needed);
            let exp_text_x = self.x + margin * 2.0 + bar_width / 2.0
                - (exp_text.len() as f32 * (text_size - 2.0) * 0.3);
            let exp_text_y = exp_bar_y + exp_bar_height / 2.0 - (text_size - 2.0) / 2.0;

            // Text shadow for better readability
            self.text_renderer.render_text(
                &exp_text,
                exp_text_x + 1.0,
                exp_text_y + 1.0,
                text_size - 2.0,
                [0.0, 0.0, 0.0, 0.8],
                screen_width,
                screen_height,
            );

            // Main text
            self.text_renderer.render_text(
                &exp_text,
                exp_text_x,
                exp_text_y,
                text_size - 2.0,
                [0.9, 0.9, 0.9, 1.0],
                screen_width,
                screen_height,
            );

            // Additional stats - compact layout below bars
            // (These can be added later if needed)
        }
    }
    /// Renders the semi-transparent dark background for the entire UI panel.
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context with VAO bound.
    unsafe fn render_background(&self, _screen_width: f32, _screen_height: f32) {
        // Semi-transparent dark background for the panel
        let vertices: Vec<f32> = vec![
            self.x,
            self.y, // Top-left
            self.x + self.width,
            self.y, // Top-right
            self.x + self.width,
            self.y + self.height, // Bottom-right
            self.x,
            self.y + self.height, // Bottom-left
        ];

        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        let color_loc = gl::GetUniformLocation(self.shader_program, c"color".as_ptr());
        gl::Uniform4f(color_loc, 0.1, 0.1, 0.15, 0.9); // Dark semi-transparent

        gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
    }

    /// Renders placeholder boxes for day/time graph and terrain information.
    ///
    /// Renders two boxes on the left side of the panel:
    /// - Day graph box (top)
    /// - Terrain box (below day graph)
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context with VAO bound.
    unsafe fn render_clock_placeholder(&self, _screen_width: f32, _screen_height: f32) {
        let margin = 10.0;
        let top_y = self.y + margin;

        // Left side: Day Graph and Terrain boxes only (text will be rendered separately)
        let left_box_width = 60.0;
        let box_height = 60.0; // Increased to make boxes more square
        let spacing = 8.0;
        let text_spacing = 18.0; // Space for text below each box

        let left_x = self.x + margin;

        // Day Graph box
        self.render_box(
            left_x,
            top_y,
            left_box_width,
            box_height,
            [0.25, 0.25, 0.3, 1.0],
        );
        self.render_border(
            left_x,
            top_y,
            left_box_width,
            box_height,
            [0.7, 0.7, 0.7, 1.0],
        );

        // Terrain box (below Day Graph + text space)
        let terrain_y = top_y + box_height + text_spacing + spacing;
        self.render_box(
            left_x,
            terrain_y,
            left_box_width,
            box_height,
            [0.2, 0.5, 0.2, 1.0],
        );
        self.render_border(
            left_x,
            terrain_y,
            left_box_width,
            box_height,
            [0.7, 0.7, 0.7, 1.0],
        );
    }

    /// Placeholder method for terrain sprite rendering.
    ///
    /// Currently unused as terrain rendering is handled in `render_clock_placeholder`.
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context.
    unsafe fn render_terrain_sprite_placeholder(&self, _screen_width: f32, _screen_height: f32) {
        // This is now rendered in render_clock_placeholder as part of the left column
    }

    /// Renders text labels for day stats and terrain stats.
    ///
    /// Renders small text labels below the day graph and terrain boxes.
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context.
    unsafe fn render_time_and_terrain_labels(&mut self, screen_width: f32, screen_height: f32) {
        let margin = 10.0;
        let top_y = self.y + margin;
        let _left_box_width = 60.0;
        let box_height = 60.0;
        let spacing = 8.0;
        let text_spacing = 18.0;
        let left_x = self.x + margin;

        let text_size = 8.0;
        let text_color = [0.8, 0.8, 0.8, 1.0];

        // Day stats text below Day Graph box
        let day_text_y = top_y + box_height + 3.0;
        self.text_renderer.render_text(
            "Day stats",
            left_x + 2.0,
            day_text_y,
            text_size,
            text_color,
            screen_width,
            screen_height,
        );

        // Terrain stats text below Terrain box
        let terrain_y = top_y + box_height + text_spacing + spacing;
        let terrain_text_y = terrain_y + box_height + 3.0;
        self.text_renderer.render_text(
            "Terrain stats",
            left_x + 2.0,
            terrain_text_y,
            text_size,
            text_color,
            screen_width,
            screen_height,
        );
    }

    /// Renders a placeholder box for the unit sprite.
    ///
    /// Renders a square box on the right side of the top section where
    /// the unit's portrait/sprite will be displayed. If unit info is available,
    /// the texture should be rendered by the renderer.
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context with VAO bound.
    unsafe fn render_unit_sprite_placeholder(
        &self,
        screen_width: f32,
        screen_height: f32,
        renderer: &crate::rendering::renderer::Renderer,
    ) {
        let margin = 10.0;
        let top_y = self.y + margin;
        let left_box_width = 60.0;
        let left_column_end = self.x + margin + left_box_width + margin;

        // Character sprite box - square, takes remaining width
        let sprite_size = self.width - left_box_width - margin * 3.0;

        // Render background box
        self.render_box(
            left_column_end,
            top_y,
            sprite_size,
            sprite_size,
            [0.15, 0.15, 0.2, 1.0],
        );

        // If we have unit info, render the actual sprite texture
        if let Some(info) = &self.unit_info {
            renderer.render_sprite_at_screen_pos(
                info.sprite_type,
                left_column_end,
                top_y,
                sprite_size,
                sprite_size,
                (screen_width, screen_height),
            );
            // Note: render_sprite_at_screen_pos now properly saves/restores OpenGL state
        }

        // Render border on top
        self.render_border(
            left_column_end,
            top_y,
            sprite_size,
            sprite_size,
            [0.7, 0.7, 0.7, 1.0],
        );
    }

    /// Renders all equipment slot placeholders.
    ///
    /// Renders a complete equipment layout including:
    /// - Helmet (top center)
    /// - Necklace (below helmet, half height)
    /// - Left and right rings (small slots on sides)
    /// - Left and right hand slots
    /// - Armor (large center piece)
    /// - Left and right boot slots (bottom)
    ///
    /// Each slot is color-coded and positioned according to standard RPG equipment layout.
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context with VAO bound.
    unsafe fn render_equipment_slots(&self, _screen_width: f32, _screen_height: f32) {
        let slot_size = 60.0;
        let small_slot_size = 50.0;
        let margin = 10.0;
        let spacing = 8.0;
        // Calculate sprite size to position equipment correctly
        let left_box_width = 60.0;
        let sprite_size = self.width - left_box_width - margin * 3.0;
        // Position equipment below: sprite + name box (25) + bars (80) + margins
        let start_y = self.y + margin + sprite_size + margin + 25.0 + 5.0 + 80.0 + margin;

        // Calculate center X of the panel
        let center_x = self.x + self.width / 2.0;

        // Layout based on the PNG diagram:
        // Row 1: Helmet (center top)
        let helmet_x = center_x - slot_size / 2.0;
        let helmet_y = start_y;

        // Row 2: Necklace (center, below helmet) - half the height of helmet
        let necklace_width = slot_size;
        let necklace_height = slot_size / 2.0;
        let necklace_x = center_x - necklace_width / 2.0;
        let necklace_y = helmet_y + slot_size + spacing;

        // Row 3: Ring 1 (left), Armor (large center), Ring 2 (right)
        let armor_y = necklace_y + slot_size + spacing;
        let armor_x = center_x - slot_size * 0.9;
        let armor_width = slot_size * 1.8;
        let armor_height = slot_size * 1.5;

        let ring1_x = self.x + margin;
        let ring1_y = armor_y;

        let ring2_x = self.x + self.width - small_slot_size - margin;
        let ring2_y = armor_y;

        // Row 3.5: Left hand and Right hand (sides of armor, vertically centered)
        let hand_y = armor_y + armor_height / 2.0 - slot_size / 2.0;
        let left_hand_x = ring1_x;
        let right_hand_x = ring2_x;

        // Row 4: Boots (two slots at bottom, centered side by side)
        let boots_y = armor_y + armor_height + spacing;
        let boots_spacing = 15.0;
        let boots_left_x = center_x - small_slot_size - boots_spacing / 2.0;
        let boots_right_x = center_x + boots_spacing / 2.0;

        // Render Helmet (center top)
        self.render_box(
            helmet_x,
            helmet_y,
            slot_size,
            slot_size,
            [0.6, 0.6, 0.7, 1.0],
        );
        self.render_border(
            helmet_x,
            helmet_y,
            slot_size,
            slot_size,
            [0.7, 0.7, 0.7, 1.0],
        );

        // Render Necklace (below helmet) - half height
        self.render_box(
            necklace_x,
            necklace_y,
            necklace_width,
            necklace_height,
            [0.8, 0.7, 0.3, 1.0],
        );
        self.render_border(
            necklace_x,
            necklace_y,
            necklace_width,
            necklace_height,
            [0.7, 0.7, 0.7, 1.0],
        );

        // Render Ring 1 (small, top left)
        self.render_box(
            ring1_x,
            ring1_y,
            small_slot_size,
            small_slot_size,
            [0.9, 0.8, 0.2, 1.0],
        );
        self.render_border(
            ring1_x,
            ring1_y,
            small_slot_size,
            small_slot_size,
            [0.7, 0.7, 0.7, 1.0],
        );

        // Render Ring 2 (small, top right)
        self.render_box(
            ring2_x,
            ring2_y,
            small_slot_size,
            small_slot_size,
            [0.9, 0.8, 0.2, 1.0],
        );
        self.render_border(
            ring2_x,
            ring2_y,
            small_slot_size,
            small_slot_size,
            [0.7, 0.7, 0.7, 1.0],
        );

        // Render Left Hand (middle left)
        self.render_box(
            left_hand_x,
            hand_y,
            slot_size,
            slot_size,
            [0.5, 0.4, 0.3, 1.0],
        );
        self.render_border(
            left_hand_x,
            hand_y,
            slot_size,
            slot_size,
            [0.7, 0.7, 0.7, 1.0],
        );

        // Render Armor (large center piece)
        self.render_box(
            armor_x,
            armor_y,
            armor_width,
            armor_height,
            [0.4, 0.5, 0.6, 1.0],
        );
        self.render_border(
            armor_x,
            armor_y,
            armor_width,
            armor_height,
            [0.7, 0.7, 0.7, 1.0],
        );

        // Render Right Hand (middle right)
        self.render_box(
            right_hand_x,
            hand_y,
            slot_size,
            slot_size,
            [0.5, 0.4, 0.3, 1.0],
        );
        self.render_border(
            right_hand_x,
            hand_y,
            slot_size,
            slot_size,
            [0.7, 0.7, 0.7, 1.0],
        );

        // Render Boots - Left Boot
        self.render_box(
            boots_left_x,
            boots_y,
            small_slot_size,
            small_slot_size,
            [0.4, 0.3, 0.2, 1.0],
        );
        self.render_border(
            boots_left_x,
            boots_y,
            small_slot_size,
            small_slot_size,
            [0.7, 0.7, 0.7, 1.0],
        );

        // Render Boots - Right Boot
        self.render_box(
            boots_right_x,
            boots_y,
            small_slot_size,
            small_slot_size,
            [0.4, 0.3, 0.2, 1.0],
        );
        self.render_border(
            boots_right_x,
            boots_y,
            small_slot_size,
            small_slot_size,
            [0.7, 0.7, 0.7, 1.0],
        );
    }

    /// Renders the item pickup prompt dialog.
    ///
    /// Displays a prompt with the item name and two buttons (Pick Up / Leave)
    /// positioned above the End Turn button. The prompt is centered horizontally
    /// within the UI panel and automatically adjusts padding if space is limited.
    ///
    /// # Arguments
    ///
    /// * `screen_width` - Current screen width in pixels
    /// * `screen_height` - Current screen height in pixels
    /// * `item_name` - Name of the item to display in the prompt
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context with VAO bound.
    unsafe fn render_pickup_prompt(
        &mut self,
        screen_width: f32,
        screen_height: f32,
        item_name: &str,
    ) {
        // Render near the bottom of the UI panel, but place above the End Turn button
        // so the pickup choices don't overlap the End Turn button.
        let margin = 10.0;
        let prompt_width = self.width - (margin * 2.0);
        let prompt_x = self.x + margin;
        // Text sizes and padding
        let item_text_size = 14.0; // slightly larger for item name
        let btn_text_size = 14.0;
        // Increase padding to give the item name a larger box like the screenshot
        let text_padding_x = 16.0;
        let mut text_padding_y = 10.0;
        let button_spacing = 10.0;
        let gap_between_text_and_buttons = 12.0;

        // Labels
        let pick_label = "Pick Up";
        let no_label = "Leave";

        // Compute widths based on label length (simple heuristic)
        // Compute button widths based on label length (simple heuristic)
        let pick_text_w = (pick_label.len() as f32) * btn_text_size * 0.6;
        let no_text_w = (no_label.len() as f32) * btn_text_size * 0.6;

        let pick_button_width = pick_text_w + text_padding_x * 2.0;
        let no_button_width = no_text_w + text_padding_x * 2.0;

        // Button height based on text size + vertical padding
        let mut button_height = btn_text_size + text_padding_y * 2.0;

        // Box dimensions for the text (padding around text)
        let measured_width = (item_name.len() as f32 * item_text_size * 0.6) + text_padding_x * 2.0;
        let min_text_box_width = 140.0; // reasonable minimum so short names look roomy
        let text_box_width = measured_width.max(min_text_box_width);
        // Make the text box slightly taller as in the screenshot
        let mut text_box_height = item_text_size + text_padding_y * 2.0 + 6.0;

        // Total width of the two buttons (without extra container padding)
        let total_buttons_width = pick_button_width + no_button_width + button_spacing;

        // Container padding inside the container around buttons
        let container_padding = 8.0;
        // Determine inner width of container (space for buttons or text box)
        let container_inner_width = total_buttons_width.max(text_box_width);
        let container_width = container_inner_width + container_padding * 2.0;

        // Compute the vertical layout so the whole prompt sits above the End Turn button
        let container_height = button_height + container_padding * 2.0;
        // total_prompt_height intentionally omitted (computed sizes used directly)

        // Place container so its bottom sits margin pixels above the End Turn button top
        let mut container_top_y = if self.end_turn_button.y > 0.0 {
            // end_turn_button.y is the top of the button; place container so it doesn't overlap
            (self.end_turn_button.y - margin) - container_height
        } else {
            // Fallback to bottom of panel
            self.y + self.height - container_height - 60.0
        };

        // Determine text box top Y above container
        let mut text_box_y = container_top_y - gap_between_text_and_buttons - text_box_height;

        // Ensure the prompt stays within the panel bounds; if text overflows, try to shrink vertical padding
        let top_limit = self.y + margin;
        if text_box_y < top_limit {
            // Try shrinking vertical padding to make it fit
            text_padding_y = 6.0;
            button_height = btn_text_size + text_padding_y * 2.0;
            text_box_height = item_text_size + text_padding_y * 2.0 + 6.0;
            let container_height_adj = button_height + container_padding * 2.0;
            // Recompute container top so bottom remains above End Turn button
            container_top_y = (self.end_turn_button.y - margin) - container_height_adj;
            text_box_y = container_top_y - gap_between_text_and_buttons - text_box_height;
            // If still overflowing, clamp to top_limit (best effort)
            if text_box_y < top_limit {
                text_box_y = top_limit;
                container_top_y = text_box_y + text_box_height + gap_between_text_and_buttons;
            }
        }

        // Compute container X so container is centered inside prompt area
        let container_x = prompt_x + (prompt_width - container_width) / 2.0;
        // Compute starting X for the buttons centered inside the container inner area
        let buttons_start_x =
            container_x + container_padding + (container_inner_width - total_buttons_width) / 2.0;
        let yes_button_x = buttons_start_x;
        let no_button_x = yes_button_x + pick_button_width + button_spacing;
        // Button Y
        let button_y = container_top_y + container_padding;

        // Compute text box X centered relative to container
        let text_box_x = container_x + (container_width - text_box_width) / 2.0;

        // Render box background and border behind item name
        self.render_box(
            text_box_x,
            text_box_y,
            text_box_width,
            text_box_height,
            [0.12, 0.12, 0.16, 1.0],
        );
        self.render_border(
            text_box_x,
            text_box_y,
            text_box_width,
            text_box_height,
            [0.6, 0.6, 0.7, 1.0],
        );

        // Clamp item name to fit inside text box and center it
        // Estimate character width using same heuristic used elsewhere (0.6 * size)
        let available_text_width = text_box_width - text_padding_x * 2.0;
        let approx_char_w = item_text_size * 0.6;
        let max_chars = (available_text_width / approx_char_w).floor() as usize;

        let display_name = if item_name.chars().count() > max_chars && max_chars > 3 {
            // Reserve 1 char for ellipsis character '…'
            let mut truncated = item_name.chars().take(max_chars - 1).collect::<String>();
            truncated.push('…');
            truncated
        } else {
            item_name.to_string()
        };

        let item_text_x =
            text_box_x + text_box_width / 2.0 - (display_name.len() as f32 * item_text_size * 0.3);
        let item_text_y = text_box_y + (text_box_height / 2.0) - (item_text_size / 2.0);
        self.text_renderer.render_text(
            &display_name,
            item_text_x,
            item_text_y,
            item_text_size,
            [1.0, 1.0, 1.0, 1.0],
            screen_width,
            screen_height,
        );

        // Store text box as a button for click detection
        self.pickup_text_button = Some(Button::new(
            text_box_x,
            text_box_y,
            text_box_width,
            text_box_height,
        ));

        // Draw the computed container and buttons using the earlier computed values
        self.render_box(
            container_x,
            container_top_y,
            container_width,
            container_height,
            [0.14, 0.14, 0.18, 1.0],
        );
        self.render_border(
            container_x,
            container_top_y,
            container_width,
            container_height,
            [0.45, 0.45, 0.5, 1.0],
        );

        // Render Pick Up button box and border
        self.render_box(
            yes_button_x,
            button_y,
            pick_button_width,
            button_height,
            [0.2, 0.6, 0.2, 1.0],
        );
        self.render_border(
            yes_button_x,
            button_y,
            pick_button_width,
            button_height,
            [0.3, 1.0, 0.3, 1.0],
        );

        // Render pick label centered in its button
        let pick_text_x = yes_button_x + pick_button_width / 2.0
            - (pick_label.len() as f32 * btn_text_size * 0.3);
        let pick_text_y = button_y + (button_height / 2.0) - (btn_text_size / 2.0);
        self.text_renderer.render_text(
            pick_label,
            pick_text_x,
            pick_text_y,
            btn_text_size,
            [1.0, 1.0, 1.0, 1.0],
            screen_width,
            screen_height,
        );

        // Render No button box and border
        self.render_box(
            no_button_x,
            button_y,
            no_button_width,
            button_height,
            [0.6, 0.2, 0.2, 1.0],
        );
        self.render_border(
            no_button_x,
            button_y,
            no_button_width,
            button_height,
            [1.0, 0.3, 0.3, 1.0],
        );

        let no_text_x =
            no_button_x + no_button_width / 2.0 - (no_label.len() as f32 * btn_text_size * 0.3);
        let no_text_y = button_y + (button_height / 2.0) - (btn_text_size / 2.0);
        self.text_renderer.render_text(
            no_label,
            no_text_x,
            no_text_y,
            btn_text_size,
            [1.0, 1.0, 1.0, 1.0],
            screen_width,
            screen_height,
        );

        // Store button positions for click detection
        self.pickup_yes_button = Some(Button::new(
            yes_button_x,
            button_y,
            pick_button_width,
            button_height,
        ));
        self.pickup_no_button = Some(Button::new(
            no_button_x,
            button_y,
            no_button_width,
            button_height,
        ));
    }

    /// Renders the End Turn button at the bottom of the UI panel.
    ///
    /// The button is styled with a blue background and lighter blue border,
    /// with centered "END TURN" text.
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context. Rebinds VAO internally.
    unsafe fn render_end_turn_button(&mut self, screen_width: f32, screen_height: f32) {
        // Render the end turn button at the bottom of the panel
        // Ensure the UI shader and VAO are bound because other renderers (text) may have changed GL state
        gl::UseProgram(self.shader_program);
        gl::BindVertexArray(self.vao);

        let btn = &self.end_turn_button;

        // Button background (blue-ish)
        self.render_box(btn.x, btn.y, btn.width, btn.height, [0.2, 0.4, 0.7, 1.0]);

        // Button border (lighter blue)
        self.render_border(btn.x, btn.y, btn.width, btn.height, [0.4, 0.6, 0.9, 1.0]);

        // Render "END TURN" text centered on button
        let text = "END TURN";
        let text_size = 14.0;
        let text_x = btn.x + (btn.width / 2.0) - (text.len() as f32 * text_size * 0.3);
        let text_y = btn.y + (btn.height / 2.0) - (text_size / 2.0);

        self.text_renderer.render_text(
            text,
            text_x,
            text_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            screen_width,
            screen_height,
        );

        // Restore VAO state (unbind) so we don't interfere with external callers
        gl::BindVertexArray(0);
    }

    /// Renders a filled rectangular box.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of top-left corner
    /// * `y` - Y coordinate of top-left corner
    /// * `width` - Width of the box
    /// * `height` - Height of the box
    /// * `color` - RGBA color array [r, g, b, a] with values 0.0-1.0
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context with VAO bound.
    unsafe fn render_box(&self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
        // Render filled box
        let vertices: Vec<f32> = vec![
            x,
            y, // Top-left
            x + width,
            y, // Top-right
            x + width,
            y + height, // Bottom-right
            x,
            y + height, // Bottom-left
        ];

        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        let color_loc = gl::GetUniformLocation(self.shader_program, c"color".as_ptr());
        gl::Uniform4f(color_loc, color[0], color[1], color[2], color[3]);

        gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
    }

    /// Renders a rectangular border outline.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of top-left corner
    /// * `y` - Y coordinate of top-left corner
    /// * `width` - Width of the border
    /// * `height` - Height of the border
    /// * `color` - RGBA color array [r, g, b, a] with values 0.0-1.0
    ///
    /// # Safety
    ///
    /// Must be called within a valid OpenGL context with VAO bound.
    unsafe fn render_border(&self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
        let vertices: Vec<f32> = vec![
            x,
            y, // Top-left
            x + width,
            y, // Top-right
            x + width,
            y + height, // Bottom-right
            x,
            y + height, // Bottom-left
            x,
            y, // Close loop
        ];

        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        let color_loc = gl::GetUniformLocation(self.shader_program, c"color".as_ptr());
        gl::Uniform4f(color_loc, color[0], color[1], color[2], color[3]);

        gl::LineWidth(2.0);
        gl::DrawArrays(gl::LINE_STRIP, 0, 5);
        gl::LineWidth(1.0);
    }
}

impl Drop for UiPanel {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.shader_program);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
