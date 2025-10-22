use crate::core::{HexGrid, Hexagon};
use crate::rendering::{TextureManager, VertexBuffer};
use crate::ui::TextRenderer;
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
        // Create default menu buttons - will be repositioned when window size is known
        // Initial positions are placeholders
        let center_x = 500.0;
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

    /// Update menu positions based on actual window dimensions
    pub fn update_for_screen_size(&mut self, screen_width: f32, screen_height: f32) {
        // Center the menu panel on screen
        let panel_width = 600.0;
        let panel_height = 600.0;
        self.position = (
            (screen_width - panel_width) / 2.0,
            (screen_height - panel_height) / 2.0,
        );
        self.size = (panel_width, panel_height);

        // Position buttons within the panel
        let button_width = 300.0;
        let button_height = 50.0;
        let button_start_x = self.position.0 + (panel_width - button_width) / 2.0;
        let button_start_y = self.position.1 + 100.0;
        let button_spacing = 60.0;

        for (i, button) in self.buttons.iter_mut().enumerate() {
            button.position = (button_start_x, button_start_y + i as f32 * button_spacing);
            button.size = (button_width, button_height);
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

/// Effects display for visual effects (particles, animations, etc.)
#[derive(Default)]
pub struct EffectsDisplay {
    pub active: bool,
    // Future: Add effect particles, animations, etc.
}

impl EffectsDisplay {
    pub fn new() -> Self {
        Self { active: false }
    }

    pub fn show(&mut self) {
        self.active = true;
    }

    pub fn hide(&mut self) {
        self.active = false;
    }
}

/// Combat log entry
#[derive(Clone, Debug)]
pub struct CombatLogEntry {
    pub message: String,
    pub timestamp: f32, // Game time when event occurred
    pub entry_type: CombatLogEntryType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CombatLogEntryType {
    Attack,
    Damage,
    Heal,
    Miss,
    Death,
    Info,
}

/// Attack option for combat dialog
#[derive(Clone, Debug)]
pub struct AttackOption {
    pub name: String,
    pub damage: u32,
    pub range: i32,
}

/// Combat confirmation dialog data
#[derive(Clone, Debug)]
pub struct CombatConfirmation {
    pub attacker_name: String,
    pub attacker_hp: u32,
    pub attacker_max_hp: u32,
    pub attacker_attack: u32,
    pub attacker_defense: u32,
    pub attacker_attacks_per_round: u32,
    pub attacker_attacks: Vec<AttackOption>,
    pub defender_name: String,
    pub defender_hp: u32,
    pub defender_max_hp: u32,
    pub defender_attack: u32,
    pub defender_defense: u32,
    pub defender_attacks_per_round: u32,
    pub defender_attacks: Vec<AttackOption>,
}

/// Combat confirmation button
#[derive(Clone, Debug)]
pub struct CombatButton {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub label: String,
    pub hovered: bool,
}

impl CombatButton {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self {
        Self {
            position: (x, y),
            size: (width, height),
            label: label.to_string(),
            hovered: false,
        }
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.position.0
            && x <= self.position.0 + self.size.0
            && y >= self.position.1
            && y <= self.position.1 + self.size.1
    }
}

/// Combat log display
pub struct CombatLogDisplay {
    pub active: bool,
    pub entries: Vec<CombatLogEntry>,
    pub max_entries: usize,
    pub position: (f32, f32), // Screen position (x, y)
    pub size: (f32, f32),     // Width and height
    pub auto_hide: bool,      // Auto-hide when no combat
    pub pending_combat: Option<CombatConfirmation>,
    pub ok_button: CombatButton,
    pub cancel_button: CombatButton,
    pub selected_attack_index: Option<usize>, // Which attacker attack is selected
    pub attacker_attack_buttons: Vec<CombatButton>, // Clickable attack options for attacker
}

impl Default for CombatLogDisplay {
    fn default() -> Self {
        // Use common default resolution (can be updated later if needed)
        Self::new(1920.0, 1080.0)
    }
}

impl CombatLogDisplay {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        // Dialog is 70% of the window height, making it larger
        let dialog_size = window_height * 0.7; // 560x560 for 800px height
        let dialog_width = dialog_size;
        let dialog_height = dialog_size;

        // Center the dialog on screen
        let dialog_x = (window_width - dialog_width) / 2.0;
        let dialog_y = (window_height - dialog_height) / 2.0;

        // Create buttons at bottom of dialog
        let button_width = 120.0;
        let button_height = 50.0;
        let button_spacing = 20.0;
        let button_margin_bottom = 30.0;

        // Position buttons centered at bottom
        let total_button_width = button_width * 2.0 + button_spacing;
        let buttons_start_x = dialog_x + (dialog_width - total_button_width) / 2.0;
        let buttons_y = dialog_y + dialog_height - button_height - button_margin_bottom;

        let ok_x = buttons_start_x;
        let cancel_x = buttons_start_x + button_width + button_spacing;

        Self {
            active: false,
            entries: Vec::new(),
            max_entries: 10,
            position: (dialog_x, dialog_y),
            size: (dialog_width, dialog_height),
            auto_hide: false,
            pending_combat: None,
            ok_button: CombatButton::new(ok_x, buttons_y, button_width, button_height, "OK"),
            cancel_button: CombatButton::new(
                cancel_x,
                buttons_y,
                button_width,
                button_height,
                "Cancel",
            ),
            selected_attack_index: Some(0), // Default to first attack
            attacker_attack_buttons: Vec::new(), // Will be populated when combat is shown
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

    pub fn show_combat_confirmation(&mut self, confirmation: CombatConfirmation) {
        // Calculate attack button positions
        let attack_box_height = 30.0;
        let attack_box_spacing = 5.0;
        let title_height = 50.0;
        let sprite_area_height = 120.0;
        let text_line_height = 25.0;
        let panel_padding = 15.0;
        let panel_height = text_line_height * 6.0 + panel_padding * 2.0;
        let panel_margin = 30.0;
        let panel_spacing = 20.0;
        let panel_width = (self.size.0 - 2.0 * panel_margin - panel_spacing) / 2.0;

        let panel_y = self.position.1 + title_height + sprite_area_height + 10.0;
        let attack_section_y = panel_y + panel_height + 10.0;
        let attacker_x = self.position.0 + panel_margin;

        // Create clickable buttons for attacker's attacks
        let mut attack_buttons = Vec::new();
        println!(
            "🎮 Creating {} attack buttons:",
            confirmation.attacker_attacks.len()
        );
        for i in 0..confirmation.attacker_attacks.len() {
            let attack_y = attack_section_y + (i as f32) * (attack_box_height + attack_box_spacing);
            let button = CombatButton::new(
                attacker_x,
                attack_y,
                panel_width,
                attack_box_height,
                &confirmation.attacker_attacks[i].name,
            );
            println!(
                "  Button {}: {} at ({}, {}) size {}x{}",
                i, button.label, button.position.0, button.position.1, button.size.0, button.size.1
            );
            attack_buttons.push(button);
        }

        self.attacker_attack_buttons = attack_buttons;
        self.selected_attack_index = Some(0); // Default to first attack
        self.pending_combat = Some(confirmation);
        self.active = true;
    }

    pub fn clear_combat_confirmation(&mut self) {
        self.pending_combat = None;
        self.active = false;
        self.attacker_attack_buttons.clear();
        self.selected_attack_index = None;
    }

    /// Check if an attack button was clicked and return its index
    pub fn check_attack_click(&mut self, x: f32, y: f32) -> Option<usize> {
        for (i, button) in self.attacker_attack_buttons.iter().enumerate() {
            if button.contains_point(x, y) {
                self.selected_attack_index = Some(i);
                println!("🎯 Attack option {} clicked: {}", i, button.label);
                return Some(i);
            }
        }
        None
    }

    /// Get the currently selected attack index
    pub fn get_selected_attack(&self) -> Option<usize> {
        self.selected_attack_index
    }

    pub fn has_pending_combat(&self) -> bool {
        self.pending_combat.is_some()
    }

    pub fn update_button_hover(&mut self, mouse_x: f32, mouse_y: f32) {
        self.ok_button.hovered = self.ok_button.contains_point(mouse_x, mouse_y);
        self.cancel_button.hovered = self.cancel_button.contains_point(mouse_x, mouse_y);
    }

    pub fn handle_click(&mut self, mouse_x: f32, mouse_y: f32) -> Option<bool> {
        if !self.has_pending_combat() {
            return None;
        }

        if self.ok_button.contains_point(mouse_x, mouse_y) {
            return Some(true); // Combat confirmed
        }

        if self.cancel_button.contains_point(mouse_x, mouse_y) {
            self.clear_combat_confirmation();
            return Some(false); // Combat cancelled
        }

        None
    }

    pub fn add_entry(&mut self, message: String, entry_type: CombatLogEntryType) {
        let entry = CombatLogEntry {
            message,
            timestamp: 0.0, // TODO: Add game time tracking
            entry_type,
        };

        self.entries.push(entry);

        // Keep only the most recent entries
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }

        // Auto-show on new entry
        if !self.active {
            self.active = true;
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

pub struct Renderer {
    pub vao: GLuint,
    pub shader_program: GLuint,
    pub vertex_buffer: VertexBuffer,
    pub texture_manager: TextureManager,
    pub guide_display: GuideDisplay,
    pub menu_display: MenuDisplay,
    pub effects_display: EffectsDisplay,
    pub combat_log_display: CombatLogDisplay,
    pub text_renderer: TextRenderer,
    pub window_width: f32,
    pub window_height: f32,
}

impl Renderer {
    pub fn new(
        vao: GLuint,
        shader_program: GLuint,
        vbo: GLuint,
        window_width: f32,
        window_height: f32,
    ) -> Result<Self, String> {
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

        let text_renderer = TextRenderer::new()?;

        let mut renderer = Self {
            vao,
            shader_program,
            vertex_buffer: VertexBuffer::new(vbo),
            texture_manager,
            guide_display: GuideDisplay::new(),
            menu_display: MenuDisplay::new(),
            effects_display: EffectsDisplay::new(),
            combat_log_display: CombatLogDisplay::new(window_width, window_height),
            text_renderer,
            window_width,
            window_height,
        };

        // Initialize menu positions based on actual window size
        renderer
            .menu_display
            .update_for_screen_size(window_width, window_height);

        Ok(renderer)
    }

    pub fn render(&mut self, hex_grid: &HexGrid) {
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

            // LAYER 6: Render effects (visual effects layer, no depth test)
            self.render_effects_layer();

            // LAYER 7: Render combat log (combat information layer, no depth test)
            self.render_combat_log_layer();
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
    unsafe fn render_menu_layer(&mut self) {
        // Only render if menu is active
        if !self.menu_display.active {
            return;
        }

        let (panel_x, panel_y) = self.menu_display.position;
        let (panel_width, panel_height) = self.menu_display.size;

        // Use actual screen dimensions
        let screen_width = self.window_width;
        let screen_height = self.window_height;

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

        // Render text labels
        self.render_menu_text();
    }

    unsafe fn render_menu_text(&mut self) {
        let window_width = self.window_width;
        let window_height = self.window_height;

        // Render title
        let title = "GAME MENU";
        let title_size = 35.0;
        let panel_width = self.menu_display.size.0;
        let title_x = self.menu_display.position.0
            + (panel_width - title.len() as f32 * title_size * 0.6) / 2.0;
        let title_y = self.menu_display.position.1 + 30.0;

        self.text_renderer.render_text(
            title,
            title_x,
            title_y,
            title_size,
            [1.0, 0.9, 0.4, 1.0], // Gold color
            window_width,
            window_height,
        );

        // Render button labels
        for button in &self.menu_display.buttons {
            let button_label = match button.action {
                MenuAction::Continue => "Continue",
                MenuAction::Settings => "Settings",
                MenuAction::Save => "Save Game",
                MenuAction::Load => "Load Game",
                MenuAction::ExitToMainMenu => "Main Menu",
                MenuAction::ExitGame => "Exit Game",
            };

            let text_size = 22.0;
            let char_width = text_size * 0.6;
            let text_width = button_label.len() as f32 * char_width;

            // Center text in button
            let text_x = button.position.0 + (button.size.0 - text_width) / 2.0;
            let text_y = button.position.1 + (button.size.1 - text_size) / 2.0 + 5.0;

            // Color based on hover state
            let text_color = if button.hovered {
                [1.0, 1.0, 0.8, 1.0] // Light yellow when hovered
            } else {
                [1.0, 1.0, 1.0, 1.0] // White normally
            };

            self.text_renderer.render_text(
                button_label,
                text_x,
                text_y,
                text_size,
                text_color,
                window_width,
                window_height,
            );
        }
    }

    unsafe fn render_effects_layer(&self) {
        // Layer 6: Visual effects (particles, animations, spell effects, etc.)
        if self.effects_display.active {
            // TODO: Implement visual effects rendering
            // - Particle systems for explosions, magic spells
            // - Animation overlays for unit actions
            // - Environmental effects (rain, snow, fire)
            // For now, this layer is transparent/empty
        }
    }

    unsafe fn render_combat_log_layer(&mut self) {
        // Layer 7: Combat log display
        if self.combat_log_display.active {
            // If there's a pending combat confirmation, show the dialog
            if let Some(ref confirmation) = self.combat_log_display.pending_combat {
                self.render_combat_confirmation_dialog(confirmation);
                // Clone confirmation to avoid borrow checker issues
                let confirmation_clone = confirmation.clone();
                self.render_combat_dialog_text(&confirmation_clone);
            } else {
                // Otherwise, show the combat log entries
                self.render_combat_log_entries();
            }
        }
    }

    unsafe fn render_combat_confirmation_dialog(&self, confirmation: &CombatConfirmation) {
        let mut vertices: Vec<f32> = Vec::new();
        let (dialog_x, dialog_y) = self.combat_log_display.position;
        let (dialog_width, dialog_height) = self.combat_log_display.size;

        // Convert screen coordinates to normalized device coordinates (-1 to 1)
        let window_width = self.window_width;
        let window_height = self.window_height;

        let to_ndc_x = |x: f32| (x / window_width) * 2.0 - 1.0;
        let to_ndc_y = |y: f32| 1.0 - (y / window_height) * 2.0;

        let x1 = to_ndc_x(dialog_x);
        let y1 = to_ndc_y(dialog_y);
        let x2 = to_ndc_x(dialog_x + dialog_width);
        let y2 = to_ndc_y(dialog_y + dialog_height);

        // Background panel (light brown, fully opaque)
        let bg_color = [0.7, 0.5, 0.3, 1.0]; // Light brown RGB with full opacity
        let depth = -0.98; // Very close to camera, above all other UI
        let tex_id = -2.0; // Use -2.0 to avoid circle rendering (which uses -1.0)

        // Background quad (2 triangles)
        vertices.extend_from_slice(&[
            // Triangle 1
            x1,
            y1,
            depth,
            0.0,
            0.0,
            tex_id,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            x2,
            y1,
            depth,
            1.0,
            0.0,
            tex_id,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            x1,
            y2,
            depth,
            0.0,
            1.0,
            tex_id,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            // Triangle 2
            x2,
            y1,
            depth,
            1.0,
            0.0,
            tex_id,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            x2,
            y2,
            depth,
            1.0,
            1.0,
            tex_id,
            bg_color[0],
            bg_color[1],
            bg_color[2],
            x1,
            y2,
            depth,
            0.0,
            1.0,
            tex_id,
            bg_color[0],
            bg_color[1],
            bg_color[2],
        ]);

        // Add border
        let border_width = 3.0;
        let border_color = [0.8, 0.6, 0.2]; // Gold border

        // Top border
        let bx1 = to_ndc_x(dialog_x);
        let by1 = to_ndc_y(dialog_y);
        let bx2 = to_ndc_x(dialog_x + dialog_width);
        let by2 = to_ndc_y(dialog_y + border_width);
        vertices.extend_from_slice(&[
            bx1,
            by1,
            depth,
            0.0,
            0.0,
            tex_id,
            border_color[0],
            border_color[1],
            border_color[2],
            bx2,
            by1,
            depth,
            1.0,
            0.0,
            tex_id,
            border_color[0],
            border_color[1],
            border_color[2],
            bx1,
            by2,
            depth,
            0.0,
            1.0,
            tex_id,
            border_color[0],
            border_color[1],
            border_color[2],
            bx2,
            by1,
            depth,
            1.0,
            0.0,
            tex_id,
            border_color[0],
            border_color[1],
            border_color[2],
            bx2,
            by2,
            depth,
            1.0,
            1.0,
            tex_id,
            border_color[0],
            border_color[1],
            border_color[2],
            bx1,
            by2,
            depth,
            0.0,
            1.0,
            tex_id,
            border_color[0],
            border_color[1],
            border_color[2],
        ]);

        // Title area (top of dialog)
        let title_height = 50.0;
        let title_color = [0.15, 0.15, 0.2]; // Slightly lighter than background

        let tx1 = to_ndc_x(dialog_x + border_width);
        let ty1 = to_ndc_y(dialog_y + border_width);
        let tx2 = to_ndc_x(dialog_x + dialog_width - border_width);
        let ty2 = to_ndc_y(dialog_y + title_height);
        vertices.extend_from_slice(&[
            tx1,
            ty1,
            depth,
            0.0,
            0.0,
            tex_id,
            title_color[0],
            title_color[1],
            title_color[2],
            tx2,
            ty1,
            depth,
            1.0,
            0.0,
            tex_id,
            title_color[0],
            title_color[1],
            title_color[2],
            tx1,
            ty2,
            depth,
            0.0,
            1.0,
            tex_id,
            title_color[0],
            title_color[1],
            title_color[2],
            tx2,
            ty1,
            depth,
            1.0,
            0.0,
            tex_id,
            title_color[0],
            title_color[1],
            title_color[2],
            tx2,
            ty2,
            depth,
            1.0,
            1.0,
            tex_id,
            title_color[0],
            title_color[1],
            title_color[2],
            tx1,
            ty2,
            depth,
            0.0,
            1.0,
            tex_id,
            title_color[0],
            title_color[1],
            title_color[2],
        ]);

        // Attacker panel (left side)
        let panel_margin = 30.0;
        let panel_spacing = 20.0;
        let panel_width = (dialog_width - 2.0 * panel_margin - panel_spacing) / 2.0;

        // Reserve space for unit sprites (top area)
        let sprite_area_height = 120.0;

        // Text panel height - just enough for text content (header + 5 lines of stats)
        let text_line_height = 25.0;
        let panel_padding = 15.0;
        let panel_height = text_line_height * 6.0 + panel_padding * 2.0; // ~180px

        let panel_y = dialog_y + title_height + sprite_area_height + 10.0;

        let attacker_x = dialog_x + panel_margin;
        let ax1 = to_ndc_x(attacker_x);
        let ay1 = to_ndc_y(panel_y);
        let ax2 = to_ndc_x(attacker_x + panel_width);
        let ay2 = to_ndc_y(panel_y + panel_height);
        let attacker_color = [0.2, 0.4, 0.7]; // Blue for attacker

        vertices.extend_from_slice(&[
            ax1,
            ay1,
            depth,
            0.0,
            0.0,
            tex_id,
            attacker_color[0],
            attacker_color[1],
            attacker_color[2],
            ax2,
            ay1,
            depth,
            1.0,
            0.0,
            tex_id,
            attacker_color[0],
            attacker_color[1],
            attacker_color[2],
            ax1,
            ay2,
            depth,
            0.0,
            1.0,
            tex_id,
            attacker_color[0],
            attacker_color[1],
            attacker_color[2],
            ax2,
            ay1,
            depth,
            1.0,
            0.0,
            tex_id,
            attacker_color[0],
            attacker_color[1],
            attacker_color[2],
            ax2,
            ay2,
            depth,
            1.0,
            1.0,
            tex_id,
            attacker_color[0],
            attacker_color[1],
            attacker_color[2],
            ax1,
            ay2,
            depth,
            0.0,
            1.0,
            tex_id,
            attacker_color[0],
            attacker_color[1],
            attacker_color[2],
        ]);

        // Defender panel (right side)
        let defender_x = dialog_x + panel_margin + panel_width + panel_spacing;
        let dx1 = to_ndc_x(defender_x);
        let dy1 = to_ndc_y(panel_y);
        let dx2 = to_ndc_x(defender_x + panel_width);
        let dy2 = to_ndc_y(panel_y + panel_height);
        let defender_color = [0.7, 0.2, 0.2]; // Red for defender

        vertices.extend_from_slice(&[
            dx1,
            dy1,
            depth,
            0.0,
            0.0,
            tex_id,
            defender_color[0],
            defender_color[1],
            defender_color[2],
            dx2,
            dy1,
            depth,
            1.0,
            0.0,
            tex_id,
            defender_color[0],
            defender_color[1],
            defender_color[2],
            dx1,
            dy2,
            depth,
            0.0,
            1.0,
            tex_id,
            defender_color[0],
            defender_color[1],
            defender_color[2],
            dx2,
            dy1,
            depth,
            1.0,
            0.0,
            tex_id,
            defender_color[0],
            defender_color[1],
            defender_color[2],
            dx2,
            dy2,
            depth,
            1.0,
            1.0,
            tex_id,
            defender_color[0],
            defender_color[1],
            defender_color[2],
            dx1,
            dy2,
            depth,
            0.0,
            1.0,
            tex_id,
            defender_color[0],
            defender_color[1],
            defender_color[2],
        ]);

        // Attack option boxes for attacker (left side, under blue panel)
        if !confirmation.attacker_attacks.is_empty() {
            let attack_box_width = panel_width;
            let attack_box_height = 30.0;
            let attack_box_spacing = 5.0;
            let attack_section_y = panel_y + panel_height + 10.0;

            for (i, _attack) in confirmation.attacker_attacks.iter().enumerate() {
                let attack_y =
                    attack_section_y + (i as f32) * (attack_box_height + attack_box_spacing);
                let attack_x = attacker_x;

                // Check if this attack is selected
                let is_selected = self.combat_log_display.selected_attack_index == Some(i);

                // Attack box background - highlight if selected
                let attack_color = if is_selected {
                    [0.6, 0.5, 0.3] // Lighter brown for selected
                } else {
                    [0.4, 0.3, 0.2] // Normal brown
                };
                let atk_x1 = to_ndc_x(attack_x);
                let atk_y1 = to_ndc_y(attack_y);
                let atk_x2 = to_ndc_x(attack_x + attack_box_width);
                let atk_y2 = to_ndc_y(attack_y + attack_box_height);

                vertices.extend_from_slice(&[
                    atk_x1,
                    atk_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x2,
                    atk_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x1,
                    atk_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x2,
                    atk_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x2,
                    atk_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x1,
                    atk_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                ]);

                // Attack box border
                let border_color_attack = [0.9, 0.7, 0.3]; // Gold border
                let border_w = 2.0;

                // Top border
                let b_x1 = to_ndc_x(attack_x);
                let b_y1 = to_ndc_y(attack_y);
                let b_x2 = to_ndc_x(attack_x + attack_box_width);
                let b_y2 = to_ndc_y(attack_y + border_w);
                vertices.extend_from_slice(&[
                    b_x1,
                    b_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                ]);

                // Bottom border
                let b_y1 = to_ndc_y(attack_y + attack_box_height - border_w);
                let b_y2 = to_ndc_y(attack_y + attack_box_height);
                vertices.extend_from_slice(&[
                    b_x1,
                    b_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                ]);

                // Left border
                let b_x2 = to_ndc_x(attack_x + border_w);
                let b_y1 = to_ndc_y(attack_y);
                let b_y2 = to_ndc_y(attack_y + attack_box_height);
                vertices.extend_from_slice(&[
                    b_x1,
                    b_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                ]);

                // Right border
                let b_x1 = to_ndc_x(attack_x + attack_box_width - border_w);
                let b_x2 = to_ndc_x(attack_x + attack_box_width);
                vertices.extend_from_slice(&[
                    b_x1,
                    b_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                ]);
            }
        }

        // Attack option boxes for defender (right side, under red panel)
        if !confirmation.defender_attacks.is_empty() {
            let attack_box_width = panel_width;
            let attack_box_height = 30.0;
            let attack_box_spacing = 5.0;
            let attack_section_y = panel_y + panel_height + 10.0;

            for (i, _attack) in confirmation.defender_attacks.iter().enumerate() {
                let attack_y =
                    attack_section_y + (i as f32) * (attack_box_height + attack_box_spacing);
                let attack_x = defender_x;

                // Attack box background
                let attack_color = [0.4, 0.3, 0.2]; // Brown color for attack boxes
                let atk_x1 = to_ndc_x(attack_x);
                let atk_y1 = to_ndc_y(attack_y);
                let atk_x2 = to_ndc_x(attack_x + attack_box_width);
                let atk_y2 = to_ndc_y(attack_y + attack_box_height);

                vertices.extend_from_slice(&[
                    atk_x1,
                    atk_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x2,
                    atk_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x1,
                    atk_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x2,
                    atk_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x2,
                    atk_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                    atk_x1,
                    atk_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    attack_color[0],
                    attack_color[1],
                    attack_color[2],
                ]);

                // Attack box border
                let border_color_attack = [0.9, 0.7, 0.3]; // Gold border
                let border_w = 2.0;

                // Top border
                let b_x1 = to_ndc_x(attack_x);
                let b_y1 = to_ndc_y(attack_y);
                let b_x2 = to_ndc_x(attack_x + attack_box_width);
                let b_y2 = to_ndc_y(attack_y + border_w);
                vertices.extend_from_slice(&[
                    b_x1,
                    b_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                ]);

                // Bottom border
                let b_y1 = to_ndc_y(attack_y + attack_box_height - border_w);
                let b_y2 = to_ndc_y(attack_y + attack_box_height);
                vertices.extend_from_slice(&[
                    b_x1,
                    b_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                ]);

                // Left border
                let b_x1 = to_ndc_x(attack_x);
                let b_y1 = to_ndc_y(attack_y);
                let b_x2 = to_ndc_x(attack_x + border_w);
                let b_y2 = to_ndc_y(attack_y + attack_box_height);
                vertices.extend_from_slice(&[
                    b_x1,
                    b_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                ]);

                // Right border
                let b_x1 = to_ndc_x(attack_x + attack_box_width - border_w);
                let b_x2 = to_ndc_x(attack_x + attack_box_width);
                vertices.extend_from_slice(&[
                    b_x1,
                    b_y1,
                    depth,
                    0.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y1,
                    depth,
                    1.0,
                    0.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x2,
                    b_y2,
                    depth,
                    1.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                    b_x1,
                    b_y2,
                    depth,
                    0.0,
                    1.0,
                    tex_id,
                    border_color_attack[0],
                    border_color_attack[1],
                    border_color_attack[2],
                ]);
            }
        }

        // OK Button
        let ok_btn = &self.combat_log_display.ok_button;
        let ok_color = if ok_btn.hovered {
            [0.3, 0.8, 0.3] // Bright green when hovered
        } else {
            [0.2, 0.6, 0.2] // Dark green
        };
        let okx1 = to_ndc_x(ok_btn.position.0);
        let oky1 = to_ndc_y(ok_btn.position.1);
        let okx2 = to_ndc_x(ok_btn.position.0 + ok_btn.size.0);
        let oky2 = to_ndc_y(ok_btn.position.1 + ok_btn.size.1);

        vertices.extend_from_slice(&[
            okx1,
            oky1,
            depth,
            0.0,
            0.0,
            tex_id,
            ok_color[0],
            ok_color[1],
            ok_color[2],
            okx2,
            oky1,
            depth,
            1.0,
            0.0,
            tex_id,
            ok_color[0],
            ok_color[1],
            ok_color[2],
            okx1,
            oky2,
            depth,
            0.0,
            1.0,
            tex_id,
            ok_color[0],
            ok_color[1],
            ok_color[2],
            okx2,
            oky1,
            depth,
            1.0,
            0.0,
            tex_id,
            ok_color[0],
            ok_color[1],
            ok_color[2],
            okx2,
            oky2,
            depth,
            1.0,
            1.0,
            tex_id,
            ok_color[0],
            ok_color[1],
            ok_color[2],
            okx1,
            oky2,
            depth,
            0.0,
            1.0,
            tex_id,
            ok_color[0],
            ok_color[1],
            ok_color[2],
        ]);

        // Cancel Button
        let cancel_btn = &self.combat_log_display.cancel_button;
        let cancel_color = if cancel_btn.hovered {
            [0.9, 0.3, 0.3] // Bright red when hovered
        } else {
            [0.6, 0.2, 0.2] // Dark red
        };
        let cx1 = to_ndc_x(cancel_btn.position.0);
        let cy1 = to_ndc_y(cancel_btn.position.1);
        let cx2 = to_ndc_x(cancel_btn.position.0 + cancel_btn.size.0);
        let cy2 = to_ndc_y(cancel_btn.position.1 + cancel_btn.size.1);

        vertices.extend_from_slice(&[
            cx1,
            cy1,
            depth,
            0.0,
            0.0,
            tex_id,
            cancel_color[0],
            cancel_color[1],
            cancel_color[2],
            cx2,
            cy1,
            depth,
            1.0,
            0.0,
            tex_id,
            cancel_color[0],
            cancel_color[1],
            cancel_color[2],
            cx1,
            cy2,
            depth,
            0.0,
            1.0,
            tex_id,
            cancel_color[0],
            cancel_color[1],
            cancel_color[2],
            cx2,
            cy1,
            depth,
            1.0,
            0.0,
            tex_id,
            cancel_color[0],
            cancel_color[1],
            cancel_color[2],
            cx2,
            cy2,
            depth,
            1.0,
            1.0,
            tex_id,
            cancel_color[0],
            cancel_color[1],
            cancel_color[2],
            cx1,
            cy2,
            depth,
            0.0,
            1.0,
            tex_id,
            cancel_color[0],
            cancel_color[1],
            cancel_color[2],
        ]);

        // Upload and draw vertices
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );
        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::TRIANGLES, 0, (vertices.len() / 9) as GLint);
    }

    unsafe fn render_combat_dialog_text(&mut self, confirmation: &CombatConfirmation) {
        let (dialog_x, dialog_y) = self.combat_log_display.position;
        let (dialog_width, _dialog_height) = self.combat_log_display.size;
        let window_width = self.window_width;
        let window_height = self.window_height;

        // Title
        let title = "COMBAT!";
        let title_size = 30.0;
        let title_x = dialog_x + (dialog_width - title.len() as f32 * title_size * 0.6) / 2.0;
        let title_y = dialog_y + 10.0;
        self.text_renderer.render_text(
            title,
            title_x,
            title_y,
            title_size,
            [1.0, 0.9, 0.4, 1.0], // Gold color
            window_width,
            window_height,
        );

        // Attacker stats (left panel)
        let title_height = 50.0;
        let sprite_area_height = 120.0;
        let attacker_x = dialog_x + 40.0;
        let mut attacker_y = dialog_y + title_height + sprite_area_height + 20.0;
        let text_size = 16.0;
        let line_height = 25.0;

        self.text_renderer.render_text(
            "ATTACKER",
            attacker_x,
            attacker_y,
            18.0,
            [0.6, 0.8, 1.0, 1.0], // Light blue
            window_width,
            window_height,
        );
        attacker_y += line_height + 5.0;

        self.text_renderer.render_text(
            &confirmation.attacker_name,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );
        attacker_y += line_height;

        let hp_text = format!(
            "HP: {}/{}",
            confirmation.attacker_hp, confirmation.attacker_max_hp
        );
        self.text_renderer.render_text(
            &hp_text,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );
        attacker_y += line_height;

        let atk_text = format!("ATK: {}", confirmation.attacker_attack);
        self.text_renderer.render_text(
            &atk_text,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );
        attacker_y += line_height;

        let def_text = format!("DEF: {}", confirmation.attacker_defense);
        self.text_renderer.render_text(
            &def_text,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );
        attacker_y += line_height;

        let atkr_text = format!("{}/round", confirmation.attacker_attacks_per_round);
        self.text_renderer.render_text(
            &atkr_text,
            attacker_x,
            attacker_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );

        // Defender stats (right panel)
        let defender_x = dialog_x + dialog_width / 2.0 + 40.0;
        let mut defender_y = dialog_y + title_height + sprite_area_height + 20.0;

        self.text_renderer.render_text(
            "DEFENDER",
            defender_x,
            defender_y,
            18.0,
            [1.0, 0.7, 0.7, 1.0], // Light red
            window_width,
            window_height,
        );
        defender_y += line_height + 5.0;

        self.text_renderer.render_text(
            &confirmation.defender_name,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );
        defender_y += line_height;

        let hp_text = format!(
            "HP: {}/{}",
            confirmation.defender_hp, confirmation.defender_max_hp
        );
        self.text_renderer.render_text(
            &hp_text,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );
        defender_y += line_height;

        let atk_text = format!("ATK: {}", confirmation.defender_attack);
        self.text_renderer.render_text(
            &atk_text,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );
        defender_y += line_height;

        let def_text = format!("DEF: {}", confirmation.defender_defense);
        self.text_renderer.render_text(
            &def_text,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );
        defender_y += line_height;

        let atkr_text = format!("{}/round", confirmation.defender_attacks_per_round);
        self.text_renderer.render_text(
            &atkr_text,
            defender_x,
            defender_y,
            text_size,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );

        // Attack option labels for attacker (left column)
        if !confirmation.attacker_attacks.is_empty() {
            let (dialog_x, dialog_y) = self.combat_log_display.position;
            let (dialog_width, _dialog_height) = self.combat_log_display.size;
            let attack_box_height = 30.0;
            let attack_box_spacing = 5.0;
            let title_height = 50.0;
            let sprite_area_height = 120.0;
            let text_line_height = 25.0;
            let panel_padding = 15.0;
            let panel_height = text_line_height * 6.0 + panel_padding * 2.0;
            let panel_y = dialog_y + title_height + sprite_area_height + 10.0;
            let attack_section_y = panel_y + panel_height + 10.0;
            let panel_margin = 30.0;
            let panel_spacing = 20.0;
            let _panel_width = (dialog_width - 2.0 * panel_margin - panel_spacing) / 2.0;
            let attacker_x = dialog_x + panel_margin;

            for (i, attack) in confirmation.attacker_attacks.iter().enumerate() {
                let attack_y =
                    attack_section_y + (i as f32) * (attack_box_height + attack_box_spacing);
                let attack_x = attacker_x + 10.0;
                let attack_text_y = attack_y + (attack_box_height - 14.0) / 2.0 + 5.0;

                // Check if this attack is selected
                let is_selected = self.combat_log_display.selected_attack_index == Some(i);

                // Attack name and damage combined - brighter color if selected
                let attack_text = format!("{} ({}x{})", attack.name, attack.damage, attack.range);
                let text_color = if is_selected {
                    [1.0, 1.0, 1.0, 1.0] // White for selected
                } else {
                    [1.0, 1.0, 0.8, 1.0] // Light yellow for unselected
                };

                self.text_renderer.render_text(
                    &attack_text,
                    attack_x,
                    attack_text_y,
                    14.0,
                    text_color,
                    window_width,
                    window_height,
                );
            }
        }

        // Attack option labels for defender (right column)
        if !confirmation.defender_attacks.is_empty() {
            let (dialog_x, dialog_y) = self.combat_log_display.position;
            let (dialog_width, _dialog_height) = self.combat_log_display.size;
            let attack_box_height = 30.0;
            let attack_box_spacing = 5.0;
            let title_height = 50.0;
            let sprite_area_height = 120.0;
            let text_line_height = 25.0;
            let panel_padding = 15.0;
            let panel_height = text_line_height * 6.0 + panel_padding * 2.0;
            let panel_y = dialog_y + title_height + sprite_area_height + 10.0;
            let attack_section_y = panel_y + panel_height + 10.0;
            let panel_margin = 30.0;
            let panel_spacing = 20.0;
            let panel_width = (dialog_width - 2.0 * panel_margin - panel_spacing) / 2.0;
            let defender_x = dialog_x + panel_margin + panel_width + panel_spacing;

            for (i, attack) in confirmation.defender_attacks.iter().enumerate() {
                let attack_y =
                    attack_section_y + (i as f32) * (attack_box_height + attack_box_spacing);
                let attack_x = defender_x + 10.0;
                let attack_text_y = attack_y + (attack_box_height - 14.0) / 2.0 + 5.0;

                // Attack name and damage combined
                let attack_text = format!("{} ({}x{})", attack.name, attack.damage, attack.range);
                self.text_renderer.render_text(
                    &attack_text,
                    attack_x,
                    attack_text_y,
                    14.0,
                    [1.0, 1.0, 0.8, 1.0], // Light yellow
                    window_width,
                    window_height,
                );
            }
        }

        // Button labels
        let ok_btn = &self.combat_log_display.ok_button;
        let ok_label_x = ok_btn.position.0 + (ok_btn.size.0 - 2.0 * 12.0 * 0.6) / 2.0; // Center "OK"
        let ok_label_y = ok_btn.position.1 + (ok_btn.size.1 - 20.0) / 2.0 + 5.0;
        self.text_renderer.render_text(
            "OK",
            ok_label_x,
            ok_label_y,
            20.0,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );

        let cancel_btn = &self.combat_log_display.cancel_button;
        let cancel_label_x = cancel_btn.position.0 + (cancel_btn.size.0 - 6.0 * 12.0 * 0.6) / 2.0; // Center "Cancel"
        let cancel_label_y = cancel_btn.position.1 + (cancel_btn.size.1 - 20.0) / 2.0 + 5.0;
        self.text_renderer.render_text(
            "Cancel",
            cancel_label_x,
            cancel_label_y,
            20.0,
            [1.0, 1.0, 1.0, 1.0],
            window_width,
            window_height,
        );
    }

    unsafe fn render_combat_log_entries(&self) {
        // TODO: Render combat log entries as scrollable text
        // For now, this is empty - will show combat history after fights
    }
}
