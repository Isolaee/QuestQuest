//! # Turn System Module
//!
//! This module implements turn-based gameplay mechanics for the game.
//! It manages which team's turn it is, tracks turn state, and handles
//! automatic turn progression for AI-controlled teams.
//!
//! ## Features
//!
//! - Team-based turns (Player, Enemy, Neutral)
//! - Automatic AI turn progression with configurable delay
//! - Unit activation tracking (which units have acted)
//! - Turn phase management
//!
/// ## Example
///
/// ```
/// use game::TurnSystem;
/// use game::Team;
/// use game::TurnPhase;
///
/// let mut turn_system = TurnSystem::new();
/// turn_system.start_game();
/// assert!(turn_system.phase() == TurnPhase::Active);
/// // Simulate passing time for AI teams (default AI delay is 3.0s)
/// turn_system.set_team_control(Team::Player, true);
/// turn_system.set_team_control(Team::Enemy, false);
/// // Ensure player turn doesn't auto-advance
/// turn_system.update(5.0);
/// assert_eq!(turn_system.current_team(), Team::Player);
/// ```
use crate::Team;
use std::collections::HashSet;
use uuid::Uuid;

/// Represents the current phase of a turn
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnPhase {
    /// Waiting to start the game
    NotStarted,
    /// Active turn - units can move and act
    Active,
    /// Turn is ending, transitioning to next team
    Ending,
}

/// Manages turn-based gameplay state
///
/// The turn system tracks which team is currently active, manages turn progression,
/// and handles automatic turn advancement for AI-controlled teams.
///
/// # Turn Flow
///
/// 1. **Player Turn**: Players can move any number of their units
/// 2. **AI Turn**: After 3 seconds, automatically advances to next team
/// 3. **Cycle**: Continues through all teams in order (Player → Enemy → Neutral)
///
/// # Examples
///
/// ```
/// use game::TurnSystem;
/// use game::Team;
///
/// let mut turn_system = TurnSystem::new();
/// turn_system.set_team_control(Team::Player, true);
/// turn_system.set_team_control(Team::Enemy, false);
/// turn_system.start_game();
/// assert!(turn_system.is_team_turn(Team::Player));
/// ```
pub struct TurnSystem {
    /// List of teams participating in the turn order
    teams: Vec<Team>,

    /// Which teams are player-controlled (true) vs AI-controlled (false)
    player_controlled: HashSet<Team>,

    /// Current team whose turn it is (index into teams vec)
    current_team_index: usize,

    /// Current turn phase
    phase: TurnPhase,

    /// Time elapsed in the current turn (seconds)
    turn_timer: f32,

    /// How long AI teams wait before passing turn (seconds)
    ai_turn_delay: f32,

    /// Units that have moved/acted this turn
    units_acted_this_turn: HashSet<Uuid>,

    /// Total number of turns completed
    turn_count: u32,
}

impl TurnSystem {
    /// Creates a new turn system with default settings
    ///
    /// By default:
    /// - AI teams wait 3 seconds before passing turn
    /// - Turn order: Player → Enemy → Neutral
    /// - All teams are AI-controlled (must call `add_team` to set player control)
    pub fn new() -> Self {
        Self {
            teams: vec![Team::Player, Team::Enemy, Team::Neutral],
            player_controlled: HashSet::new(),
            current_team_index: 0,
            phase: TurnPhase::NotStarted,
            turn_timer: 0.0,
            ai_turn_delay: 3.0,
            units_acted_this_turn: HashSet::new(),
            turn_count: 0,
        }
    }

    /// Sets whether a team is player-controlled
    ///
    /// # Arguments
    ///
    /// * `team` - The team to configure
    /// * `is_player_controlled` - If true, turn waits for player input. If false, auto-advances.
    pub fn set_team_control(&mut self, team: Team, is_player_controlled: bool) {
        if is_player_controlled {
            self.player_controlled.insert(team);
        } else {
            self.player_controlled.remove(&team);
        }
    }

    /// Starts the game and begins the first turn
    pub fn start_game(&mut self) {
        self.phase = TurnPhase::Active;
        self.turn_timer = 0.0;
        self.turn_count = 0;
        self.units_acted_this_turn.clear();
        println!("🎮 Game started! {:?}'s turn", self.current_team());
    }

    /// Updates the turn system, handling AI turn delays
    ///
    /// # Arguments
    ///
    /// * `delta_time` - Time elapsed since last update (seconds)
    pub fn update(&mut self, delta_time: f32) {
        if self.phase != TurnPhase::Active {
            return;
        }

        self.turn_timer += delta_time;

        // Check if AI team should auto-pass turn
        if !self.is_current_team_player_controlled() && self.turn_timer >= self.ai_turn_delay {
            self.end_turn();
        }
    }

    /// Ends the current turn and advances to the next team
    pub fn end_turn(&mut self) {
        if self.phase == TurnPhase::NotStarted {
            return;
        }

        // Clear acted units
        self.units_acted_this_turn.clear();

        self.phase = TurnPhase::Ending;

        println!("⏭️  {:?} turn ended", self.current_team());

        // Advance to next team
        self.current_team_index = (self.current_team_index + 1) % self.teams.len();

        // If we've cycled back to the first team, increment turn count
        if self.current_team_index == 0 {
            self.turn_count += 1;
        }

        // Reset timer and start next turn
        self.turn_timer = 0.0;
        self.phase = TurnPhase::Active;

        println!(
            "▶️  {:?}'s turn (Turn {})",
            self.current_team(),
            self.turn_count + 1
        );
    }

    /// Returns the team whose turn it currently is
    pub fn current_team(&self) -> Team {
        self.teams[self.current_team_index]
    }

    /// Checks if it's currently the specified team's turn
    ///
    /// # Arguments
    ///
    /// * `team` - The team to check
    ///
    /// # Returns
    ///
    /// `true` if it's the team's turn, `false` otherwise
    pub fn is_team_turn(&self, team: Team) -> bool {
        self.phase == TurnPhase::Active && self.current_team() == team
    }

    /// Checks if the current team is player-controlled
    pub fn is_current_team_player_controlled(&self) -> bool {
        self.player_controlled.contains(&self.current_team())
    }

    /// Returns the current turn phase
    pub fn phase(&self) -> TurnPhase {
        self.phase
    }

    /// Returns the current turn number (0-based)
    pub fn turn_number(&self) -> u32 {
        self.turn_count
    }

    /// Returns time remaining in AI turn (0 if player turn)
    pub fn ai_turn_time_remaining(&self) -> f32 {
        if self.is_current_team_player_controlled() {
            0.0
        } else {
            (self.ai_turn_delay - self.turn_timer).max(0.0)
        }
    }

    /// Marks a unit as having acted this turn
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit that acted
    pub fn mark_unit_acted(&mut self, unit_id: Uuid) {
        self.units_acted_this_turn.insert(unit_id);
    }

    /// Checks if a unit has already acted this turn
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to check
    ///
    /// # Returns
    ///
    /// `true` if the unit has acted, `false` otherwise
    pub fn has_unit_acted(&self, unit_id: Uuid) -> bool {
        self.units_acted_this_turn.contains(&unit_id)
    }

    /// Resets a unit's acted status (useful for abilities that grant extra actions)
    ///
    /// # Arguments
    ///
    /// * `unit_id` - UUID of the unit to reset
    pub fn reset_unit_acted(&mut self, unit_id: Uuid) {
        self.units_acted_this_turn.remove(&unit_id);
    }

    /// Checks if the game has started
    pub fn is_game_started(&self) -> bool {
        self.phase != TurnPhase::NotStarted
    }

    /// Sets the AI turn delay
    ///
    /// # Arguments
    ///
    /// * `delay` - Time in seconds to wait before auto-passing AI turns
    pub fn set_ai_turn_delay(&mut self, delay: f32) {
        self.ai_turn_delay = delay.max(0.0);
    }
}

impl Default for TurnSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_system_creation() {
        let turn_system = TurnSystem::new();
        assert_eq!(turn_system.phase(), TurnPhase::NotStarted);
        assert_eq!(turn_system.turn_number(), 0);
    }

    #[test]
    fn test_game_start() {
        let mut turn_system = TurnSystem::new();
        turn_system.start_game();

        assert_eq!(turn_system.phase(), TurnPhase::Active);
        assert_eq!(turn_system.current_team(), Team::Player);
    }

    #[test]
    fn test_turn_progression() {
        let mut turn_system = TurnSystem::new();
        turn_system.start_game();

        assert_eq!(turn_system.current_team(), Team::Player);

        turn_system.end_turn();
        assert_eq!(turn_system.current_team(), Team::Enemy);

        turn_system.end_turn();
        assert_eq!(turn_system.current_team(), Team::Neutral);

        turn_system.end_turn();
        assert_eq!(turn_system.current_team(), Team::Player);
        assert_eq!(turn_system.turn_number(), 1);
    }

    #[test]
    fn test_player_control() {
        let mut turn_system = TurnSystem::new();
        turn_system.set_team_control(Team::Player, true);
        turn_system.set_team_control(Team::Enemy, false);

        turn_system.start_game();

        // Player turn should not auto-advance
        turn_system.update(5.0);
        assert_eq!(turn_system.current_team(), Team::Player);

        // Move to enemy turn
        turn_system.end_turn();
        assert_eq!(turn_system.current_team(), Team::Enemy);

        // Enemy turn should auto-advance after 3 seconds
        turn_system.update(3.5);
        assert_eq!(turn_system.current_team(), Team::Neutral);
    }
}
