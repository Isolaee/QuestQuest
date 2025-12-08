//! House structure implementation.
//!
//! Houses are civilian buildings that provide shelter and healing
//! for occupying units. They offer moderate protection and allow
//! units to rest and recover.

use crate::attack::Attack;
use crate::combat::Resistances;
use crate::structures::structure_stats::StructureStats;
use crate::structures::structure_trait::{Structure, StructureId};
use crate::structures::structure_type::StructureType;
use crate::team::Team;
use crate::unit_race::Terrain;
use crate::unit_trait::UnitId;
use graphics::HexCoord;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A house building structure.
///
/// Houses are civilian structures that provide shelter and healing.
/// Units can occupy houses to gain defensive bonuses and heal over time.
/// Houses don't block movement and allow friendly passage.
///
/// # Statistics
///
/// - **Durability**: 200 HP
/// - **Max Occupants**: 2 units
/// - **Defense Bonus**: +5
/// - **Healing**: +10 HP per turn
/// - **Resistances**: Moderate against most damage, vulnerable to fire
/// - **Siege Vulnerability**: 2.0x damage from siege units
/// - **Movement**: Allows friendly passage
///
/// # Examples
///
/// ```rust,no_run
/// use units::structures::{House, Structure};
/// use graphics::HexCoord;
/// use units::Team;
///
/// let mut house = House::new(HexCoord::new(5, 5), Team::Player);
/// assert_eq!(house.max_durability(), 200);
/// assert_eq!(house.defense_bonus(), 5);
/// assert_eq!(house.healing_per_turn(), 10);
/// assert!(!house.blocks_movement());
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House {
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

impl House {
    /// Creates a new house at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - Hex coordinate where the house will be built
    /// * `team` - The team that controls this house
    pub fn new(position: HexCoord, team: Team) -> Self {
        let mut stats = StructureStats::new();

        // Durability - houses are less sturdy than walls
        stats.max_durability = 200;
        stats.current_durability = 200;
        stats.repair_rate = 3; // Repairs 3 HP/turn when occupied

        // Resistances - wood and thatch construction
        stats.resistances = Resistances::new(
            40, // blunt - moderate resistance
            30, // pierce - low resistance
            20, // fire - vulnerable (wood burns!)
            40, // dark - moderate resistance
            35, // slash - moderate resistance
            30, // crush - low resistance
        );

        // Siege weapons can damage buildings
        stats.siege_vulnerability = 2.0;

        // Occupation - houses can shelter more units
        stats.max_occupants = 2;

        // Bonuses for occupants
        stats.defense_bonus = 5; // Some cover from walls
        stats.attack_bonus = 0;
        stats.range_bonus = 0;
        stats.resistance_bonuses = Resistances::default();
        stats.vision_bonus = 0;
        stats.healing_per_turn = 10; // Rest and recover

        // Movement - houses allow friendly passage
        stats.blocks_movement = false;
        stats.allows_passage_team = Some(team); // Only controlling team can pass
        stats.movement_cost_modifier = 1; // Slight movement cost to enter

        // Terrain - can build on flat terrain
        stats.buildable_on = vec![Terrain::Grasslands, Terrain::Hills];
        stats.provides_terrain_bonus = None;

        // Combat
        stats.thorns_damage = 0;
        stats.explosive_on_destroy = None;
        stats.can_attack = false;
        stats.attacks = Vec::new();

        Self {
            id: Uuid::new_v4(),
            name: "House".to_string(),
            position,
            team,
            stats,
        }
    }
}

impl Structure for House {
    fn id(&self) -> StructureId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn structure_type(&self) -> StructureType {
        StructureType::House
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
        // Update passage permission when team changes
        self.stats.allows_passage_team = Some(team);
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

        // Apply crush resistance for siege damage
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

    fn can_pass_through(&self, team: Team) -> bool {
        // Houses allow friendly units to pass through
        self.stats.allows_passage_team == Some(team)
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
