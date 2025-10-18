use game::*;
use graphics::HexCoord;
use units::{item::ItemProperties, Item, Race, Terrain, Unit, UnitClass};

fn main() {
    println!("🎮 Game Object System Demo");
    println!("==========================\n");

    // Create a new game world
    let mut world = GameWorld::new(5); // 5-hex radius world

    println!("🌍 Generating world terrain...");
    world.generate_terrain();
    println!("   Generated {} terrain tiles", world.terrain().len());

    // Create some units
    let warrior_unit = Unit::new(
        "Thorin Ironshield".to_string(),
        HexCoord::new(0, 0),
        Race::Dwarf,
        UnitClass::Warrior,
        Terrain::Mountain,
    );

    let archer_unit = Unit::new(
        "Legolas Greenleaf".to_string(),
        HexCoord::new(2, -1),
        Race::Elf,
        UnitClass::Archer,
        Terrain::Forest0,
    );

    // Wrap them as game objects
    let mut game_warrior = GameUnit::new(warrior_unit);
    let mut game_archer = GameUnit::new(archer_unit);

    println!("⚔️ Created game units:");
    println!(
        "   {} at {:?}",
        game_warrior.unit().name,
        game_warrior.position()
    );
    println!(
        "   {} at {:?}",
        game_archer.unit().name,
        game_archer.position()
    );

    // Add units to the world
    let warrior_id = world.add_unit(game_warrior);
    let archer_id = world.add_unit(game_archer);

    // Create some interactive objects (item pickups)
    let sword = Item::new(
        "Orcrist".to_string(),
        "An ancient elvish blade".to_string(),
        ItemProperties::Weapon {
            attack_bonus: 5,
            range_modifier: 0,
            range_type_override: None,
        },
    );

    let item_pickup = InteractiveObject::new_item_pickup(HexCoord::new(1, 0), sword);
    let item_id = world.add_interactive_object(item_pickup);

    println!("💎 Created item pickup at {:?}", HexCoord::new(1, 0));

    // Demonstrate world queries
    println!("\n🔍 World Analysis:");

    // Check terrain at various positions
    for q in -2..=2 {
        for r in -2..=2 {
            let coord = HexCoord::new(q, r);
            if let Some(terrain) = world.get_terrain(coord) {
                println!(
                    "   {:?}: {:?} (movement cost: {:.1})",
                    coord,
                    terrain.sprite_type(),
                    terrain.movement_cost()
                );
            }
        }
    }

    // Test movement validation
    println!("\n🚶 Movement Testing:");
    let test_position = HexCoord::new(1, 1);
    let can_move = world.is_position_valid_for_movement(test_position, Some(warrior_id));
    println!("   Can warrior move to {:?}? {}", test_position, can_move);

    // Try to move the warrior
    match world.move_unit(warrior_id, test_position) {
        Ok(()) => println!("   ✅ Warrior moved to {:?}", test_position),
        Err(e) => println!("   ❌ Move failed: {}", e),
    }

    // Simulate game updates
    println!("\n⏰ Simulating game time...");
    for i in 1..=3 {
        world.update(1.0); // 1 second per update
        println!("   Update {}: Game time = {:.1}s", i, world.game_time());

        // Check if warrior can act
        if let Some(warrior) = world.get_unit(warrior_id) {
            println!(
                "     Warrior can act: {}",
                warrior.can_act(world.game_time())
            );
        }
    }

    // Test object interactions at positions
    println!("\n🎯 Object Interactions:");
    let positions_to_check = vec![
        HexCoord::new(0, 0),
        HexCoord::new(1, 0),  // Item pickup location
        HexCoord::new(2, -1), // Archer location
    ];

    for pos in positions_to_check {
        // Collect all objects at the given position
        let mut objects: Vec<&dyn GameObject> = Vec::new();
        for unit in world.units().values() {
            if unit.position() == pos {
                objects.push(unit as &dyn GameObject);
            }
        }
        for obj in world.interactive_objects().values() {
            if obj.position() == pos {
                objects.push(obj as &dyn GameObject);
            }
        }
        println!("   Position {:?} contains {} objects:", pos, objects.len());
        for (i, obj) in objects.iter().enumerate() {
            println!(
                "     {}: {} ({})",
                i + 1,
                obj.type_name(),
                if obj.blocks_movement() {
                    "blocks"
                } else {
                    "passable"
                }
            );
        }
    }

    // Demonstrate terrain modification
    println!("\n🏗️ Terrain Modification:");
    if let Some(terrain) = world.get_terrain_mut(HexCoord::new(0, 1)) {
        let old_cost = terrain.movement_cost();
        terrain.set_movement_cost(0.5); // Make it easier to traverse
        terrain.set_metadata("modified".to_string(), "true".to_string());
        println!(
            "   Modified terrain at (0,1): cost {} -> {}",
            old_cost,
            terrain.movement_cost()
        );
        println!("   Added metadata: {:?}", terrain.get_metadata("modified"));
    }

    println!("\n✅ Game Object System Demo Complete!");
    println!("   - Created {} terrain tiles", world.terrain().len());
    println!("   - Created {} units", world.units().len());
    println!(
        "   - Created {} interactive objects",
        world.interactive_objects().len()
    );
}
