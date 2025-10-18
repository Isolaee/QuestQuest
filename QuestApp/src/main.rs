use game::*;
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use graphics::core::hexagon::SpriteType;
use graphics::math::Vec2;
use graphics::{
    setup_dynamic_hexagons, HexCoord, HexGrid, HighlightType, Renderer, UiPanel, UnitDisplayInfo,
};
use raw_window_handle::HasWindowHandle;
use std::ffi::CString;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

// Screen size constants
const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 800.0;

struct GameApp {
    window: Option<Window>,
    gl_context: Option<glutin::context::PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    hex_grid: HexGrid,
    renderer: Option<Renderer>,
    ui_panel: Option<UiPanel>,
    game_world: GameWorld,
    selected_unit: Option<uuid::Uuid>,
    show_unit_info: bool,
    unit_info_text: Vec<String>,
    cursor_position: (f64, f64),       // Track cursor position for clicks
    movement_range: Vec<HexCoord>,     // Available movement hexes for selected unit
    hower_debug_hex: Option<HexCoord>, // Debug: hex under cursor
    hower_debug_enabled: bool,         // Toggle for hower debug mode
}

impl GameApp {
    fn new() -> Self {
        let mut game_world = GameWorld::new(8); // World radius of 8

        // Add demo units to the world
        let hero = units::Unit::new(
            "Thorin".to_string(),
            HexCoord::new(0, 0),
            units::Race::Human,
            units::UnitClass::Warrior,
            units::Terrain::Grasslands,
        );
        game_world.add_unit(GameUnit::new(hero));

        let archer = units::Unit::new(
            "Legolas".to_string(),
            HexCoord::new(2, -1),
            units::Race::Elf,
            units::UnitClass::Archer,
            units::Terrain::Forest0,
        );
        game_world.add_unit(GameUnit::new(archer));

        let paladin = units::Unit::new(
            "Gimli".to_string(),
            HexCoord::new(-2, 1),
            units::Race::Dwarf,
            units::UnitClass::Paladin,
            units::Terrain::Mountain,
        );
        game_world.add_unit(GameUnit::new(paladin));

        let app = Self {
            window: None,
            gl_context: None,
            gl_surface: None,
            hex_grid: HexGrid::new(),
            renderer: None,
            ui_panel: None,
            game_world,
            selected_unit: None,
            show_unit_info: false,
            unit_info_text: Vec::new(),
            cursor_position: (0.0, 0.0),
            movement_range: Vec::new(),
            hower_debug_hex: None,
            hower_debug_enabled: true, // Start with debug enabled
        };

        app
    }

    fn handle_left_click(&mut self, x: f64, y: f64) {
        if let Some(hex_coord) = self.screen_to_hex_coord(x, y) {
            if let Some(unit_id) = self.selected_unit {
                // STATE 1: Unit is already selected
                if self.movement_range.contains(&hex_coord) {
                    // Valid move - execute movement
                    if let Err(e) = self.game_world.move_unit(unit_id, hex_coord) {
                        println!("Failed to move unit: {}", e);
                    }
                    self.clear_selection(); // Reset state
                } else {
                    // Invalid move - clear selection
                    self.clear_selection();
                }
            } else {
                // STATE 2: No unit selected
                if let Some(unit_id) = self.find_unit_at_hex(hex_coord) {
                    self.select_unit(unit_id); // Enter selection state
                }
            }
        }
    }

    fn handle_right_click(&mut self, x: f64, y: f64) {
        if let Some(hex_coord) = self.screen_to_hex_coord(x, y) {
            println!("Right-clicked hex {:?}", hex_coord);

            // Check if there's a unit at this hex coordinate
            if let Some(unit_id) = self.find_unit_at_hex(hex_coord) {
                self.select_unit(unit_id);
            } else {
                self.clear_selection();
                println!("No unit found at hex {:?}", hex_coord);
            }
        }
    }

    fn select_unit(&mut self, unit_id: uuid::Uuid) {
        self.selected_unit = Some(unit_id);
        self.show_unit_info = true;
        self.update_unit_info_display(unit_id);

        // Get movement range for the selected unit
        if let Some(game_unit) = self.game_world.units.get(&unit_id) {
            let all_coords = game_unit.unit().get_movement_range();

            // Filter movement range to only include valid hexes
            self.movement_range = all_coords
                .into_iter()
                .filter(|&coord| {
                    self.game_world
                        .is_position_valid_for_movement(coord, Some(unit_id))
                })
                .collect();

            // Update hex grid highlighting
            self.update_highlight_display();

            println!(
                "Unit selected: {:?} with {} valid movement options",
                unit_id,
                self.movement_range.len()
            );

            // Call the unit's on_click method to show detailed info in console
            self.call_unit_on_click(unit_id);
        }
    }

    fn clear_selection(&mut self) {
        self.selected_unit = None;
        self.show_unit_info = false;
        self.unit_info_text.clear();
        self.movement_range.clear();

        // Clear all highlights
        self.hex_grid.clear_all_highlights();

        // Clear UI panel
        if let Some(ui_panel) = &mut self.ui_panel {
            ui_panel.clear_unit_info();
        }
    }

    fn update_highlight_display(&mut self) {
        // Clear existing highlights
        self.hex_grid.clear_all_highlights();

        // Highlight selected unit position
        if let Some(unit_id) = self.selected_unit {
            if let Some(game_unit) = self.game_world.units.get(&unit_id) {
                self.hex_grid
                    .highlight_hex(game_unit.position(), HighlightType::Selected);
            }
        }

        // Highlight movement range
        self.hex_grid
            .highlight_hexes(&self.movement_range, HighlightType::MovementRange);
    }

    fn call_unit_on_click(&self, unit_id: uuid::Uuid) {
        // Try to get the unit and call its on_click method
        if let Some(game_obj) = self.game_world.units.get(&unit_id) {
            println!("\n🖱️  CALLING UNIT'S ON_CLICK METHOD:");
            println!("═══════════════════════════════════");

            // This will call the actual Unit's on_click() method!
            game_obj.show_details();

            println!("═══════════════════════════════════\n");
        } else {
            println!("Unit with ID {} not found!", unit_id);
        }
    }

    fn screen_to_hex_coord(&self, x: f64, y: f64) -> Option<HexCoord> {
        // Prefer geometric conversion via renderer's camera and grid
        let screen = Vec2::new(x as f32, y as f32);
        let window = Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        self.hex_grid.screen_to_hex_coord(screen, window)
    }

    fn find_unit_at_hex(&self, hex_coord: HexCoord) -> Option<uuid::Uuid> {
        // Find if there's a unit at the given hex coordinate
        for (id, obj) in &self.game_world.units {
            if obj.position() == hex_coord {
                return Some(*id);
            }
        }
        None
    }

    fn update_unit_info_display(&mut self, unit_id: uuid::Uuid) {
        if let Some(game_unit) = self.game_world.units.get(&unit_id) {
            let position = game_unit.position();
            let name = game_unit.name();
            let unit = game_unit.unit();

            // Create formatted text for display
            self.unit_info_text = vec![
                "┌─────────────────────────────────────────────────────┐".to_string(),
                "│                  UNIT DETAILS                       │".to_string(),
                "├─────────────────────────────────────────────────────┤".to_string(),
                format!("│ Name: {:<43} │", name),
                format!("│ Position: {:<39} │", format!("{:?}", position)),
                "│                                                     │".to_string(),
                "│ Press 'C' to show detailed console info             │".to_string(),
                "│ Press ESC to close this panel                       │".to_string(),
                "└─────────────────────────────────────────────────────┘".to_string(),
            ];

            // Update UI panel with unit info
            if let Some(ui_panel) = &mut self.ui_panel {
                let display_info = UnitDisplayInfo {
                    name: unit.name.clone(),
                    race: format!("{:?}", unit.race),
                    class: format!("{:?}", unit.class),
                    level: unit.level,
                    experience: unit.experience,
                    health: unit.combat_stats.health as u32,
                    max_health: unit.combat_stats.max_health as u32,
                    terrain: format!("{:?}", unit.current_terrain),
                    position_q: position.q,
                    position_r: position.r,
                };
                ui_panel.set_unit_info(display_info);
            }
        }
    }

    fn render_ui(&self) {
        // UI info now rendered by UiPanel, no terminal output needed
    }

    // Add this method to update the hex grid when units move
    fn update_hex_grid_units(&mut self) {
        // Clear all existing unit sprites (but keep terrain)
        for hex in self.hex_grid.hexagons.values_mut() {
            if let Some(unit_sprite) = hex.unit_sprite {
                if unit_sprite == SpriteType::Unit {
                    hex.set_unit_sprite(None);
                }
            }
        }

        // Add current unit positions as unit sprites on top of terrain
        for unit in self.game_world.units.values() {
            let pos = unit.position();
            self.hex_grid.set_unit_at(pos, SpriteType::Unit);
        }
    }

    /// hower method: Debug method to highlight hex under cursor
    fn hower(&mut self, x: f64, y: f64) {
        if !self.hower_debug_enabled {
            return;
        }

        // Convert to hex coordinate using geometric conversion
        if let Some(hex_coord) = self.screen_to_hex_coord(x, y) {
            // Clear previous debug highlighting
            if let Some(prev_hex) = self.hower_debug_hex {
                if let Some(hex) = self.hex_grid.hexagons.get_mut(&prev_hex) {
                    // Reset to original color
                    hex.color = [
                        0.3 + 0.4 * ((prev_hex.q + prev_hex.r) % 3) as f32 / 3.0,
                        0.4 + 0.3 * (prev_hex.q % 4) as f32 / 4.0,
                        0.5 + 0.3 * (prev_hex.r % 5) as f32 / 5.0,
                    ];
                }
            }

            // Set new debug hex
            self.hower_debug_hex = Some(hex_coord);

            // Apply bright debug color to current hex under cursor
            if let Some(hex) = self.hex_grid.hexagons.get_mut(&hex_coord) {
                hex.color = [1.0, 1.0, 0.0]; // Bright yellow for debugging
            }
        }
    }
}

impl ApplicationHandler for GameApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title("QuestQuest - Interactive Game Window")
            .with_inner_size(winit::dpi::LogicalSize::new(
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            ));

        let template = glutin::config::ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();

        let gl_display = gl_config.display();
        let context_attributes =
            ContextAttributesBuilder::new().build(Some(window.window_handle().unwrap().into()));
        let mut not_current_gl_context =
            Some(unsafe { gl_display.create_context(&gl_config, &context_attributes) }.unwrap());

        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window.window_handle().unwrap().into(),
            std::num::NonZeroU32::new(SCREEN_WIDTH as u32).unwrap(),
            std::num::NonZeroU32::new(SCREEN_HEIGHT as u32).unwrap(),
        );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &surface_attributes)
                .unwrap()
        };

        let gl_context = not_current_gl_context
            .take()
            .unwrap()
            .make_current(&gl_surface)
            .unwrap();

        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

        unsafe {
            gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            gl::ClearColor(0.05, 0.05, 0.1, 1.0);
        }

        let (vao, shader_program, dynamic_vbo) = unsafe { setup_dynamic_hexagons() };
        match Renderer::new(vao, shader_program, dynamic_vbo) {
            Ok(renderer) => {
                self.renderer = Some(renderer);

                // Initialize UI panel
                match UiPanel::new(SCREEN_WIDTH, SCREEN_HEIGHT) {
                    Ok(ui_panel) => {
                        self.ui_panel = Some(ui_panel);
                        println!("✅ UI Panel initialized!");
                    }
                    Err(e) => {
                        println!("⚠️  Failed to create UI panel: {}", e);
                    }
                }

                println!("🎮 QuestQuest Game Window Started!");
                println!("📍 Units: Thorin at (0,0), Legolas at (2,-1), Gimli at (-2,1)");
                println!("🖱️  RIGHT-CLICK on a unit to select it and show movement range");
                println!("🖱️  LEFT-CLICK on blue hexes to move the selected unit");
                println!("⌨️  Use arrow keys to move camera");
                println!("🔤 Press 'C' to show detailed unit info in console");
                println!("🔤 Press 'H' to toggle hower debug mode (highlights hex under cursor)");
                println!("🔤 Press ESC to deselect unit");
                println!("🔍 hower DEBUG: Currently ENABLED - cursor highlights hexes in yellow");
            }
            Err(e) => {
                println!("Failed to create renderer: {}", e);
                event_loop.exit();
                return;
            }
        }

        self.window = Some(window);
        self.gl_context = Some(gl_context);
        self.gl_surface = Some(gl_surface);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Store cursor position for click handling
                self.cursor_position = (position.x, position.y);

                // hower DEBUG: Highlight hex under cursor
                self.hower(position.x, position.y);

                // Request redraw to show the debug highlighting
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                match button {
                    MouseButton::Left => {
                        self.handle_left_click(self.cursor_position.0, self.cursor_position.1);
                    }
                    MouseButton::Right => {
                        self.handle_right_click(self.cursor_position.0, self.cursor_position.1);
                    }
                    _ => {}
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    let move_speed = 0.1;
                    match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowUp) => {
                            self.hex_grid.move_camera(0.0, move_speed);
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowDown) => {
                            self.hex_grid.move_camera(0.0, -move_speed);
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft) => {
                            self.hex_grid.move_camera(-move_speed, 0.0);
                        }
                        winit::keyboard::PhysicalKey::Code(
                            winit::keyboard::KeyCode::ArrowRight,
                        ) => {
                            self.hex_grid.move_camera(move_speed, 0.0);
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyC) => {
                            // Show detailed unit info in console
                            if let Some(unit_id) = self.selected_unit {
                                self.call_unit_on_click(unit_id);
                            } else {
                                println!("No unit selected. Click on a unit first!");
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) => {
                            self.clear_selection();
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyH) => {
                            // Toggle hower debug mode
                            self.hower_debug_enabled = !self.hower_debug_enabled;
                            if self.hower_debug_enabled {
                                println!("🔍 hower DEBUG: Enabled - cursor will highlight hexes");
                            } else {
                                println!("🔍 hower DEBUG: Disabled");
                                // Clear any existing debug highlighting
                                if let Some(prev_hex) = self.hower_debug_hex {
                                    if let Some(hex) = self.hex_grid.hexagons.get_mut(&prev_hex) {
                                        hex.color = [
                                            0.3 + 0.4 * ((prev_hex.q + prev_hex.r) % 3) as f32
                                                / 3.0,
                                            0.4 + 0.3 * (prev_hex.q % 4) as f32 / 4.0,
                                            0.5 + 0.3 * (prev_hex.r % 5) as f32 / 5.0,
                                        ];
                                    }
                                }
                                self.hower_debug_hex = None;
                            }
                        }
                        _ => {}
                    }
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                // Update unit positions on hex grid before rendering
                self.update_hex_grid_units();

                if let Some(renderer) = &self.renderer {
                    renderer.render(&self.hex_grid);

                    // Render UI panel
                    if let Some(ui_panel) = &mut self.ui_panel {
                        ui_panel.render(SCREEN_WIDTH, SCREEN_HEIGHT);
                    }

                    // Render UI overlay (in a real implementation)
                    self.render_ui();

                    if let (Some(gl_context), Some(gl_surface)) =
                        (&self.gl_context, &self.gl_surface)
                    {
                        gl_surface.swap_buffers(gl_context).unwrap();
                    }
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = GameApp::new();

    println!("🎮 Starting QuestQuest Interactive Game Window...");
    event_loop.run_app(&mut app).unwrap();
}
