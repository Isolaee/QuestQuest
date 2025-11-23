//! Team affiliation type for units and structures.
//!
//! This module defines the `Team` enum used throughout the game to determine
//! friend-or-foe relationships and control ownership of entities.

use serde::{Deserialize, Serialize};

/// Team affiliation for units and structures in the game world.
///
/// Determines friend-or-foe relationships for combat and movement validation.
/// Units and structures of the same team cannot attack each other, while
/// entities of different teams can engage in combat.
///
/// # Examples
///
/// ```
/// use units::Team;
///
/// let player_team = Team::Player;
/// let enemy_team = Team::Enemy;
/// assert_ne!(player_team, enemy_team);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Team {
    /// Player-controlled units and structures
    Player,
    /// Enemy-controlled units and structures that can be attacked
    Enemy,
    /// Neutral entities that don't participate in combat
    Neutral,
}

impl Team {
    /// Checks if this team is hostile to another team.
    ///
    /// # Arguments
    ///
    /// * `other` - The other team to check against
    ///
    /// # Returns
    ///
    /// `true` if the teams are hostile to each other
    pub fn is_hostile_to(&self, other: Team) -> bool {
        matches!(
            (self, other),
            (Team::Player, Team::Enemy) | (Team::Enemy, Team::Player)
        )
    }

    /// Checks if this team is allied with another team.
    ///
    /// # Arguments
    ///
    /// * `other` - The other team to check against
    ///
    /// # Returns
    ///
    /// `true` if the teams are allied or the same
    pub fn is_allied_with(&self, other: Team) -> bool {
        self == &other || *self == Team::Neutral || other == Team::Neutral
    }
}
