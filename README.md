# QuestQuest: Hexagonal Game Engine

A complete Rust-based hexagonal grid game engine with comprehensive unit management system, trait-based architecture, and advanced rendering layers.

**Last Updated:** October 19, 2025

## 🏗️ Project Structure

```
QuestQuest/
├── Combat/             # Combat resolution system (NEW!)
│   ├── src/
│   │   ├── lib.rs      # Combat mechanics and damage calculation
│   └── Cargo.toml
├── Graphics/           # Rendering and hexagonal grid system
│   ├── src/
│   │   ├── lib.rs      # Main library entry point
│   │   ├── main.rs     # Standalone graphics demo
│   │   ├── core/       # Core game logic
│   │   │   ├── mod.rs
│   │   │   ├── camera.rs     # Camera with view culling
│   │   │   ├── grid.rs       # Hexagonal grid management
│   │   │   ├── hex_lookup.rs # Hex coordinate lookup
│   │   │   ├── hexagon.rs    # Hexagonal coordinate system
│   │   │   └── simple_hex.rs # Simplified hex representation
│   │   ├── math/       # Mathematical utilities
│   │   │   ├── mod.rs
│   │   │   └── vec2.rs       # 2D vector operations
│   │   ├── rendering/  # OpenGL rendering layer
│   │   │   ├── mod.rs
│   │   │   ├── renderer.rs   # Multi-layer renderer (terrain/units/items)
│   │   │   ├── shaders.rs    # Shader management
│   │   │   ├── texture_manager.rs # Texture loading and binding
│   │   │   └── vertex_buffer.rs   # Vertex buffer handling
│   │   └── ui/         # User interface
│   │       ├── mod.rs
│   │       ├── text_renderer.rs # Text rendering system
│   │       └── ui_panel.rs      # UI panel components
│   └── Cargo.toml
├── Units/              # Trait-based unit system (REFACTORED!)
│   ├── src/
│   │   ├── lib.rs      # Unit system library entry
│   │   ├── combat/     # Combat subsystem
│   │   │   ├── mod.rs
│   │   │   └── stats.rs      # Combat statistics
│   │   ├── items/      # Item subsystem
│   │   │   ├── mod.rs
│   │   │   ├── consumable.rs # Consumable items
│   │   │   ├── equipment.rs  # Equipment system
│   │   │   └── item.rs       # Base item implementation
│   │   ├── traits/     # Core trait definitions
│   │   │   ├── mod.rs
│   │   │   ├── base_unit.rs  # Base unit data structure
│   │   │   ├── factory.rs    # Unit factory pattern
│   │   │   └── unit_trait.rs # Main Unit trait
│   │   ├── units/      # Concrete unit implementations
│   │   │   ├── mod.rs
│   │   │   ├── dwarf/        # Dwarf race units
│   │   │   ├── elf/          # Elf race units
│   │   │   ├── human/        # Human race units
│   │   │   └── orc/          # Orc race units
│   │   ├── unit_class.rs # Character classes
│   │   └── unit_race.rs  # Character races
│   ├── tests/          # Comprehensive test suite
│   │   └── new_system_tests.rs # Tests for trait-based system
│   └── Cargo.toml
├── Game/               # Game world management
│   ├── src/
│   │   ├── lib.rs      # Game world library
│   │   ├── objects.rs  # Interactive objects
│   │   └── world.rs    # Game world state
│   └── Cargo.toml
├── QuestApp/           # Main application
│   ├── src/
│   │   └── main.rs     # Interactive game window
│   └── Cargo.toml
├── example.rs          # Demonstration program
├── MIGRATION_COMPLETE.md    # Migration documentation
├── TEST_MIGRATION_STATUS.md # Test migration status
├── TODO_COMPLETE.md         # Completed TODO items
├── ITEM_RENDERING_UPDATE.md # Latest rendering update
└── Cargo.toml          # Workspace configuration
```

## 🎮 Features

### Graphics Crate
- **OpenGL 4.x Rendering**: Modern OpenGL with programmable shaders
- **Hexagonal Grid System**: Efficient axial coordinate system with flat-top hexagons
- **Multi-Layer Rendering**: Separate rendering layers for terrain, units, and items
- **Camera System**: View frustum culling for performance optimization
- **Terrain System**: 7 distinct terrain sprites (Forest, Forest2, Grasslands, Haunted Woods, Hills, Mountain, Swamp)
- **Sprite Support**: Textured and colored sprite rendering with proper depth ordering
- **Item Positioning**: Smart item placement (corner positioning when unit present)
- **UI System**: Text rendering and interactive UI panels
- **Modular Architecture**: Clean separation of concerns with core, math, rendering, and UI modules

### Units Crate (Trait-Based System)
- **Trait-Based Architecture**: Polymorphic unit system with Unit trait
- **Factory Pattern**: UnitFactory for creating units by race and class
- **12 Concrete Unit Types**: 
  - Humans: Warrior, Archer, Mage, Paladin
  - Elves: Warrior, Archer, Mage, Paladin
  - Dwarves: Warrior, Archer, Mage, Paladin
  - Orcs: Warrior, Archer, Mage, Paladin (planned)
- **Combat System**: Separated combat crate with damage types and resistances
- **Equipment System**: Weapons, armor, and accessories with stat bonuses
- **Item Management**: Inventory system with consumables and equipment
- **Character Progression**: Experience-based leveling with stat increases
- **Race & Class System**: Multiple races and classes with unique bonuses

### Combat Crate
- **Damage Types**: Slash, Pierce, Blunt, Crush, Fire, Dark
- **Resistance System**: Per-damage-type resistance calculations
- **Combat Resolution**: `resolve_combat()` function for battles
- **Terrain Bonuses**: Terrain-based hit chance modifiers
- **Range System**: Melee, Range, and Siege categories

### Game Crate
- **World Management**: Game world state with unit tracking
- **Interactive Objects**: Item pickups and environmental interactions
- **Position Tracking**: Unit and object positioning on hex grid

### QuestApp - Interactive Application
- **Real-time Graphics**: Live hex grid rendering with units and items
- **Unit Selection**: Click-based unit selection and movement
- **Movement Range**: Visual display of valid movement hexes
- **Item Pickup**: Interactive item collection system
- **Camera Controls**: Arrow key navigation
- **Debug Mode**: Hover highlighting for development

### Key Features
- ✅ **Hexagonal Coordinates**: Proper axial coordinate system with distance calculations
- ✅ **Camera Culling**: Only render hexagons within camera view distance
- ✅ **Multi-Layer Rendering**: Terrain (z=0.0), Units (z=-0.5), Items (z=-0.6)
- ✅ **Smart Item Positioning**: Items shift to corner when unit occupies same hex
- ✅ **Trait Polymorphism**: Units as `Box<dyn Unit>` for flexible gameplay
- ✅ **Equipment Bonuses**: Weapons and armor modify unit stats
- ✅ **Damage Type System**: 6 damage types with resistance calculations
- ✅ **Range Modifiers**: Equipment can extend attack range
- ✅ **Serialization**: Full serde support for save/load functionality
- ✅ **Comprehensive Tests**: 12+ unit tests covering all functionality
- ✅ **Type Safety**: Strong typing throughout with custom types
- ✅ **Factory Pattern**: Flexible unit creation with UnitFactory

## 🧪 Test Coverage

**Units Crate Tests: 12 tests, 100% passing** (New trait-based system)
- **Unit Creation**: Factory pattern validation
- **Combat Stats**: Attack, defense, and resistance calculations
- **Experience & Leveling**: XP gain and level progression
- **Equipment System**: Stat bonuses from items
- **Combat Resolution**: New combat system with damage types
- **Damage Types**: Per-class damage type assignments
- **Resistances**: Per-class resistance calculations
- **Terrain Effects**: Hit chance modifiers by terrain
- **Movement**: Position and movement range
- **Range Categories**: Melee, Range, and Siege classifications
- **Health Management**: Damage and healing
- **Inventory**: Item management and equipment slots

**Graphics Crate Tests**: Coordinate conversion and rendering validation

**All Tests**: `cargo test --workspace` - ✅ 100% passing

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

# Specific crate tests
cargo test -p units
cargo test -p graphics
cargo test -p game

# Specific test file
cd Units
cargo test --test new_system_tests
```

### Generate Documentation
```bash
# Generate documentation for all workspace crates
cargo doc --workspace

# Generate and open documentation in browser
cargo doc --workspace --open

# Generate documentation without dependencies (faster)
cargo doc --workspace --no-deps

# Generate documentation for a specific crate
cargo doc -p units --open
cargo doc -p graphics --open
cargo doc -p combat --open
```

**Documentation Features:**
- 📚 **Comprehensive API docs** for all public types and functions
- 💡 **Code examples** embedded in documentation
- 🔗 **Cross-referenced types** with clickable links between related items
- 📖 **Module-level documentation** explaining each crate's purpose
- ✅ **Doc tests** that are automatically verified during testing

**Generated Documentation Includes:**
- **Units Crate**: Unit trait, race/class system, factory patterns, combat stats
- **Graphics Crate**: Rendering system, hexagonal coordinates, camera, UI components
- **Combat Crate**: Damage types, resistances, combat resolution
- **Game Crate**: World management, interactive objects
- **Items Crate**: Equipment system, consumables, item properties

**Documentation Location:**
The generated HTML documentation is placed in `target/doc/`. The main index page will be at:
```
target/doc/units/index.html
target/doc/graphics/index.html
target/doc/combat/index.html
target/doc/game/index.html
```

**Note:** Documentation tests (code examples in doc comments) are automatically run with `cargo test --workspace` to ensure examples stay up-to-date.

### Run Applications
```bash
# Main interactive game application (recommended!)
cargo run -p questapp

# Text-based game mechanics demo
cargo run --bin example

# Graphics rendering demo (standalone)
cd Graphics
cargo run
```

## 🖼️ Interactive Game Window (QuestApp)

**Window Title:** "QuestQuest Interactive Game"  
**Resolution:** 1200x800 pixels

**Controls:**
- **RIGHT-CLICK** on unit - Select unit and show movement range (blue hexes)
- **LEFT-CLICK** on blue hex - Move selected unit
- **Arrow Keys** (↑ ↓ ← →) - Move camera
- **C Key** - Show detailed unit info in console
- **H Key** - Toggle hover debug mode (yellow hex highlighting)
- **Y Key** - Pick up item (when prompt shown)
- **ESC Key** - Deselect unit

**Features:**
- Real-time hexagonal terrain rendering
- Unit selection and movement
- Item pickup system with prompts
- Visual movement range display
- Camera controls for world navigation
- Debug mode for development

**What You'll See:**
- 3 demo units: Thorin (Human Warrior), Legolas (Elf Archer), Gimli (Dwarf Paladin)
- 1 test item: Iron Sword at position (1,1)
- Interactive hex grid with terrain
- Unit sprites (red circles, 60% of hex size)
- Item sprites (gold circles, 50% size or 25% in corner if unit present)
- Movement range visualization (blue highlight)
- Selected unit highlight (yellow)

**Requirements:**
- OpenGL 4.x compatible graphics card
- Updated graphics drivers
- Windows with proper GPU support

## 🎯 Demo Output Example

**QuestApp (Interactive Game):**
```
🎮 Starting QuestQuest Interactive Game Window...
✅ UI Panel initialized!
🎮 QuestQuest Game Window Started!
📍 Units: Thorin at (0,0), Legolas at (2,-1), Gimli at (-2,1)
🎁 Item: Iron Sword at (1,1) - available for pickup!
🖱️  RIGHT-CLICK on a unit to select it and show movement range
🖱️  LEFT-CLICK on blue hexes to move the selected unit
⌨️  Use arrow keys to move camera
🔤 Press 'C' to show detailed unit info in console
🔤 Press 'H' to toggle hover debug mode
🔤 Press ESC to deselect unit
```

**Example.rs (Text-based Demo):**
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
Combat Result: Hit! Damage dealt: 12
```

## 🏛️ Architecture Highlights

### Modular Design
- **Graphics**: Handles all rendering, hexagonal math, camera systems, and UI
- **Units**: Trait-based unit system with race/class combinations
- **Combat**: Separated combat resolution with damage types and resistances
- **Game**: World management, unit tracking, and interactive objects
- **QuestApp**: Main interactive application tying everything together
- **Clear Separation**: Each crate has well-defined responsibilities
- **Type Safety**: Custom types prevent coordinate system errors

### Trait-Based Unit System
- **Unit Trait**: Common interface for all units
- **BaseUnit**: Shared data structure for common properties
- **Concrete Types**: Specific implementations per race/class combination
- **Factory Pattern**: UnitFactory creates units dynamically
- **Polymorphism**: Units stored as `Box<dyn Unit>` for flexibility

### Multi-Layer Rendering
- **Layer 1: Terrain** (z = 0.0) - Background hexagonal grid
- **Layer 2: Units** (z = -0.5) - Unit sprites at 60% of hex size
- **Layer 3: Items** (z = -0.6) - Item sprites with smart positioning
  - Centered at 50% size when alone
  - Corner positioned at 25% size when unit present
- **Depth Testing**: OpenGL depth buffer ensures correct layer ordering

### Hexagonal Mathematics  
- **Axial Coordinates**: Efficient (q, r) coordinate system
- **Flat-Top Hexagons**: Horizontal flat edges, vertical pointy edges
- **Distance Calculation**: Proper hexagonal distance using axial coordinates
- **Neighbor Finding**: Calculate adjacent hexagons in all 6 directions
- **Grid Operations**: Convert between screen and grid coordinates
- **Axial Rounding**: Robust fractional coordinate rounding

### Performance Optimizations
- **Camera Culling**: Only process hexagons within view distance
- **Efficient Rendering**: Batch vertex updates and minimize GL calls
- **Smart Caching**: Cache calculated stats until equipment changes
- **Memory Management**: Use appropriate data structures for performance
- **Separate Rendering Layers**: Independent updates for terrain/units/items

## 🔧 Technical Stack

- **Language**: Rust 2021 Edition
- **Graphics**: OpenGL 4.x with gl, glutin, winit crates
- **Windowing**: glutin-winit for window management
- **Serialization**: serde with JSON support  
- **UUID**: Unique identifiers for units and items
- **Testing**: Built-in Rust testing framework
- **Build System**: Cargo workspace for multi-crate management
- **Item System**: items crate for equipment and consumables

### Workspace Crates
1. **graphics** - Rendering engine
2. **units** - Trait-based unit system
3. **combat** - Combat resolution
4. **game** - World management
5. **items** - Item and equipment system
6. **questapp** - Main application
7. **questquest** (root) - Workspace examples

## �️ Development Setup

### Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com/) to ensure code quality before commits. The hooks automatically run:

1. **Rust Format Check** (`cargo fmt --check`): Ensures code is properly formatted
2. **Rust Clippy Lints** (`cargo clippy`): Catches common mistakes and enforces best practices
3. **Rust Tests** (`cargo test --all`): Runs all 31+ tests across all packages

#### Installation

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install the git hooks
pre-commit install

# Optionally, run hooks manually on all files
pre-commit run --all-files
```

#### What Gets Tested

- **Units**: 12 tests covering:
  - Unit creation with factory
  - Combat stats and calculations
  - Experience and leveling
  - Equipment effects
  - Combat resolution
  - Damage types per class
  - Resistances per class
  - Terrain effects on hit chance
  - Movement and positioning
  - Range categories
  - Health and damage
  - Inventory management
- **Graphics**: Coordinate conversion and rendering
- **Game**: World management and object interactions

The pre-commit hooks will automatically prevent commits if:
- Code is not formatted correctly
- Clippy lints fail
- Any tests fail

To bypass hooks (not recommended): `git commit --no-verify`

## 📈 Future Expansion Ideas

- **Pathfinding**: A* algorithm for hexagonal grids
- **Game States**: Turn-based combat system implementation
- **More Units**: Complete Orc race implementation
- **Unique Abilities**: Special powers per race/class
- **Status Effects**: Buffs, debuffs, and conditions
- **More Damage Types**: Expand beyond 6 current types
- **Networking**: Multiplayer support with unit synchronization
- **Save/Load**: Game state persistence (serde already integrated)
- **AI**: Computer-controlled unit behavior
- **Advanced Graphics**: Textures for all units, animations, effects
- **Audio**: Sound effects and music integration
- **Enhanced UI**: Better unit info display, minimap, tooltips
- **Scripting**: Lua or similar for moddable game logic
- **Map Editor**: Tool for creating custom maps
- **Campaign Mode**: Story-driven single-player experience

## 📚 Documentation

- **MIGRATION_COMPLETE.md** - Details of trait-based system migration
- **TEST_MIGRATION_STATUS.md** - Test migration documentation
- **TODO_COMPLETE.md** - Completed migration tasks
- **ITEM_RENDERING_UPDATE.md** - Latest rendering layer updates
- **README.md** (this file) - Project overview and usage

## 🎉 Recent Updates

### October 19, 2025 - Item Rendering System
- ✅ Added separate `item_sprite` field to Hexagon
- ✅ Implemented three-layer rendering (terrain/units/items)
- ✅ Smart item positioning (corner when unit present)
- ✅ Added `set_item_at()` and `remove_item_at()` to HexGrid
- ✅ Updated QuestApp to use separate item tracking

### Completed Migration
- ✅ Migrated from monolithic Unit struct to trait-based system
- ✅ Created 12 concrete unit implementations
- ✅ Separated combat into dedicated crate
- ✅ Implemented damage types and resistances
- ✅ Created comprehensive new test suite (12 tests, 100% passing)
- ✅ Updated all examples and demos

This project demonstrates a solid foundation for a hex-based strategy game with clean architecture, trait-based polymorphism, multi-layer rendering, comprehensive testing, and room for future expansion.