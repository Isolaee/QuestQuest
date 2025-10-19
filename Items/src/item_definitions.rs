use crate::{DamageType, Item, ItemAttack, ItemProperties};

/// Create the Iron Sword item
pub fn create_iron_sword() -> Item {
    Item::new(
        "Iron Sword".to_string(),
        "A sturdy iron sword with a sharp blade.".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 1,
            range_modifier: 0,
            range_type_override: None,
            attacks: vec![ItemAttack::new("Slash", 8, 2, DamageType::Slash)],
        },
    )
}

// TODO: Add more item definitions here
// Examples:
// pub fn create_steel_axe() -> Item { ... }
// pub fn create_leather_armor() -> Item { ... }
// pub fn create_health_potion() -> Item { ... }
