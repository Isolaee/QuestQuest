//! Structure statistics and properties.
//!
//! This module defines the core statistics that govern structure behavior,
//! durability, bonuses, and combat interactions.

use crate::attack::Attack;
use crate::combat::Resistances;
use crate::team::Team;
use crate::unit_race::Terrain;
use crate::unit_trait::UnitId;
use serde::{Deserialize, Serialize};

/// Complete statistics and properties for a structure.
///
/// `StructureStats` defines all the properties that control how a structure
/// behaves in the game, including its durability, bonuses granted to occupants,
/// combat interactions, and movement restrictions.
///

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructureStats {
    // === Durability ===
    /// Maximum durability (hit points)
    pub max_durability: u32,
    /// Current durability
    pub current_durability: u32,
    /// Hit points restored per turn when occupied
    pub repair_rate: u32,

    // === Damage Resistance ===
    /// Resistance to various damage types
    pub resistances: Resistances,
    /// Multiplier for siege damage (higher = more vulnerable to siege)
    pub siege_vulnerability: f32,

    // === Occupation ===
    /// Maximum number of units that can occupy this structure (always 1)
    pub max_occupants: u32,
    /// IDs of currently occupying units (max 1)
    pub current_occupants: Vec<UnitId>,

    // === Bonuses Granted to Occupants ===
    /// Defense bonus added to occupying units
    pub defense_bonus: i32,
    /// Attack bonus added to occupying units
    pub attack_bonus: i32,
    /// Range bonus added to occupying units' attacks
    pub range_bonus: i32,
    /// Additional resistances granted to occupying units
    pub resistance_bonuses: Resistances,
    /// Extended vision range for occupying units
    pub vision_bonus: i32,
    /// HP restored to occupying units per turn
    pub healing_per_turn: u32,
    /// Special ability tags
    pub special_abilities: Vec<String>,

    // === Movement & Blocking ===
    /// Whether this structure blocks movement
    pub blocks_movement: bool,
    /// Which team (if any) can pass through this structure
    pub allows_passage_team: Option<Team>,
    /// Extra movement cost to enter this structure
    pub movement_cost_modifier: i32,

    // === Terrain Restrictions ===
    /// Terrain types where this structure can be built
    pub buildable_on: Vec<Terrain>,
    /// Terrain type this structure simulates for bonuses
    pub provides_terrain_bonus: Option<Terrain>,

    // === Combat Interactions ===
    /// Damage dealt to melee attackers (thorns/spikes effect)
    pub thorns_damage: u32,
    /// Area damage when structure is destroyed
    pub explosive_on_destroy: Option<u32>,
    /// Whether this structure can initiate attacks
    pub can_attack: bool,
    /// Attacks this structure can perform (e.g., tower ballista)
    pub attacks: Vec<Attack>,
}

impl StructureStats {
    /// Creates a new `StructureStats` with default values.
    ///
    /// Default structure has:
    /// - 100 durability
    /// - 1 occupant max (structures only allow single occupant)
    /// - No bonuses
    /// - No special abilities
    pub fn new() -> Self {
        Self {
            max_durability: 100,
            current_durability: 100,
            repair_rate: 0,
            resistances: Resistances::default(),
            siege_vulnerability: 1.0,
            max_occupants: 1,
            current_occupants: Vec::new(),
            defense_bonus: 0,
            attack_bonus: 0,
            range_bonus: 0,
            resistance_bonuses: Resistances::default(),
            vision_bonus: 0,
            healing_per_turn: 0,
            special_abilities: Vec::new(),
            blocks_movement: false,
            allows_passage_team: None,
            movement_cost_modifier: 0,
            buildable_on: Vec::new(),
            provides_terrain_bonus: None,
            thorns_damage: 0,
            explosive_on_destroy: None,
            can_attack: false,
            attacks: Vec::new(),
        }
    }

    /// Checks if the structure is destroyed (durability <= 0).
    pub fn is_destroyed(&self) -> bool {
        self.current_durability == 0
    }

    /// Checks if the structure has space for more occupants.
    pub fn has_space(&self) -> bool {
        (self.current_occupants.len() as u32) < self.max_occupants
    }

    /// Checks if a specific unit is occupying this structure.
    pub fn is_occupied_by(&self, unit_id: UnitId) -> bool {
        self.current_occupants.contains(&unit_id)
    }

    /// Adds an occupant to this structure.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the unit was added, `Err(String)` if the structure is full.
    pub fn add_occupant(&mut self, unit_id: UnitId) -> Result<(), String> {
        if !self.has_space() {
            return Err("Structure is at maximum occupancy".to_string());
        }
        if self.is_occupied_by(unit_id) {
            return Err("Unit is already occupying this structure".to_string());
        }
        self.current_occupants.push(unit_id);
        Ok(())
    }

    /// Removes an occupant from this structure.
    ///
    /// # Returns
    ///
    /// `true` if the unit was removed, `false` if they weren't occupying.
    pub fn remove_occupant(&mut self, unit_id: UnitId) -> bool {
        if let Some(pos) = self.current_occupants.iter().position(|&id| id == unit_id) {
            self.current_occupants.remove(pos);
            true
        } else {
            false
        }
    }

    /// Applies damage to the structure.
    ///
    /// # Arguments
    ///
    /// * `damage` - Amount of damage to apply
    ///
    /// # Returns
    ///
    /// Actual damage dealt after clamping
    pub fn take_damage(&mut self, damage: u32) -> u32 {
        let actual_damage = damage.min(self.current_durability);
        self.current_durability = self.current_durability.saturating_sub(damage);
        actual_damage
    }

    /// Repairs the structure by a specific amount.
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount of durability to restore
    ///
    /// # Returns
    ///
    /// Actual amount repaired
    pub fn repair(&mut self, amount: u32) -> u32 {
        let old_durability = self.current_durability;
        self.current_durability = (self.current_durability + amount).min(self.max_durability);
        self.current_durability - old_durability
    }

    /// Performs automatic repair if the structure is occupied.
    ///
    /// Called each turn to apply the `repair_rate`.
    ///
    /// # Returns
    ///
    /// Amount repaired
    pub fn auto_repair(&mut self) -> u32 {
        if self.current_occupants.is_empty() || self.repair_rate == 0 {
            return 0;
        }
        self.repair(self.repair_rate)
    }
}

impl Default for StructureStats {
    fn default() -> Self {
        Self::new()
    }
}
