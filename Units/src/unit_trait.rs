//! Core unit trait and related types.
//!
//! This module defines the main [`Unit`] trait that all units in the game must implement.
//! The trait provides a unified interface for unit identity, movement, combat, inventory
//! management, and progression.

use crate::attack::Attack;
use crate::unit_race::{Race, Terrain};
use combat::CombatStats;
use graphics::HexCoord;
use graphics::SpriteType;
use items::{Equipment, Item, ItemId};
use uuid::Uuid;

/// Unique identifier for units.
///
/// Each unit has a UUID to uniquely identify it in the game world.
pub type UnitId = Uuid;

/// Core trait that all units must implement.
///
/// This trait provides a unified interface for all unit types in the game,
/// including heroes, enemies, and NPCs. It handles:
///
/// - **Identity**: Name, ID, position, race, and type
/// - **Combat**: Access to combat statistics and equipment
/// - **Inventory**: Item management and equipment handling
/// - **Progression**: Level and experience tracking
/// - **Movement**: Position and terrain management
///
/// # Examples
///
/// ```rust,no_run
/// use units::{Unit, UnitFactory, Terrain};
/// use graphics::HexCoord;
///
/// let mut unit = UnitFactory::create_human_warrior(
///     "Knight".to_string(),
///     HexCoord::new(0, 0),
///     Terrain::Grasslands,
/// );
///
/// // Check unit identity
/// println!("Name: {}", unit.name());
/// println!("Race: {:?}", unit.race());
/// println!("Level: {}", unit.level());
///
/// // Move the unit
/// unit.move_to(HexCoord::new(1, 0));
///
/// // Check health
/// if unit.is_alive() {
///     println!("Unit is alive with {} HP", unit.combat_stats().health);
/// }
/// ```
pub trait Unit {
    // ===== Identity =====

    /// Returns the unit's unique identifier.
    ///
    /// This UUID persists for the lifetime of the unit and can be used
    /// to track the unit across saves and network synchronization.
    fn id(&self) -> UnitId;

    /// Returns the unit's display name.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use units::{Unit, UnitFactory, Terrain};
    /// # use graphics::HexCoord;
    /// # let unit = UnitFactory::create_human_warrior("Aragorn".to_string(), HexCoord::new(0, 0), Terrain::Grasslands);
    /// assert_eq!(unit.name(), "Aragorn");
    /// ```
    fn name(&self) -> &str;

    /// Returns the unit's current position on the hex grid.
    fn position(&self) -> HexCoord;

    /// Returns the unit's race.
    ///
    /// Race affects terrain bonuses, base stats, and visual appearance.
    fn race(&self) -> Race;

    /// Returns the unit's type identifier.
    ///
    /// This is a string like "Human Warrior", "Elf Archer", or "Goblin Grunt"
    /// that identifies the specific unit template.
    fn unit_type(&self) -> &str;

    /// Returns the unit's description for wiki/gameplay purposes.
    ///
    /// This description provides lore, tactical information, and gameplay context
    /// about the unit that will be displayed in game wikis and tooltips.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use units::{Unit, UnitFactory, Terrain};
    /// # use graphics::HexCoord;
    /// # let unit = UnitFactory::create_human_warrior("Knight".to_string(), HexCoord::new(0, 0), Terrain::Grasslands);
    /// let description = unit.description();
    /// println!("Unit description: {}", description);
    /// ```
    fn description(&self) -> &str;

    // ===== Movement =====

    /// Moves the unit to a new position.
    ///
    /// # Arguments
    ///
    /// * `position` - The target hex coordinate
    ///
    /// # Returns
    ///
    /// Returns `true` if the move was successful, `false` otherwise.
    /// Movement may fail due to obstacles, lack of movement points, or other game rules.
    fn move_to(&mut self, position: HexCoord) -> bool;

    // ===== Stats Access =====

    /// Returns a reference to the unit's combat statistics.
    ///
    /// Combat stats include health, attack power, defense, movement speed,
    /// resistances, and other combat-relevant values.
    fn combat_stats(&self) -> &CombatStats;

    /// Returns a mutable reference to the unit's combat statistics.
    ///
    /// Use this to modify health, apply buffs/debuffs, or update other combat values.
    fn combat_stats_mut(&mut self) -> &mut CombatStats;

    // ===== Equipment & Inventory =====

    /// Returns a reference to the unit's equipped items.
    ///
    /// Equipment includes weapons, armor, accessories, and other worn items
    /// that provide stat bonuses.
    fn equipment(&self) -> &Equipment;

    /// Returns a mutable reference to the unit's equipped items.
    fn equipment_mut(&mut self) -> &mut Equipment;

    /// Returns a reference to the unit's inventory.
    ///
    /// The inventory contains all items the unit is carrying but not currently equipped.
    fn inventory(&self) -> &[Item];

    /// Returns a mutable reference to the unit's inventory.
    fn inventory_mut(&mut self) -> &mut Vec<Item>;

    /// Equips an item from the inventory.
    ///
    /// # Arguments
    ///
    /// * `item_id` - The unique identifier of the item to equip
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The item is not in the inventory
    /// - The item cannot be equipped by this unit
    /// - The equipment slot is incompatible
    fn equip_item(&mut self, item_id: ItemId) -> Result<(), String>;

    /// Unequips an item and moves it to the inventory.
    ///
    /// # Arguments
    ///
    /// * `item_id` - The unique identifier of the equipped item to remove
    ///
    /// # Errors
    ///
    /// Returns an error if the item is not currently equipped.
    fn unequip_item(&mut self, item_id: ItemId) -> Result<(), String>;

    /// Adds an item to the unit's inventory.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add
    fn add_item_to_inventory(&mut self, item: Item);

    /// Removes an item from the inventory and returns it.
    ///
    /// # Arguments
    ///
    /// * `item_id` - The unique identifier of the item to remove
    ///
    /// # Returns
    ///
    /// Returns `Some(item)` if the item was found and removed, `None` otherwise.
    fn remove_item_from_inventory(&mut self, item_id: ItemId) -> Option<Item>;

    // ===== Level & Experience =====

    /// Returns the unit's current level.
    ///
    /// Level affects base stats and unlocks new abilities.
    fn level(&self) -> i32;

    /// Returns the unit's current experience points.
    fn experience(&self) -> i32;

    /// Adds experience points to the unit.
    ///
    /// # Arguments
    ///
    /// * `exp` - The amount of experience to add
    ///
    /// # Returns
    ///
    /// Returns `true` if the unit leveled up, `false` otherwise.
    /// When leveling up, base stats are automatically increased.
    fn add_experience(&mut self, exp: i32) -> bool;

    /// Returns the experience required to reach the next level.
    fn experience_for_next_level(&self) -> i32;

    /// Returns the progress toward the next level as a percentage.
    ///
    /// # Returns
    ///
    /// A value between 0.0 and 1.0, where 0.0 is no progress and 1.0 is ready to level up.
    fn level_progress(&self) -> f32;

    /// Checks if the unit has enough XP to level up.
    fn can_level_up(&self) -> bool;

    /// Performs a level-up with evolution to next unit type.
    ///
    /// # Arguments
    ///
    /// * `new_stats` - Combat stats for the new level
    /// * `new_attacks` - Attacks available at the new level  
    /// * `new_unit_type` - New unit type name
    /// * `heal_to_full` - Whether to restore unit to full health
    fn perform_level_up_evolution(
        &mut self,
        new_stats: CombatStats,
        new_attacks: Vec<crate::attack::Attack>,
        new_unit_type: String,
        heal_to_full: bool,
    );

    /// Performs incremental level-up for max-level units (no evolution).
    /// Grants +2 max HP and +1 attack.
    ///
    /// # Arguments
    ///
    /// * `heal_to_full` - Whether to restore unit to full health
    fn perform_level_up_incremental(&mut self, heal_to_full: bool);

    // ===== Terrain =====

    /// Returns the terrain type the unit is currently standing on.
    ///
    /// Terrain affects movement cost, combat bonuses, and visibility.
    fn current_terrain(&self) -> Terrain;

    /// Sets the terrain type for the unit's current position.
    ///
    /// This should be called when the unit moves to update terrain-based modifiers.
    fn set_terrain(&mut self, terrain: Terrain);

    // ===== Utility Methods =====

    /// Checks if the unit is alive.
    ///
    /// # Returns
    ///
    /// Returns `true` if the unit's health is greater than 0, `false` otherwise.
    fn is_alive(&self) -> bool;

    /// Checks if the unit can attack a target at the given position.
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

    // ===== Attack Methods =====

    /// Get all available attacks for this unit (including equipped items)
    fn get_attacks(&self) -> Vec<Attack>;

    /// Get the unit's defense value (base hit chance for enemies)
    ///
    /// This returns the percentage chance (0-100) that an enemy has to hit this unit
    /// before applying attack modifiers. Lower values mean harder to hit.
    ///
    /// Default implementation uses race + terrain, but units can override for
    /// custom defense values (e.g., evasive archers, heavily armored warriors).
    fn get_defense(&self) -> u8 {
        // Default: use terrain hit chance from combat stats
        // This is calculated from race + terrain in recalculate_stats()
        self.combat_stats().terrain_hit_chance
    }

    // ===== Visual Representation =====

    /// Returns the sprite type for rendering this unit.
    ///
    /// Each unit type has its own sprite for visual representation.
    /// This enables proper encapsulation where units know their own appearance.
    ///
    /// # Returns
    ///
    /// The `SpriteType` that should be used to render this unit on the hex grid.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use units::{Unit, UnitFactory, Terrain};
    /// # use graphics::{HexCoord, SpriteType};
    /// # let dwarf = UnitFactory::create("Dwarf Warrior", Some("Thorin".to_string()), Some(HexCoord::new(0, 0)), Some(Terrain::Mountain)).unwrap();
    /// let sprite = dwarf.sprite();
    /// assert_eq!(sprite, SpriteType::DwarfWarrior);
    /// ```
    fn sprite(&self) -> SpriteType {
        // Default implementation returns generic Unit sprite
        // Override this in specific unit implementations for custom sprites
        SpriteType::Unit
    }

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
