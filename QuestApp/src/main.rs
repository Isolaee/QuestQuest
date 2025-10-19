use game::*;
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use graphics::core::hexagon::SpriteType;
use graphics::math::Vec2;
use graphics::{
    setup_dynamic_hexagons, GuideLibrary, HexCoord, HexGrid, HighlightType, Renderer, UiPanel,
    UnitDisplayInfo,
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
    pickup_prompt: Option<PickupPrompt>, // Item pickup prompt state
}

#[derive(Clone)]
struct PickupPrompt {
    unit_id: uuid::Uuid,
    item_id: uuid::Uuid,
    item_name: String,
}

impl GameApp {
    fn new() -> Self {
        let mut game_world = GameWorld::new(8); // World radius of 8

        // ========================================
        // DEMO UNITS (2 units for simplicity)
        // ========================================
        // Player unit
        let hero = units::UnitFactory::create_unit(
            "Thorin".to_string(),
            HexCoord::new(0, 0),
            units::Race::Human,
            units::UnitClass::Warrior,
            units::Terrain::Grasslands,
        );
        game_world.add_unit(GameUnit::new(hero));

        // Enemy unit
        let orc_warrior = units::UnitFactory::create_unit(
            "Orc Grunt".to_string(),
            HexCoord::new(4, 2),
            units::Race::Orc,
            units::UnitClass::Warrior,
            units::Terrain::Grasslands,
        );
        game_world.add_unit(GameUnit::new(orc_warrior));

        // Add a test item on the ground for pickup testing
        let test_sword = items::Item::new(
            "Iron Sword".to_string(),
            "A sturdy iron sword with a sharp blade.".to_string(),
            items::ItemProperties::Weapon {
                attack_bonus: 5,
                range_modifier: 0,
                range_type_override: None,
            },
        );
        let item_pickup = InteractiveObject::new_item_pickup(HexCoord::new(1, 1), test_sword);
        game_world.add_interactive_object(item_pickup);

        Self {
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
            pickup_prompt: None,
        }
    }

    fn handle_left_click(&mut self, x: f64, y: f64) {
        // Priority 1: Check if clicking on menu buttons (highest priority)
        if let Some(renderer) = &mut self.renderer {
            if renderer.menu_display.active {
                if let Some(action) = renderer.menu_display.get_button_action(x as f32, y as f32) {
                    use graphics::MenuAction;
                    match action {
                        MenuAction::Continue => {
                            renderer.menu_display.hide();
                            println!("🎮 Menu: Closed - Continuing game");
                        }
                        MenuAction::Settings => {
                            println!("⚙️  Settings: Not yet implemented");
                        }
                        MenuAction::Save => {
                            println!("💾 Save Game: Not yet implemented");
                        }
                        MenuAction::Load => {
                            println!("📂 Load Game: Not yet implemented");
                        }
                        MenuAction::ExitToMainMenu => {
                            println!("🏠 Exit to Main Menu: Not yet implemented");
                        }
                        MenuAction::ExitGame => {
                            println!("👋 Exit Game: Not yet implemented");
                            // TODO: Implement proper exit
                        }
                    }
                    return; // Don't process other clicks when menu is active
                }
                // If menu is active but no button was clicked, ignore the click
                return;
            }
        }

        // Priority 2: Check if clicking on UI buttons
        if let Some(ui_panel) = &self.ui_panel {
            // Check pickup prompt buttons
            if self.pickup_prompt.is_some() {
                if ui_panel.check_yes_button_click(x as f32, y as f32) {
                    // Handle "Pick Up" button click
                    if let Some(prompt) = self.pickup_prompt.take() {
                        self.handle_item_pickup(prompt.unit_id, prompt.item_id);
                        // Clear UI prompt
                        if let Some(ui_panel) = &mut self.ui_panel {
                            ui_panel.clear_pickup_prompt();
                        }
                    }
                    return; // Don't process hex click
                } else if ui_panel.check_no_button_click(x as f32, y as f32) {
                    // Handle "Leave It" button click
                    if let Some(prompt) = self.pickup_prompt.take() {
                        println!("❌ Declined to pick up '{}'", prompt.item_name);
                        // Clear UI prompt
                        if let Some(ui_panel) = &mut self.ui_panel {
                            ui_panel.clear_pickup_prompt();
                        }
                    }
                    return; // Don't process hex click
                }
            }
        }

        // Process hex clicks if no button was clicked
        if let Some(hex_coord) = self.screen_to_hex_coord(x, y) {
            if let Some(unit_id) = self.selected_unit {
                // STATE 1: Unit is already selected
                if self.movement_range.contains(&hex_coord) {
                    // Valid move - execute movement
                    if let Err(e) = self.game_world.move_unit(unit_id, hex_coord) {
                        println!("Failed to move unit: {}", e);
                    }

                    // Check if there's an item at the destination
                    if let Some(item_obj_id) = self.find_item_at_hex(hex_coord) {
                        if let Some(item_obj) =
                            self.game_world.interactive_objects.get(&item_obj_id)
                        {
                            if item_obj.can_interact() {
                                let item_name = item_obj.name().to_string();
                                // Show pickup prompt
                                self.pickup_prompt = Some(PickupPrompt {
                                    unit_id,
                                    item_id: item_obj_id,
                                    item_name: item_name.clone(),
                                });

                                // Show prompt in UI panel
                                if let Some(ui_panel) = &mut self.ui_panel {
                                    ui_panel.set_pickup_prompt(item_name);
                                }

                                println!("📦 Item found! Click 'Pick Up' button or press 'Y' to pick up.");
                            }
                        }
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

    fn find_item_at_hex(&self, hex_coord: HexCoord) -> Option<uuid::Uuid> {
        // Find if there's an interactive object (item) at the given hex coordinate
        for (id, obj) in &self.game_world.interactive_objects {
            if obj.position() == hex_coord {
                return Some(*id);
            }
        }
        None
    }

    fn handle_item_pickup(&mut self, unit_id: uuid::Uuid, item_id: uuid::Uuid) {
        // Get the item from the interactive object
        if let Some(item_obj) = self.game_world.interactive_objects.get_mut(&item_id) {
            if let Some(item) = item_obj.take_item() {
                // Add item to unit's inventory
                if let Some(game_unit) = self.game_world.units.get_mut(&unit_id) {
                    let item_name = item.name.clone();
                    game_unit.unit_mut().add_item_to_inventory(item);
                    println!("✅ Picked up '{}'!", item_name);

                    // Remove the interactive object from the world (it's been picked up)
                    self.game_world.remove_interactive_object(item_id);
                } else {
                    println!("⚠️  Unit not found!");
                }
            } else {
                println!("⚠️  Item no longer available!");
            }
        } else {
            println!("⚠️  Item object not found!");
        }
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
                let stats = unit.combat_stats();
                let display_info = UnitDisplayInfo {
                    name: unit.name().to_string(),
                    race: format!("{:?}", unit.race()),
                    class: format!("{:?}", unit.class()),
                    level: unit.level(),
                    experience: unit.experience(),
                    health: stats.health as u32,
                    max_health: stats.max_health as u32,
                    terrain: format!("{:?}", unit.current_terrain()),
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
        // Clear existing unit and item sprites (keep terrain)
        for hex in self.hex_grid.hexagons.values_mut() {
            hex.set_unit_sprite(None);
            hex.set_item_sprite(None);
        }

        // Add current unit positions as unit sprites on top of terrain
        for unit in self.game_world.units.values() {
            let pos = unit.position();
            self.hex_grid.set_unit_at(pos, SpriteType::Unit);
        }

        // Add items on the ground
        for item_obj in self.game_world.interactive_objects.values() {
            let pos = item_obj.position();
            self.hex_grid.set_item_at(pos, SpriteType::Item);
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
                println!();
                println!("=== DEMO UNITS ===");
                println!("⚔️  Thorin (Human Warrior) at (0,0) - Player Unit");
                println!("👹 Orc Grunt (Orc Warrior) at (4,2) - Enemy Unit");
                println!();
                println!("🎁 Item: Iron Sword at (1,1) - available for pickup!");
                println!();
                println!("=== CONTROLS ===");
                println!("🖱️  RIGHT-CLICK on a unit to select it and show movement range");
                println!("🖱️  LEFT-CLICK on blue hexes to move the selected unit");
                println!("⌨️  Arrow Keys - Move camera");
                println!();
                println!("=== HOTKEYS ===");
                println!("🔤 C - Show detailed unit info in console");
                println!("🔤 H - Toggle hover debug mode");
                println!("🔤 G - Toggle guide display on/off");
                println!("🔤 I - Show info for selected unit");
                println!("🔤 ESC - Deselect unit / Close guide / Toggle menu");
                println!();
                println!("=== GUIDE ENCYCLOPEDIA (F-Keys) ===");
                println!("📚 F1 - Combat System Guide");
                println!("📚 F2 - Movement System Guide");
                println!("� F3 - Character Classes Guide");
                println!("📚 F4 - Character Races Guide");
                println!("📚 F5 - Equipment System Guide");
                println!("📚 F6 - Terrain Types Guide");
                println!();
                println!("🔍 Hover Debug: Currently ENABLED - cursor highlights hexes in yellow");
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

                // Update menu button hover states
                if let Some(renderer) = &mut self.renderer {
                    if renderer.menu_display.active {
                        renderer
                            .menu_display
                            .update_hover(position.x as f32, position.y as f32);
                    }
                }

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
                            // Priority 1: Check if guide is open, close it
                            let mut handled = false;
                            if let Some(renderer) = &mut self.renderer {
                                if renderer.guide_display.active {
                                    renderer.guide_display.hide();
                                    println!("📚 Guide: Closed");
                                    handled = true;
                                }
                            }

                            // Priority 2: Toggle menu (only if guide wasn't closed)
                            if !handled {
                                if let Some(renderer) = &mut self.renderer {
                                    renderer.menu_display.toggle();
                                    if renderer.menu_display.active {
                                        println!("🎮 Menu: Opened (Press ESC again to close)");
                                    } else {
                                        println!("🎮 Menu: Closed");
                                    }
                                    handled = true;
                                }
                            }

                            // Priority 3: Clear unit selection (only if nothing else was handled)
                            if !handled {
                                self.clear_selection();
                            }
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
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyY) => {
                            // Accept item pickup
                            if let Some(prompt) = self.pickup_prompt.take() {
                                self.handle_item_pickup(prompt.unit_id, prompt.item_id);
                                // Clear UI prompt
                                if let Some(ui_panel) = &mut self.ui_panel {
                                    ui_panel.clear_pickup_prompt();
                                }
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyN) => {
                            // Decline item pickup
                            if let Some(prompt) = self.pickup_prompt.take() {
                                println!("❌ Declined to pick up '{}'", prompt.item_name);
                                // Clear UI prompt
                                if let Some(ui_panel) = &mut self.ui_panel {
                                    ui_panel.clear_pickup_prompt();
                                }
                            }
                        }
                        // Guide/Encyclopedia Hotkeys (F1-F6)
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F1) => {
                            // Show combat system guide
                            if let Some(renderer) = &mut self.renderer {
                                let guide = GuideLibrary::combat_system();
                                renderer.guide_display.show(guide);
                                println!("📚 Guide: Combat System");
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F2) => {
                            // Show movement system guide
                            if let Some(renderer) = &mut self.renderer {
                                let guide = GuideLibrary::movement_system();
                                renderer.guide_display.show(guide);
                                println!("📚 Guide: Movement System");
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F3) => {
                            // Show character classes guide
                            if let Some(renderer) = &mut self.renderer {
                                let guide = GuideLibrary::character_classes();
                                renderer.guide_display.show(guide);
                                println!("📚 Guide: Character Classes");
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F4) => {
                            // Show character races guide
                            if let Some(renderer) = &mut self.renderer {
                                let guide = GuideLibrary::character_races();
                                renderer.guide_display.show(guide);
                                println!("📚 Guide: Character Races");
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F5) => {
                            // Show equipment system guide
                            if let Some(renderer) = &mut self.renderer {
                                let guide = GuideLibrary::equipment_system();
                                renderer.guide_display.show(guide);
                                println!("📚 Guide: Equipment System");
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F6) => {
                            // Show terrain types guide
                            if let Some(renderer) = &mut self.renderer {
                                let guide = GuideLibrary::terrain_types();
                                renderer.guide_display.show(guide);
                                println!("📚 Guide: Terrain Types");
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyG) => {
                            // Toggle guide display (hide if already showing)
                            if let Some(renderer) = &mut self.renderer {
                                renderer.guide_display.toggle();
                                if renderer.guide_display.active {
                                    println!("📚 Guide: Shown");
                                } else {
                                    println!("📚 Guide: Hidden");
                                }
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyI) => {
                            // Show info for selected unit (class-specific guide)
                            if let Some(unit_id) = self.selected_unit {
                                if let Some(game_unit) = self.game_world.units.get(&unit_id) {
                                    let class_name =
                                        format!("{:?}", game_unit.unit().class()).to_lowercase();
                                    if let Some(renderer) = &mut self.renderer {
                                        let guide = GuideLibrary::unit_class_guide(&class_name);
                                        renderer.guide_display.show(guide);
                                        println!("📚 Guide: {} Info", class_name);
                                    }
                                }
                            } else {
                                println!("❌ No unit selected. Select a unit first!");
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
