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
- **Terrain System**: 7 distinct terrain sprites (Forest, Forest2, Grasslands, Haunted Woods, Hills, Mountain, Swamp)
- **Sprite Support**: Colored sprite rendering on hexagonal tiles with procedural distribution
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
- ✅ **Terrain Sprites**: 7 distinct terrain types with procedural distribution
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

# Graphics rendering demo (requires OpenGL 4.x graphics drivers)
cd Graphics
cargo run
```

## 🖼️ Graphics Window Controls

**Window Title:** "Hexagon Grid - Modular"  
**Resolution:** 1200x800 pixels

**Camera Controls:**
- **↑ Arrow Key** - Move camera up
- **↓ Arrow Key** - Move camera down  
- **← Arrow Key** - Move camera left
- **→ Arrow Key** - Move camera right
- **Close Window** - Exit application

**What You'll See:**
- Interactive hexagonal terrain grid
- 7 different colored terrain sprites
- Smooth camera movement across the world
- Real-time OpenGL rendering

**Requirements:**
- OpenGL 4.x compatible graphics card
- Updated graphics drivers
- Windows with proper GPU support

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

🗺️ TERRAIN SYSTEM DEMONSTRATION:
Available terrain sprites:
  1. Forest - RGB(0.2, 0.7, 0.2)
  2. Forest2 - RGB(0.3, 0.8, 0.3)  
  3. Grasslands - RGB(0.4, 0.9, 0.3)
  4. HauntedWoods - RGB(0.4, 0.2, 0.6)
  5. Hills - RGB(0.7, 0.6, 0.4)
  6. Mountain - RGB(0.6, 0.6, 0.7)
  7. Swamp - RGB(0.3, 0.5, 0.2)

Sample terrain distribution around origin:
🌿 🗻 ⛰️ 🌚 🌱
🌿 ⛰️ 🌱 🌲 🌱
🌿 🌚 ⬡ 🌚 🌿
🌿 🗻 🌱 ⬡ 🌚
🌳 🌿 ⛰️ 🌱 🌲
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