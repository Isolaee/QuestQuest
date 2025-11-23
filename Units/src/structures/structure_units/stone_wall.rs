//! Stone Wall structure implementation.
//!
//! Stone walls are heavy fortifications that provide excellent protection
//! and block enemy movement. They are resistant to most damage types but
//! vulnerable to siege weapons.

use crate::attack::Attack;
use crate::structures::structure_stats::StructureStats;
use crate::structures::structure_trait::{Structure, StructureId};
use crate::structures::structure_type::StructureType;
use crate::team::Team;
use crate::unit_race::Terrain;
use crate::unit_trait::UnitId;
use combat::Resistances;
use graphics::HexCoord;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A stone wall fortification.
///
/// Stone walls are the primary defensive structure for protecting territory.
/// They block all movement, provide significant defense bonuses to occupants,
/// and can withstand considerable punishment before being destroyed.
///
/// # Statistics
///
/// - **Durability**: 500 HP
/// - **Max Occupants**: 1 unit
/// - **Defense Bonus**: +15
/// - **Resistances**: High against physical damage, excellent against fire
/// - **Siege Vulnerability**: 2.5x damage from siege units
/// - **Movement**: Blocks all units
///
/// # Examples
///
/// ```rust,no_run
/// use units::structures::{StoneWall, Structure};
/// use graphics::HexCoord;
/// use units::Team;
///
/// let mut wall = StoneWall::new(HexCoord::new(5, 5), Team::Player);
/// assert_eq!(wall.max_durability(), 500);
/// assert_eq!(wall.defense_bonus(), 15);
/// assert!(wall.blocks_movement());
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneWall {
    /// Unique identifier
    id: StructureId,
    /// Display name
    name: String,
    /// Position on the hex grid
    position: HexCoord,
    /// Controlling team
    team: Team,
    /// Structure statistics
    stats: StructureStats,
}

impl StoneWall {
    /// Creates a new stone wall at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate where the wall will be built
    /// * `team` - The team that controls this wall
    pub fn new(position: HexCoord, team: Team) -> Self {
        let mut stats = StructureStats::new();

        // Durability
        stats.max_durability = 500;
        stats.current_durability = 500;
        stats.repair_rate = 5; // Repairs 5 HP/turn when occupied

        // Resistances - stone is tough!
        stats.resistances = Resistances::new(
            70, // blunt - good resistance
            60, // pierce - decent resistance
            90, // fire - excellent resistance (stone doesn't burn)
            50, // dark - moderate resistance
            80, // slash - excellent resistance
            50, // crush - moderate (siege weapons use crush)
        );

        // Siege weapons are designed to break walls
        stats.siege_vulnerability = 2.5;

        // Occupation
        stats.max_occupants = 1; // Only one unit can occupy

        // Bonuses for occupants
        stats.defense_bonus = 15; // Excellent cover
        stats.attack_bonus = 0;
        stats.range_bonus = 0;
        stats.resistance_bonuses = Resistances::default();
        stats.vision_bonus = 1; // Slightly elevated
        stats.healing_per_turn = 0;

        // Movement
        stats.blocks_movement = true; // Walls block everyone
        stats.allows_passage_team = None; // No one passes through
        stats.movement_cost_modifier = 0;

        // Terrain - can build on most flat/hilly terrain
        stats.buildable_on = vec![Terrain::Grasslands, Terrain::Hills, Terrain::Forest0];
        stats.provides_terrain_bonus = None;

        // Combat
        stats.thorns_damage = 0; // No passive damage
        stats.explosive_on_destroy = None;
        stats.can_attack = false;
        stats.attacks = Vec::new();

        Self {
            id: Uuid::new_v4(),
            name: "Stone Wall".to_string(),
            position,
            team,
            stats,
        }
    }
}

impl Structure for StoneWall {
    fn id(&self) -> StructureId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn structure_type(&self) -> StructureType {
        StructureType::StoneWall
    }

    fn position(&self) -> HexCoord {
        self.position
    }

    fn set_position(&mut self, position: HexCoord) {
        self.position = position;
    }

    fn team(&self) -> Team {
        self.team
    }

    fn set_team(&mut self, team: Team) {
        self.team = team;
    }

    // Durability
    fn max_durability(&self) -> u32 {
        self.stats.max_durability
    }

    fn current_durability(&self) -> u32 {
        self.stats.current_durability
    }

    fn is_destroyed(&self) -> bool {
        self.stats.is_destroyed()
    }

    fn take_damage(&mut self, damage: u32, is_siege: bool) -> u32 {
        let mut final_damage = damage as f32;

        // Apply siege vulnerability
        if is_siege {
            final_damage *= self.stats.siege_vulnerability;
        }

        // For simplicity, we'll apply resistance based on crush damage type
        // (most siege weapons deal crush damage)
        if is_siege {
            let resistance = self.stats.resistances.crush as f32 / 100.0;
            final_damage *= 1.0 - resistance;
        }

        let final_damage = final_damage.max(0.0) as u32;
        self.stats.take_damage(final_damage)
    }

    fn repair(&mut self, amount: u32) -> u32 {
        self.stats.repair(amount)
    }

    fn auto_repair(&mut self) -> u32 {
        self.stats.auto_repair()
    }

    // Occupation
    fn max_occupants(&self) -> u32 {
        self.stats.max_occupants
    }

    fn occupants(&self) -> &[UnitId] {
        &self.stats.current_occupants
    }

    fn has_space(&self) -> bool {
        self.stats.has_space()
    }

    fn is_occupied_by(&self, unit_id: UnitId) -> bool {
        self.stats.is_occupied_by(unit_id)
    }

    fn add_occupant(&mut self, unit_id: UnitId) -> Result<(), String> {
        self.stats.add_occupant(unit_id)
    }

    fn remove_occupant(&mut self, unit_id: UnitId) -> bool {
        self.stats.remove_occupant(unit_id)
    }

    // Bonuses
    fn defense_bonus(&self) -> i32 {
        self.stats.defense_bonus
    }

    fn attack_bonus(&self) -> i32 {
        self.stats.attack_bonus
    }

    fn range_bonus(&self) -> i32 {
        self.stats.range_bonus
    }

    fn resistance_bonuses(&self) -> &Resistances {
        &self.stats.resistance_bonuses
    }

    fn vision_bonus(&self) -> i32 {
        self.stats.vision_bonus
    }

    fn healing_per_turn(&self) -> u32 {
        self.stats.healing_per_turn
    }

    // Movement
    fn blocks_movement(&self) -> bool {
        self.stats.blocks_movement
    }

    fn allows_passage_team(&self) -> Option<Team> {
        self.stats.allows_passage_team
    }

    fn movement_cost_modifier(&self) -> i32 {
        self.stats.movement_cost_modifier
    }

    fn can_pass_through(&self, _team: Team) -> bool {
        // Stone walls don't let anyone through
        false
    }

    // Combat
    fn thorns_damage(&self) -> u32 {
        self.stats.thorns_damage
    }

    fn can_attack(&self) -> bool {
        self.stats.can_attack
    }

    fn attacks(&self) -> &[Attack] {
        &self.stats.attacks
    }

    // Terrain
    fn buildable_on(&self) -> &[Terrain] {
        &self.stats.buildable_on
    }

    fn can_build_on(&self, terrain: Terrain) -> bool {
        self.stats.buildable_on.contains(&terrain)
    }

    fn provides_terrain_bonus(&self) -> Option<Terrain> {
        self.stats.provides_terrain_bonus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_wall_creation() {
        let wall = StoneWall::new(HexCoord::new(0, 0), Team::Player);
        assert_eq!(wall.name(), "Stone Wall");
        assert_eq!(wall.max_durability(), 500);
        assert_eq!(wall.current_durability(), 500);
        assert_eq!(wall.team(), Team::Player);
    }

    #[test]
    fn test_stone_wall_blocks_movement() {
        let wall = StoneWall::new(HexCoord::new(0, 0), Team::Player);
        assert!(wall.blocks_movement());
        assert!(!wall.can_pass_through(Team::Player));
        assert!(!wall.can_pass_through(Team::Enemy));
    }

    #[test]
    fn test_stone_wall_occupation() {
        let mut wall = StoneWall::new(HexCoord::new(0, 0), Team::Player);
        let unit1_id = Uuid::new_v4();
        let unit2_id = Uuid::new_v4();

        assert!(wall.has_space());
        assert!(wall.add_occupant(unit1_id).is_ok());
        assert!(wall.is_occupied_by(unit1_id));
        assert_eq!(wall.occupants().len(), 1);

        // Second unit should not be able to occupy
        assert!(!wall.has_space());
        assert!(wall.add_occupant(unit2_id).is_err());
    }

    #[test]
    fn test_stone_wall_damage() {
        let mut wall = StoneWall::new(HexCoord::new(0, 0), Team::Player);

        // Normal damage
        let damage_dealt = wall.take_damage(100, false);
        assert!(damage_dealt > 0);
        assert!(wall.current_durability() < 500);

        // Siege damage should be more effective
        let siege_damage = wall.take_damage(100, true);
        assert!(siege_damage > damage_dealt);
    }

    #[test]
    fn test_stone_wall_repair() {
        let mut wall = StoneWall::new(HexCoord::new(0, 0), Team::Player);

        wall.take_damage(200, false);
        let initial = wall.current_durability();

        let repaired = wall.repair(50);
        assert_eq!(repaired, 50);
        assert_eq!(wall.current_durability(), initial + 50);
    }

    #[test]
    fn test_stone_wall_bonuses() {
        let wall = StoneWall::new(HexCoord::new(0, 0), Team::Player);

        assert_eq!(wall.defense_bonus(), 15);
        assert_eq!(wall.vision_bonus(), 1);
        assert_eq!(wall.attack_bonus(), 0);
    }
}
