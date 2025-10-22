//! Main Menu Scene
//!
//! The initial scene shown when the game starts, providing options to start
//! a scenario, load saved games, adjust settings, or exit.

use crate::scene_manager::{Scene, SceneType};
use graphics::ui::text_renderer::TextRenderer;
use std::cell::RefCell;
use std::rc::Rc;

/// A clickable button on the main menu
#[derive(Clone)]
struct MenuButton {
    /// Display label
    label: String,
    /// Position (center x, center y)
    position: (f32, f32),
    /// Size (width, height)
    size: (f32, f32),
    /// Action to perform when clicked
    action: MenuButtonAction,
}

/// Actions that menu buttons can trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuButtonAction {
    /// Load the game scenario
    Scenarios,
    /// Show saved games screen
    SavedGames,
    /// Open settings menu
    Settings,
    /// Exit the application
    Exit,
}

impl MenuButton {
    /// Create a new menu button
    fn new(label: impl Into<String>, action: MenuButtonAction, position: (f32, f32)) -> Self {
        Self {
            label: label.into(),
            position,
            size: (300.0, 60.0), // Default button size
            action,
        }
    }

    /// Check if a point is inside this button
    fn contains(&self, x: f32, y: f32) -> bool {
        let half_width = self.size.0 / 2.0;
        let half_height = self.size.1 / 2.0;

        x >= self.position.0 - half_width
            && x <= self.position.0 + half_width
            && y >= self.position.1 - half_height
            && y <= self.position.1 + half_height
    }
}

/// Main Menu Scene
pub struct MainMenuScene {
    /// Menu buttons
    buttons: Vec<MenuButton>,

    /// Screen dimensions
    screen_width: f32,
    screen_height: f32,

    /// Text renderer for drawing UI elements
    text_renderer: Option<Rc<RefCell<TextRenderer>>>,
}

impl MainMenuScene {
    /// Create a new main menu scene
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let center_x = screen_width / 2.0;
        let start_y = screen_height / 2.0 - 150.0;
        let button_spacing = 80.0;

        let buttons = vec![
            MenuButton::new(
                "Scenarios",
                MenuButtonAction::Scenarios,
                (center_x, start_y),
            ),
            MenuButton::new(
                "Saved Games",
                MenuButtonAction::SavedGames,
                (center_x, start_y + button_spacing),
            ),
            MenuButton::new(
                "Settings",
                MenuButtonAction::Settings,
                (center_x, start_y + button_spacing * 2.0),
            ),
            MenuButton::new(
                "Exit",
                MenuButtonAction::Exit,
                (center_x, start_y + button_spacing * 3.0),
            ),
        ];

        Self {
            buttons,
            screen_width,
            screen_height,
            text_renderer: None,
        }
    }

    /// Set the text renderer for this scene
    pub fn set_text_renderer(&mut self, text_renderer: Rc<RefCell<TextRenderer>>) {
        self.text_renderer = Some(text_renderer);
    }

    /// Update screen dimensions (e.g., on window resize)
    pub fn update_screen_size(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;

        // Recenter buttons
        let center_x = width / 2.0;
        let start_y = height / 2.0 - 150.0;
        let button_spacing = 80.0;

        for (i, button) in self.buttons.iter_mut().enumerate() {
            button.position = (center_x, start_y + button_spacing * i as f32);
        }
    }

    /// Check which button was clicked, if any
    fn check_button_click(&self, x: f32, y: f32) -> Option<MenuButtonAction> {
        for button in &self.buttons {
            if button.contains(x, y) {
                return Some(button.action);
            }
        }
        None
    }

    /// Render the main menu
    pub fn render_menu(&mut self) {
        if let Some(text_renderer) = &self.text_renderer {
            let mut renderer = text_renderer.borrow_mut();

            unsafe {
                // Clear background
                gl::ClearColor(0.1, 0.1, 0.15, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            // Font aspect ratio (5x7 bitmap font)
            let font_aspect = 5.0 / 7.0;

            // Title
            let title = "QuestQuest";
            let title_size = 24.0; // Character height in pixels
            let title_y = self.screen_height / 2.0 - 280.0;

            // Calculate title width and center it
            let title_char_width = title_size * font_aspect;
            let title_width = title.len() as f32 * title_char_width;
            let title_x = (self.screen_width - title_width) / 2.0;

            renderer.render_text(
                title,
                title_x,
                title_y,
                title_size,
                [1.0, 1.0, 1.0, 1.0],
                self.screen_width,
                self.screen_height,
            );

            // Render buttons
            for button in &self.buttons {
                // Button text - centered at button position
                let button_size = 12.0; // Character height in pixels
                let button_char_width = button_size * font_aspect;
                let button_text_width = button.label.len() as f32 * button_char_width;

                // Center text at button position
                let text_x = button.position.0 - (button_text_width / 2.0);
                let text_y = button.position.1 - (button_size / 2.0); // Center vertically too

                renderer.render_text(
                    &button.label,
                    text_x,
                    text_y,
                    button_size,
                    [0.9, 0.9, 1.0, 1.0], // Slightly blue-tinted white
                    self.screen_width,
                    self.screen_height,
                );
            }
        }
    }
}

impl Scene for MainMenuScene {
    fn on_enter(&mut self) {
        println!("🏠 Entering Main Menu");
    }

    fn on_exit(&mut self) {
        println!("🏠 Exiting Main Menu");
    }

    fn update(&mut self, _delta_time: f32) {
        // Main menu doesn't need per-frame updates
    }

    fn render(&mut self) {
        self.render_menu();
    }

    fn handle_click(&mut self, x: f64, y: f64, is_left_button: bool) -> Option<SceneType> {
        if !is_left_button {
            return None;
        }

        if let Some(action) = self.check_button_click(x as f32, y as f32) {
            println!("🖱️  Main Menu button clicked: {:?}", action);

            match action {
                MenuButtonAction::Scenarios => {
                    println!("🎮 Loading game scenario...");
                    return Some(SceneType::Game);
                }
                MenuButtonAction::SavedGames => {
                    println!("💾 Opening saved games...");
                    return Some(SceneType::SavedGames);
                }
                MenuButtonAction::Settings => {
                    println!("⚙️  Opening settings...");
                    return Some(SceneType::Settings);
                }
                MenuButtonAction::Exit => {
                    println!("👋 Exiting application...");
                    std::process::exit(0);
                }
            }
        }

        None
    }

    fn handle_key(&mut self, key: winit::keyboard::KeyCode) -> Option<SceneType> {
        use winit::keyboard::KeyCode;

        match key {
            KeyCode::Escape => {
                println!("👋 Exiting via ESC key...");
                std::process::exit(0);
            }
            _ => None,
        }
    }

    fn handle_cursor_move(&mut self, _x: f64, _y: f64) {
        // Could add hover effects here later
    }
}
