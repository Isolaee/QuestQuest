use super::text_renderer::TextRenderer;
use gl::types::*;
use std::ffi::CString;

const UI_PANEL_WIDTH: f32 = 350.0;

#[derive(Debug, Clone, Copy)]
pub struct Button {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}

#[derive(Debug, Clone)]
pub struct UnitDisplayInfo {
    pub name: String,
    pub race: String,
    pub class: String,
    pub level: i32,
    pub experience: i32,
    pub health: u32,
    pub max_health: u32,
    pub terrain: String,
    pub position_q: i32,
    pub position_r: i32,
}

pub struct UiPanel {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    vao: GLuint,
    vbo: GLuint,
    shader_program: GLuint,
    unit_info: Option<UnitDisplayInfo>,
    text_renderer: TextRenderer,
    pickup_prompt: Option<String>,      // Item name for pickup prompt
    pickup_yes_button: Option<Button>,  // Pick up button
    pickup_no_button: Option<Button>,   // Leave it button
    pickup_text_button: Option<Button>, // Box around the item name
    pub end_turn_button: Button,        // End turn button
}

impl UiPanel {
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

    /// Check if a click position hits the "Yes" button
    pub fn check_yes_button_click(&self, x: f32, y: f32) -> bool {
        if let Some(button) = &self.pickup_yes_button {
            button.contains(x, y)
        } else {
            false
        }
    }

    /// Check if a click position hits the "No" button
    pub fn check_no_button_click(&self, x: f32, y: f32) -> bool {
        if let Some(button) = &self.pickup_no_button {
            button.contains(x, y)
        } else {
            false
        }
    }

    /// Check if a click position hits the "End Turn" button
    pub fn check_end_turn_button_click(&self, x: f32, y: f32) -> bool {
        self.end_turn_button.contains(x, y)
    }

    pub fn render(&mut self, screen_width: f32, screen_height: f32) {
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
            self.render_unit_sprite_placeholder(screen_width, screen_height);

            // Render unit info section
            self.render_unit_info_section(screen_width, screen_height);

            // Render equipment slots placeholders
            self.render_equipment_slots(screen_width, screen_height);

            // Render pickup prompt if active (clone to avoid borrow issues)
            if let Some(item_name) = self.pickup_prompt.clone() {
                self.render_pickup_prompt(screen_width, screen_height, &item_name);
            }

            gl::BindVertexArray(0);

            // Render text on top of everything else
            self.render_unit_text(screen_width, screen_height);

            // Render end turn button last so it remains visible above any prompts
            self.render_end_turn_button(screen_width, screen_height);
        }
    }

    unsafe fn render_unit_info_section(&self, _screen_width: f32, _screen_height: f32) {
        // Render the info section below the sprites
        let section_y = self.y + 100.0;
        let margin = 10.0;
        let section_height = 70.0;

        // Background for info section
        self.render_box(
            self.x + margin,
            section_y,
            self.width - margin * 2.0,
            section_height,
            [0.15, 0.15, 0.2, 0.95],
        );

        // If we have unit info, display it (text rendering would go here)
        if let Some(info) = &self.unit_info {
            // TODO: Render actual text when text rendering is implemented
            // For now, this creates visual placeholders

            // Health bar
            let health_bar_y = section_y + 10.0;
            let health_bar_width = self.width - margin * 4.0;
            let health_percentage = info.health as f32 / info.max_health as f32;

            // Background bar
            self.render_box(
                self.x + margin * 2.0,
                health_bar_y,
                health_bar_width,
                20.0,
                [0.2, 0.2, 0.2, 1.0],
            );

            // Health fill
            let health_color = if health_percentage > 0.6 {
                [0.2, 0.8, 0.2, 1.0] // Green
            } else if health_percentage > 0.3 {
                [0.8, 0.8, 0.2, 1.0] // Yellow
            } else {
                [0.8, 0.2, 0.2, 1.0] // Red
            };

            self.render_box(
                self.x + margin * 2.0,
                health_bar_y,
                health_bar_width * health_percentage,
                20.0,
                health_color,
            );

            // Experience bar (smaller)
            let exp_bar_y = section_y + 40.0;
            let exp_percentage = (info.experience % 1000) as f32 / 1000.0; // Assume 1000 exp per level

            self.render_box(
                self.x + margin * 2.0,
                exp_bar_y,
                health_bar_width,
                12.0,
                [0.2, 0.2, 0.3, 1.0],
            );

            self.render_box(
                self.x + margin * 2.0,
                exp_bar_y,
                health_bar_width * exp_percentage,
                12.0,
                [0.3, 0.5, 0.8, 1.0],
            );
        }
    }

    unsafe fn render_unit_text(&mut self, screen_width: f32, screen_height: f32) {
        if let Some(info) = &self.unit_info {
            let margin = 15.0;
            let text_y_start = self.y + 180.0; // Position below the info section
            let text_x = self.x + margin;
            let text_size = 10.0;
            let line_height = 18.0;
            let white = [1.0, 1.0, 1.0, 1.0];

            // Render unit name
            self.text_renderer.render_text(
                &info.name,
                text_x,
                text_y_start,
                text_size + 2.0,
                white,
                screen_width,
                screen_height,
            );

            // Render race and class
            let race_class = format!("{} {}", info.race, info.class);
            self.text_renderer.render_text(
                &race_class,
                text_x,
                text_y_start + line_height,
                text_size - 1.0,
                [0.8, 0.8, 0.8, 1.0],
                screen_width,
                screen_height,
            );

            // Render level
            let level_text = format!("Lvl {}", info.level);
            self.text_renderer.render_text(
                &level_text,
                text_x,
                text_y_start + line_height * 2.5,
                text_size,
                [0.9, 0.9, 0.6, 1.0],
                screen_width,
                screen_height,
            );

            // Render health
            let health_text = format!("HP {}/{}", info.health, info.max_health);
            self.text_renderer.render_text(
                &health_text,
                text_x,
                text_y_start + line_height * 3.5,
                text_size,
                [0.6, 0.9, 0.6, 1.0],
                screen_width,
                screen_height,
            );

            // Render experience
            let exp_text = format!("EXP {}", info.experience);
            self.text_renderer.render_text(
                &exp_text,
                text_x,
                text_y_start + line_height * 4.5,
                text_size,
                [0.6, 0.7, 0.9, 1.0],
                screen_width,
                screen_height,
            );

            // Render terrain (shortened)
            let terrain_short = if info.terrain.len() > 10 {
                &info.terrain[..10]
            } else {
                &info.terrain
            };
            self.text_renderer.render_text(
                terrain_short,
                text_x,
                text_y_start + line_height * 5.5,
                text_size - 1.0,
                [0.7, 0.9, 0.7, 1.0],
                screen_width,
                screen_height,
            );

            // Render position coordinates (consolidated)
            let pos_text_y = text_y_start + line_height * 7.0;
            let pos_text = format!(
                "Pos: Q{} R{} S{}",
                info.position_q,
                info.position_r,
                -info.position_q - info.position_r
            );
            self.text_renderer.render_text(
                &pos_text,
                text_x,
                pos_text_y,
                text_size - 1.0,
                [0.7, 0.8, 0.9, 1.0],
                screen_width,
                screen_height,
            );
        }
    }
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

    unsafe fn render_clock_placeholder(&self, _screen_width: f32, _screen_height: f32) {
        let box_size = 80.0;
        let margin = 10.0;
        let x = self.x + margin;
        let y = self.y + margin;

        self.render_box(x, y, box_size, box_size, [0.3, 0.3, 0.4, 1.0]);
    }

    unsafe fn render_terrain_sprite_placeholder(&self, _screen_width: f32, _screen_height: f32) {
        let box_size = 80.0;
        let margin = 10.0;
        let x = self.x + self.width / 2.0 - box_size / 2.0;
        let y = self.y + margin;

        self.render_box(x, y, box_size, box_size, [0.2, 0.5, 0.2, 1.0]);
    }

    unsafe fn render_unit_sprite_placeholder(&self, _screen_width: f32, _screen_height: f32) {
        let box_size = 80.0;
        let margin = 10.0;
        let x = self.x + self.width - box_size - margin;
        let y = self.y + margin;

        self.render_box(x, y, box_size, box_size, [0.5, 0.3, 0.3, 1.0]);
    }

    unsafe fn render_equipment_slots(&self, _screen_width: f32, _screen_height: f32) {
        let slot_size = 50.0;
        let margin = 10.0;
        let spacing = 5.0;
        let start_y = self.y + 330.0; // Below text section

        // Equipment slot layout
        let slots = vec![
            (
                "Helmet",
                self.x + self.width / 2.0 - slot_size / 2.0,
                start_y,
            ),
            (
                "Necklace",
                self.x + self.width / 2.0 - slot_size / 2.0,
                start_y + slot_size + spacing,
            ),
            (
                "Breastplate",
                self.x + self.width / 2.0 - slot_size / 2.0,
                start_y + (slot_size + spacing) * 2.0,
            ),
            (
                "Right Hand",
                self.x + margin,
                start_y + (slot_size + spacing) * 3.0,
            ),
            (
                "Left Hand",
                self.x + self.width - slot_size - margin,
                start_y + (slot_size + spacing) * 3.0,
            ),
            (
                "Ring 1",
                self.x + margin,
                start_y + (slot_size + spacing) * 4.0,
            ),
            (
                "Ring 2",
                self.x + self.width / 2.0 - slot_size / 2.0,
                start_y + (slot_size + spacing) * 4.0,
            ),
            (
                "Ring 3",
                self.x + self.width - slot_size - margin,
                start_y + (slot_size + spacing) * 4.0,
            ),
            (
                "Cape",
                self.x + self.width / 2.0 - slot_size / 2.0,
                start_y + (slot_size + spacing) * 5.0,
            ),
            (
                "Boots",
                self.x + self.width / 2.0 - slot_size / 2.0,
                start_y + (slot_size + spacing) * 6.0,
            ),
        ];

        for (i, (_name, x, y)) in slots.iter().enumerate() {
            // Different colors for different slot types
            let color = match i {
                0 => [0.6, 0.5, 0.2, 1.0],     // Helmet - gold
                1 => [0.4, 0.6, 0.6, 1.0],     // Necklace - cyan
                2 => [0.5, 0.5, 0.5, 1.0],     // Breastplate - gray
                3 | 4 => [0.6, 0.3, 0.2, 1.0], // Hands - brown
                5..=7 => [0.8, 0.7, 0.2, 1.0], // Rings - bright gold
                8 => [0.4, 0.2, 0.6, 1.0],     // Cape - purple
                9 => [0.3, 0.3, 0.3, 1.0],     // Boots - dark gray
                _ => [0.3, 0.3, 0.3, 1.0],
            };

            self.render_box(*x, *y, slot_size, slot_size, color);
        }
    }

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

        // Render border
        self.render_border(x, y, width, height, [0.7, 0.7, 0.7, 1.0]);
    }

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
