use super::text_renderer::TextRenderer;
use gl::types::*;
use std::ffi::CString;

const UI_PANEL_WIDTH: f32 = 350.0;

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
}

impl UiPanel {
    pub fn new(screen_width: f32, screen_height: f32) -> Result<Self, String> {
        let width = UI_PANEL_WIDTH;
        let height = screen_height;
        let x = screen_width - width;
        let y = 0.0;

        let (vao, vbo, shader_program) = unsafe { Self::setup_ui_rendering()? };
        let text_renderer = TextRenderer::new()?;

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

            gl::BindVertexArray(0);
        }

        // Render text on top of everything else
        unsafe {
            self.render_unit_text(screen_width, screen_height);
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
