# QuestQuest: Hexagonal Game Engine

A complete Rust-based hexagonal grid game engine with Old scholl turn-based combat. On technical side, project is well tested and aims to be modular and expandable.

**Last Updated:** December 3, 2025  
**Documentation Status:** ✅ Comprehensive Rust docs across all crates

## 🏗️ Project Structure

```
QuestQuest/
├── AI/                 # GOAP-based AI planning system (NEW!)
│   └── Cargo.toml
├── Combat/             # Combat resolution system
│   └── Cargo.toml
├── Encyclopedia/       # Dynamic runtime encyclopedia system (NEW!)
│   └── Cargo.toml
├── Graphics/           # Rendering and hexagonal grid system
│   │   ├── math/       # Mathematical utilities
│   │   ├── rendering/  # OpenGL rendering layer
│   │   └── ui/         # User interface
│   └── Cargo.toml
├── Units/              # Trait-based unit system with abilities
│   └── Cargo.toml
├── Game/               # Game world management
├── QuestApp/           # Main application
│   └── Cargo.toml
└── Cargo.toml          # Workspace configuration
└── README.md
```

## 🎮 Features

### Graphics Crate
- **OpenGL 4.x Rendering**: Modern OpenGL with programmable shaders
- **Hexagonal Grid System**: Efficient axial coordinate system with flat-top hexagons
- **Multi-Layer Rendering**: Separate rendering layers for terrain, units, and items
- **Camera System**: View frustum culling for performance optimization
- **Terrain System**: Terrain system ready for expansion
- **Sprite Support**: Textured and colored sprite rendering with proper depth ordering
- **Item Positioning**: Smart item placement (corner positioning when unit present)
- **UI System**: Text rendering and interactive UI panels
- **Modular Architecture**: Clean separation of concerns with core, math, rendering, and UI modules

### Units Crate (Trait-Based System)
- **Trait-Based Architecture**: Polymorphic unit system with Unit trait
- **Factory Pattern**: UnitFactory for creating units by race and class
- **Comprehensive Ability System**: Passive, Active and Aura abilites for units
- **Multiple Unit Types**: Unit system ready for expansion
- **Equipment System**: Weapons, armor, and accessories with stat bonuses
- **Item Management**: Inventory system with consumables and equipment
- **Character Progression**: Experience-based leveling with stat increases
- **Race & Class System**: Multiple races and classes with unique bonuses
- **Team System**: Unit affiliation and team-based mechanics

### Combat Crate
- **Damage Types**: Slash, Pierce, Blunt, Crush, Fire, Dark
- **Resistance System**: Per-damage-type resistance calculations
- **Combat Resolution**: `resolve_combat()` function for battles
- **Terrain Bonuses**: Terrain-based hit chance modifiers
- **Range System**: Melee, Range, and Siege categories

### AI Crate
- **GOAP Planning**: Goal-Oriented Action Planning system
- **Forward A* Planner**: Efficient pathfinding through action space
- **Action Templates**: Reusable action definitions
- **World State**: Fact-based state representation
- **Team Planning**: Multi-unit coordination support
- **Action Execution**: Runtime action execution system

### Encyclopedia Crate
- **Dynamic Generation**: Runtime-generated game encyclopedia
- **Unit Entries**: Comprehensive unit information and stats
- **Terrain Guide**: Terrain types and movement effects
- **Mechanics Reference**: Combat, experience, equipment documentation
- **Search & Filter**: Find entries by category, race, or keywords
- **Formatted Display**: Clean, readable console output

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
- ✅ **Multi-Layer Rendering**: Layers for Terrain, units, items, etc
- ✅ **Ability System**: Passive, Active, and Aura abilities with flexible effects
- ✅ **Equipment Bonuses**: Weapons and armor modify unit stats
- ✅ **Damage Type System**: 6 damage types with resistance calculations
- ✅ **Range Modifiers**: Equipment can extend attack range
- ✅ **GOAP AI**: Goal-oriented planning for intelligent unit behavior
- ✅ **Dynamic Encyclopedia**: Runtime-generated comprehensive game documentation
- ✅ **Serialization**: Full serde support for save/load functionality
- ✅ **Comprehensive Tests**
- ✅ **Type Safety**: Strong typing throughout with custom types
- ✅ **Factory Pattern**: Flexible unit creation with UnitFactory
- ✅ **Develpemtn Pipeline**: Devpipeline with pre-commit

## 🧪 Test Coverage

**Workspace Status:** Extensive test suite across all crates; use `cargo test --workspace` to verify on your machine.


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
cargo doc -p questquest --open  # Root crate with overview + architecture
cargo doc -p units --open
cargo doc -p graphics --open
cargo doc -p combat --open
cargo doc -p game --open
cargo doc -p items --open
```

**🎯 Start Here:** The `questquest` root crate documentation (`cargo doc -p questquest --open`) contains:
- **Complete project overview** from README
- **Architecture documentation** from ARCHITECTURE.md
- **Links to all sub-crates**
- **Design patterns** and data flow diagrams

## 📚 Documentation

- All crates include Rust doc comments and crate-level overviews.
- Generate docs with `cargo doc --workspace` (use `--open` to view).

### Run Application
```bash
# Launch the main interactive game
cargo run -p questapp
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

**Requirements:**
- OpenGL 4.x compatible graphics card
- Updated graphics drivers
- Windows with proper GPU support

 

## 🏛️ Architecture Highlights

### System Architecture: How Everything Ties Together

QuestQuest uses a layered, modular architecture. `questapp` orchestrates the game loop and rendering, `game` owns world state and rules, `graphics` renders, while `units`, `items`, and `combat` provide core data and calculations. `ai` plans actions (GOAP) against the world state, and `encyclopedia` aggregates runtime data for documentation.

```
  ┌──────────────────────────────────────────────────────────────┐
  │                         QuestApp                             │
  │               (Main Application Entry Point)                 │
  │  • Window & event loop • Input • Game loop                   │
  └───────────────┬─────────────────────────────┬───────────────┘
                  │                             │
            ┌─────▼─────┐                 ┌─────▼─────┐
            │    Game   │                 │  Graphics │
            │    Crate  │                 │    Crate  │
            └─────┬─────┘                 └─────┬─────┘
                  │                             │
             ┌────▼───┐                  ┌──────▼─────┐
             │  Units │                  │   Items    │
             └────┬───┘                  └──────┬─────┘
                  │                             │
              ┌───▼────┐                        │
              │ Combat │◄───────────────────────┘
              └───┬────┘
                  │
            ┌─────▼────┐                      ┌──────────────┐
            │    AI    │ (GOAP plans against  │ Encyclopedia │
            │   Crate  │  Game world & Units) │    Crate     │
            └─────┬────┘                      └───────┬──────┘
                  └────────────┬──────────────────────┘
                               │
                  Common Types & Traits (positions, ids)
```

### Responsibilities at a Glance
- QuestApp: Runs window, input, and the main loop.
- Game: Owns world state and validates actions.
- Graphics: Renders terrain, units, items, and UI.
- Units: Provides unit trait, stats, leveling, equipment.
- Items: Defines equipment, consumables, and modifiers.
- Combat: Resolves turn-based battles and damage.
- AI: Plans actions via GOAP against game state.
- Encyclopedia: Aggregates runtime data for documentation.

### Data Flow: From User Action to Visual Update

**Example: Moving a Unit**

1. **User Input** (QuestApp)
   - Player clicks on a unit → Unit selected
   - Player clicks on valid hex → Move request generated

2. **Game Logic** (Game Crate)
   - `GameWorld::move_unit()` validates movement
   - Checks terrain blocking, unit collisions, world bounds
   - Detects enemy units → Initiates combat confirmation

3. **Combat Resolution** (Combat Crate)
   - `PendingCombat` created with both units' stats
   - Player selects attack type
   - `resolve_combat()` executes turn-based combat
   - Damage calculated with resistance modifiers

4. **State Update** (Multiple Crates)
   - Unit positions updated in `GameWorld`
   - Unit health modified via `Unit` trait methods
   - Defeated units removed from world

5. **Visual Rendering** (Graphics Crate)
   - `HexGrid` receives position updates
   - Camera determines visible hexes
   - Multi-layer renderer draws:
     - Layer 1: Terrain tiles
     - Layer 2: Unit sprites
     - Layer 3: Item sprites
   - OpenGL displays final frame

### Crate Responsibilities

#### **Game Crate** - World State Management
**Role:** Central authority for game state

**Responsibilities:** Track entities, validate movement, manage combat flow, handle teams, process interactions.

**Key Types:**
- `GameWorld`: Central state container
- `GameObject` trait: Unified entity interface
- `GameUnit`: Game-aware unit wrapper
- `TerrainTile`: Hex terrain data

**Dependencies:**
- Uses `graphics` for coordinate types
- Uses `units` for unit trait objects
- Uses `combat` for combat stats
- Uses `items` for equipment

#### **Combat Crate** - Battle Mechanics
**Role:** Isolated combat calculation engine

**Responsibilities:** Resolve turn-based combat, calculate damage/resistances, report results.

**Key Types:**
- `CombatStats`: All combat-related numbers
- `DamageType`: Type of damage (6 variants)
- `Resistances`: Damage reduction values
- `resolve_combat()`: Main algorithm

**Dependencies:**
- Uses `items` for equipment data
- Standalone calculation logic
- No game state dependencies

#### **Units Crate** - Unit System
**Role:** Polymorphic unit definitions

**Responsibilities:** Provide `Unit` trait, equipment/inventory, leveling, combat stats.

**Key Types:**
- `Unit` trait: Core interface
- `BaseUnit`: Shared data
- `UnitFactory`: Creation pattern
- Race-specific implementations

**Dependencies:**
- Uses `combat` for `CombatStats`
- Uses `items` for equipment
- Uses `graphics` for positions

#### **Items Crate** - Equipment & Consumables
**Role:** Item definitions and properties

**Responsibilities:** Define items, equipment bonuses, templates, consumables.

**Key Types:**
- `Item`: Core item structure
- `Equipment`: Wearable items
- Item properties and modifiers

**Dependencies:**
- Standalone definitions
- Used by other crates

#### **Graphics Crate** - Rendering Engine
**Role:** Visual presentation layer

**Responsibilities:** Render UI/hex grid, manage camera, textures/shaders.

**Key Types:**
- `HexCoord`: Axial coordinates
- `HexGrid`: Hex world representation
- `Camera`: View management
- `Renderer`: OpenGL interface

**Dependencies:**
- Standalone rendering
- Provides types to other crates

### Design Principles

- Clear separation of concerns across crates.
- Trait-based polymorphism for units and game entities.
- Reusable data types for positions, ids, and combat stats.

### Modular Design
- Graphics renders; Game orchestrates; Units/Items/Combat provide data & rules; QuestApp ties UI and loop.

### Trait-Based Unit System
- `Unit` trait with flexible implementations and factory-based creation.

### Multi-Layer Rendering
- Terrain, Units, Items rendered in separate layers with proper depth ordering.

### Hexagonal Mathematics
- Axial coordinates, neighbor lookup, and screen-to-grid conversions.

### Performance
- Camera culling, batched rendering, and smart caching.

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
1. **ai** - GOAP-based AI planning system
2. **encyclopedia** - Dynamic runtime encyclopedia
3. **graphics** - Rendering engine
4. **units** - Trait-based unit system with abilities
5. **combat** - Combat resolution
6. **game** - World management
7. **items** - Item and equipment system
8. **questapp** - Main application
9. **questquest** (root) - Workspace examples

## 🛠️ Development Setup

### Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com/) to ensure code quality before commits. The hooks automatically run:

1. **Rust Format Check** (`cargo fmt --check`): Ensures code is properly formatted
2. **Rust Clippy Lints** (`cargo clippy`): Catches common mistakes and enforces best practices
3. **Rust Tests** (`cargo test --workspace`): Runs tests across the workspace

#### Installation

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install the git hooks
pre-commit install

# Optionally, run hooks manually on all files
pre-commit run --all-files
```

The pre-commit hooks will automatically prevent commits if:
- Code is not formatted correctly
- Clippy lints fail
- Any tests fail

To bypass hooks (not recommended): `git commit --no-verify`

## 📈 Future Expansion Ideas

- **Pathfinding**: A* algorithm for hexagonal grids (foundation in AI crate)
- **AI Integration**: Connect GOAP planner to Game crate for NPC behavior
- **Status Effects**: Extend ability system with timed buffs/debuffs
- **More Damage Types**: Expand beyond 6 current types
- **Save/Load**: Game state persistence (serde already integrated)
- **Encyclopedia UI**: In-game encyclopedia browser
- **Advanced Graphics**: Textures for all units, animations, effects
- **Audio**: Sound effects and music integration
- **Enhanced UI**: Better unit info display, minimap, tooltips
- **Map Editor**: Tool for creating custom maps
- **Campaign Mode**: Story-driven single-player experience with GOAP AI

## 📚 Documentation

- **MIGRATION_COMPLETE.md** - Details of trait-based system migration
- **TEST_MIGRATION_STATUS.md** - Test migration documentation
- **TODO_COMPLETE.md** - Completed migration tasks
- **ITEM_RENDERING_UPDATE.md** - Latest rendering layer updates
- **README.md** (this file) - Project overview and usage

## 🎉 Recent Updates

### November 28, 2025 - AI & Encyclopedia Systems
- ✅ **AI Crate**: GOAP-based planning system with forward A* search
- ✅ **Encyclopedia Crate**: Dynamic runtime documentation system
- ✅ **Ability System**: Comprehensive passive, active, and aura abilities
- ✅ **Unit Registry**: Centralized unit type management
- ✅ **Team System**: Unit affiliation and team-based mechanics
- ✅ **Enhanced Documentation**: Units crate docs with ability guides
- ✅ **Test Suite Expansion**: 170+ tests across all crates

### October 22, 2025 - Comprehensive Documentation
- ✅ **Full Rust Doc Coverage**: All public APIs documented
- ✅ **Game Crate**: Complete docs for world management, objects, combat flow
- ✅ **Combat Crate**: Detailed docs for combat system, damage types, resistances
- ✅ **Crate-Level Docs**: High-level overviews and architecture explanations
- ✅ **Usage Examples**: Embedded examples in all major types
- ✅ **Architecture Guide**: Added system design and data flow documentation
- ✅ **Cross-References**: Linked related types and functions
- ✅ **README Update**: Expanded with documentation overview and patterns

**Documentation Statistics:**
- 📖 8 fully documented crates (AI, Encyclopedia, Game, Combat, Units, Items, Graphics, QuestApp)
- 💡 200+ documented public types and functions
- 🔧 100+ documented methods with examples
- 📝 Module-level documentation in all crates
- 📄 Extended markdown documentation (ABILITIES.md, ARCHITECTURE.md)
- ✅ Doc tests integrated with test suite

### October 19, 2025 - Item Rendering System
- ✅ Added separate `item_sprite` field to Hexagon
- ✅ Implemented three-layer rendering (terrain/units/items)
- ✅ Smart item positioning (corner when unit present)
- ✅ Added `set_item_at()` and `remove_item_at()` to HexGrid
- ✅ Updated QuestApp to use separate item tracking

### Completed Migration (Summary)
- Trait-based unit system, separated combat crate, new test suite, updated examples.

## 🎯 Project Status

**Maturity Level:** Production-Ready Architecture

✅ **Complete:**
- Modular workspace architecture (9 crates)
- Trait-based unit system with abilities
- GOAP AI planning system
- Dynamic encyclopedia system
- Combat resolution system
- Multi-layer rendering
- Hexagonal coordinate math
- Camera and view culling
- Equipment and inventory
- **Comprehensive documentation**
- Test coverage (170+ tests)

🚧 **In Progress / Planned:**
- AI-Game integration
- Encyclopedia UI integration
- Pathfinding with AI planner
- Save/load system
- Enhanced UI components
- Sound and music
- Advanced ability effects

This project demonstrates a solid foundation for a hex-based strategy game with:
- ✅ Clean modular architecture
- ✅ Trait-based polymorphism
- ✅ Multi-layer rendering
- ✅ Comprehensive testing
- ✅ **Full API documentation**
- ✅ Room for future expansion