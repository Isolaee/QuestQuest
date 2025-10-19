use crate::unit_class::UnitClass;
use crate::unit_race::{Race, Terrain};
use combat::CombatStats;
use graphics::HexCoord;
use items::{Equipment, Item, ItemId};
use uuid::Uuid;

/// Unique identifier for units
pub type UnitId = Uuid;

/// Core trait that all units must implement
/// Units can move and manage inventory. Combat is handled by combat stats.
pub trait Unit {
    // ===== Identity =====

    /// Get the unit's unique identifier
    fn id(&self) -> UnitId;

    /// Get the unit's name
    fn name(&self) -> &str;

    /// Get the unit's current position
    fn position(&self) -> HexCoord;

    /// Get the unit's race
    fn race(&self) -> Race;

    /// Get the unit's class
    fn class(&self) -> UnitClass;

    // ===== Combat Methods =====

    /// Move to a new position, returns true if successful
    fn move_to(&mut self, position: HexCoord) -> bool;

    // ===== Stats Access =====

    /// Get reference to combat stats
    fn combat_stats(&self) -> &CombatStats;

    /// Get mutable reference to combat stats
    fn combat_stats_mut(&mut self) -> &mut CombatStats;

    // ===== Equipment & Inventory =====

    /// Get reference to equipment
    fn equipment(&self) -> &Equipment;

    /// Get mutable reference to equipment
    fn equipment_mut(&mut self) -> &mut Equipment;

    /// Get reference to inventory
    fn inventory(&self) -> &[Item];

    /// Get mutable reference to inventory
    fn inventory_mut(&mut self) -> &mut Vec<Item>;

    /// Equip an item from inventory
    fn equip_item(&mut self, item_id: ItemId) -> Result<(), String>;

    /// Unequip an item to inventory
    fn unequip_item(&mut self, item_id: ItemId) -> Result<(), String>;

    /// Add item to inventory
    fn add_item_to_inventory(&mut self, item: Item);

    /// Remove item from inventory
    fn remove_item_from_inventory(&mut self, item_id: ItemId) -> Option<Item>;

    // ===== Level & Experience =====

    /// Get current level
    fn level(&self) -> i32;

    /// Get current experience
    fn experience(&self) -> i32;

    /// Add experience and return true if leveled up
    fn add_experience(&mut self, exp: i32) -> bool;

    /// Get experience required for next level
    fn experience_for_next_level(&self) -> i32;

    /// Get level progress as percentage (0.0 to 1.0)
    fn level_progress(&self) -> f32;

    // ===== Terrain =====

    /// Get current terrain
    fn current_terrain(&self) -> Terrain;

    /// Set current terrain
    fn set_terrain(&mut self, terrain: Terrain);

    // ===== Utility Methods =====

    /// Check if unit is alive
    fn is_alive(&self) -> bool;

    /// Check if unit can attack target position
    fn can_attack(&self, target_position: HexCoord) -> bool;

    /// Check if unit can move to target position
    fn can_move_to(&self, target: HexCoord) -> bool;

    /// Get all hexagonal coordinates within movement range
    fn get_movement_range(&self) -> Vec<HexCoord>;

    /// Take damage
    fn take_damage(&mut self, damage: u32);

    /// Heal the unit
    fn heal(&mut self, amount: i32);

    /// Recalculate all derived stats
    fn recalculate_stats(&mut self);

    // ===== Display Methods =====

    /// Get unit's display color based on race
    fn get_display_color(&self) -> [f32; 3];

    /// Get detailed unit information for display
    fn get_info(&self) -> String;

    /// Get a short summary of the unit
    fn get_summary(&self) -> String;

    /// Display comprehensive unit information
    fn display_unit_info(&self);

    /// Display quick unit info
    fn display_quick_info(&self);

    /// Handle click event
    fn on_click(&self);
}
