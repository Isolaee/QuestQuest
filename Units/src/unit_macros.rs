//! # Unit Macros Module
//!
//! This module provides declarative macros for implementing the [`Unit`](crate::unit_trait::Unit) trait
//! on concrete unit types. It eliminates boilerplate code by providing automatic implementations
//! of common trait methods that delegate to the underlying [`BaseUnit`](crate::base_unit::BaseUnit).
//!
//! ## Overview
//!
//! In the QuestQuest game, all unit types (warriors, mages, rangers, etc.) share a common
//! set of behaviors defined by the `Unit` trait. Rather than manually implementing dozens
//! of trait methods for each unit type, this module provides the `impl_unit_delegate!` macro
//! to automatically generate these implementations.
//!
//! ## Features
//!
//! The macro implements the following categories of functionality:
//!
//! - **Identity Methods**: Access to unit ID, name, position, race, and type
//! - **Movement System**: Position updates, movement validation, and range calculation
//! - **Combat Statistics**: Access to health, attack, defense, and other combat-related stats
//! - **Equipment Management**: Equipping/unequipping items and inventory operations
//! - **Experience System**: Level progression, experience tracking, and stat recalculation
//! - **Terrain Handling**: Current terrain tracking and terrain-based stat modifications
//! - **Display Functions**: Information formatting for UI and debugging
//! - **Attack System**: Attack collection from unit abilities and equipped weapons
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use units::{BaseUnit, Race, Terrain};
//! use units::attack::Attack;
//! use graphics::HexCoord;
//! use combat::{CombatStats, RangeCategory, Resistances};
//!
//! pub struct DwarfWarrior {
//!     base: BaseUnit,
//!     attacks: Vec<Attack>,
//! }
//!
//! impl DwarfWarrior {
//!     pub fn new(name: String, position: HexCoord) -> Self {
//!         let stats = CombatStats::new(30, 5, 2, RangeCategory::Melee, Resistances::default());
//!         let base = BaseUnit::new(name, position, Race::Human, "DwarfWarrior".to_string(), "A sturdy dwarf warrior".to_string(), Terrain::Grasslands, stats);
//!         Self { base, attacks: vec![] }
//!     }
//! }
//!
//! // Implement the Unit trait for the type using the macro (compile-only example)
//! units::impl_unit_delegate!(DwarfWarrior);
//! ```
//!
//! ## Implementation Details
//!
//! The macro generates implementations that:
//! - Delegate most operations to the `self.base` field (which must be a `BaseUnit`)
//! - Automatically handle stat recalculation when equipment or terrain changes
//! - Merge attacks from the unit's native attacks and equipped weapons
//! - Convert between item damage types and combat damage types
//! - Provide formatted output for display purposes
//!
//! ## Requirements
//!
//! For a struct to use this macro, it must:
//! - Have a field named `base` of type `BaseUnit`
//! - Have a field named `attacks` of type `Vec<Attack>`
//! - Be in scope of all necessary imports (handled automatically by the macro)

/// Converts an item damage type to a combat damage type.
///
/// This helper function maps the damage types defined in the items system
/// to the corresponding combat system damage types. Some item damage types
/// may not have direct combat equivalents and are mapped to a default type.
///
/// # Arguments
///
/// * `dt` - The damage type from the items system
///
/// # Returns
///
/// Implements the `Unit` trait for a concrete unit type by delegating to `BaseUnit`.
///
/// This macro generates a complete implementation of the [`Unit`](crate::unit_trait::Unit) trait
/// for any struct that contains a `base: BaseUnit` field and an `attacks: Vec<Attack>` field.
/// It eliminates the need to manually write dozens of boilerplate delegation methods.
///
/// # Generated Methods
///
/// The macro implements the following trait methods (organized by category):
///
/// ## Identity Methods
/// - `id()` - Returns the unique unit identifier
/// - `name()` - Returns the unit's name
/// - `position()` - Returns the unit's hex coordinate position
/// - `race()` - Returns the unit's race (e.g., Dwarf, Elf, Human)
/// - `unit_type()` - Returns the unit's type as a string (e.g., "Warrior", "Mage")
///
/// ## Movement Methods
/// - `move_to(position)` - Attempts to move the unit to a new position
/// - `can_move_to(position)` - Checks if a position is within movement range
/// - `get_movement_range()` - Returns all valid positions the unit can move to
///
/// ## Combat Stats Methods
/// - `combat_stats()` - Returns an immutable reference to combat statistics
/// - `combat_stats_mut()` - Returns a mutable reference to combat statistics
/// - `take_damage(damage)` - Applies damage to the unit
/// - `heal(amount)` - Heals the unit by the specified amount
/// - `is_alive()` - Checks if the unit's health is above zero
/// - `can_attack(target_position)` - Checks if a position is within attack range
///
/// ## Equipment & Inventory Methods
/// - `equipment()` - Returns an immutable reference to equipped items
/// - `equipment_mut()` - Returns a mutable reference to equipped items
/// - `inventory()` - Returns the unit's inventory items
/// - `inventory_mut()` - Returns a mutable reference to the inventory
/// - `equip_item(item_id)` - Equips an item from inventory, triggers stat recalculation
/// - `unequip_item(item_id)` - Unequips an item to inventory, triggers stat recalculation
/// - `add_item_to_inventory(item)` - Adds an item to the inventory
/// - `remove_item_from_inventory(item_id)` - Removes and returns an item from inventory
///
/// ## Level & Experience Methods
/// - `level()` - Returns the current level
/// - `experience()` - Returns the current experience points
/// - `add_experience(exp)` - Adds experience, returns `true` if leveled up
/// - `experience_for_next_level()` - Returns experience needed for next level
/// - `level_progress()` - Returns a 0.0-1.0 float representing progress to next level
///
/// ## Terrain Methods
/// - `current_terrain()` - Returns the terrain the unit is currently on
/// - `set_terrain(terrain)` - Updates terrain and recalculates stats
///
/// ## Utility Methods
/// - `recalculate_stats()` - Recalculates stats based on level, equipment, and terrain
///
/// ## Display Methods
/// - `get_display_color()` - Returns RGB color array for rendering
/// - `get_info()` - Returns detailed multi-line unit information string
/// - `get_summary()` - Returns a brief one-line unit status
/// - `display_unit_info()` - Prints detailed information to console
/// - `display_quick_info()` - Prints brief summary to console
/// - `on_click()` - Handles click events, prints unit identification
///
/// ## Attack Methods
/// - `get_attacks()` - Returns all available attacks (from unit + equipped weapons)
///
/// # Requirements
///
/// The target struct must have:
/// - A `base` field of type [`BaseUnit`](crate::base_unit::BaseUnit)
/// - An `attacks` field of type `Vec<`[`Attack`](crate::attack::Attack)`>`
///
/// ```rust,no_run
/// use units::{BaseUnit, Race, Terrain};
/// use units::attack::Attack;
/// use graphics::HexCoord;
/// use combat::{CombatStats, RangeCategory, Resistances, DamageType};
///
/// pub struct ElvenArcher {
///     base: BaseUnit,
///     attacks: Vec<Attack>,
/// }
///
/// impl ElvenArcher {
///     pub fn new(name: String, position: HexCoord) -> Self {
///         let stats = CombatStats::new(25, 6, 3, RangeCategory::Melee, Resistances::default());
///         let base = BaseUnit::new(name, position, Race::Elf, "Archer".to_string(), "A skilled elven archer".to_string(), Terrain::Grasslands, stats);
///
///         let attacks = vec![
///             Attack::ranged("Long Shot", 15, 1, DamageType::Pierce, 4),
///             Attack::melee("Quick Strike", 8, 2, DamageType::Slash),
///         ];
///
///         Self { base, attacks }
///     }
/// }
///
/// // Compile-only example showing macro application
/// units::impl_unit_delegate!(ElvenArcher);
/// ```
///
/// The macro automatically triggers stat recalculation in these scenarios:
/// - When items are equipped or unequipped
/// - When the unit levels up (after gaining experience)
/// - When the terrain changes
///
/// This ensures that equipment bonuses, terrain modifiers, and level-based stats
/// are always up-to-date.
///
/// # Attack System Integration
///
/// The `get_attacks()` method merges attacks from multiple sources:
/// 1. The unit's native `attacks` field (innate abilities)
/// 2. Attacks provided by the equipped weapon (if any)
/// 3. Attacks provided by equipped accessories (if any)
///
/// Item attacks are automatically converted from the items system's damage types
/// to combat system damage types using the helper function `item_damage_type_to_combat`.
///
/// # Example: Creating a New Unit Type
///
/// ```rust,no_run
/// use units::{BaseUnit, Race, Terrain};
/// use units::attack::Attack;
/// use graphics::HexCoord;
/// use combat::{CombatStats, RangeCategory, Resistances, DamageType};
///
/// pub struct ElvenArcher {
///     base: BaseUnit,
///     attacks: Vec<Attack>,
/// }
///
/// impl ElvenArcher {
///     pub fn new(name: String, position: HexCoord) -> Self {
///         let stats = CombatStats::new(25, 6, 3, RangeCategory::Melee, Resistances::default());
///         let base = BaseUnit::new(name, position, Race::Elf, "Archer".to_string(), "A skilled elven archer".to_string(), Terrain::Grasslands, stats);
///
///         let attacks = vec![
///             Attack::ranged("Long Shot", 15, 1, DamageType::Pierce, 4),
///             Attack::melee("Quick Strike", 8, 2, DamageType::Slash),
///         ];
///
///         Self { base, attacks }
///     }
/// }
///
/// // Compile-only example showing macro application
/// units::impl_unit_delegate!(ElvenArcher);
/// ```
///
/// # See Also
///
/// - [`Unit`](crate::unit_trait::Unit) - The trait being implemented
/// - [`BaseUnit`](crate::base_unit::BaseUnit) - The underlying unit implementation
/// - [`Attack`](crate::attack::Attack) - Attack definitions
/// - [`CombatStats`](combat::CombatStats) - Combat statistics structure
#[macro_export]
macro_rules! impl_unit_delegate {
    ($unit_type:ty) => {
        use items::item_properties::ItemProperties;
        impl $crate::unit_trait::Unit for $unit_type {
            // ===== Identity Methods =====
            // These methods provide basic identification and location information for the unit.
            // All delegate directly to the BaseUnit fields.

            fn id(&self) -> $crate::unit_trait::UnitId {
                self.base.id
            }

            fn name(&self) -> &str {
                &self.base.name
            }

            fn position(&self) -> graphics::HexCoord {
                self.base.position
            }

            fn race(&self) -> $crate::unit_race::Race {
                self.base.race
            }

            fn unit_type(&self) -> &str {
                &self.base.unit_type
            }

            fn description(&self) -> &str {
                &self.base.description
            }

            // ===== Movement Methods =====
            // Handles unit movement, including validation and range calculation.
            // Movement is constrained by the unit's movement_speed stat.

            fn move_to(&mut self, position: graphics::HexCoord) -> bool {
                if self.can_move_to(position) {
                    self.base.position = position;
                    true
                } else {
                    false
                }
            }

            // ===== Combat Stats Methods =====
            // Provides access to combat statistics such as health, attack, defense, etc.
            // Both immutable and mutable access is provided for reading and modifying stats.

            fn combat_stats(&self) -> &combat::CombatStats {
                &self.base.combat_stats
            }

            fn combat_stats_mut(&mut self) -> &mut combat::CombatStats {
                &mut self.base.combat_stats
            }

            // ===== Equipment & Inventory Methods =====
            // Manages the unit's equipment slots and inventory system.
            // Equipping/unequipping items automatically triggers stat recalculation
            // to apply item bonuses and modifiers.

            fn equipment(&self) -> &items::Equipment {
                &self.base.equipment
            }

            fn equipment_mut(&mut self) -> &mut items::Equipment {
                &mut self.base.equipment
            }

            fn inventory(&self) -> &[items::Item] {
                &self.base.inventory
            }

            fn inventory_mut(&mut self) -> &mut Vec<items::Item> {
                &mut self.base.inventory
            }

            fn equip_item(&mut self, item_id: items::ItemId) -> Result<(), String> {
                if let Some(pos) = self
                    .base
                    .inventory
                    .iter()
                    .position(|item| item.id == item_id)
                {
                    let item = self.base.inventory.remove(pos);
                    if let Some(old_item) = self.base.equipment.equip_item(item) {
                        self.base.inventory.push(old_item);
                    }
                    self.recalculate_stats();
                    Ok(())
                } else {
                    Err("Item not found in inventory".to_string())
                }
            }

            fn unequip_item(&mut self, item_id: items::ItemId) -> Result<(), String> {
                if let Some(item) = self.base.equipment.unequip_item(item_id) {
                    self.base.inventory.push(item);
                    self.recalculate_stats();
                    Ok(())
                } else {
                    Err("Item not equipped".to_string())
                }
            }

            fn add_item_to_inventory(&mut self, item: items::Item) {
                self.base.inventory.push(item);
            }

            fn remove_item_from_inventory(
                &mut self,
                item_id: items::ItemId,
            ) -> Option<items::Item> {
                if let Some(pos) = self
                    .base
                    .inventory
                    .iter()
                    .position(|item| item.id == item_id)
                {
                    Some(self.base.inventory.remove(pos))
                } else {
                    None
                }
            }

            // ===== Level & Experience Methods =====
            // Handles the progression system including experience gain and leveling up.
            // Experience requirements scale quadratically with level (level² × 100).
            // Leveling up triggers stat recalculation and fully restores health.

            fn level(&self) -> i32 {
                self.base.level
            }

            fn experience(&self) -> i32 {
                self.base.experience
            }

            fn add_experience(&mut self, exp: i32) -> bool {
                self.base.add_experience(exp)
            }

            fn experience_for_next_level(&self) -> i32 {
                self.base.xp_remaining_for_level_up()
            }

            fn level_progress(&self) -> f32 {
                let current_level_exp = BaseUnit::xp_required_for_level(self.base.level);
                let next_level_exp = BaseUnit::xp_required_for_level(self.base.level + 1);
                let progress_exp = self.base.experience - current_level_exp;
                let level_exp_range = next_level_exp - current_level_exp;

                if level_exp_range > 0 {
                    progress_exp as f32 / level_exp_range as f32
                } else {
                    0.0
                }
            }

            fn can_level_up(&self) -> bool {
                self.base.can_level_up()
            }

            fn perform_level_up_evolution(
                &mut self,
                new_stats: combat::CombatStats,
                new_attacks: Vec<$crate::attack::Attack>,
                new_unit_type: String,
                heal_to_full: bool,
            ) {
                self.attacks = self.base.level_up_evolution(
                    new_stats,
                    new_attacks,
                    new_unit_type,
                    heal_to_full,
                );
            }

            fn perform_level_up_incremental(&mut self, heal_to_full: bool) {
                let new_attacks = self.base.level_up_incremental(heal_to_full);
                if !new_attacks.is_empty() {
                    self.attacks = new_attacks;
                }
                // If empty, keep existing attacks
            }

            // ===== Terrain Methods =====
            // Manages the terrain type the unit is currently standing on.
            // Different terrains can provide bonuses or penalties to unit stats,
            // so changing terrain triggers stat recalculation.

            fn current_terrain(&self) -> $crate::unit_race::Terrain {
                self.base.current_terrain
            }

            fn set_terrain(&mut self, terrain: $crate::unit_race::Terrain) {
                self.base.current_terrain = terrain;
                self.recalculate_stats();
            }

            // ===== Utility Methods =====
            // General utility functions for common unit operations.

            fn is_alive(&self) -> bool {
                self.base.combat_stats.is_alive()
            }

            fn can_attack(&self, target_position: graphics::HexCoord) -> bool {
                let distance = self.base.position.distance(target_position);
                distance > 0 && distance <= self.base.combat_stats.attack_range
            }

            fn can_move_to(&self, position: graphics::HexCoord) -> bool {
                let distance = self.base.position.distance(position);
                distance > 0 && distance <= self.base.combat_stats.movement_speed
            }

            fn get_movement_range(&self) -> Vec<graphics::HexCoord> {
                self.base.get_movement_range()
            }

            fn recalculate_stats(&mut self) {
                self.base.recalculate_stats();
            }

            // ===== Display Methods =====
            // Formatting and display functions for UI and debugging purposes.
            // These provide various levels of detail from brief summaries to full info displays.

            fn get_display_color(&self) -> [f32; 3] {
                self.base.race.get_display_color()
            }

            fn sprite(&self) -> graphics::SpriteType {
                // Map race to sprite type
                match self.base.race {
                    $crate::unit_race::Race::Dwarf => graphics::SpriteType::DwarfWarrior,
                    $crate::unit_race::Race::Orc => graphics::SpriteType::OrcWarrior,
                    _ => graphics::SpriteType::Unit, // Fallback for other races
                }
            }

            fn get_info(&self) -> String {
                format!(
                    "{} (Lv.{} {:?} {})\nHP: {}/{}\nATK: {}\nExp: {}/{}",
                    self.name(),
                    self.level(),
                    self.base.race,
                    self.base.unit_type,
                    self.base.combat_stats.health,
                    self.base.combat_stats.max_health,
                    self.base.combat_stats.get_total_attack(),
                    self.experience(),
                    self.experience_for_next_level()
                )
            }

            fn get_summary(&self) -> String {
                format!(
                    "{} L{} HP:{}/{}",
                    self.name(),
                    self.level(),
                    self.base.combat_stats.health,
                    self.base.combat_stats.max_health
                )
            }

            fn display_unit_info(&self) {
                println!("{}", self.get_info());
            }

            fn display_quick_info(&self) {
                println!("{}", self.get_summary());
            }

            fn on_click(&self) {
                println!(
                    "{:?} {} {}",
                    self.base.race,
                    self.base.unit_type,
                    self.name()
                );
            }

            fn take_damage(&mut self, damage: u32) {
                self.base.combat_stats.take_damage(damage as i32);
            }

            fn heal(&mut self, amount: i32) {
                self.base.combat_stats.heal(amount);
            }

            // ===== Attack Methods =====
            // Aggregates all available attacks from multiple sources:
            // 1. The unit's innate attacks (from the attacks field)
            // 2. Attacks provided by the equipped weapon
            // 3. Attacks provided by equipped accessories
            // This allows weapons and items to dynamically extend a unit's combat capabilities.

            fn get_attacks(&self) -> Vec<$crate::attack::Attack> {
                /// Helper function to convert item damage types to combat damage types.
                /// Some item damage types may not map directly and default to Slash.
                fn item_damage_type_to_combat(
                    dt: items::item_properties::DamageType,
                ) -> combat::DamageType {
                    match dt {
                        items::item_properties::DamageType::Slash => combat::DamageType::Slash,
                        items::item_properties::DamageType::Pierce => combat::DamageType::Pierce,
                        items::item_properties::DamageType::Blunt => combat::DamageType::Blunt,
                        items::item_properties::DamageType::Fire => combat::DamageType::Fire,
                        items::item_properties::DamageType::Dark => combat::DamageType::Dark,
                        // Map others to closest or ignore (default to Slash)
                        _ => combat::DamageType::Slash,
                    }
                }

                // Start with the unit's innate attacks
                let mut all_attacks = self.attacks.clone();

                // Add attacks from equipped weapon
                if let Some(weapon) = &self.base.equipment.weapon {
                    if let ItemProperties::Weapon { attacks, .. } = &weapon.properties {
                        for item_attack in attacks {
                            all_attacks.push($crate::attack::Attack::melee(
                                item_attack.name.clone(),
                                item_attack.damage,
                                item_attack.attack_times,
                                item_damage_type_to_combat(item_attack.damage_type),
                            ));
                        }
                    }
                }

                // Add attacks from equipped accessories (if any provide attacks)
                for accessory in &self.base.equipment.accessories {
                    if let ItemProperties::Weapon { attacks, .. } = &accessory.properties {
                        for item_attack in attacks {
                            all_attacks.push($crate::attack::Attack::melee(
                                item_attack.name.clone(),
                                item_attack.damage,
                                item_attack.attack_times,
                                item_damage_type_to_combat(item_attack.damage_type),
                            ));
                        }
                    }
                }

                all_attacks
            }
        }
    };
}
