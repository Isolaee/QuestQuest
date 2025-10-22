//! Game Scene
//!
//! The active gameplay scene where the player interacts with units,
//! explores the map, and engages in combat.

use crate::scene_manager::{Scene, SceneType};

/// Game Scene - wraps the actual gameplay logic
pub struct GameScene {
    /// Reference to the game state
    /// This is a marker - the actual game logic remains in GameApp
    _marker: std::marker::PhantomData<()>,
}

impl GameScene {
    /// Create a new game scene
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl Scene for GameScene {
    fn on_enter(&mut self) {
        println!("🎮 Entering Game Scene");
    }

    fn on_exit(&mut self) {
        println!("🎮 Exiting Game Scene");
    }

    fn update(&mut self, _delta_time: f32) {
        // Game logic is handled in GameApp
    }

    fn render(&mut self) {
        // Rendering is handled in GameApp
    }

    fn handle_click(&mut self, _x: f64, _y: f64, _is_left_button: bool) -> Option<SceneType> {
        // Click handling is done in GameApp
        None
    }

    fn handle_key(&mut self, key: winit::keyboard::KeyCode) -> Option<SceneType> {
        use winit::keyboard::KeyCode;

        // ESC key returns to main menu
        match key {
            KeyCode::Escape => {
                // When menu is open, ESC should close it (handled in GameApp)
                // When menu is closed, ESC should open it (handled in GameApp)
                // For now, we don't transition scenes from ESC in game
                None
            }
            _ => None,
        }
    }

    fn handle_cursor_move(&mut self, _x: f64, _y: f64) {
        // Cursor movement is handled in GameApp
    }
}

impl Default for GameScene {
    fn default() -> Self {
        Self::new()
    }
}
