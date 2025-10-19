use crate::core::{HexGrid, Hexagon};
use crate::rendering::{TextureManager, VertexBuffer};
use gl::types::*;
use std::ffi::CString;

/// Guide/Encyclopedia entry for displaying game information
#[derive(Clone, Debug)]
pub struct GuideEntry {
    pub title: String,
    pub description: Vec<String>,     // Multiple lines of text
    pub stats: Vec<(String, String)>, // Key-value pairs for stats
    pub tips: Vec<String>,
}

/// Guide display state
pub struct GuideDisplay {
    pub active: bool,
    pub current_entry: Option<GuideEntry>,
    pub position: (f32, f32), // Screen position (x, y)
    pub size: (f32, f32),     // Width and height
}

impl Default for GuideDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl GuideDisplay {
    pub fn new() -> Self {
        Self {
            active: false,
            current_entry: None,
            position: (100.0, 100.0),
            size: (400.0, 500.0),
        }
    }

    pub fn show(&mut self, entry: GuideEntry) {
        self.current_entry = Some(entry);
        self.active = true;
    }

    pub fn hide(&mut self) {
        self.active = false;
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
}

/// Menu button action types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MenuAction {
    Continue,
    Settings,
    Save,
    Load,
    ExitToMainMenu,
    ExitGame,
}

/// Menu button with position and size
#[derive(Clone, Debug)]
pub struct MenuButton {
    pub label: String,
    pub action: MenuAction,
    pub position: (f32, f32), // Screen position (x, y)
    pub size: (f32, f32),     // Width and height
    pub hovered: bool,
    pub enabled: bool,
}

impl MenuButton {
    pub fn new(label: impl Into<String>, action: MenuAction, position: (f32, f32)) -> Self {
        Self {
            label: label.into(),
            action,
            position,
            size: (200.0, 50.0), // Default button size
            hovered: false,
            enabled: true,
        }
    }

    /// Check if a point (screen coordinates) is inside the button
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        let (bx, by) = self.position;
        let (bw, bh) = self.size;
        x >= bx && x <= bx + bw && y >= by && y <= by + bh
    }
}

/// Menu display state
pub struct MenuDisplay {
    pub active: bool,
    pub buttons: Vec<MenuButton>,
    pub position: (f32, f32), // Menu panel position
    pub size: (f32, f32),     // Menu panel size
}

impl Default for MenuDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuDisplay {
    pub fn new() -> Self {
        // Create default menu buttons centered on screen
        let center_x = (1200.0 - 200.0) / 2.0; // Center horizontally (assuming 1200px screen)
        let start_y = 150.0;
        let button_spacing = 60.0;

        let buttons = vec![
            MenuButton::new("Continue", MenuAction::Continue, (center_x, start_y)),
            MenuButton::new(
                "Settings",
                MenuAction::Settings,
                (center_x, start_y + button_spacing),
            ),
            MenuButton::new(
                "Save",
                MenuAction::Save,
                (center_x, start_y + button_spacing * 2.0),
            ),
            MenuButton::new(
                "Load",
                MenuAction::Load,
                (center_x, start_y + button_spacing * 3.0),
            ),
            MenuButton::new(
                "Exit to Main Menu",
                MenuAction::ExitToMainMenu,
                (center_x, start_y + button_spacing * 4.0),
            ),
            MenuButton::new(
                "Exit Game",
                MenuAction::ExitGame,
                (center_x, start_y + button_spacing * 5.0),
            ),
        ];

        Self {
            active: false,
            buttons,
            position: (300.0, 100.0),
            size: (600.0, 600.0),
        }
    }

    pub fn show(&mut self) {
        self.active = true;
    }

    pub fn hide(&mut self) {
        self.active = false;
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    /// Update hover state for buttons based on mouse position
    pub fn update_hover(&mut self, mouse_x: f32, mouse_y: f32) {
        for button in &mut self.buttons {
            button.hovered = button.contains_point(mouse_x, mouse_y);
        }
    }

    /// Get the action of the button at the given position, if any
    pub fn get_button_action(&self, x: f32, y: f32) -> Option<MenuAction> {
        for button in &self.buttons {
            if button.enabled && button.contains_point(x, y) {
                return Some(button.action.clone());
            }
        }
        None
    }
}

pub struct Renderer {
    pub vao: GLuint,
    pub shader_program: GLuint,
    pub vertex_buffer: VertexBuffer,
    pub texture_manager: TextureManager,
    pub guide_display: GuideDisplay,
    pub menu_display: MenuDisplay,
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
            guide_display: GuideDisplay::new(),
            menu_display: MenuDisplay::new(),
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

                // LAYER 2: Render units (middle layer, z = -0.5)
                self.render_unit_sprites_layer(&visible_hexagons, hex_grid);

                // LAYER 3: Render items (top layer, z = -0.6)
                self.render_item_sprites_layer(&visible_hexagons, hex_grid);
            }

            // LAYER 4: Render guide/encyclopedia (UI overlay, no depth test)
            gl::Disable(gl::DEPTH_TEST);
            self.render_guide_layer();

            // LAYER 5: Render menu (topmost UI layer, no depth test)
            self.render_menu_layer();
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

    unsafe fn render_unit_sprites_layer(&self, visible_hexagons: &[&Hexagon], hex_grid: &HexGrid) {
        // Build vertices for units only
        let mut vertices = Vec::new();

        for hex in visible_hexagons {
            if let Some(unit_sprite) = hex.unit_sprite {
                // Only render units, skip items
                if unit_sprite == crate::core::hexagon::SpriteType::Unit {
                    let center_x = hex.world_pos.x - hex_grid.camera.position.x;
                    let center_y = hex.world_pos.y - hex_grid.camera.position.y;
                    let texture_id = unit_sprite.get_texture_id();
                    let sprite_color = unit_sprite.get_color_tint();

                    // Scale factor for units (60% of hex size)
                    let scale_factor = 0.6;
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
        }

        if vertices.is_empty() {
            return;
        }

        // Upload and draw unit sprites
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

    unsafe fn render_item_sprites_layer(&self, visible_hexagons: &[&Hexagon], hex_grid: &HexGrid) {
        // Build vertices for items only
        let mut vertices = Vec::new();

        for hex in visible_hexagons {
            // Check if there's an item on this hex using the dedicated item_sprite field
            if let Some(item_sprite) = hex.item_sprite {
                if item_sprite == crate::core::hexagon::SpriteType::Item {
                    // Check if there's also a unit on this hex
                    let has_unit = hex.unit_sprite.is_some()
                        && matches!(
                            hex.unit_sprite,
                            Some(crate::core::hexagon::SpriteType::Unit)
                        );

                    let center_x = hex.world_pos.x - hex_grid.camera.position.x;
                    let center_y = hex.world_pos.y - hex_grid.camera.position.y;
                    let texture_id = item_sprite.get_texture_id();
                    let sprite_color = item_sprite.get_color_tint();

                    // Determine size and position based on whether unit is present
                    let (scale_factor, offset_x, offset_y) = if has_unit {
                        // Smaller size and shifted to upper-right corner
                        let scale = 0.25; // 25% of hex size (smaller when unit present)
                        let offset_distance = hex_grid.hex_size * 0.4; // Distance from center
                        let angle = std::f32::consts::PI / 6.0; // 30 degrees (upper-right)
                        let x_offset = offset_distance * angle.cos();
                        let y_offset = -offset_distance * angle.sin(); // Negative for "up"
                        (scale, x_offset, y_offset)
                    } else {
                        // Normal size, centered
                        (0.5, 0.0, 0.0)
                    };

                    let small_hex_size = hex_grid.hex_size * scale_factor;
                    let item_x = center_x + offset_x;
                    let item_y = center_y + offset_y;

                    // Center vertex (z = -0.6 to render on top of units)
                    vertices.extend_from_slice(&[
                        item_x,
                        item_y,
                        -0.6, // Slightly closer than units (-0.5) so it renders on top
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
                        let x = item_x + small_hex_size * angle.cos();
                        let y = item_y + small_hex_size * angle.sin();
                        let u = 0.5 + 0.4 * angle.cos();
                        let v = 1.0 - (0.5 + 0.4 * angle.sin());

                        vertices.extend_from_slice(&[
                            x,
                            y,
                            -0.6, // Same depth as center
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
        }

        if vertices.is_empty() {
            return;
        }

        // Upload and draw item sprites
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

    /// Render the guide/encyclopedia UI overlay
    unsafe fn render_guide_layer(&self) {
        // Only render if guide is active
        if !self.guide_display.active {
            return;
        }

        // Get guide entry if present
        let _entry = match &self.guide_display.current_entry {
            Some(entry) => entry,
            None => return,
        };

        let (x, y) = self.guide_display.position;
        let (width, height) = self.guide_display.size;

        // Build vertices for the guide panel background
        let mut vertices = Vec::new();

        // Background panel (semi-transparent black rectangle)
        let bg_color = [0.1, 0.1, 0.15, 0.9]; // Dark blue-gray with alpha
        let depth = -0.9; // Very close to camera (on top of everything)

        // Create a rectangle using two triangles
        // Triangle 1: Top-left, Top-right, Bottom-left
        // Triangle 2: Top-right, Bottom-right, Bottom-left

        // Convert screen coordinates to normalized device coordinates (-1 to 1)
        // Assuming screen is 1200x800 (we'll need to pass this in eventually)
        let screen_width = 1200.0;
        let screen_height = 800.0;

        let x1 = (x / screen_width) * 2.0 - 1.0;
        let y1 = 1.0 - (y / screen_height) * 2.0;
        let x2 = ((x + width) / screen_width) * 2.0 - 1.0;
        let y2 = 1.0 - ((y + height) / screen_height) * 2.0;

        // Background rectangle vertices (2 triangles = 6 vertices)
        let bg_vertices = [
            // Triangle 1
            x1,
            y1,
            depth,
            0.0,
            0.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2], // Top-left
            x2,
            y1,
            depth,
            1.0,
            0.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2], // Top-right
            x1,
            y2,
            depth,
            0.0,
            1.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2], // Bottom-left
            // Triangle 2
            x2,
            y1,
            depth,
            1.0,
            0.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2], // Top-right
            x2,
            y2,
            depth,
            1.0,
            1.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2], // Bottom-right
            x1,
            y2,
            depth,
            0.0,
            1.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2], // Bottom-left
        ];

        vertices.extend_from_slice(&bg_vertices);

        // Upload and draw background
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        // TODO: Render text content
        // For now, we just draw the background panel
        // Text rendering will be added using a text rendering system

        // Future implementation will include:
        // - Title text at top
        // - Description lines
        // - Stats in columns
        // - Tips at bottom
        // - Borders and decorative elements
    }

    /// Render the menu UI overlay
    unsafe fn render_menu_layer(&self) {
        // Only render if menu is active
        if !self.menu_display.active {
            return;
        }

        let (panel_x, panel_y) = self.menu_display.position;
        let (panel_width, panel_height) = self.menu_display.size;

        // Screen dimensions (should match window size)
        let screen_width = 1200.0;
        let screen_height = 800.0;

        let mut vertices = Vec::new();

        // ========================================
        // 1. Render semi-transparent background overlay (full screen)
        // ========================================
        let overlay_color = [0.0, 0.0, 0.0, 0.7]; // Black with 70% opacity
        let depth = -0.95; // Very close to camera

        // Full-screen overlay (2 triangles)
        let overlay_vertices = [
            // Triangle 1
            -1.0,
            1.0,
            depth,
            0.0,
            0.0,
            -1.0,
            overlay_color[0],
            overlay_color[1],
            overlay_color[2],
            1.0,
            1.0,
            depth,
            1.0,
            0.0,
            -1.0,
            overlay_color[0],
            overlay_color[1],
            overlay_color[2],
            -1.0,
            -1.0,
            depth,
            0.0,
            1.0,
            -1.0,
            overlay_color[0],
            overlay_color[1],
            overlay_color[2],
            // Triangle 2
            1.0,
            1.0,
            depth,
            1.0,
            0.0,
            -1.0,
            overlay_color[0],
            overlay_color[1],
            overlay_color[2],
            1.0,
            -1.0,
            depth,
            1.0,
            1.0,
            -1.0,
            overlay_color[0],
            overlay_color[1],
            overlay_color[2],
            -1.0,
            -1.0,
            depth,
            0.0,
            1.0,
            -1.0,
            overlay_color[0],
            overlay_color[1],
            overlay_color[2],
        ];

        vertices.extend_from_slice(&overlay_vertices);

        // ========================================
        // 2. Render menu background panel
        // ========================================
        let bg_color = [0.15, 0.15, 0.2, 0.95]; // Dark blue-gray with high opacity
        let panel_depth = -0.96; // Slightly closer than overlay

        // Convert screen coordinates to NDC
        let x1 = (panel_x / screen_width) * 2.0 - 1.0;
        let y1 = 1.0 - (panel_y / screen_height) * 2.0;
        let x2 = ((panel_x + panel_width) / screen_width) * 2.0 - 1.0;
        let y2 = 1.0 - ((panel_y + panel_height) / screen_height) * 2.0;

        let panel_vertices = [
            // Triangle 1
            x1,
            y1,
            panel_depth,
            0.0,
            0.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            x2,
            y1,
            panel_depth,
            1.0,
            0.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            x1,
            y2,
            panel_depth,
            0.0,
            1.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            // Triangle 2
            x2,
            y1,
            panel_depth,
            1.0,
            0.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            x2,
            y2,
            panel_depth,
            1.0,
            1.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            x1,
            y2,
            panel_depth,
            0.0,
            1.0,
            -1.0,
            bg_color[0],
            bg_color[1],
            bg_color[2],
        ];

        vertices.extend_from_slice(&panel_vertices);

        // ========================================
        // 3. Render menu buttons
        // ========================================
        let button_depth = -0.97; // Even closer than panel

        for button in &self.menu_display.buttons {
            let (bx, by) = button.position;
            let (bw, bh) = button.size;

            // Button color (different for hovered/normal)
            let button_color = if button.hovered {
                [0.3, 0.4, 0.6, 1.0] // Lighter blue when hovered
            } else {
                [0.2, 0.25, 0.35, 1.0] // Dark blue-gray
            };

            // Convert button coordinates to NDC
            let bx1 = (bx / screen_width) * 2.0 - 1.0;
            let by1 = 1.0 - (by / screen_height) * 2.0;
            let bx2 = ((bx + bw) / screen_width) * 2.0 - 1.0;
            let by2 = 1.0 - ((by + bh) / screen_height) * 2.0;

            let button_vertices = [
                // Triangle 1
                bx1,
                by1,
                button_depth,
                0.0,
                0.0,
                -1.0,
                button_color[0],
                button_color[1],
                button_color[2],
                bx2,
                by1,
                button_depth,
                1.0,
                0.0,
                -1.0,
                button_color[0],
                button_color[1],
                button_color[2],
                bx1,
                by2,
                button_depth,
                0.0,
                1.0,
                -1.0,
                button_color[0],
                button_color[1],
                button_color[2],
                // Triangle 2
                bx2,
                by1,
                button_depth,
                1.0,
                0.0,
                -1.0,
                button_color[0],
                button_color[1],
                button_color[2],
                bx2,
                by2,
                button_depth,
                1.0,
                1.0,
                -1.0,
                button_color[0],
                button_color[1],
                button_color[2],
                bx1,
                by2,
                button_depth,
                0.0,
                1.0,
                -1.0,
                button_color[0],
                button_color[1],
                button_color[2],
            ];

            vertices.extend_from_slice(&button_vertices);
        }

        // Upload and draw all vertices
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        // Draw overlay (6 vertices)
        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        // Draw panel (6 vertices)
        gl::DrawArrays(gl::TRIANGLES, 6, 6);

        // Draw buttons (6 vertices each)
        let button_count = self.menu_display.buttons.len();
        for i in 0..button_count {
            let vertex_offset = (12 + i * 6) as GLint;
            gl::DrawArrays(gl::TRIANGLES, vertex_offset, 6);
        }

        // TODO: Render text labels on buttons
        // For now, buttons are drawn as colored rectangles
        // Text rendering will show button labels like "Continue", "Settings", etc.
    }
}
