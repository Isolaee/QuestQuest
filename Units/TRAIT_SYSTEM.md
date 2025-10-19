# Trait-Based Unit System

This document describes the new trait-based unit system implementation in QuestQuest.

## Overview

The unit system has been refactored to use a trait-based architecture where:
- **Unit is a trait** that defines the interface all units must implement
- **BaseUnit** is a struct that holds common data for all units
- **UnitFactory** creates unit instances based on race and class combinations
- **Concrete implementations** (to be developed) will define specific behavior for each race/class combo

## Architecture

### 1. Unit Trait (`unit_trait.rs`)

Defines the interface that all units must implement:

```rust
pub trait Unit {
    // Identity
    fn id(&self) -> UnitId;
    fn name(&self) -> &str;
    fn position(&self) -> HexCoord;
    
    // Combat
    fn attack(&mut self, target: &mut dyn Unit) -> CombatResult;
    fn defend(&mut self, incoming_damage: i32) -> i32;
    fn move_to(&mut self, position: HexCoord) -> bool;
    
    // Stats, equipment, inventory, terrain, etc.
    // ... (see unit_trait.rs for full interface)
}
```

### 2. BaseUnit (`base_unit.rs`)

Common data structure used by all unit implementations:

```rust
pub struct BaseUnit {
    pub id: UnitId,
    pub name: String,
    pub position: HexCoord,
    pub race: Race,
    pub class: UnitClass,
    pub experience: i32,
    pub level: i32,
    pub combat_stats: CombatStats,
    pub equipment: Equipment,
    pub inventory: Vec<Item>,
    // ... cached values
}
```

### 3. UnitFactory (`unit_factory.rs`)

Factory pattern for creating units:

```rust
let unit = UnitFactory::create_unit(
    "Thorin".to_string(),
    HexCoord::new(0, 0),
    Race::Dwarf,
    UnitClass::Warrior,
    Terrain::Mountain,
);

// Returns: Box<dyn Unit>
```

### 4. Concrete Implementations (Future)

Each race/class combination will have its own implementation:

```
Units/src/units/
├── human/
│   ├── human_warrior.rs
│   ├── human_archer.rs
│   └── human_mage.rs
├── elf/
│   ├── elf_warrior.rs
│   ├── elf_archer.rs
│   └── elf_mage.rs
└── dwarf/
    └── ...
```

## Usage

### Creating Units

```rust
use units::{UnitFactory, UnitTrait, Race, UnitClass, Terrain};
use graphics::HexCoord;

// Create a new unit
let warrior = UnitFactory::create_unit(
    "Thorin".to_string(),
    HexCoord::new(0, 0),
    Race::Dwarf,
    UnitClass::Warrior,
    Terrain::Mountain,
);

// Create with level
let mage = UnitFactory::create_unit_with_level(
    "Gandalf".to_string(),
    HexCoord::new(5, 5),
    Race::Human,
    UnitClass::Mage,
    5,    // level
    500,  // experience
    Terrain::Grasslands,
);
```

### Working with Trait Objects

```rust
// Store units polymorphically
let mut units: Vec<Box<dyn UnitTrait>> = vec![
    UnitFactory::create_unit(...),
    UnitFactory::create_unit(...),
];

// Use trait methods
for unit in &units {
    println!("{}", unit.get_summary());
}

// Modify units
for unit in &mut units {
    unit.add_experience(100);
    if unit.can_move_to(target) {
        unit.move_to(target);
    }
}
```

### Trait Methods Examples

```rust
// Movement
if unit.can_move_to(HexCoord::new(2, 2)) {
    unit.move_to(HexCoord::new(2, 2));
}

// Combat
if unit.can_attack(enemy_position) {
    let result = unit.attack(&mut enemy);
}

// Equipment
unit.add_item_to_inventory(sword);
unit.equip_item(sword.id)?;

// Experience
if unit.add_experience(150) {
    println!("Leveled up!");
}

// Display
unit.display_unit_info();
unit.display_quick_info();
```

## Current Implementation Status

### ✅ Completed
- [x] Unit trait definition
- [x] BaseUnit struct with common data
- [x] UnitFactory with creation methods
- [x] GenericUnit implementation (temporary)
- [x] All trait methods implemented
- [x] Backward compatibility maintained (old Unit struct still works)
- [x] All tests passing

### 🔄 In Progress
- [ ] Concrete unit implementations (HumanWarrior, ElfArcher, etc.)
- [ ] Race/class specific abilities
- [ ] Custom attack/defend logic per unit type

### 📋 Planned
- [ ] Migration guide from old Unit struct
- [ ] Performance optimization for trait objects
- [ ] Serialization/deserialization for Box<dyn Unit>
- [ ] Builder pattern for complex unit creation
- [ ] Unit ability system

## Migration from Old System

The old `Unit` struct is still available for backward compatibility:

```rust
// Old way (still works)
use units::Unit;
let unit = Unit::new(...);

// New way (recommended)
use units::{UnitFactory, UnitTrait};
let unit = UnitFactory::create_unit(...);
```

Both systems can coexist during the migration period.

## Benefits

1. **Type Safety**: Each race/class combination is a distinct type
2. **Polymorphism**: Store different unit types in same collection
3. **Customization**: Each combo can have unique behavior
4. **Extensibility**: Easy to add new races/classes
5. **Testability**: Can mock Unit trait for testing
6. **Flexibility**: Race/class combinations define behavior at compile time

## Examples

See `Units/examples/trait_demo.rs` for a complete working example:

```bash
cargo run --example trait_demo -p units
```

## Next Steps

To complete the trait-based system:

1. Implement concrete unit types (e.g., `HumanWarrior`)
2. Update UnitFactory to return concrete types
3. Add race/class specific abilities
4. Migrate existing code to use UnitFactory
5. Remove GenericUnit implementation
6. Update documentation and examples

## API Reference

For full API details, see:
- `unit_trait.rs` - Trait definition
- `base_unit.rs` - Common data structure
- `unit_factory.rs` - Factory methods
