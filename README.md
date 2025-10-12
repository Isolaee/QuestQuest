# QuestQuest: Hexagonal Game Engine

A complete Rust-based hexagonal grid game engine with comprehensive unit management system.

## 🏗️ Project Structure

```
QuestQuest/
├── Graphics/           # Rendering and hexagonal grid system
│   ├── src/
│   │   ├── lib.rs      # Main library entry point
│   │   ├── main.rs     # Standalone graphics demo
│   │   ├── core/       # Core game logic
│   │   │   ├── mod.rs
│   │   │   ├── camera.rs     # Camera with view culling
│   │   │   ├── grid.rs       # Hexagonal grid management
│   │   │   └── hexagon.rs    # Hexagonal coordinate system
│   │   ├── math/       # Mathematical utilities
│   │   │   ├── mod.rs
│   │   │   └── vec2.rs       # 2D vector operations
│   │   └── rendering/  # OpenGL rendering layer
│   │       ├── mod.rs
│   │       ├── renderer.rs   # Main renderer
│   │       ├── shaders.rs    # Shader management
│   │       └── vertex_buffer.rs # Vertex buffer handling
│   └── Cargo.toml
├── Units/              # Game unit system
│   ├── src/
│   │   ├── lib.rs      # Unit system library entry
│   │   ├── combat.rs   # Combat mechanics and stats
│   │   ├── item.rs     # Item and equipment system
│   │   ├── race.rs     # Character races
│   │   ├── unit_class.rs # Character classes
│   │   └── unit.rs     # Main unit implementation
│   ├── tests/          # Comprehensive test suite
│   │   ├── combat_tests.rs
│   │   ├── integration_tests.rs
│   │   └── item_tests.rs
│   └── Cargo.toml
├── example.rs          # Demonstration program
└── Cargo.toml          # Workspace configuration
```

## 🎮 Features

### Graphics Crate
- **OpenGL 4.x Rendering**: Modern OpenGL with programmable shaders
- **Hexagonal Grid System**: Efficient axial coordinate system with pointy-top hexagons
- **Camera System**: View frustum culling for performance optimization
- **Sprite Support**: Colored sprite rendering on hexagonal tiles
- **Modular Architecture**: Clean separation of concerns with core, math, and rendering modules

### Units Crate
- **Complete Unit System**: Units with position, race, class, level, and experience
- **Combat Mechanics**: Attack, defense, health, movement, and range calculations
- **Equipment System**: Weapons, armor, and accessories with stat bonuses
- **Item Management**: Inventory system with consumables and equipment
- **Character Progression**: Experience-based leveling with stat increases
- **Race & Class System**: Multiple races and classes with unique bonuses

### Key Features
- ✅ **Hexagonal Coordinates**: Proper axial coordinate system with distance calculations
- ✅ **Camera Culling**: Only render hexagons within camera view distance
- ✅ **Equipment Bonuses**: Weapons and armor modify unit stats
- ✅ **Range Modifiers**: Equipment can extend attack range
- ✅ **Serialization**: Full serde support for save/load functionality
- ✅ **Comprehensive Tests**: 25 unit tests covering all functionality
- ✅ **Type Safety**: Strong typing throughout with custom types
- ✅ **Builder Pattern**: Flexible unit creation with UnitBuilder

## 🧪 Test Coverage

**Units Crate Tests: 25 tests, 100% passing**
- **Combat Tests (8 tests)**: Damage calculation, healing, leveling, range mechanics
- **Integration Tests (10 tests)**: End-to-end unit functionality, cross-system integration  
- **Item Tests (7 tests)**: Equipment system, inventory management, consumables

## 🚀 Running the Project

### Build Everything
```bash
cd QuestQuest
cargo build --workspace
```

### Run Tests
```bash
# All tests
cargo test --workspace

# Specific test suites
cd Units
cargo test --test combat_tests
cargo test --test integration_tests
cargo test --test item_tests
```

### Run Demo
```bash
# Text-based game mechanics demo
cargo run --bin example

# Graphics rendering demo (requires graphics drivers)
cd Graphics
cargo run
```

## 🎯 Demo Output Example

```
🎮 QuestQuest: Hexagonal Game Engine Demo
==========================================

⚔️ INITIAL UNITS:
📋 Thorin the Bold (Lv.1 Dwarf Warrior):
   Position: HexCoord { q: 0, r: 0 } | Health: 120/120 | Attack: 2 | Defense: 5 | Range: 1 (Melee)
   Movement: 2 | Weapon: None | Exp: 0

📋 Legolas Greenleaf (Lv.1 Elf Archer):
   Position: HexCoord { q: 3, r: -2 } | Health: 80/80 | Attack: 2 | Defense: -1 | Range: 3 (Ranged)
   Movement: 5 | Weapon: None | Exp: 0

🛡️ AFTER EQUIPPING WEAPONS:
📋 Legolas Greenleaf (Lv.1 Elf Archer):
   Position: HexCoord { q: 3, r: -2 } | Health: 80/80 | Attack: 5 | Defense: -1 | Range: 5 (Ranged)
   Movement: 5 | Weapon: Bow of the Galadhrim | Exp: 0

⚔️ COMBAT SIMULATION:
Distance from Archer to Warrior: 3
Can Archer attack Warrior? true
Archer shoots at Warrior for 1 damage!
```

## 🏛️ Architecture Highlights

### Modular Design
- **Graphics**: Handles all rendering, hexagonal math, and camera systems
- **Units**: Manages game logic, unit stats, equipment, and combat
- **Clear Separation**: Graphics focuses on rendering, Units focuses on game mechanics
- **Type Safety**: Custom types prevent coordinate system errors

### Hexagonal Mathematics  
- **Axial Coordinates**: Efficient (q, r) coordinate system
- **Distance Calculation**: Proper hexagonal distance using axial coordinates
- **Neighbor Finding**: Calculate adjacent hexagons in all 6 directions
- **Grid Operations**: Convert between screen and grid coordinates

### Performance Optimizations
- **Camera Culling**: Only process hexagons within view distance
- **Efficient Rendering**: Batch vertex updates and minimize GL calls
- **Smart Caching**: Cache calculated stats until equipment changes
- **Memory Management**: Use appropriate data structures for performance

## 🔧 Technical Stack

- **Language**: Rust 2021 Edition
- **Graphics**: OpenGL 4.x with gl, glutin, winit crates
- **Serialization**: serde with JSON support  
- **UUID**: Unique identifiers for units and items
- **Testing**: Built-in Rust testing with criterion benchmarks
- **Build System**: Cargo workspace for multi-crate management

## 📈 Future Expansion Ideas

- **Pathfinding**: A* algorithm for hexagonal grids
- **Game States**: Turn-based combat system implementation
- **Networking**: Multiplayer support with unit synchronization
- **Save/Load**: Game state persistence using serde
- **AI**: Computer-controlled unit behavior
- **Graphics**: 3D rendering and advanced visual effects
- **Audio**: Sound effects and music integration
- **UI**: Game interface and unit selection
- **Scripting**: Lua or similar for moddable game logic

This project demonstrates a solid foundation for a hex-based strategy game with clean architecture, comprehensive testing, and room for future expansion.