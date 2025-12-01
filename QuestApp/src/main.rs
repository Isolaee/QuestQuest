//! # QuestQuest Interactive Game Application
//!
//! This is the main interactive game application that brings together the graphics,
//! units, combat, and game systems into a playable game with a windowed interface.
//!
//! ## Architecture
//!
//! QuestApp follows a clear separation of concerns:
//!
//! - **Game/ScenarioWorld**: Core game state, turn management, world state queries
//! - **Combat crate**: Combat resolution logic (damage calculation, hit chances)
//! - **AI crate**: Enemy AI planning and decision-making (non-player logic)
//! - **QuestApp**: Player controls, scene management, UI rendering, and event handling
//!
//! This module is responsible for:
//! - Handling player input (keyboard, mouse)
//! - Managing game scenes (main menu, game, etc.)
//! - Rendering graphics (terrain, units, UI panels)
//! - Displaying combat confirmation dialogs (execution delegated to ScenarioWorld)
//! - Synchronizing visual state with game state
//!
//! **NOT responsible for**:
//! - Combat logic (handled by Combat crate via ScenarioWorld)
//! - AI decision making (handled by AI crate via ScenarioWorld)
//! - Game rules and turn management (handled by ScenarioWorld)
//!
//! ## Features
//!
//! - **Real-time rendering** with OpenGL 4.x
//! - **Hexagonal grid** with terrain and unit visualization
//! - **Interactive unit selection** and movement
//! - **Combat system** with attack selection and confirmation
//! - **Item pickup** and inventory management
//! - **Camera controls** for navigating the game world
//! - **UI panels** for unit information and prompts
//! - **In-game menu** for settings and game management
//!
//! ## Controls
//!
//! - **Mouse Hover**: View unit details in UI panel (automatic)
//! - **Left Click**: Move selected unit, confirm actions, interact with UI
//! - **Right Click**: Select unit, cancel actions
//! - **Arrow Keys**: Move camera (scroll encyclopedia when open)
//! - **E**: Toggle encyclopedia wiki
//! - **1/2/3**: Switch encyclopedia categories (Units/Terrain/Mechanics) when open
//! - **C**: Show detailed unit info in console
//! - **H**: Toggle hover debug mode (hex highlighting)
//! - **SPACE**: End turn
//! - **ESC**: Close encyclopedia/menu, deselect unit
//!
//! ## Architecture
//!
//! The application uses the winit event loop with glutin for OpenGL context management.
//! The main [`GameApp`] structure manages the game state, rendering, and event handling.

mod encyclopedia_builder;
mod game_scene;
mod main_menu;
mod scene_manager;

// Import the new game scene state management
use game_scene::{GameSceneState, GameState};

use crate::encyclopedia_builder::EncyclopediaLibrary;
use encyclopedia::Encyclopedia;
use game::scenario_instance::ScenarioWorld;
use game::*;
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use graphics::core::hexagon::SpriteType;
use graphics::math::Vec2;
use graphics::{
    find_path, setup_dynamic_hexagons, AttackDisplayInfo, EncyclopediaCategory, EncyclopediaPanel,
    HexCoord, HexGrid, HighlightType, Renderer, UiPanel, UnitAnimation, UnitDisplayInfo,
};

use main_menu::MainMenuScene;
use raw_window_handle::HasWindowHandle;
use scene_manager::{Scene, SceneManager, SceneType};
use std::ffi::CString;
use units::Terrain;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

/// Temporary helper to convert SpriteType to Terrain until proper terrain storage is implemented
fn sprite_to_terrain(sprite: SpriteType) -> Terrain {
    match sprite {
        SpriteType::Forest | SpriteType::Forest2 => Terrain::Forest0,
        SpriteType::Grasslands => Terrain::Grasslands,
        SpriteType::HauntedWoods => Terrain::HauntedWoods,
        SpriteType::Hills => Terrain::Hills,
        SpriteType::Mountain => Terrain::Mountain,
        SpriteType::Swamp => Terrain::Swamp,
        _ => Terrain::Grasslands, // Default fallback
    }
}

/// Screen width in pixels.
const SCREEN_WIDTH: f32 = 1920.0;

/// Screen height in pixels.
const SCREEN_HEIGHT: f32 = 1080.0;

/// Main game application structure.
///
/// `GameApp` is the presentation layer that handles user interaction, rendering,
/// and scene management. The game logic, combat resolution, and AI planning are
/// delegated to `ScenarioWorld`, `Combat` crate, and `AI` crate respectively.
///
/// # Architecture Responsibilities
///
/// - **Input Handling**: Processes keyboard and mouse events
/// - **Scene Management**: Transitions between menu, game, and other scenes
/// - **Rendering**: Displays terrain, units, UI panels, and effects
/// - **State Synchronization**: Keeps visual state in sync with ScenarioWorld
///
/// # Fields
///
/// - `window`: The OS window for rendering
/// - `gl_context`: OpenGL rendering context
/// - `gl_surface`: OpenGL surface for the window
/// - `hex_grid`: The hexagonal grid system
/// - `renderer`: Multi-layer renderer for terrain, units, and items
/// - `game_world`: ScenarioWorld instance (game state, delegated logic)
/// - `ui_panel`: UI overlay for prompts and unit information (updates on hover)
/// - `game_world`: Game state including units and objects
/// - `selected_unit`: Currently selected unit (if any)
/// - `show_unit_info`: Whether to display detailed unit info
/// - `unit_info_text`: Cached unit information text
/// - `cursor_position`: Current mouse cursor position (actively tracked)
/// - `movement_range`: Valid movement hexes for selected unit
/// - `hower_debug_hex`: Hex currently under cursor (debug mode)
/// - `hower_debug_enabled`: Whether hover debug mode is active (toggle with 'H')
/// - `pickup_prompt`: Active item pickup prompt (if any)
///
/// # Example Usage
///
/// ```rust,no_run
/// use winit::event_loop::EventLoop;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create event loop and game app
/// let event_loop = EventLoop::new()?;
/// let mut app = GameApp::new();
///
/// // Run the event loop
/// event_loop.run_app(&mut app)?;
/// # Ok(())
/// # }
/// ```
/// Maps a unit's race to the appropriate sprite type for UI display.
///
/// # Arguments
///
struct GameApp {
    window: Option<Window>,
    gl_context: Option<glutin::context::PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    hex_grid: HexGrid,
    renderer: Option<Renderer>,
    ui_panel: Option<UiPanel>,
    game_world: ScenarioWorld,
    show_unit_info: bool,
    unit_info_text: Vec<String>,
    cursor_position: (f64, f64),       // Track cursor position for clicks
    hower_debug_hex: Option<HexCoord>, // Debug: hex under cursor
    hower_debug_enabled: bool,         // Toggle for hower debug mode

    // Scene management
    scene_manager: SceneManager,
    main_menu_scene: MainMenuScene,
    game_initialized: bool, // Track if game scene has been initialized
    exit_requested: bool,   // Flag to request application exit

    // Turn system tracking
    last_update_time: std::time::Instant, // Track time for delta calculations
    last_ai_turn_team: Option<game::Team>, // Track which team last had AI execute

    // Movement animation
    active_animation: Option<UnitAnimation>, // Currently animating unit

    // Encyclopedia system
    encyclopedia: Encyclopedia,                    // Encyclopedia data
    encyclopedia_panel: Option<EncyclopediaPanel>, // Encyclopedia UI panel

    // Game state management (replaces scattered state variables)
    game_state: GameSceneState,
}

/// Item pickup prompt state.
///
/// Stores information about a pending item pickup action, including
/// which unit is picking up which item.
#[derive(Clone)]
struct PickupPrompt {
    /// UUID of the unit picking up the item
    unit_id: uuid::Uuid,
    /// UUID of the item to be picked up
    item_id: uuid::Uuid,
    /// Display name of the item
    item_name: String,
}

impl GameApp {
    /// Creates a new game application loading map from Maps/test_map.json.
    ///
    /// Initializes the game world with terrain and any units/items defined in the map.
    ///
    /// # Returns
    ///
    /// A new `GameApp` instance ready to be initialized with a window.
    fn new() -> Self {
        // Load map JSON from file
        let map_json = std::fs::read_to_string("Maps/test_map.json")
            .expect("Failed to read Maps/test_map.json");

        // Initialize game world (ScenarioWorld handles game logic)
        let game_world = ScenarioWorld::new(map_json);

        // Architecture note: ScenarioWorld is the single source of truth for:
        // - Unit positions, stats, and inventory
        // - Turn management and team control
        // - Combat initiation and resolution (via Combat crate)
        // - AI execution (via AI crate)
        // QuestApp reads from ScenarioWorld for rendering and writes player actions to it

        Self {
            window: None,
            gl_context: None,
            gl_surface: None,
            hex_grid: HexGrid::new(),
            renderer: None,
            ui_panel: None,
            game_world,
            show_unit_info: false,
            unit_info_text: Vec::new(),
            cursor_position: (0.0, 0.0),
            hower_debug_hex: None,
            hower_debug_enabled: true, // Start with debug enabled

            // Scene management - start at main menu
            scene_manager: SceneManager::new(),
            main_menu_scene: MainMenuScene::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            game_initialized: false,
            exit_requested: false,

            // Turn system tracking
            last_update_time: std::time::Instant::now(),
            last_ai_turn_team: None,

            // Movement animation
            active_animation: None,

            // Encyclopedia system
            encyclopedia: Encyclopedia::new(),
            encyclopedia_panel: None,

            // Game state management
            game_state: GameSceneState::new(),
        }
    }

    /// Initialize the game scene - called when transitioning from menu to game
    fn initialize_game_scene(&mut self) {
        println!("🎮 Initializing Game Scene...");

        // Hex grid is already initialized with terrain in new(), just update units
        self.update_hex_grid_units();

        // Start the turn-based game
        self.game_world.start_turn_based_game();

        println!("✅ Game scene initialized!");
    }

    // ===== State Access Helpers =====
    // These provide backward-compatible access to state managed by GameSceneState

    /// Get the currently selected unit ID (if any)
    fn selected_unit(&self) -> Option<uuid::Uuid> {
        self.game_state.exploring.selected_unit()
    }

    /// Check if encyclopedia is visible
    fn encyclopedia_visible(&self) -> bool {
        self.game_state.encyclopedia.is_visible()
    }

    /// Set encyclopedia visibility
    fn set_encyclopedia_visible(&mut self, visible: bool) {
        if visible {
            self.game_state.encyclopedia.visible = true;
            self.game_state.transition_to(GameState::Encyclopedia);
        } else {
            self.game_state.encyclopedia.visible = false;
            self.game_state.transition_to(GameState::Exploring);
        }
    }

    /// Get pickup prompt data from state (if in ItemPickup state)
    fn pickup_prompt(&self) -> Option<PickupPrompt> {
        if let GameState::ItemPickup {
            unit_id,
            item_id,
            item_name,
        } = &self.game_state.current_state
        {
            Some(PickupPrompt {
                unit_id: *unit_id,
                item_id: *item_id,
                item_name: item_name.clone(),
            })
        } else {
            None
        }
    }

    /// Take (consume and return) the pickup prompt, transitioning state to Exploring
    fn take_pickup_prompt(&mut self) -> Option<PickupPrompt> {
        let prompt = self.pickup_prompt();
        if prompt.is_some() {
            self.game_state.transition_to(GameState::Exploring);
        }
        prompt
    }

    /// Start animating unit movement along a path
    fn start_movement_animation(&mut self, unit_id: uuid::Uuid, path: Vec<HexCoord>) {
        if path.len() < 2 {
            return; // Nothing to animate
        }

        // Movement speed in hexes per second
        const MOVEMENT_SPEED: f32 = 4.0; // Adjust this value to change animation speed

        self.active_animation = Some(UnitAnimation::new(unit_id, path, MOVEMENT_SPEED));
    }

    /// Update animation state and move unit along path
    fn update_animation(&mut self, delta_time: f32) {
        if let Some(mut anim) = self.active_animation.take() {
            // Update animation and get hexes that were stepped through
            let stepped_hexes = anim.update(delta_time);

            // Update visual position for each stepped hex
            for hex in stepped_hexes {
                if let Some(game_unit) = self.game_world.units.get_mut(&anim.unit_id()) {
                    game_unit.set_position(hex);
                }
                self.update_hex_grid_units();
            }

            // Check if animation is complete
            if anim.is_complete() {
                // Animation finished - ensure final position is set
                let final_pos = anim.destination();
                if let Some(game_unit) = self.game_world.units.get_mut(&anim.unit_id()) {
                    game_unit.set_position(final_pos);
                }
                self.update_hex_grid_units();
                println!("✅ Unit movement animation complete");
            } else {
                // Continue animation
                self.active_animation = Some(anim);
            }
        }
    }

    /// Handle keyboard input for the game scene
    fn handle_game_keyboard_input(&mut self, physical_key: winit::keyboard::PhysicalKey) {
        let move_speed = 0.1;
        match physical_key {
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowUp) => {
                if self.encyclopedia_visible() {
                    // Scroll encyclopedia up
                    if let Some(panel) = &mut self.encyclopedia_panel {
                        panel.scroll_up(3);
                    }
                } else {
                    self.hex_grid.move_camera(0.0, move_speed);
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowDown) => {
                if self.encyclopedia_visible() {
                    // Scroll encyclopedia down
                    if let Some(panel) = &mut self.encyclopedia_panel {
                        panel.scroll_down(3);
                    }
                } else {
                    self.hex_grid.move_camera(0.0, -move_speed);
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft) => {
                if !self.encyclopedia_visible() {
                    self.hex_grid.move_camera(-move_speed, 0.0);
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowRight) => {
                if !self.encyclopedia_visible() {
                    self.hex_grid.move_camera(move_speed, 0.0);
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit1) => {
                if self.encyclopedia_visible() {
                    if let Some(panel) = &mut self.encyclopedia_panel {
                        panel.set_category(EncyclopediaCategory::Units);
                        self.update_encyclopedia_content();
                    }
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit2) => {
                if self.encyclopedia_visible() {
                    if let Some(panel) = &mut self.encyclopedia_panel {
                        panel.set_category(EncyclopediaCategory::Terrain);
                        self.update_encyclopedia_content();
                    }
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit3) => {
                if self.encyclopedia_visible() {
                    if let Some(panel) = &mut self.encyclopedia_panel {
                        panel.set_category(EncyclopediaCategory::Mechanics);
                        self.update_encyclopedia_content();
                    }
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyC) => {
                // Show detailed unit info in console
                if let Some(unit_id) = self.selected_unit() {
                    self.call_unit_on_click(unit_id);
                } else {
                    println!("No unit selected. Click on a unit first!");
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Space) => {
                // End current turn (only works if it's a player turn)
                if self.game_world.is_current_team_player_controlled() {
                    self.game_world.end_current_turn();
                    self.clear_selection(); // Clear any unit selection when turn ends
                } else {
                    println!("⚠️  Cannot end turn - not your turn!");
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) => {
                // Priority 1: Check if encyclopedia is open, close it
                let mut handled = false;
                if self.encyclopedia_visible() {
                    self.set_encyclopedia_visible(false);
                    println!("📚 Encyclopedia: Closed");
                    handled = true;
                }

                // Priority 2: Check if guide is open, close it (now handled by encyclopedia_panel)
                // No-op: All guide/encyclopedia display is now handled by encyclopedia_panel

                // Priority 3: Toggle menu (only if guide/encyclopedia wasn't closed)
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

                // Priority 4: Clear unit selection (only if nothing else was handled)
                if !handled {
                    self.clear_selection();
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyE) => {
                // Toggle encyclopedia
                let new_visible = !self.encyclopedia_visible();
                self.set_encyclopedia_visible(new_visible);
                if new_visible {
                    println!("📚 Encyclopedia: Opened (Press E or ESC to close)");
                    // Update content based on current category
                    self.update_encyclopedia_content();
                } else {
                    println!("📚 Encyclopedia: Closed");
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
                                0.3 + 0.4 * ((prev_hex.q + prev_hex.r) % 3) as f32 / 3.0,
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
                if let Some(prompt) = self.take_pickup_prompt() {
                    self.handle_item_pickup(prompt.unit_id, prompt.item_id);
                    // Clear UI prompt
                    if let Some(ui_panel) = &mut self.ui_panel {
                        ui_panel.clear_pickup_prompt();
                    }
                }
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyN) => {
                // Decline item pickup
                if let Some(prompt) = self.take_pickup_prompt() {
                    println!("❌ Declined to pick up '{}'", prompt.item_name);
                    // Clear UI prompt
                    if let Some(ui_panel) = &mut self.ui_panel {
                        ui_panel.clear_pickup_prompt();
                    }
                }
            }
            // (F1-F6 static encyclopedia hotkeys removed; only dynamic/automatic encyclopedia remains)
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyG) => {
                // Toggle encyclopedia panel visibility
                let new_visible = !self.encyclopedia_visible();
                self.set_encyclopedia_visible(new_visible);
                println!(
                    "📚 Encyclopedia: {}",
                    if new_visible { "Shown" } else { "Hidden" }
                );
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyI) => {
                // Show info for selected unit (class-specific encyclopedia entry)
                if let Some(unit_id) = self.selected_unit() {
                    if let Some(game_unit) = self.game_world.units.get(&unit_id) {
                        let class_name = game_unit.unit().unit_type().to_lowercase();
                        let entry = EncyclopediaLibrary::unit_class_entry(&class_name);
                        self.set_encyclopedia_visible(true);
                        if let Some(panel) = &mut self.encyclopedia_panel {
                            if let encyclopedia::EncyclopediaEntry::Mechanic(mech) = entry {
                                let mut lines = vec![format!("# {}", mech.title)];
                                for desc in mech.description.split('\n') {
                                    lines.push(desc.to_string());
                                }
                                if !mech.details.is_empty() {
                                    lines.push(String::new());
                                    for d in &mech.details {
                                        lines.push(d.clone());
                                    }
                                }
                                panel.update_content(lines);
                            } else {
                                panel.update_content(vec!["[Not a class guide entry]".to_string()]);
                            }
                        }
                        println!("📚 Encyclopedia: {} Info", class_name);
                    }
                } else {
                    println!("❌ No unit selected. Select a unit first!");
                }
            }
            _ => {}
        }
    }

    /// Handles left mouse button clicks.
    ///
    /// Processes clicks in priority order:
    /// 1. Combat confirmation dialog (if active)
    /// 2. Menu buttons (if menu is open)
    /// 3. UI panel buttons (pickup prompts, etc.)
    /// 4. Unit movement (if unit is selected and clicking on valid hex)
    /// 5. Hex selection for combat or interaction
    ///
    /// # Arguments
    ///
    /// * `x` - Screen X coordinate of the click
    /// * `y` - Screen Y coordinate of the click
    fn handle_left_click(&mut self, x: f64, y: f64) {
        // Priority 0: Check if clicking on combat confirmation dialog (highest priority)
        if self.has_pending_combat() {
            if let Some(renderer) = &mut self.renderer {
                // First check if clicking on an attack option
                println!("🖱️  Click at ({}, {}) - checking attack buttons", x, y);
                if let Some(idx) = renderer
                    .combat_log_display
                    .check_attack_click(x as f32, y as f32)
                {
                    // Attack option was clicked and selected - don't do anything else
                    println!("✅ Attack {} selected, not executing combat yet", idx);
                    return;
                }

                if let Some(confirmed) =
                    renderer.combat_log_display.handle_click(x as f32, y as f32)
                {
                    if confirmed {
                        // Update the selected attack index in pending combat before executing
                        let selected_idx = renderer.combat_log_display.get_selected_attack();
                        println!("🔍 Selected attack from UI: {:?}", selected_idx);

                        if let Some(idx) = selected_idx {
                            if let Some(pending) = &mut self.game_world.pending_combat {
                                println!(
                                    "🔍 Before update: pending.selected_attack_index = {}",
                                    pending.selected_attack_index
                                );
                                pending.selected_attack_index = idx;
                                println!(
                                    "🔍 After update: pending.selected_attack_index = {}",
                                    pending.selected_attack_index
                                );
                                println!("⚔️  Executing combat with attack index: {}", idx);
                            }
                        }

                        // Delegate combat execution to ScenarioWorld
                        // (ScenarioWorld uses Combat crate for resolution logic)
                        if let Err(e) = self.game_world.execute_pending_combat() {
                            println!("❌ Combat failed: {}", e);
                        }
                        renderer.combat_log_display.clear_combat_confirmation();
                        self.clear_selection();
                    } else {
                        // Cancel combat
                        self.game_world.cancel_pending_combat();
                        println!("❌ Combat cancelled");
                    }
                    return;
                }
            }
            // If combat dialog is active but no button was clicked, ignore other clicks
            return;
        }

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
                            println!("🏠 Exiting to Main Menu...");
                            // Hide the menu first
                            renderer.menu_display.hide();
                            // Transition back to main menu
                            self.scene_manager.transition_to(SceneType::MainMenu);
                        }
                        MenuAction::ExitGame => {
                            println!("👋 Exiting game...");
                            self.exit_requested = true;
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
            // Check end turn button
            if ui_panel.check_end_turn_button_click(x as f32, y as f32) {
                if self.game_world.is_current_team_player_controlled() {
                    self.game_world.end_current_turn();
                    self.clear_selection();
                    println!("⏭️  Turn ended via UI button");
                } else {
                    println!("⚠️  Cannot end turn - not your turn!");
                }
                return; // Don't process hex click
            }

            // Check pickup prompt buttons
            if self.pickup_prompt().is_some() {
                if ui_panel.check_yes_button_click(x as f32, y as f32) {
                    // Handle "Pick Up" button click
                    if let Some(prompt) = self.take_pickup_prompt() {
                        self.handle_item_pickup(prompt.unit_id, prompt.item_id);
                        // Clear UI prompt
                        if let Some(ui_panel) = &mut self.ui_panel {
                            ui_panel.clear_pickup_prompt();
                        }
                    }
                    return; // Don't process hex click
                } else if ui_panel.check_no_button_click(x as f32, y as f32) {
                    // Handle "Leave It" button click
                    if let Some(prompt) = self.take_pickup_prompt() {
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

        // --- Player Control Flow: select if none, else move/attack ---
        if let Some(hex_coord) = self.screen_to_hex_coord(x, y) {
            if self.selected_unit().is_none() {
                // No unit selected: try to select one
                if let Some(unit_id) = self.find_unit_at_hex(hex_coord) {
                    self.select_unit(unit_id);
                    // (Optional: print message)
                }
                // If no unit, do nothing
            } else {
                // Unit is already selected: run original movement/combat logic
                let unit_id = self.selected_unit().unwrap();
                // --- Begin original logic ---
                // Check if clicking on an enemy within attack range
                if self.is_within_attack_range(unit_id, hex_coord)
                    && self.has_enemy_unit(unit_id, hex_coord)
                {
                    // Enemy in range - delegate combat request to ScenarioWorld
                    // ScenarioWorld creates PendingCombat which QuestApp displays for confirmation
                    if let Err(e) = self.game_world.move_unit(unit_id, hex_coord) {
                        println!("Failed to initiate combat: {}", e);
                    } else {
                        // ScenarioWorld created PendingCombat - display confirmation UI
                        // (Combat execution is deferred until player confirms)
                        if let Some(pending) = &self.game_world.pending_combat {
                            if let Some(renderer) = &mut self.renderer {
                                use graphics::{AttackOption, CombatConfirmation};

                                // Convert game AttackInfo to graphics AttackOption
                                let attacker_attacks = pending
                                    .attacker_attacks
                                    .iter()
                                    .map(|attack| AttackOption {
                                        name: attack.name.clone(),
                                        damage: attack.damage,
                                        range: attack.range,
                                    })
                                    .collect();

                                let defender_attacks = pending
                                    .defender_attacks
                                    .iter()
                                    .map(|attack| AttackOption {
                                        name: attack.name.clone(),
                                        damage: attack.damage,
                                        range: attack.range,
                                    })
                                    .collect();

                                let confirmation = CombatConfirmation {
                                    attacker_name: pending.attacker_name.clone(),
                                    attacker_hp: pending.attacker_hp,
                                    attacker_max_hp: pending.attacker_max_hp,
                                    attacker_attack: pending.attacker_attack,
                                    attacker_defense: pending.attacker_defense,
                                    attacker_attacks_per_round: pending.attacker_attacks_per_round,
                                    attacker_attacks,
                                    defender_name: pending.defender_name.clone(),
                                    defender_hp: pending.defender_hp,
                                    defender_max_hp: pending.defender_max_hp,
                                    defender_attack: pending.defender_attack,
                                    defender_defense: pending.defender_defense,
                                    defender_attacks_per_round: pending.defender_attacks_per_round,
                                    defender_attacks,
                                };
                                renderer
                                    .combat_log_display
                                    .show_combat_confirmation(confirmation);
                                println!(
                                    "⚔️  Combat requested! Click OK to confirm or Cancel to abort."
                                );
                            }
                        }
                    }
                } else if self
                    .game_state
                    .exploring
                    .movement_range()
                    .contains(&hex_coord)
                {
                    // Valid move - animate movement (non-combat)
                    if let Some(game_unit) = self.game_world.units.get(&unit_id) {
                        let start_pos = game_unit.position();

                        // Find path from current position to target using graphics::find_path
                        if let Some(path) = find_path(start_pos, hex_coord) {
                            // Consume movement points
                            let move_cost = start_pos.distance(hex_coord);
                            if self
                                .game_world
                                .units
                                .get_mut(&unit_id)
                                .unwrap()
                                .consume_moves(move_cost)
                            {
                                // Start animation
                                self.start_movement_animation(unit_id, path);
                                println!("🚶 Unit moving to {:?}", hex_coord);
                            } else {
                                println!("❌ Not enough movement points");
                            }
                        } else {
                            println!("❌ No path found to target hex");
                        }
                    }

                    // Check if there's an item at the destination
                    if let Some(item_obj_id) = self.find_item_at_hex(hex_coord) {
                        if let Some(item_obj) =
                            self.game_world.interactive_objects.get(&item_obj_id)
                        {
                            if item_obj.can_interact() {
                                let item_name = item_obj.name().to_string();
                                // Transition to pickup state
                                self.game_state.transition_to(GameState::ItemPickup {
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
                // --- End original logic ---
            }
        }
    }

    /// Handles right mouse button clicks.
    ///
    /// Right-clicking on a unit selects it and displays its movement range.
    /// Right-clicking on empty space deselects the current unit.
    ///
    /// # Arguments
    ///
    /// * `x` - Screen X coordinate of the click
    /// * `y` - Screen Y coordinate of the click
    fn handle_right_click(&mut self, x: f64, y: f64) {
        // --- Player Control Flow: cancel selection or open menu placeholder ---
        if self.selected_unit().is_some() {
            // Cancel selection
            self.clear_selection();
            println!("[Right Click] Selection cancelled.");
        } else if let Some(hex_coord) = self.screen_to_hex_coord(x, y) {
            // No unit selected: open unit menu if unit present (placeholder)
            if let Some(unit_id) = self.find_unit_at_hex(hex_coord) {
                println!(
                    "[Right Click] Would open unit menu for unit {:?} (feature not implemented)",
                    unit_id
                );
            } else {
                // Do nothing if no unit at hex
            }
        }
    }

    /// Selects a unit and displays its movement range.
    ///
    /// Queries ScenarioWorld for legal moves and updates UI state to show
    /// unit information and movement range highlights.
    ///
    /// # Architecture
    ///
    /// - Queries movement range from ScenarioWorld (game logic)
    /// - Updates GameSceneState (UI state)
    /// - Triggers rendering updates (presentation)
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to select
    fn select_unit(&mut self, unit_id: uuid::Uuid) {
        // Update UI state to track selected unit
        self.game_state.exploring.set_selected_unit(Some(unit_id));

        // Query ScenarioWorld for legal moves (game logic)
        let legal_moves = self.game_world.all_legal_moves(unit_id);
        let movement_coords: Vec<HexCoord> = legal_moves
            .into_iter()
            .map(|(coord, _cost)| coord)
            .collect();

        // Update UI state with movement range
        self.game_state
            .exploring
            .set_movement_range(movement_coords);

        // Update display
        self.show_unit_info = true;
        self.update_unit_info_display(unit_id);
        self.update_highlight_display();

        println!(
            "Unit selected: {:?} with {} valid movement options",
            unit_id,
            self.game_state.exploring.movement_range().len()
        );

        // Show detailed info in console
        self.call_unit_on_click(unit_id);
    }

    /// Clears the current unit selection and related UI state.
    ///
    /// Resets:
    /// - Selected unit
    /// - Unit info display
    /// - Movement range highlights
    /// - Pickup prompts
    fn clear_selection(&mut self) {
        self.game_state.exploring.deselect_unit();
        self.show_unit_info = false;
        self.unit_info_text.clear();

        // Clear all highlights
        self.hex_grid.clear_all_highlights();

        // Clear all text overlays
        self.hex_grid.clear_all_text_overlays();

        // Clear UI panel
        if let Some(ui_panel) = &mut self.ui_panel {
            ui_panel.clear_unit_info();
        }
    }

    /// Checks if a hex is within the unit's attack range.
    ///
    /// Determines whether a target hex coordinate is within the attack range of the
    /// specified unit, excluding the unit's own position.
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the attacking unit
    /// * `target_hex` - Target hex coordinate to check
    ///
    /// # Returns
    ///
    /// `true` if the target is within attack range (but not same hex), `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```text
    /// if self.is_within_attack_range(selected_unit, enemy_hex) {
    ///     // Can attack enemy at this position
    /// }
    /// ```
    fn is_within_attack_range(&self, unit_id: uuid::Uuid, target_hex: HexCoord) -> bool {
        // Query unit from ScenarioWorld
        if let Some(game_unit) = self.get_unit(unit_id) {
            let unit_pos = game_unit.position();
            let distance = unit_pos.distance(target_hex);
            let attack_range = game_unit.unit().combat_stats().attack_range;
            distance <= attack_range && distance > 0 // Within range but not same hex
        } else {
            false
        }
    }

    /// Checks if the target hex contains an enemy unit.
    ///
    /// Determines whether the specified hex coordinate contains a unit that is
    /// on a different team than the attacker.
    ///
    /// # Arguments
    ///
    /// * `attacker_id` - UUID of the attacking unit
    /// * `target_hex` - Hex coordinate to check for enemy units
    ///
    /// # Returns
    ///
    /// `true` if an enemy unit is present at the target hex, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```text
    /// if self.has_enemy_unit(player_unit, hex_coord) {
    ///     // There's an enemy at this position
    /// }
    /// ```
    fn has_enemy_unit(&self, attacker_id: uuid::Uuid, target_hex: HexCoord) -> bool {
        if let Some(attacker) = self.game_world.units.get(&attacker_id) {
            let attacker_team = attacker.team();

            // Check all units at target hex
            for unit in self.game_world.units.values() {
                if unit.position() == target_hex && unit.id() != attacker_id {
                    return unit.team() != attacker_team;
                }
            }
        }
        false
    }

    /// Updates hex grid highlighting to show selected unit, movement range, and attack targets.
    ///
    /// Clears all existing highlights and applies new ones based on the currently
    /// selected unit. Highlights include:
    /// - Yellow highlight for selected unit's position
    /// - Red highlight for enemies within attack range
    /// - Blue highlight for valid movement hexes
    fn update_highlight_display(&mut self) {
        // Clear existing highlights
        self.hex_grid.clear_all_highlights();

        // Highlight selected unit position
        if let Some(unit_id) = self.selected_unit() {
            if let Some(game_unit) = self.game_world.units.get(&unit_id) {
                self.hex_grid
                    .highlight_hex(game_unit.position(), HighlightType::Selected);

                // Highlight enemies within attack range in red
                let unit_pos = game_unit.position();
                let attacker_team = game_unit.team();
                let attack_range = game_unit.unit().combat_stats().attack_range;

                for enemy in self.game_world.units.values() {
                    if enemy.id() != unit_id && enemy.team() != attacker_team {
                        let enemy_pos = enemy.position();
                        let distance = unit_pos.distance(enemy_pos);
                        if distance <= attack_range && distance > 0 {
                            // Enemy within range - highlight in red
                            self.hex_grid
                                .highlight_hex(enemy_pos, HighlightType::Selected);
                        }
                    }
                }
            }
        }

        // Highlight movement range in blue and display defense values
        self.hex_grid.highlight_hexes(
            self.game_state.exploring.movement_range(),
            HighlightType::MovementRange,
        );

        // Display defense value on each movement range tile (terrain-based)
        if let Some(unit_id) = self.game_state.exploring.selected_unit() {
            if let Some(game_unit) = self.game_world.units.get(&unit_id) {
                let movement_range = self.game_state.exploring.movement_range();
                println!(
                    "🛡️  Setting defense overlay on {} tiles",
                    movement_range.len()
                );
                for &hex_coord in movement_range {
                    // Get terrain at this position
                    if let Some(terrain_tile) = self.game_world.get_terrain(hex_coord) {
                        // Convert sprite type to terrain (temporary mapping until terrain is stored properly)
                        let terrain = sprite_to_terrain(terrain_tile.sprite_type());
                        // Calculate terrain-based defense for this unit at this position
                        let defense = game_unit.unit().get_terrain_hit_chance(terrain);
                        self.hex_grid
                            .set_hex_text_overlay(hex_coord, Some(format!("DEF:{}", defense)));
                        println!(
                            "   - Set DEF:{} on tile {:?} ({:?})",
                            defense, hex_coord, terrain
                        );
                    }
                }
            }
        }
    }

    /// Calls the unit's detailed information display method.
    ///
    /// Queries unit from ScenarioWorld and invokes its `show_details()` method
    /// to print comprehensive unit information to the console.
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to display details for
    fn call_unit_on_click(&self, unit_id: uuid::Uuid) {
        // Query unit from ScenarioWorld
        if let Some(game_obj) = self.get_unit(unit_id) {
            println!("\n🖱️  CALLING UNIT'S ON_CLICK METHOD:");
            println!("═══════════════════════════════════");

            // This will call the actual Unit's on_click() method!
            game_obj.show_details();

            println!("═══════════════════════════════════\n");
        } else {
            println!("Unit with ID {} not found!", unit_id);
        }
    }

    /// Converts screen coordinates to hex grid coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - Screen X coordinate in pixels
    /// * `y` - Screen Y coordinate in pixels
    ///
    /// # Returns
    ///
    /// `Some(HexCoord)` if the screen position maps to a valid hex, `None` otherwise.
    fn screen_to_hex_coord(&self, x: f64, y: f64) -> Option<HexCoord> {
        // Prefer geometric conversion via renderer's camera and grid
        let screen = Vec2::new(x as f32, y as f32);
        let window = Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        self.hex_grid.screen_to_hex_coord(screen, window)
    }

    /// Finds a unit at the specified hex coordinate.
    ///
    /// Queries ScenarioWorld to find any unit occupying the given hex.
    ///
    /// # Arguments
    ///
    /// * `hex_coord` - The hex coordinate to search
    ///
    /// # Returns
    ///
    /// `Some(unit_id)` if a unit is found at the coordinate, `None` otherwise.
    fn find_unit_at_hex(&self, hex_coord: HexCoord) -> Option<uuid::Uuid> {
        // Delegate to ScenarioWorld for unit queries
        self.game_world
            .units
            .iter()
            .find(|(_, unit)| unit.position() == hex_coord)
            .map(|(id, _)| *id)
    }

    // ===== ScenarioWorld Query Helpers =====
    // These methods encapsulate access to ScenarioWorld state for presentation layer

    /// Gets a unit reference from ScenarioWorld.
    ///
    /// Helper method to encapsulate ScenarioWorld access.
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to retrieve
    ///
    /// # Returns
    ///
    /// Reference to the GameUnit if it exists, None otherwise
    fn get_unit(&self, unit_id: uuid::Uuid) -> Option<&game::GameUnit> {
        self.game_world.units.get(&unit_id)
    }

    /// Checks if combat is pending confirmation.
    ///
    /// # Returns
    ///
    /// true if there is a pending combat awaiting player confirmation
    fn has_pending_combat(&self) -> bool {
        self.game_world.pending_combat.is_some()
    }

    /// Finds an interactive item at the specified hex coordinate.
    ///
    /// Queries ScenarioWorld for interactive objects at the given position.
    ///
    /// # Arguments
    ///
    /// * `hex_coord` - The hex coordinate to search
    ///
    /// # Returns
    ///
    /// `Some(item_id)` if an item is found at the coordinate, `None` otherwise.
    fn find_item_at_hex(&self, hex_coord: HexCoord) -> Option<uuid::Uuid> {
        // Delegate to ScenarioWorld for object queries
        self.game_world
            .interactive_objects
            .iter()
            .find(|(_, obj)| obj.position() == hex_coord)
            .map(|(id, _)| *id)
    }

    /// Handles item pickup by a unit.
    ///
    /// Coordinates item pickup between UI and ScenarioWorld:
    /// 1. Retrieves item from interactive object in ScenarioWorld
    /// 2. Adds item to unit's inventory
    /// 3. Attempts auto-equip if not consumable
    /// 4. Removes interactive object from world
    /// 5. Updates UI feedback
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit picking up the item
    /// * `item_id` - UUID of the interactive object containing the item
    fn handle_item_pickup(&mut self, unit_id: uuid::Uuid, item_id: uuid::Uuid) {
        // Query ScenarioWorld for item and unit
        if let Some(item_obj) = self.game_world.interactive_objects.get_mut(&item_id) {
            if let Some(item) = item_obj.take_item() {
                // Add item to unit's inventory and auto-equip
                if let Some(game_unit) = self.game_world.units.get_mut(&unit_id) {
                    let item_name = item.name.clone();
                    let item_internal_id = item.id;
                    let item_type = item.item_type.clone();

                    // Add to inventory first
                    game_unit.unit_mut().add_item_to_inventory(item);
                    println!("✅ Picked up '{}'!", item_name);

                    // Try to auto-equip the item (will fail for consumables)
                    if item_type != items::ItemType::Consumable {
                        match game_unit.unit_mut().equip_item(item_internal_id) {
                            Ok(_) => {
                                let unit = game_unit.unit();
                                let stats = unit.combat_stats();
                                println!("⚔️  Auto-equipped '{}' ({:?})!", item_name, item_type);
                                println!(
                                    "📊 Current Stats - ATK: {} (+{}), HP: {}/{}, Movement: {}",
                                    stats.get_total_attack(),
                                    stats.attack_modifier,
                                    stats.health,
                                    stats.max_health,
                                    stats.movement_speed
                                );
                            }
                            Err(e) => {
                                println!("📦 '{}' added to inventory ({})", item_name, e);
                            }
                        }
                    } else {
                        println!("💊 Consumable '{}' stored in inventory", item_name);
                    }

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

    /// Updates the unit information display in the UI panel.
    ///
    /// Synchronizes UI with ScenarioWorld state by querying unit data and
    /// updating the display panels. This is a pure presentation operation.
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to display information for
    fn update_unit_info_display(&mut self, unit_id: uuid::Uuid) {
        // Query unit from ScenarioWorld
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

                // Get attacks from the unit
                let attacks = unit
                    .get_attacks()
                    .into_iter()
                    .map(|attack| AttackDisplayInfo {
                        name: attack.name,
                        damage: attack.damage,
                        description: attack.description,
                    })
                    .collect();

                // Get inventory from the unit
                let inventory = unit
                    .inventory()
                    .iter()
                    .map(|item| {
                        let item_type_str = match item.item_type {
                            units::ItemType::Weapon => "Weapon",
                            units::ItemType::Armor => "Armor",
                            units::ItemType::Accessory => "Accessory",
                            units::ItemType::Consumable => "Consumable",
                        };
                        (item.name.clone(), item_type_str.to_string())
                    })
                    .collect();

                let display_info = UnitDisplayInfo {
                    name: unit.name().to_string(),
                    race: format!("{:?}", unit.race()),
                    class: unit.unit_type().to_string(),
                    level: unit.level(),
                    experience: unit.experience(),
                    health: stats.health as u32,
                    max_health: stats.max_health as u32,
                    terrain: "N/A".to_string(), // TODO: Get terrain from map at position
                    position_q: position.q,
                    position_r: position.r,
                    moves_left: game_unit.moves_left() as u32,
                    max_moves: stats.movement_speed as u32,
                    sprite_type: unit.sprite(),
                    attacks,
                    inventory,
                };
                ui_panel.set_unit_info(display_info);
            }
        }
    }

    /// Renders UI elements (currently handled by UiPanel).
    ///
    /// Renders additional UI elements like turn information.
    ///
    /// This method renders turn-based game UI on top of the main rendering.
    /// Current UI rendering for unit info is handled by the `UiPanel` component.
    fn render_ui(&mut self) {
        // Display turn information
        if let Some(renderer) = &mut self.renderer {
            let current_team = self.game_world.current_turn_team();
            let turn_number = self.game_world.turn_number();
            let is_player_turn = self.game_world.is_current_team_player_controlled();

            let turn_text = if is_player_turn {
                format!(
                    "Turn {}: {:?}'s Turn (Your Turn)",
                    turn_number + 1,
                    current_team
                )
            } else {
                let time_remaining = self.game_world.ai_turn_time_remaining();
                format!(
                    "Turn {}: {:?}'s Turn (AI - {:.1}s)",
                    turn_number + 1,
                    current_team,
                    time_remaining
                )
            };

            // Render turn info at top of screen
            renderer.text_renderer.render_text(
                &turn_text,
                10.0,
                SCREEN_HEIGHT - 30.0,
                0.5,
                [1.0, 1.0, 1.0, 1.0],
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            );

            // Show "End Turn" button for player turns
            if is_player_turn {
                renderer.text_renderer.render_text(
                    "[SPACE] End Turn",
                    SCREEN_WIDTH - 200.0,
                    SCREEN_HEIGHT - 30.0,
                    0.4,
                    [0.8, 1.0, 0.8, 1.0],
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                );
            }
        }
    }

    /// Updates the hex grid with current unit and item positions.
    ///
    /// Synchronizes visual representation with ScenarioWorld state.
    /// Queries all units and interactive objects from ScenarioWorld and updates
    /// the hex grid sprites accordingly. Terrain sprites are preserved.
    ///
    /// This is a presentation-layer operation that ensures rendering matches game state.
    fn update_hex_grid_units(&mut self) {
        // Clear existing unit and item sprites (keep terrain)
        for hex in self.hex_grid.hexagons.values_mut() {
            hex.set_unit_sprite(None);
            hex.set_item_sprite(None);
        }

        // Query ScenarioWorld for current unit positions
        for unit in self.game_world.units.values() {
            let pos = unit.position();
            let sprite = unit.unit().sprite();
            self.hex_grid.set_unit_at(pos, sprite);
        }

        // Add items on the ground
        for item_obj in self.game_world.interactive_objects.values() {
            let pos = item_obj.position();
            self.hex_grid.set_item_at(pos, SpriteType::Item);
        }
    }

    /// Updates the encyclopedia panel content based on the current category
    fn update_encyclopedia_content(&mut self) {
        use crate::encyclopedia_builder::{
            get_mechanics_content_comprehensive, get_terrain_content_comprehensive,
            get_units_content_comprehensive,
        };

        let current_category = if let Some(panel) = &self.encyclopedia_panel {
            panel.current_category
        } else {
            return;
        };

        let content = match current_category {
            EncyclopediaCategory::Units => get_units_content_comprehensive(&self.encyclopedia),
            EncyclopediaCategory::Terrain => get_terrain_content_comprehensive(&self.encyclopedia),
            EncyclopediaCategory::Mechanics => {
                get_mechanics_content_comprehensive(&self.encyclopedia)
            }
        };

        if let Some(panel) = &mut self.encyclopedia_panel {
            panel.update_content(content);
        }
    }

    // ...existing code...

    /// Gets formatted content for the Units category
    #[allow(dead_code)]
    fn get_units_content(&self) -> Vec<String> {
        let mut lines = vec![
            "╔═══════════════════════════════════════════════════════════════════════╗".to_string(),
            "║                        📖 UNIT ENCYCLOPEDIA                            ║"
                .to_string(),
            "╠═══════════════════════════════════════════════════════════════════════╣".to_string(),
            "".to_string(),
        ];

        // Group units by race
        for race in [
            units::Race::Human,
            units::Race::Elf,
            units::Race::Dwarf,
            units::Race::Orc,
            units::Race::Goblin,
        ] {
            let race_units = self.encyclopedia.units_by_race(race);
            if !race_units.is_empty() {
                lines.push(format!("║  {:?} Units:", race));
                for unit in race_units {
                    lines.push(format!("║    • {} - {}", unit.unit_type, unit.class));
                }
                lines.push("║".to_string());
            }
        }

        lines.push(
            "╚═══════════════════════════════════════════════════════════════════════╝".to_string(),
        );
        lines
    }

    /// Gets formatted content for the Terrain category
    #[allow(dead_code)]
    fn get_terrain_content(&self) -> Vec<String> {
        let mut lines = vec![
            "╔═══════════════════════════════════════════════════════════════════════╗".to_string(),
            "║                       🗺️  TERRAIN GUIDE                                ║"
                .to_string(),
            "╠═══════════════════════════════════════════════════════════════════════╣".to_string(),
            "".to_string(),
        ];

        let terrain_types = self.encyclopedia.all_terrain();
        for terrain in terrain_types {
            lines.push(format!("║  {}", terrain.terrain_type.name()));
            lines.push(format!("║    Movement Cost: {}", terrain.movement_cost));
            lines.push(format!("║    {}", terrain.description));
            lines.push("║".to_string());
        }

        lines.push(
            "╚═══════════════════════════════════════════════════════════════════════╝".to_string(),
        );
        lines
    }

    /// Gets formatted content for the Mechanics category
    #[allow(dead_code)]
    fn get_mechanics_content(&self) -> Vec<String> {
        let mut lines = vec![
            "╔═══════════════════════════════════════════════════════════════════════╗".to_string(),
            "║                      ⚙️  GAME MECHANICS                                ║"
                .to_string(),
            "╠═══════════════════════════════════════════════════════════════════════╣".to_string(),
            "".to_string(),
        ];

        let mechanics = self.encyclopedia.all_mechanics();
        for mechanic in mechanics {
            lines.push(format!("║  {}", mechanic.title));
            lines.push(format!("║    {}", mechanic.description));
            lines.push("║".to_string());
            for detail in &mechanic.details {
                lines.push(format!("║      • {}", detail));
            }
            lines.push("║".to_string());
        }

        lines.push(
            "╚═══════════════════════════════════════════════════════════════════════╝".to_string(),
        );
        lines
    }

    /// Tracks mouse hover position and updates UI with unit details.
    ///
    /// Continuously monitors the mouse position and performs the following:
    /// - Converts screen coordinates to hex coordinates
    /// - Detects units at the hovered hex
    /// - Updates the UI panel with unit information when hovering over units
    /// - Clears UI panel when hovering over empty hexes
    /// - Optionally highlights the hovered hex (debug mode, toggled with 'H' key)
    ///
    /// # Arguments
    ///
    /// * `x` - Screen X coordinate of mouse position in pixels
    /// * `y` - Screen Y coordinate of mouse position in pixels
    ///
    /// # Behavior
    ///
    /// - **Unit Found**: Updates UI panel with name, stats, position, etc.
    /// - **No Unit**: Clears UI panel unless a unit is currently selected
    /// - **Debug Mode**: Highlights hovered hex in bright yellow
    ///
    /// # Examples
    ///
    /// ```text
    /// // Called automatically on CursorMoved event
    /// self.hower(position.x, position.y);
    /// ```
    fn hower(&mut self, x: f64, y: f64) {
        // Convert to hex coordinate using geometric conversion
        if let Some(hex_coord) = self.screen_to_hex_coord(x, y) {
            // Check if there's a unit at the hovered hex
            if let Some(unit_id) = self.find_unit_at_hex(hex_coord) {
                // Unit found - only update UI if it's different from selected unit
                // Priority: Show selected unit's data unless hovering over a different unit
                if self.selected_unit().is_none() || self.selected_unit() != Some(unit_id) {
                    // No unit selected, or hovering over a different unit - show hovered unit
                    if let Some(game_unit) = self.game_world.units.get(&unit_id) {
                        let unit = game_unit.unit();
                        let stats = unit.combat_stats();
                        let position = unit.position();

                        // Get attacks from the unit
                        let attacks = unit
                            .get_attacks()
                            .into_iter()
                            .map(|attack| AttackDisplayInfo {
                                name: attack.name,
                                damage: attack.damage,
                                description: attack.description,
                            })
                            .collect();

                        // Get inventory from the unit
                        let inventory = unit
                            .inventory()
                            .iter()
                            .map(|item| {
                                let item_type_str = match item.item_type {
                                    units::ItemType::Weapon => "Weapon",
                                    units::ItemType::Armor => "Armor",
                                    units::ItemType::Accessory => "Accessory",
                                    units::ItemType::Consumable => "Consumable",
                                };
                                (item.name.clone(), item_type_str.to_string())
                            })
                            .collect();

                        let display_info = UnitDisplayInfo {
                            name: unit.name().to_string(),
                            race: format!("{:?}", unit.race()),
                            class: unit.unit_type().to_string(),
                            level: unit.level(),
                            experience: unit.experience(),
                            health: stats.health as u32,
                            max_health: stats.max_health as u32,
                            terrain: "N/A".to_string(), // TODO: Get terrain from map at position
                            position_q: position.q,
                            position_r: position.r,
                            moves_left: game_unit.moves_left() as u32,
                            max_moves: stats.movement_speed as u32,
                            sprite_type: unit.sprite(),
                            attacks,
                            inventory,
                        };

                        if let Some(ui_panel) = &mut self.ui_panel {
                            ui_panel.set_unit_info(display_info);
                        }
                    }
                }
                // If hovering over the selected unit, keep showing selected unit's data (do nothing)
            } else {
                // No unit at this hex - keep showing selected unit if one is selected
                // Only clear if no unit is selected
                if self.selected_unit().is_none() {
                    if let Some(ui_panel) = &mut self.ui_panel {
                        ui_panel.clear_unit_info();
                    }
                }
            }

            // Debug highlighting (optional - can be toggled with 'H' key)
            if self.hower_debug_enabled {
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
        // If mouse is not over any valid hex, keep selected unit data (don't clear)
    }
}

/// Implementation of winit's ApplicationHandler trait for the game application.
///
/// This implementation manages the window lifecycle and event processing for the game,
/// including initialization, rendering, and user input handling.
impl ApplicationHandler for GameApp {
    /// Called when the application is resumed or first started.
    ///
    /// Initializes the OpenGL context, creates the rendering window, loads game assets,
    /// and sets up the initial game state. This is the main initialization point for
    /// all graphics and game systems.
    ///
    /// # Arguments
    ///
    /// * `event_loop` - The active event loop managing the application lifecycle
    ///
    /// # Initialization Steps
    ///
    /// 1. Creates the game window with specified dimensions
    /// 2. Initializes OpenGL context and surface
    /// 3. Loads texture assets (terrain and items)
    /// 4. Creates renderer with multi-layer support
    /// 5. Initializes UI panel for information display
    /// 6. Populates hex grid with terrain and game objects
    /// 7. Loads guide encyclopedia entries
    /// 8. Prints control instructions to console
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title("QuestQuest - Interactive Game Window")
            .with_inner_size(winit::dpi::LogicalSize::new(
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            ));

        let template = glutin::config::ConfigTemplateBuilder::new().with_depth_size(24); // Request 24-bit depth buffer for proper layering
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
        match Renderer::new(
            vao,
            shader_program,
            dynamic_vbo,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        ) {
            Ok(renderer) => {
                self.renderer = Some(renderer);

                // Initialize UI panel
                match UiPanel::new(SCREEN_WIDTH, SCREEN_HEIGHT) {
                    Ok(ui_panel) => {
                        self.ui_panel = Some(ui_panel);
                        println!("✅ UI Panel initialized!");

                        // Create a separate text renderer for the main menu
                        match graphics::ui::text_renderer::TextRenderer::new() {
                            Ok(text_renderer) => {
                                let shared_renderer =
                                    std::rc::Rc::new(std::cell::RefCell::new(text_renderer));
                                self.main_menu_scene.set_text_renderer(shared_renderer);
                            }
                            Err(e) => {
                                println!("Failed to create text renderer for main menu: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("⚠️  Failed to create UI panel: {}", e);
                    }
                }

                // Initialize Encyclopedia panel
                match EncyclopediaPanel::new(SCREEN_WIDTH, SCREEN_HEIGHT) {
                    Ok(panel) => {
                        self.encyclopedia_panel = Some(panel);
                        println!("✅ Encyclopedia Panel initialized!");
                    }
                    Err(e) => {
                        println!("⚠️  Failed to create Encyclopedia panel: {}", e);
                    }
                }

                // Populate hex grid with terrain (for game scene)
                // This is radius 8 for the game world
                let world_radius = 8;
                for q in -world_radius..=world_radius {
                    let r1 = (-world_radius).max(-q - world_radius);
                    let r2 = world_radius.min(-q + world_radius);
                    for r in r1..=r2 {
                        let coord = HexCoord::new(q, r);
                        self.hex_grid.hexagons.entry(coord).or_insert_with(|| {
                            let mut hex = graphics::Hexagon::new(coord, 50.0);
                            hex.set_sprite(SpriteType::Grasslands);
                            hex
                        });
                    }
                }

                println!("🎮 QuestQuest Started!");
                println!("� Showing Main Menu...");
                println!();
                println!("=== MAIN MENU ===");
                println!("Click 'Scenarios' to start the game");
                println!("Press ESC to exit");
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

    /// Handles window events from the operating system.
    ///
    /// Processes all user input and window events, including mouse movement, clicks,
    /// keyboard input, and window closure. Events are handled with specific priority
    /// orders to ensure correct interaction behavior.
    ///
    /// # Arguments
    ///
    /// * `event_loop` - The active event loop
    /// * `_window_id` - Window identifier (unused, single window)
    /// * `event` - The window event to process
    ///
    /// # Event Handling
    ///
    /// - **CloseRequested**: Exits the application
    /// - **CursorMoved**: Updates hover states and UI panel with unit info
    /// - **MouseInput**: Processes left/right clicks for unit selection and movement
    /// - **KeyboardInput**: Handles camera controls, hotkeys, and game commands
    /// - **RedrawRequested**: Renders the current game state to screen
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Check for scene transitions first
        if self.scene_manager.process_transition() {
            let current = self.scene_manager.current_scene();

            match current {
                SceneType::Game => {
                    // Initialize game scene if not already done
                    if !self.game_initialized {
                        self.initialize_game_scene();
                        self.game_initialized = true;
                    }
                }
                SceneType::MainMenu => {
                    // Return to main menu
                    println!("🏠 Returned to Main Menu");
                }
                _ => {}
            }

            // Request redraw after scene change
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        // Check if exit was requested
        if self.exit_requested {
            println!("👋 Goodbye!");
            event_loop.exit();
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Store cursor position for click handling
                self.cursor_position = (position.x, position.y);

                // Handle cursor movement based on current scene
                match self.scene_manager.current_scene() {
                    SceneType::MainMenu => {
                        self.main_menu_scene
                            .handle_cursor_move(position.x, position.y);
                    }
                    SceneType::Game => {
                        // Check combat state first to avoid borrow conflicts
                        let has_combat = self.has_pending_combat();

                        // Update combat confirmation button hover states
                        if let Some(renderer) = &mut self.renderer {
                            if has_combat {
                                renderer
                                    .combat_log_display
                                    .update_button_hover(position.x as f32, position.y as f32);
                            }
                        }

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
                    }
                    _ => {}
                }

                // Request redraw
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                let is_left = button == MouseButton::Left;

                // Handle click based on current scene
                match self.scene_manager.current_scene() {
                    SceneType::MainMenu => {
                        if let Some(new_scene) = self.main_menu_scene.handle_click(
                            self.cursor_position.0,
                            self.cursor_position.1,
                            is_left,
                        ) {
                            self.scene_manager.transition_to(new_scene);
                        }
                    }
                    SceneType::Game => match button {
                        MouseButton::Left => {
                            self.handle_left_click(self.cursor_position.0, self.cursor_position.1);
                        }
                        MouseButton::Right => {
                            self.handle_right_click(self.cursor_position.0, self.cursor_position.1);
                        }
                        _ => {}
                    },
                    _ => {}
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    // Handle keyboard input based on current scene
                    match self.scene_manager.current_scene() {
                        SceneType::MainMenu => {
                            if let winit::keyboard::PhysicalKey::Code(key_code) = event.physical_key
                            {
                                if let Some(new_scene) = self.main_menu_scene.handle_key(key_code) {
                                    self.scene_manager.transition_to(new_scene);
                                }
                            }
                        }
                        SceneType::Game => {
                            self.handle_game_keyboard_input(event.physical_key);
                        }
                        _ => {}
                    }

                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                // Calculate delta time for game updates
                let now = std::time::Instant::now();
                let delta_time = (now - self.last_update_time).as_secs_f32();
                self.last_update_time = now;

                // Render based on current scene
                match self.scene_manager.current_scene() {
                    SceneType::MainMenu => {
                        self.main_menu_scene.render();

                        // Swap buffers
                        if let (Some(gl_context), Some(gl_surface)) =
                            (&self.gl_context, &self.gl_surface)
                        {
                            gl_surface.swap_buffers(gl_context).unwrap();
                        }
                    }
                    SceneType::Game => {
                        // Update movement animation
                        self.update_animation(delta_time);

                        // AI execution: Delegate to ScenarioWorld (which uses AI crate)
                        // QuestApp only tracks which team had AI run to avoid duplicate execution
                        let current_team = self.game_world.current_turn_team();
                        if !self.game_world.is_current_team_player_controlled() {
                            // Check if this is a new AI turn (team changed or first AI turn)
                            if self.last_ai_turn_team != Some(current_team) {
                                println!("🤖 AI executing for team {:?}", current_team);
                                // Delegate to ScenarioWorld which uses AI crate for planning
                                self.game_world.run_ai_for_current_team();
                                self.last_ai_turn_team = Some(current_team);
                            }
                        } else {
                            // Reset tracker when it's a player turn
                            self.last_ai_turn_team = None;
                        }

                        // Update game state (turn system, AI events, etc.)
                        // ScenarioWorld handles turn advancement, AI action processing
                        self.game_world.update(delta_time);

                        // Update unit positions on hex grid before rendering
                        self.update_hex_grid_units();

                        // Render all game layers and UI elements
                        if let Some(renderer) = &mut self.renderer {
                            renderer.render(&self.hex_grid);

                            // Render UI panel
                            if let Some(ui_panel) = &mut self.ui_panel {
                                ui_panel.render(SCREEN_WIDTH, SCREEN_HEIGHT, renderer);
                            }

                            // Render UI overlay
                            self.render_ui();

                            // Render encyclopedia panel if visible
                            if self.encyclopedia_visible() {
                                if let Some(encyclopedia_panel) = &mut self.encyclopedia_panel {
                                    encyclopedia_panel.render(SCREEN_WIDTH, SCREEN_HEIGHT);
                                }
                            }

                            if let (Some(gl_context), Some(gl_surface)) =
                                (&self.gl_context, &self.gl_surface)
                            {
                                gl_surface.swap_buffers(gl_context).unwrap();
                            }
                        }
                    }
                    _ => {
                        // Other scenes (Settings, SavedGames) - just clear for now
                        unsafe {
                            gl::ClearColor(0.1, 0.1, 0.15, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                        }

                        if let (Some(gl_context), Some(gl_surface)) =
                            (&self.gl_context, &self.gl_surface)
                        {
                            gl_surface.swap_buffers(gl_context).unwrap();
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

/// Application entry point.
///
/// Creates and runs the QuestQuest interactive game window with:
/// - OpenGL rendering context
/// - Event loop for user input
/// - Demo units and items for testing
///
/// # Controls
///
/// - **Left Click**: Move units, interact with UI
/// - **Right Click**: Select units
/// - **Arrow Keys**: Move camera (scroll encyclopedia when open)
/// - **E**: Toggle encyclopedia wiki
/// - **1/2/3**: Switch encyclopedia categories when open
/// - **C**: Show unit info
/// - **H**: Toggle hover debug
/// - **SPACE**: End turn
/// - **ESC**: Close encyclopedia/menu, deselect unit
///
/// # Panics
///
/// Panics if the event loop or window creation fails.
fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = GameApp::new();

    println!("🎮 Starting QuestQuest Interactive Game Window...");
    println!("📚 Press E to open the Encyclopedia at any time!");
    event_loop.run_app(&mut app).unwrap();
}
