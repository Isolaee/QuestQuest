//! Scene Manager
//!
//! Manages different scenes/states of the game (Main Menu, Game, Settings, etc.)
//! and handles transitions between them.

/// Different scenes in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneType {
    /// Main menu scene
    MainMenu,
    /// Active gameplay scene
    Game,
    /// Settings menu
    Settings,
    /// Saved games screen
    SavedGames,
}

/// Trait that all scenes must implement
pub trait Scene {
    /// Called when the scene becomes active
    fn on_enter(&mut self);

    /// Called when the scene is about to be deactivated
    fn on_exit(&mut self);

    /// Update scene logic
    fn update(&mut self, delta_time: f32);

    /// Render the scene
    fn render(&mut self);

    /// Handle mouse click events
    /// Returns Some(SceneType) if a scene transition should occur
    fn handle_click(&mut self, x: f64, y: f64, is_left_button: bool) -> Option<SceneType>;

    /// Handle keyboard input
    fn handle_key(&mut self, key: winit::keyboard::KeyCode) -> Option<SceneType>;

    /// Handle cursor movement
    fn handle_cursor_move(&mut self, x: f64, y: f64);
}

/// Scene Manager that coordinates scene transitions
pub struct SceneManager {
    /// Currently active scene
    current_scene: SceneType,

    /// Whether a scene transition is pending
    pending_transition: Option<SceneType>,
}

impl SceneManager {
    /// Create a new scene manager starting at the main menu
    pub fn new() -> Self {
        Self {
            current_scene: SceneType::MainMenu,
            pending_transition: None,
        }
    }

    /// Get the current scene type
    pub fn current_scene(&self) -> SceneType {
        self.current_scene
    }

    /// Request a transition to a new scene
    pub fn transition_to(&mut self, scene: SceneType) {
        self.pending_transition = Some(scene);
    }

    /// Check if there's a pending transition and execute it
    pub fn process_transition(&mut self) -> bool {
        if let Some(new_scene) = self.pending_transition.take() {
            println!(
                "🎬 Scene transition: {:?} -> {:?}",
                self.current_scene, new_scene
            );
            self.current_scene = new_scene;
            true
        } else {
            false
        }
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
