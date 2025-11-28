# Units Crate Architecture

## Overview

The Units crate provides a comprehensive, trait-based system for managing game units with combat, equipment, progression, and evolution mechanics. The architecture emphasizes type safety, modularity, and extensibility.

## Core Components

### 1. Unit Trait (`unit_trait.rs`)

The `Unit` trait is the central abstraction that all concrete units must implement:

```rust
pub trait Unit {
    fn base(&self) -> &BaseUnit;
    fn base_mut(&mut self) -> &mut BaseUnit;
    fn attacks(&self) -> &[Attack];
}
```

**Responsibilities:**
- Provides unified interface for all unit operations
- Offers 50+ default methods for combat, movement, inventory, progression
- Enables polymorphism via `Box<dyn Unit>`

**Key Method Categories:**
- **Identity**: `id()`, `name()`, `unit_type()`, `race()`
- **Combat**: `combat_stats()`, `get_attacks()`, `can_attack()`
- **Movement**: `move_to()`, `get_movement_range()`
- **Progression**: `add_experience()`, `can_level_up()`, `level_up()`
- **Evolution**: `evolution_previous()`, `evolution_next()`, `evolve()`
- **Equipment**: `equip_item()`, `unequip_item()`, `equipment()`
- **Inventory**: `add_item_to_inventory()`, `remove_item_from_inventory()`

### 2. BaseUnit (`base_unit.rs`)

The `BaseUnit` struct contains all shared data for units:

```rust
pub struct BaseUnit {
    // Identity
    pub id: UnitId,
    pub name: String,
    pub race: Race,
    pub unit_type: String,
    
    // Progression
    pub level: i32,
    pub experience: i32,
    
    // Combat
    pub combat_stats: CombatStats,
    pub equipment: Equipment,
    
    // Evolution
    pub evolution_previous: Option<UnitType>,
    pub evolution_next: Vec<UnitType>,
    
    // Attacks
    pub attacks: Vec<Attack>,
}
```

**Key Features:**
- Centralized stat caching and recalculation
- Equipment bonus management
- Experience/level progression formulas
- Evolution chain tracking (type-safe with `UnitType` enum)
- Helper methods for attack creation

### 3. UnitType Enum (`unit_type.rs`)

Type-safe enumeration of all unit types:

```rust
pub enum UnitType {
    // Dwarf line
    DwarfYoungWarrior,
    DwarfWarrior,
    DwarfVeteranWarrior,
    
    // Orc line
    OrcYoungSwordsman,
    OrcSwordsman,
    OrcEliteSwordsman,
    
    // Human Noble line
    HumanNoble,
    HumanPrince,
    HumanKing,
    
    // ... and more
}
```

**Benefits:**
- Compile-time safety (no typos)
- IDE autocomplete support
- Pattern matching capability
- Bidirectional conversion: `as_str()` / `from_str()`

## Concrete Unit Implementation Pattern

All units follow this standard pattern:

### Step 1: Define the Struct

```rust
pub struct OrcSwordsman {
    base: BaseUnit,
}
```

### Step 2: Define Constants

```rust
impl OrcSwordsman {
    const LEVEL: i32 = 2;
    const BASE_HEALTH: i32 = 100;
    const BASE_ATTACK: u32 = 18;
    const BASE_MOVEMENT: i32 = 4;
    // ... resistances, race, etc.
}
```

### Step 3: Implement Constructor

```rust
impl OrcSwordsman {
    pub fn new(name: String, position: HexCoord, terrain: Terrain) -> Self {
        let combat_stats = CombatStats::new_with_attacks(
            Self::BASE_HEALTH,
            Self::BASE_ATTACK,
            Self::BASE_MOVEMENT,
            RangeCategory::Melee,
            Resistances::new(/* ... */),
            Self::ATTACK_STRENGTH,
            Self::ATTACKS_PER_ROUND,
        );

        let mut base = BaseUnit::new(
            name,
            position,
            Self::RACE,
            Self::UNIT_TYPE.to_string(),
            description,
            terrain,
            sprite,
            Some(UnitType::OrcYoungSwordsman),  // Previous evolution
            vec![UnitType::OrcEliteSwordsman],  // Next evolution(s)
            combat_stats,
        );

        base.level = Self::LEVEL;
        base.attacks = vec![/* define attacks */];

        Self { base }
    }
}
```

### Step 4: Implement Unit Trait

```rust
impl Unit for OrcSwordsman {
    fn base(&self) -> &BaseUnit { &self.base }
    fn base_mut(&mut self) -> &mut BaseUnit { &mut self.base }
    fn attacks(&self) -> &[Attack] { &self.base.attacks }
}
```

### Step 5: Register with Factory

```rust
crate::submit_unit!(
    OrcSwordsman,
    "Orc Swordsman",
    "Description...",
    Terrain::Grasslands,
    "Orc",
    "Swordsman"
);
```

## Unit Factory System

The factory enables runtime unit creation:

### Dynamic Creation

```rust
// Create by string name
let unit = UnitFactory::create(
    "Orc Swordsman",
    Some("Ugluk".to_string()),
    Some(HexCoord::new(0, 0)),
    Some(Terrain::Grasslands)
)?;
```

### Registry System

Uses the `inventory` crate for compile-time registration:

```rust
// Automatically collected at compile time
inventory::collect!(UnitTypeInfo);

pub struct UnitTypeInfo {
    pub type_name: &'static str,
    pub description: &'static str,
    pub default_terrain: Terrain,
    pub race: &'static str,
    pub class: &'static str,
    pub constructor: fn(String, HexCoord, Terrain) -> Box<dyn Unit>,
}
```

## Evolution System

Units can evolve through multiple tiers:

### Evolution Chain Example

```
DwarfYoungWarrior (L1)
    ↓
DwarfWarrior (L2)
    ↓
DwarfVeteranWarrior (L3)
```

### Branching Evolution

Some units have multiple evolution paths:

```
HumanKnight (L2)
    ├→ HumanKnightCommander (L3)
    └→ HumanGrandKnight (L3)
```

### Evolution Process

```rust
// Check if evolution available
if unit.can_level_up() {
    // Get evolution options
    let evolutions = unit.evolution_next();
    
    // Choose path (0 = first option)
    if let Some(evolved) = unit.evolve(0, true) {
        // New unit with transferred inventory/equipment
        println!("Evolved to: {}", evolved.unit_type());
    }
}
```

### Incremental Leveling

Max-level units without further evolutions gain small stat increases:
- +2 max HP per level
- +1 attack per level

## Progression System

### Experience Requirements

Exact xp requirements are TBD

Quadratic formula: `level² × 50`

| Level | Required XP | Total XP |
|-------|-------------|----------|
| 1→2   | 100         | 100      |
| 2→3   | 250         | 250      |
| 3→4   | 450         | 450      |
| 4→5   | 700         | 700      |

### Level Up Process

```rust
// Add experience
unit.add_experience(150);

// Check if can level up
if unit.can_level_up() {
    // Level up (creates new evolved unit)
    if let Some(evolved) = unit.level_up(true) {
        // Replace old unit with evolved version
        *unit_box = evolved;
    }
}
```

## Combat System

### Attack Structure

```rust
pub struct Attack {
    pub name: String,
    pub damage: u32,
    pub attack_times: u32,
    pub damage_type: DamageType,
    pub range: i32,
}
```

### Attack Types

- **Melee**: Range 1, close combat
- **Ranged**: Range 2-5, projectile attacks
- **Magic**: Range 2-4, elemental damage

### Damage Types

```rust
pub enum DamageType {
    Slash,    // Swords, axes
    Pierce,   // Arrows, spears
    Blunt,    // Hammers, clubs
    Fire,     // Magic fire
    Dark,     // Shadow magic
    Crush,    // Heavy impact
}
```

## Equipment System

### Equipment Slots

```rust
pub struct Equipment {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub accessories: Vec<Item>,  // Multiple slots
}
```

### Stat Bonuses

Equipment provides:
- Attack bonus
- Movement modifier
- Range modifier
- Health bonus
- Range type override

### Auto-recalculation

Stats automatically recalculate on:
- Equipment changes
- Level up
- Terrain changes

## Race System

### Available Races

```rust
pub enum Race {
    Human,
    Dwarf,
    Elf,
    Orc,
    Goblin,
    // ... and more
}
```

### Race Bonuses

Each race provides:
- Movement speed modifier
- Terrain-specific hit chance bonuses
- Base defense values

Example:
```rust
// Dwarves excel in mountains
Race::Dwarf.get_terrain_hit_chance(Terrain::Mountain) // 95%
Race::Dwarf.get_terrain_hit_chance(Terrain::Grasslands) // 75%
```

## Terrain System

### Terrain Types

- **Grasslands**: Standard terrain
- **Forest**: Elves gain bonuses
- **Mountain**: Dwarves gain bonuses
- **Swamp**: Goblins gain bonuses
- **Desert**: Harsh conditions
- **Water**: Movement restricted

### Terrain Effects

```rust
// Hit chance varies by race + terrain
unit.combat_stats().terrain_hit_chance // 0-100%

// Automatically updated on movement
unit.move_to(new_position);
// Stats recalculated with new terrain bonuses
```

## Module Organization

```
Units/
├── src/
│   ├── lib.rs                  # Public exports
│   ├── unit_trait.rs           # Core Unit trait
│   ├── base_unit.rs            # Shared BaseUnit struct
│   ├── unit_type.rs            # UnitType enum
│   ├── unit_factory.rs         # Factory + registry
│   ├── unit_registry.rs        # Registry infrastructure
│   ├── unit_macros.rs          # Helper macros
│   ├── unit_race.rs            # Race definitions
│   ├── attack.rs               # Attack system
│   ├── team.rs                 # Team affiliation
│   ├── combat/                 # Combat subsystem
│   ├── structures/             # Buildings/walls
│   └── units/                  # Concrete implementations
│       ├── dwarf/
│       │   └── dwarf_warrior/
│       │       ├── young_warrior.rs
│       │       ├── warrior.rs
│       │       └── veteran_warrior.rs
│       ├── orc/
│       │   └── orc_swordman/
│       ├── human/
│       │   ├── noble/
│       │   └── knights/
│       ├── elf/
│       └── goblin/
└── tests/
    ├── unit_tests.rs
    ├── unit_type_tests.rs
    └── branching_evolution_tests.rs
```

## Design Patterns

### 1. Trait-Based Polymorphism

All units implement `Unit`, enabling:
```rust
let unit: Box<dyn Unit> = UnitFactory::create("Orc Swordsman", ...)?;
unit.move_to(position);
unit.add_experience(50);
```

### 2. Composition over Inheritance

Units compose `BaseUnit` rather than inheriting:
```rust
struct OrcSwordsman {
    base: BaseUnit,  // Composition
}
```

### 3. Builder Pattern (Implicit)

`BaseUnit::new()` uses many parameters but provides clear intent:
```rust
BaseUnit::new(
    name,
    position,
    race,
    unit_type,
    description,
    terrain,
    sprite,
    evolution_previous,
    evolution_next,
    combat_stats,
)
```

### 4. Factory Pattern

`UnitFactory` abstracts unit creation:
```rust
UnitFactory::create(type_name, ...)  // Dynamic
UnitFactory::create_orc_swordsman(...)  // Static
```

### 5. Registry Pattern

`inventory` crate for compile-time registration:
```rust
submit_unit!(OrcSwordsman, "Orc Swordsman", ...);
// Automatically added to global registry
```

## Best Practices

### When Adding New Units

1. **Create struct** in appropriate race/class folder
2. **Define constants** for all base stats
3. **Implement constructor** following the standard pattern
4. **Implement Unit trait** (3 required methods)
5. **Register with factory** using `submit_unit!` macro
6. **Add to UnitType enum** in `unit_type.rs`
7. **Update evolution chains** for connected units
8. **Write tests** in `tests/` directory

### Stat Balancing Guidelines

- **Level 1**: 30-50 HP, 5-10 attack
- **Level 2**: 70-100 HP, 12-18 attack
- **Level 3**: 120-150 HP, 20-28 attack
- **Movement**: 3-5 tiles (races provide +1 bonus)
- **Range**: Melee=1, Ranged=3-5, Magic=2-4

### Evolution Design

- **Linear chains**: 2-3 tiers (Young → Experienced → Elite)
- **Branching**: Only at tier 2+ (Knight → Commander/Grand Knight)
- **XP scaling**: Use default formula unless starter unit
- **Stat growth**: 40-50% increase per tier

## Testing Strategy

### Unit Tests (`tests/unit_type_tests.rs`)

- String conversion (as_str/from_str)
- Roundtrip testing
- Evolution chain validation

### Integration Tests (`tests/unit_tests.rs`)

- Factory creation
- Stat verification
- Race bonuses
- Evolution mechanics

### Example Test

```rust
#[test]
fn test_orc_evolution_chain() {
    let young = UnitFactory::create("Orc Young Swordsman", ...)?;
    
    // Check evolution
    assert_eq!(young.evolution_previous(), None);
    assert_eq!(young.evolution_next(), vec![UnitType::OrcSwordsman]);
    
    // Level up
    young.add_experience(100);
    let swordsman = young.evolve(0, true)?;
    
    // Verify
    assert_eq!(swordsman.level(), 2);
    assert_eq!(swordsman.unit_type(), "Orc Swordsman");
}
```

## Future Enhancements

### Potential Improvements

1. **Abilities System**: Special powers beyond basic attacks
2. **Trait System**: Passive bonuses (e.g., "Berserker", "Healer")
3. **Formation Bonuses**: Adjacent unit synergies
4. **Morale System**: Performance based on battle state
5. **Veterancy**: Permanent bonuses from combat experience
6. **Equipment Sets**: Bonuses for wearing matching items
7. **Dynamic Stats**: Buffs/debuffs with duration
8. **AI Behavior**: Personality traits affecting decisions

### Extensibility Points

The system is designed for extension:

- **New races**: Add to `Race` enum, implement terrain bonuses
- **New damage types**: Extend `DamageType`, update resistances
- **New equipment slots**: Expand `Equipment` struct
- **Custom XP formulas**: Override `xp_required_for_level()`
- **Special attacks**: Add to unit's `attacks` vector
- **Unique mechanics**: Override trait methods in concrete units

## Performance Considerations

### Optimizations

1. **Stat Caching**: `cached_*` fields avoid recalculation
2. **Lazy Evaluation**: Stats recalculated only when needed
3. **Copy-on-Write**: Cloning units is cheap (most data is primitive)
4. **Registry Pattern**: O(1) lookup by unit type name

### Memory Footprint

Typical unit size: ~500-800 bytes
- BaseUnit: ~400 bytes
- Combat stats: ~100 bytes
- Equipment: ~50-200 bytes (depending on items)

### Scalability

- Hundreds of units: No issues
- Thousands of units: Consider spatial partitioning for movement
- Evolution operations: O(1) with type-safe enum

## Debugging Tips

### Common Issues

**Problem**: Stats not updating after equipment change
**Solution**: Call `unit.base_mut().recalculate_stats()`

**Problem**: Evolution fails with "Unknown unit type"
**Solution**: Ensure unit is registered with `submit_unit!` macro

**Problem**: Movement validation fails
**Solution**: Check terrain type and race bonuses

**Problem**: XP not triggering level up
**Solution**: Use `can_level_up()` to check, verify XP formula

### Debug Helpers

```rust
// Print full unit state
println!("{:#?}", unit.base());

// Check evolution chain
dbg!(unit.evolution_previous(), unit.evolution_next());

// Verify stats
dbg!(unit.combat_stats());

// Test movement range
dbg!(unit.get_movement_range());
```

---

**Last Updated**: November 28, 2025
**Version**: 0.1.0
**Maintainer**: QuestQuest Team
