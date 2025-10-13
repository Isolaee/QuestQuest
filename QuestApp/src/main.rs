use game::*;
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use graphics::core::hexagon::SpriteType;
use graphics::{setup_dynamic_hexagons, HexCoord, HexGrid, HighlightType, Renderer};
use raw_window_handle::HasWindowHandle;
use std::ffi::CString;
use units::*;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct GameApp {
    window: Option<Window>,
    gl_context: Option<glutin::context::PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    hex_grid: HexGrid,
    renderer: Option<Renderer>,
    game_world: GameWorld,
    selected_unit: Option<uuid::Uuid>,
    show_unit_info: bool,
    unit_info_text: Vec<String>,
    cursor_position: (f64, f64),   // Track cursor position for clicks
    movement_range: Vec<HexCoord>, // Available movement hexes for selected unit
}

impl GameApp {
    fn new() -> Self {
        let mut game_world = GameWorld::new(8); // World radius of 8
        let mut hex_grid = HexGrid::new();

        // Create some test units
        let mut warrior = Unit::new(
            "Thorin Okenshield".to_string(),
            HexCoord::new(0, 0),
            unit_race::Race::Dwarf,
            unit_class::UnitClass::Warrior,
        );
        warrior.experience = 150;
        warrior.level = 2;
        warrior.recalculate_stats();

        // Add equipment to warrior
        let sword = Item::new(
            "Orcrist".to_string(),
            "An ancient elvish blade".to_string(),
            item::ItemProperties::Weapon {
                attack_bonus: 8,
                range_modifier: 0,
                range_type_override: None,
            },
        );

        let armor = Item::new(
            "Mithril Chainmail".to_string(),
            "Lightweight but strong armor".to_string(),
            item::ItemProperties::Armor {
                defense_bonus: 12,
                movement_penalty: -1,
            },
        );

        warrior.add_item_to_inventory(sword.clone());
        warrior.add_item_to_inventory(armor.clone());
        let _ = warrior.equip_item(sword.id);
        let _ = warrior.equip_item(armor.id);

        // Damage warrior for demonstration
        warrior.combat_stats.health = warrior.combat_stats.max_health / 3;

        // Add units to game world
        let warrior_unit = GameUnit::new(warrior);
        let unit_pos = warrior_unit.position();

        game_world.add_unit(warrior_unit);

        // IMPORTANT: Add unit ON TOP of terrain (not replacing it)
        hex_grid.set_unit_at(unit_pos, SpriteType::Unit);

        Self {
            window: None,
            gl_context: None,
            gl_surface: None,
            hex_grid,
            renderer: None,
            game_world,
            selected_unit: None,
            show_unit_info: false,
            unit_info_text: Vec::new(),
            cursor_position: (0.0, 0.0),
            movement_range: Vec::new(),
        }
    }

    fn handle_left_click(&mut self, x: f64, y: f64) {
        let hex_coord = self.screen_to_hex_coord(x, y);

        println!("Left-clicked hex {:?}", hex_coord);

        // If we have a unit selected and clicked on a valid movement hex, move the unit
        if let Some(unit_id) = self.selected_unit {
            if self.movement_range.contains(&hex_coord) {
                // Move the unit
                if let Err(error) = self.game_world.move_unit(unit_id, hex_coord) {
                    println!("Failed to move unit: {}", error);
                } else {
                    println!("Unit moved to {:?}", hex_coord);
                    // Clear selection after successful move
                    self.clear_selection();
                }
            } else {
                // Clicked outside movement range, clear selection
                self.clear_selection();
                println!("Cleared selection (clicked outside movement range)");
            }
        } else {
            // No unit selected, try to select a unit at this position
            if let Some(unit_id) = self.find_unit_at_hex(hex_coord) {
                self.select_unit(unit_id);
            }
        }
    }

    fn handle_right_click(&mut self, x: f64, y: f64) {
        let hex_coord = self.screen_to_hex_coord(x, y);

        println!("Right-clicked hex {:?}", hex_coord);

        // Check if there's a unit at this hex coordinate
        if let Some(unit_id) = self.find_unit_at_hex(hex_coord) {
            self.select_unit(unit_id);
        } else {
            self.clear_selection();
            println!("No unit found at hex {:?}", hex_coord);
        }
    }

    fn select_unit(&mut self, unit_id: uuid::Uuid) {
        self.selected_unit = Some(unit_id);
        self.show_unit_info = true;
        self.update_unit_info_display(unit_id);

        // Get movement range for the selected unit
        if let Some(game_unit) = self.game_world.units.get(&unit_id) {
            let all_coords = game_unit.unit().get_movement_range();

            // Filter movement range by world validity
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

    fn screen_to_hex_coord(&self, screen_x: f64, screen_y: f64) -> HexCoord {
        // Convert screen coordinates to world coordinates
        // Account for camera offset
        let world_x = ((screen_x - 600.0) / 600.0 * 2.0) as f32 + self.hex_grid.camera.position.x;
        let world_y = (-(screen_y - 400.0) / 400.0 * 1.5) as f32 + self.hex_grid.camera.position.y;

        // Convert world coordinates to hex coordinates for flat-top hexagons
        // Using the inverse of: x = hex_size * (3/2 * q), y = hex_size * (sqrt(3) * (r + q/2))
        let hex_size = self.hex_grid.hex_size;

        // For flat-top hexagons - inverse transformation to fractional coordinates
        let q_frac = (2.0 / 3.0 * world_x) / hex_size;
        let r_frac = (-1.0 / 3.0 * world_x + (3.0_f32.sqrt()) / 3.0 * world_y) / hex_size;

        // Convert to cube coordinates for proper rounding
        // In axial: q, r; In cube: x=q, z=r, y=(-x-z)
        let x = q_frac;
        let z = r_frac;
        let y = -x - z;

        // Round to nearest integer cube coordinates
        let mut rx = x.round();
        let mut ry = y.round();
        let mut rz = z.round();

        // Calculate rounding errors
        let x_diff = (rx - x).abs();
        let y_diff = (ry - y).abs();
        let z_diff = (rz - z).abs();

        // Reset the component with the largest error to maintain x+y+z=0
        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry - rz;
        } else if y_diff > z_diff {
            ry = -rx - rz;
        } else {
            rz = -rx - ry;
        }

        // Convert back to axial coordinates
        HexCoord::new(rx as i32, rz as i32)
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
        }
    }

    fn render_ui(&self) {
        if self.show_unit_info && !self.unit_info_text.is_empty() {
            // In a real implementation, you'd render text to the screen using OpenGL
            // For now, we'll just print to indicate the UI would be shown
            println!("\n=== ON-SCREEN UNIT INFO ===");
            for line in &self.unit_info_text {
                println!("{}", line);
            }
            println!("=============================\n");
        }
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
}

impl ApplicationHandler for GameApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title("QuestQuest - Interactive Game Window")
            .with_inner_size(winit::dpi::LogicalSize::new(1200, 800));

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
            std::num::NonZeroU32::new(1200).unwrap(),
            std::num::NonZeroU32::new(800).unwrap(),
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
            gl::Viewport(0, 0, 1200, 800);
            gl::ClearColor(0.05, 0.05, 0.1, 1.0);
        }

        let (vao, shader_program, dynamic_vbo) = unsafe { setup_dynamic_hexagons() };
        match Renderer::new(vao, shader_program, dynamic_vbo) {
            Ok(renderer) => {
                self.renderer = Some(renderer);
                println!("🎮 QuestQuest Game Window Started!");
                println!("📍 Units placed at (0,0)");
                println!("🖱️  RIGHT-CLICK on a unit to select it and show movement range");
                println!("🖱️  LEFT-CLICK on blue hexes to move the selected unit");
                println!("⌨️  Use arrow keys to move camera");
                println!("🔤 Press 'C' to show detailed unit info in console");
                println!("🔤 Press ESC to deselect unit");
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
