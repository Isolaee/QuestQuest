//! Core unit trait and related types.
//!
//! This module defines the main [`Unit`] trait that all units in the game must implement.
//! The trait provides a unified interface for unit identity, movement, combat, inventory
//! management, and progression.

use crate::attack::Attack;
use crate::unit_race::{Race, Terrain};
use crate::unit_type::UnitType;
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
/// # Implementation Requirements
///
/// Units must implement only these required methods:
/// - `base()` - Returns a reference to the BaseUnit
/// - `base_mut()` - Returns a mutable reference to the BaseUnit
/// - `attacks()` - Returns the unit's attack list
///
/// All other methods have default implementations that delegate to the BaseUnit.
pub trait Unit {
    // ===== Core Required Methods =====
    // These MUST be implemented by each unit type

    /// Returns a reference to the underlying BaseUnit.
    ///
    /// This is the only required method for basic unit implementation.
    /// All default implementations use this to access shared data.
    fn base(&self) -> &crate::base_unit::BaseUnit;

    /// Returns a mutable reference to the underlying BaseUnit.
    ///
    /// This allows default implementations to modify shared state.
    fn base_mut(&mut self) -> &mut crate::base_unit::BaseUnit;

    /// Returns the unit's innate attacks (before equipment bonuses).
    ///
    /// Units must implement this to define their natural attacks.
    /// Equipment attacks are automatically added by `get_attacks()`.
    fn attacks(&self) -> &[Attack];

    // ===== Identity Methods (Default Implementations) =====

    /// Returns the unit's unique identifier.
    ///
    /// This UUID persists for the lifetime of the unit and can be used
    /// to track the unit across saves and network synchronization.
    fn id(&self) -> UnitId {
        self.base().id
    }

    /// Returns the unit's display name.
    fn name(&self) -> &str {
        &self.base().name
    }

    /// Returns the unit's current position on the hex grid.
    fn position(&self) -> HexCoord {
        self.base().position
    }

    /// Returns the unit's race.
    ///
    /// Race affects terrain bonuses, base stats, and visual appearance.
    fn race(&self) -> Race {
        self.base().race
    }

    /// Returns the unit's type identifier.
    ///
    /// This is a string like "Human Warrior", "Elf Archer", or "Goblin Grunt"
    /// that identifies the specific unit template.
    fn unit_type(&self) -> &str {
        &self.base().unit_type
    }

    /// Returns the unit's description for wiki/gameplay purposes.
    ///
    /// This description provides lore, tactical information, and gameplay context
    /// about the unit that will be displayed in game wikis and tooltips.
    fn description(&self) -> &str {
        &self.base().description
    }

    // ===== Movement Methods (Default Implementations) =====

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
    fn move_to(&mut self, position: HexCoord) -> bool {
        if self.can_move_to(position) {
            self.base_mut().position = position;
            true
        } else {
            false
        }
    }

    // ===== Stats Access Methods (Default Implementations) =====

    /// Returns a reference to the unit's combat statistics.
    ///
    /// Combat stats include health, attack power, defense, movement speed,
    /// resistances, and other combat-relevant values.
    fn combat_stats(&self) -> &CombatStats {
        &self.base().combat_stats
    }

    /// Returns a mutable reference to the unit's combat statistics.
    ///
    /// Use this to modify health, apply buffs/debuffs, or update other combat values.
    fn combat_stats_mut(&mut self) -> &mut CombatStats {
        &mut self.base_mut().combat_stats
    }

    // ===== Equipment & Inventory Methods (Default Implementations) =====

    /// Returns a reference to the unit's equipped items.
    ///
    /// Equipment includes weapons, armor, accessories, and other worn items
    /// that provide stat bonuses.
    fn equipment(&self) -> &Equipment {
        &self.base().equipment
    }

    /// Returns a mutable reference to the unit's equipped items.
    fn equipment_mut(&mut self) -> &mut Equipment {
        &mut self.base_mut().equipment
    }

    /// Returns a reference to the unit's inventory.
    ///
    /// The inventory contains all items the unit is carrying but not currently equipped.
    fn inventory(&self) -> &[Item] {
        &self.base().inventory
    }

    /// Returns a mutable reference to the unit's inventory.
    fn inventory_mut(&mut self) -> &mut Vec<Item> {
        &mut self.base_mut().inventory
    }

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
    fn equip_item(&mut self, item_id: ItemId) -> Result<(), String> {
        let base = self.base_mut();
        if let Some(pos) = base.inventory.iter().position(|item| item.id == item_id) {
            let item = base.inventory.remove(pos);
            if let Some(old_item) = base.equipment.equip_item(item) {
                base.inventory.push(old_item);
            }
            base.recalculate_stats();
            Ok(())
        } else {
            Err("Item not found in inventory".to_string())
        }
    }

    /// Unequips an item and moves it to the inventory.
    ///
    /// # Arguments
    ///
    /// * `item_id` - The unique identifier of the equipped item to remove
    ///
    /// # Errors
    ///
    /// Returns an error if the item is not currently equipped.
    fn unequip_item(&mut self, item_id: ItemId) -> Result<(), String> {
        let base = self.base_mut();
        if let Some(item) = base.equipment.unequip_item(item_id) {
            base.inventory.push(item);
            base.recalculate_stats();
            Ok(())
        } else {
            Err("Item not equipped".to_string())
        }
    }

    /// Adds an item to the unit's inventory.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add
    fn add_item_to_inventory(&mut self, item: Item) {
        self.base_mut().inventory.push(item);
    }

    /// Removes an item from the inventory and returns it.
    ///
    /// # Arguments
    ///
    /// * `item_id` - The unique identifier of the item to remove
    ///
    /// # Returns
    ///
    /// Returns `Some(item)` if the item was found and removed, `None` otherwise.
    fn remove_item_from_inventory(&mut self, item_id: ItemId) -> Option<Item> {
        let base = self.base_mut();
        if let Some(pos) = base.inventory.iter().position(|item| item.id == item_id) {
            Some(base.inventory.remove(pos))
        } else {
            None
        }
    }

    // ===== Level & Experience Methods (Default Implementations) =====

    /// Returns the unit's current level.
    ///
    /// Level affects base stats and unlocks new abilities.
    fn level(&self) -> i32 {
        self.base().level
    }

    /// Returns the unit's current experience points.
    fn experience(&self) -> i32 {
        self.base().experience
    }

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
    fn add_experience(&mut self, exp: i32) -> bool {
        let next_level = self.level() + 1;
        let xp_threshold = self.xp_required_for_level(next_level);
        self.base_mut().experience += exp;
        self.base().experience >= xp_threshold
    }

    /// Returns the experience required to reach a specific level.
    ///
    /// Override this method in your unit implementation to customize XP thresholds.
    /// Default implementation uses quadratic progression: level² × 50
    ///
    /// # Arguments
    ///
    /// * `level` - The target level
    ///
    /// # Returns
    ///
    /// Total experience required to reach that level
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use units::{Unit, UnitFactory};
    /// # let unit = UnitFactory::create("Dwarf Young Warrior", None, None).unwrap();
    /// // Default: level² × 50
    /// assert_eq!(unit.xp_required_for_level(2), 200);  // Level 2: 200 XP
    /// assert_eq!(unit.xp_required_for_level(3), 450);  // Level 3: 450 XP
    /// assert_eq!(unit.xp_required_for_level(4), 800);  // Level 4: 800 XP
    /// ```
    fn xp_required_for_level(&self, level: i32) -> i32 {
        if level <= 1 {
            return 0;
        }
        // Default: Quadratic progression
        level * level * 50
    }

    /// Returns the experience required to reach the next level.
    fn experience_for_next_level(&self) -> i32 {
        let next_level = self.level() + 1;
        let current_xp = self.experience();
        let next_level_xp = self.xp_required_for_level(next_level);
        (next_level_xp - current_xp).max(0)
    }

    /// Returns the progress toward the next level as a percentage.
    ///
    /// # Returns
    ///
    /// A value between 0.0 and 1.0, where 0.0 is no progress and 1.0 is ready to level up.
    fn level_progress(&self) -> f32 {
        let base = self.base();
        let current_level_exp = self.xp_required_for_level(base.level);
        let next_level_exp = self.xp_required_for_level(base.level + 1);
        let progress_exp = base.experience - current_level_exp;
        let level_exp_range = next_level_exp - current_level_exp;

        if level_exp_range > 0 {
            progress_exp as f32 / level_exp_range as f32
        } else {
            0.0
        }
    }

    /// Checks if the unit has enough XP to level up.
    fn can_level_up(&self) -> bool {
        let next_level = self.level() + 1;
        let xp_threshold = self.xp_required_for_level(next_level);
        self.base().experience >= xp_threshold
    }

    /// Performs a level-up with evolution to next unit type.
    ///
    /// Automatically updates attacks in BaseUnit, no override needed.
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
    ) {
        self.base_mut()
            .level_up_evolution(new_stats, new_attacks, new_unit_type, heal_to_full);
    }

    /// Performs incremental level-up for max-level units (no evolution).
    /// Grants +2 max HP and +1 attack. Attacks stay the same.
    ///
    /// # Arguments
    ///
    /// * `heal_to_full` - Whether to restore unit to full health
    fn perform_level_up_incremental(&mut self, heal_to_full: bool) {
        self.base_mut().level_up_incremental(heal_to_full);
    }

    // ===== Terrain Methods (Default Implementations) =====

    /// Calculate terrain hit chance for the given terrain.
    ///
    /// Terrain affects combat bonuses and hit chances. This should be called
    /// when determining combat effectiveness based on the terrain at the unit's
    /// current map position.
    ///
    /// # Arguments
    ///
    /// * `terrain` - The terrain type at the unit's position
    ///
    /// # Returns
    ///
    /// The hit chance percentage (0-100) for the given terrain
    fn get_terrain_hit_chance(&self, terrain: Terrain) -> u8 {
        self.base().get_terrain_hit_chance(terrain)
    }

    // ===== Utility Methods (Default Implementations) =====

    /// Checks if the unit is alive.
    ///
    /// # Returns
    ///
    /// Returns `true` if the unit's health is greater than 0, `false` otherwise.
    fn is_alive(&self) -> bool {
        self.base().combat_stats.is_alive()
    }

    /// Checks if the unit can attack a target at the given position.
    fn can_attack(&self, target_position: HexCoord) -> bool {
        let distance = self.base().position.distance(target_position);
        distance > 0 && distance <= self.base().combat_stats.attack_range
    }

    /// Check if unit can move to target position
    fn can_move_to(&self, target: HexCoord) -> bool {
        let distance = self.base().position.distance(target);
        distance > 0 && distance <= self.base().combat_stats.movement_speed
    }

    /// Get all hexagonal coordinates within movement range
    fn get_movement_range(&self) -> Vec<HexCoord> {
        self.base().get_movement_range()
    }

    /// Take damage
    fn take_damage(&mut self, damage: u32) {
        self.base_mut().combat_stats.take_damage(damage as i32);
    }

    /// Heal the unit
    fn heal(&mut self, amount: i32) {
        self.base_mut().combat_stats.heal(amount);
    }

    /// Recalculate all derived stats
    fn recalculate_stats(&mut self) {
        self.base_mut().recalculate_stats();
    }

    // ===== Attack Methods (Default Implementations) =====

    /// Get all available attacks for this unit (including equipped items)
    fn get_attacks(&self) -> Vec<Attack> {
        use items::item_properties::ItemProperties;

        /// Helper function to convert item damage types to combat damage types.
        fn item_damage_type_to_combat(
            dt: items::item_properties::DamageType,
        ) -> combat::DamageType {
            match dt {
                items::item_properties::DamageType::Slash => combat::DamageType::Slash,
                items::item_properties::DamageType::Pierce => combat::DamageType::Pierce,
                items::item_properties::DamageType::Blunt => combat::DamageType::Blunt,
                items::item_properties::DamageType::Fire => combat::DamageType::Fire,
                items::item_properties::DamageType::Dark => combat::DamageType::Dark,
                _ => combat::DamageType::Slash,
            }
        }

        // Start with the unit's innate attacks
        let mut all_attacks = self.attacks().to_vec();

        // Add attacks from equipped weapon
        if let Some(weapon) = &self.base().equipment.weapon {
            if let ItemProperties::Weapon { attacks, .. } = &weapon.properties {
                for item_attack in attacks {
                    all_attacks.push(Attack::melee(
                        item_attack.name.clone(),
                        item_attack.damage,
                        item_attack.attack_times,
                        item_damage_type_to_combat(item_attack.damage_type),
                    ));
                }
            }
        }

        // Add attacks from equipped accessories (if any provide attacks)
        for accessory in &self.base().equipment.accessories {
            if let ItemProperties::Weapon { attacks, .. } = &accessory.properties {
                for item_attack in attacks {
                    all_attacks.push(Attack::melee(
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
    /// The sprite is stored in the BaseUnit and set during unit construction.
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
    /// # let dwarf = UnitFactory::create("Dwarf Warrior", Some("Thorin".to_string()), Some(HexCoord::new(0, 0))).unwrap();
    /// let sprite = dwarf.sprite();
    /// assert_eq!(sprite, SpriteType::DwarfWarrior);
    /// ```
    fn sprite(&self) -> SpriteType {
        // Default implementation reads from BaseUnit's sprite_type field
        self.base().sprite_type
    }

    // ===== Display Methods (Default Implementations) =====

    /// Get unit's display color based on race
    fn get_display_color(&self) -> [f32; 3] {
        self.base().race.get_display_color()
    }

    /// Get detailed unit information for display
    fn get_info(&self) -> String {
        let base = self.base();
        format!(
            "{} (Lv.{} {:?} {})\nHP: {}/{}\nATK: {}\nExp: {}/{}",
            base.name,
            base.level,
            base.race,
            base.unit_type,
            base.combat_stats.health,
            base.combat_stats.max_health,
            base.combat_stats.get_total_attack(),
            base.experience,
            self.experience_for_next_level()
        )
    }

    /// Get a short summary of the unit
    fn get_summary(&self) -> String {
        let base = self.base();
        format!(
            "{} L{} HP:{}/{}",
            base.name, base.level, base.combat_stats.health, base.combat_stats.max_health
        )
    }

    /// Display comprehensive unit information
    fn display_unit_info(&self) {
        println!("{}", self.get_info());
    }

    /// Display quick unit info
    fn display_quick_info(&self) {
        println!("{}", self.get_summary());
    }

    /// Handle click event
    fn on_click(&self) {
        let base = self.base();
        println!("{:?} {} {}", base.race, base.unit_type, base.name);
    }

    // ===== Evolution Methods =====

    /// Get the previous unit type in the evolution chain (if any)
    ///
    /// Returns `Some(unit_type)` if this unit evolved from another type,
    /// `None` if this is the base form in the evolution chain.
    /// The evolution is stored in BaseUnit and set during construction.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use units::{Unit, UnitFactory, Terrain, UnitType};
    /// # use graphics::HexCoord;
    /// # let warrior = UnitFactory::create("Dwarf Warrior", None, None).unwrap();
    /// if let Some(prev) = warrior.evolution_previous() {
    ///     println!("Evolved from: {}", prev);
    /// }
    /// ```
    fn evolution_previous(&self) -> Option<UnitType> {
        self.base().evolution_previous
    }

    /// Get the possible evolution paths for this unit
    ///
    /// Returns a vector of unit types that this unit can evolve into.
    /// Returns empty vector if this is the final form in the evolution chain.
    /// The evolutions are stored in BaseUnit and set during construction.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use units::{Unit, UnitFactory, Terrain, UnitType};
    /// # use graphics::HexCoord;
    /// # let young_warrior = UnitFactory::create("Dwarf Young Warrior", None, None).unwrap();
    /// let evolutions = young_warrior.evolution_next();
    /// for evolution in evolutions {
    ///     println!("Can evolve into: {}", evolution);
    /// }
    /// ```
    fn evolution_next(&self) -> Vec<UnitType> {
        self.base().evolution_next.clone()
    }

    /// Check if this unit has any evolution paths.
    ///
    /// Returns `true` if the unit can evolve to a higher form, `false` if it's at max level.
    /// This is a convenience method that checks if `evolution_next()` returns a non-empty vector.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use units::{Unit, UnitFactory};
    /// # let young_warrior = UnitFactory::create("Dwarf Young Warrior", None, None).unwrap();
    /// if young_warrior.has_next_evolution() {
    ///     println!("This unit can evolve!");
    /// }
    /// ```
    fn has_next_evolution(&self) -> bool {
        !self.evolution_next().is_empty()
    }

    /// Creates an evolved version of this unit, preserving inventory and equipment.
    ///
    /// This method creates a new unit of the next evolution type, transferring:
    /// - Name
    /// - Position
    /// - Current terrain
    /// - All inventory items
    /// - All equipped items
    /// - Current experience (reset to what's appropriate for the new level)
    ///
    /// The new unit will have:
    /// - Increased level (current level + 1)
    /// - Better base stats
    /// - New/improved attacks
    /// - Optionally full health
    ///
    /// # Arguments
    ///
    /// * `heal_to_full` - Whether to restore the evolved unit to full health
    ///
    /// # Returns
    ///
    /// Returns `Some(Box<dyn Unit>)` with the evolved unit, or `None` if:
    /// - This unit has no next evolution (is max level)
    /// - The evolution unit type is not registered
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use units::{Unit, UnitFactory};
    /// # let mut young_warrior = UnitFactory::create("Dwarf Young Warrior", None, None).unwrap();
    /// young_warrior.add_experience(100);
    /// // Choose first evolution path
    /// if let Some(evolved) = young_warrior.evolve(0, true) {
    ///     println!("Evolved to: {}", evolved.unit_type());
    /// }
    /// ```
    fn evolve(&self, evolution_index: usize, heal_to_full: bool) -> Option<Box<dyn Unit>> {
        use crate::unit_factory::UnitFactory;

        // Check if this unit can evolve and get the specific evolution path
        let evolutions = self.evolution_next();
        let next_type = evolutions.get(evolution_index)?;

        // Create the evolved unit with same name and position
        let mut evolved = UnitFactory::create(
            next_type.as_str(),
            Some(self.name().to_string()),
            Some(self.position()),
        )
        .ok()?;

        // Transfer inventory
        for item in self.inventory() {
            evolved.add_item_to_inventory(item.clone());
        }

        // Transfer equipped items
        let old_equipment = self.equipment();
        if let Some(weapon) = &old_equipment.weapon {
            evolved.add_item_to_inventory(weapon.clone());
            let _ = evolved.equip_item(weapon.id);
        }
        if let Some(armor) = &old_equipment.armor {
            evolved.add_item_to_inventory(armor.clone());
            let _ = evolved.equip_item(armor.id);
        }
        for accessory in &old_equipment.accessories {
            evolved.add_item_to_inventory(accessory.clone());
            let _ = evolved.equip_item(accessory.id);
        }

        // Set experience to current (the new unit starts with some XP)
        let current_xp = self.experience();
        evolved.base_mut().experience = current_xp;

        // Optionally heal to full
        if heal_to_full {
            evolved.combat_stats_mut().health = evolved.combat_stats().max_health;
        }

        Some(evolved)
    }

    // ===== Ability Methods (Default Implementations) =====

    /// Get all abilities for this unit
    fn abilities(&self) -> &[crate::ability::Ability] {
        self.base().get_abilities()
    }

    /// Get mutable reference to abilities
    fn abilities_mut(&mut self) -> &mut Vec<crate::ability::Ability> {
        self.base_mut().get_abilities_mut()
    }

    /// Add an ability to the unit
    fn add_ability(&mut self, ability: crate::ability::Ability) {
        self.base_mut().add_ability(ability);
    }

    /// Remove an ability by ID
    fn remove_ability(&mut self, ability_id: crate::ability::AbilityId) -> bool {
        self.base_mut().remove_ability(ability_id)
    }

    /// Find an ability by ID
    fn find_ability(
        &self,
        ability_id: crate::ability::AbilityId,
    ) -> Option<&crate::ability::Ability> {
        self.base().find_ability(ability_id)
    }

    /// Find a mutable ability by ID
    fn find_ability_mut(
        &mut self,
        ability_id: crate::ability::AbilityId,
    ) -> Option<&mut crate::ability::Ability> {
        self.base_mut().find_ability_mut(ability_id)
    }

    /// Get the ability state (active effects and cooldowns)
    fn ability_state(&self) -> &crate::ability::AbilityState {
        &self.base().ability_state
    }

    /// Get mutable reference to ability state
    fn ability_state_mut(&mut self) -> &mut crate::ability::AbilityState {
        &mut self.base_mut().ability_state
    }

    /// Tick abilities (reduce cooldowns and effect durations)
    ///
    /// Should be called at the start of each turn.
    fn tick_abilities(&mut self) {
        self.base_mut().tick_abilities();
    }

    /// Use an active ability
    ///
    /// # Arguments
    ///
    /// * `ability_id` - The ID of the ability to use
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the ability was used successfully, or an error message if:
    /// - The ability doesn't exist
    /// - The ability is not an active ability
    /// - The ability is on cooldown
    fn use_active_ability(&mut self, ability_id: crate::ability::AbilityId) -> Result<(), String> {
        use crate::ability::Ability;

        // Check if ability is on cooldown in ability state
        if self.ability_state().is_on_cooldown(ability_id) {
            let remaining = self.ability_state().get_cooldown(ability_id);
            return Err(format!(
                "Ability on cooldown ({} turns remaining)",
                remaining
            ));
        }

        // Check if ability exists and is active
        let ability = self.find_ability(ability_id).ok_or("Ability not found")?;

        let cooldown_max = match ability {
            Ability::Active(active) => active.cooldown_max,
            _ => return Err("Not an active ability".to_string()),
        };

        // Set cooldown
        self.ability_state_mut()
            .set_cooldown(ability_id, cooldown_max);

        Ok(())
    }

    /// Check if an active ability is ready to use
    fn is_ability_ready(&self, ability_id: crate::ability::AbilityId) -> bool {
        use crate::ability::Ability;

        if let Some(Ability::Active(active)) = self.find_ability(ability_id) {
            active.is_ready() && !self.ability_state().is_on_cooldown(ability_id)
        } else {
            false
        }
    }

    /// Get all passive abilities
    fn get_passive_abilities(&self) -> Vec<&crate::ability::PassiveAbility> {
        use crate::ability::Ability;

        self.abilities()
            .iter()
            .filter_map(|a| {
                if let Ability::Passive(p) = a {
                    Some(p)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all active abilities
    fn get_active_abilities(&self) -> Vec<&crate::ability::ActiveAbility> {
        use crate::ability::Ability;

        self.abilities()
            .iter()
            .filter_map(|a| {
                if let Ability::Active(a) = a {
                    Some(a)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all aura abilities
    fn get_aura_abilities(&self) -> Vec<&crate::ability::AuraAbility> {
        use crate::ability::Ability;

        self.abilities()
            .iter()
            .filter_map(|a| {
                if let Ability::Aura(a) = a {
                    Some(a)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all auras affecting a specific position
    fn get_auras_at_position(&self, target: HexCoord) -> Vec<&crate::ability::AuraAbility> {
        self.get_aura_abilities()
            .into_iter()
            .filter(|aura| aura.is_in_range(self.position(), target))
            .collect()
    }
}
