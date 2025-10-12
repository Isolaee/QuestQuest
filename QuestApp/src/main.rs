use game::*;
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use graphics::core::hexagon::SpriteType;
use graphics::{setup_dynamic_hexagons, HexCoord, HexGrid, Renderer};
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
    cursor_position: (f64, f64), // Track cursor position for clicks
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
        }
    }

    fn handle_mouse_click(&mut self, x: f64, y: f64) {
        // Convert screen coordinates to world coordinates
        let world_x = (x - 600.0) / 600.0 * 2.0; // Assuming 1200x800 window
        let world_y = -(y - 400.0) / 400.0 * 1.5; // Convert to OpenGL coords

        // Convert world coordinates to hex coordinates (approximate)
        let hex_coord = self.screen_to_hex_coord(world_x as f32, world_y as f32);

        println!(
            "Clicked at screen ({}, {}), world ({}, {}), hex {:?}",
            x, y, world_x, world_y, hex_coord
        );

        // Check if there's a unit at this hex coordinate
        if let Some(unit_id) = self.find_unit_at_hex(hex_coord) {
            self.selected_unit = Some(unit_id);
            self.show_unit_info = true;
            self.update_unit_info_display(unit_id);
            println!("Unit selected: {:?}", unit_id);

            // Call the unit's on_click method to show detailed info in console
            self.call_unit_on_click(unit_id);
        } else {
            self.selected_unit = None;
            self.show_unit_info = false;
            self.unit_info_text.clear();
            println!("No unit found at hex {:?}", hex_coord);
        }
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

    fn screen_to_hex_coord(&self, world_x: f32, world_y: f32) -> HexCoord {
        // Convert world coordinates to hex coordinates for flat-top hexagons
        let hex_size = self.hex_grid.hex_size;

        // For flat-top hexagons (corrected formula)
        let q = (2.0 / 3.0 * world_x) / hex_size;
        let r = (-1.0 / 3.0 * world_x + (3.0_f32.sqrt()) / 3.0 * world_y) / hex_size;

        HexCoord::new(q.round() as i32, r.round() as i32)
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
                println!("📍 Units placed at (0,0) and (2,1)");
                println!("🖱️  Click on hexagons to select units");
                println!("⌨️  Use arrow keys to move camera");
                println!("🔤 Press 'C' to show detailed unit info in console");
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
                button: MouseButton::Left,
                ..
            } => {
                // Use the actual cursor position
                self.handle_mouse_click(self.cursor_position.0, self.cursor_position.1);
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
                            self.show_unit_info = false;
                            self.selected_unit = None;
                            self.unit_info_text.clear();
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
