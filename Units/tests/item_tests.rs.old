use graphics::HexCoord;
use units::*;

// Import the old Unit struct for these tests (not yet migrated)
use units::unit::Unit as LegacyUnit;

#[test]
fn test_item_creation() {
    let weapon = Item::new(
        "Test Sword".to_string(),
        "A basic sword for testing".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 5,
            range_modifier: 0,
            range_type_override: None,
        },
    );

    assert_eq!(weapon.name, "Test Sword");
    assert_eq!(weapon.description, "A basic sword for testing");

    if let ItemProperties::Weapon { attack_bonus, .. } = weapon.properties {
        assert_eq!(attack_bonus, 5);
    } else {
        panic!("Expected weapon properties");
    }
}

#[test]
fn test_armor_properties() {
    let armor = Item::new(
        "Chain Mail".to_string(),
        "Protective chain mail".to_string(),
        ItemProperties::Armor {
            defense_bonus: 3,
            movement_penalty: 1,
        },
    );

    if let ItemProperties::Armor {
        defense_bonus,
        movement_penalty,
    } = armor.properties
    {
        assert_eq!(defense_bonus, 3);
        assert_eq!(movement_penalty, 1);
    } else {
        panic!("Expected armor properties");
    }
}

#[test]
fn test_consumable_item() {
    let potion = Item::new(
        "Health Potion".to_string(),
        "Restores health when consumed".to_string(),
        ItemProperties::Consumable {
            uses: 3,
            effect: ConsumableEffect::Heal { amount: 50 },
        },
    );

    if let ItemProperties::Consumable { effect, uses } = potion.properties {
        assert_eq!(uses, 3);
        if let ConsumableEffect::Heal { amount } = effect {
            assert_eq!(amount, 50);
        } else {
            panic!("Expected health heal effect");
        }
    } else {
        panic!("Expected consumable properties");
    }
}

#[test]
fn test_inventory_management() {
    let position = HexCoord::new(0, 0);
    let mut unit = LegacyUnit::new(
        "Test Unit".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let sword = Item::new(
        "Iron Sword".to_string(),
        "A sturdy iron sword".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 3,
            range_modifier: 0,
            range_type_override: None,
        },
    );

    let sword_id = sword.id;
    unit.add_item_to_inventory(sword);

    assert_eq!(unit.inventory.len(), 1);
    assert!(unit.inventory.iter().any(|item| item.id == sword_id));
}

#[test]
fn test_equipment_system() {
    let position = HexCoord::new(0, 0);
    let mut unit = LegacyUnit::new(
        "Test Unit".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    let sword = Item::new(
        "Iron Sword".to_string(),
        "A sturdy iron sword".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 3,
            range_modifier: 0,
            range_type_override: None,
        },
    );

    let armor = Item::new(
        "Leather Armor".to_string(),
        "Basic leather protection".to_string(),
        ItemProperties::Armor {
            defense_bonus: 2,
            movement_penalty: 0,
        },
    );

    let sword_id = sword.id;
    let armor_id = armor.id;

    unit.add_item_to_inventory(sword);
    unit.add_item_to_inventory(armor);

    // Test weapon equipment
    let result = unit.equip_item(sword_id);
    assert!(result.is_ok());
    assert!(unit.equipment.weapon.as_ref().map(|w| w.id) == Some(sword_id));

    // Test armor equipment
    let result = unit.equip_item(armor_id);
    assert!(result.is_ok());
    assert!(unit.equipment.armor.as_ref().map(|a| a.id) == Some(armor_id));
}

#[test]
fn test_consumable_usage() {
    let position = HexCoord::new(0, 0);
    let mut unit = LegacyUnit::new(
        "Test Unit".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior,
        Terrain::Grasslands,
    );

    // Damage the unit first
    unit.combat_stats.health = unit.combat_stats.max_health / 2;
    let damaged_health = unit.combat_stats.health;

    let potion = Item::new(
        "Health Potion".to_string(),
        "Restores health when consumed".to_string(),
        ItemProperties::Consumable {
            uses: 1,
            effect: ConsumableEffect::Heal { amount: 25 },
        },
    );

    let potion_id = potion.id;
    unit.add_item_to_inventory(potion);

    // Use the consumable (simulate manual use for now since method doesn't exist)
    // Find the potion and apply its effect
    if let Some(potion_pos) = unit.inventory.iter().position(|item| item.id == potion_id) {
        if let ItemProperties::Consumable { effect, .. } = &unit.inventory[potion_pos].properties {
            if let ConsumableEffect::Heal { amount } = effect {
                unit.heal(*amount);
            }
        }
        // Remove the item after use
        unit.inventory.remove(potion_pos);
    }

    // Check that health was restored
    assert!(unit.combat_stats.health > damaged_health);

    // Check that the item was consumed (removed from inventory)
    assert!(!unit.inventory.iter().any(|item| item.id == potion_id));
}

#[test]
fn test_weapon_range_override() {
    let position = HexCoord::new(0, 0);
    let mut unit = LegacyUnit::new(
        "Test Unit".to_string(),
        position,
        Race::Human,
        UnitClass::Warrior, // Normally melee
        Terrain::Grasslands,
    );

    let throwing_weapon = Item::new(
        "Throwing Spear".to_string(),
        "A spear that can be thrown".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 2,
            range_modifier: 1,
            range_type_override: Some(RangeType::Ranged),
        },
    );

    let weapon_id = throwing_weapon.id;
    unit.add_item_to_inventory(throwing_weapon);
    let _ = unit.equip_item(weapon_id);

    // Should now have ranged attacks due to weapon override
    assert_eq!(unit.combat_stats.range_type, RangeType::Ranged);
    assert!(unit.can_attack(HexCoord::new(3, 0))); // Should be able to attack at range
}
