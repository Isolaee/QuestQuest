/// Macro to implement the common Unit trait methods that just delegate to BaseUnit
/// This eliminates the massive boilerplate in each concrete unit type
///
/// # Usage
/// ```ignore
/// impl_unit_delegate!(DwarfWarrior);
/// ```
///
/// This will implement all the standard Unit trait methods that simply delegate to `self.base`
#[macro_export]
macro_rules! impl_unit_delegate {
    ($unit_type:ty) => {
        impl $crate::unit_trait::Unit for $unit_type {
            // ===== Identity =====

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

            fn class(&self) -> $crate::unit_class::UnitClass {
                self.base.class
            }

            // ===== Movement =====

            fn move_to(&mut self, position: graphics::HexCoord) -> bool {
                if self.can_move_to(position) {
                    self.base.position = position;
                    true
                } else {
                    false
                }
            }

            // ===== Combat Stats =====

            fn combat_stats(&self) -> &combat::CombatStats {
                &self.base.combat_stats
            }

            fn combat_stats_mut(&mut self) -> &mut combat::CombatStats {
                &mut self.base.combat_stats
            }

            // ===== Equipment & Inventory =====

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

            // ===== Level & Experience =====

            fn level(&self) -> i32 {
                self.base.level
            }

            fn experience(&self) -> i32 {
                self.base.experience
            }

            fn add_experience(&mut self, exp: i32) -> bool {
                self.base.experience += exp.max(0);
                let required_exp = self.base.level * self.base.level * 100;

                if self.base.experience >= required_exp {
                    self.base.level += 1;
                    self.recalculate_stats();
                    self.base.combat_stats.health = self.base.combat_stats.max_health;
                    true
                } else {
                    false
                }
            }

            fn experience_for_next_level(&self) -> i32 {
                let required = self.base.level * self.base.level * 100;
                (required - self.base.experience).max(0)
            }

            fn level_progress(&self) -> f32 {
                let current_level_exp = (self.base.level - 1) * (self.base.level - 1) * 100;
                let next_level_exp = self.base.level * self.base.level * 100;
                let progress_exp = self.base.experience - current_level_exp;
                let level_exp_range = next_level_exp - current_level_exp;

                if level_exp_range > 0 {
                    progress_exp as f32 / level_exp_range as f32
                } else {
                    0.0
                }
            }

            // ===== Terrain =====

            fn current_terrain(&self) -> $crate::unit_race::Terrain {
                self.base.current_terrain
            }

            fn set_terrain(&mut self, terrain: $crate::unit_race::Terrain) {
                self.base.current_terrain = terrain;
                self.recalculate_stats();
            }

            // ===== Utility Methods =====

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

            fn get_display_color(&self) -> [f32; 3] {
                self.base.race.get_display_color()
            }

            fn get_info(&self) -> String {
                format!(
                    "{} (Lv.{} {:?} {:?})\nHP: {}/{}\nATK: {}\nExp: {}/{}",
                    self.name(),
                    self.level(),
                    self.base.race,
                    self.base.class,
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
                println!("{:?} {} {}", self.base.race, self.base.class, self.name());
            }

            fn take_damage(&mut self, damage: u32) {
                self.base.combat_stats.take_damage(damage as i32);
            }

            fn heal(&mut self, amount: i32) {
                self.base.combat_stats.heal(amount);
            }

            // ===== Attack Methods =====
            fn get_attacks(&self) -> &[$crate::attack::Attack] {
                // For units with attacks field, this will work
                // For units without, they should add: attacks: Vec::new()
                &self.attacks
            }
        }
    };
}
