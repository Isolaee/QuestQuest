# QuestQuest: Hexagonal Game Engine

A complete Rust-based hexagonal grid game engine with comprehensive unit management system, trait-based architecture, advanced rendering layers, and fully documented codebase.

**Last Updated:** November 28, 2025  
**Documentation Status:** ✅ Comprehensive Rust docs across all crates

## 🏗️ Project Structure

```
QuestQuest/
├── AI/                 # GOAP-based AI planning system (NEW!)
│   ├── src/
│   │   ├── lib.rs      # AI system entry point
│   │   ├── planner.rs  # Forward A* GOAP planner
│   │   ├── action.rs   # Action templates and instances
│   │   ├── executor.rs # Action execution
│   │   ├── goals.rs    # Goal definitions
│   │   └── world_state.rs # World state representation
│   └── Cargo.toml
├── Combat/             # Combat resolution system
│   ├── src/
│   │   ├── lib.rs      # Combat mechanics and damage calculation
│   └── Cargo.toml
├── Encyclopedia/       # Dynamic runtime encyclopedia system (NEW!)
│   ├── src/
│   │   ├── lib.rs      # Encyclopedia entry point
│   │   ├── encyclopedia.rs # Main encyclopedia system
│   │   ├── entries.rs  # Entry types (units, terrain, mechanics)
│   │   └── formatters.rs # Display formatting
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
│   │   │   └── hexagon.rs    # Hexagonal coordinate system
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
├── Units/              # Trait-based unit system with abilities
│   ├── src/
│   │   ├── lib.rs      # Unit system library entry
│   │   ├── ability.rs  # Comprehensive ability system (NEW!)
│   │   ├── attack.rs   # Attack mechanics
│   │   ├── base_unit.rs # Base unit data structure
│   │   ├── team.rs     # Team affiliation system
│   │   ├── unit_factory.rs # Unit creation factory
│   │   ├── unit_race.rs  # Race definitions and bonuses
│   │   ├── unit_registry.rs # Unit type registry
│   │   ├── unit_trait.rs # Main Unit trait
│   │   └── unit_type.rs  # Unit type enumeration
│   ├── docs/           # Comprehensive documentation
│   │   ├── ABILITIES.md       # Full ability system guide
│   │   ├── ABILITIES_QUICK_REF.md # Quick reference
│   │   └── ARCHITECTURE.md    # Units crate architecture
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
- **Comprehensive Ability System**: (NEW!)
  - **Passive Abilities**: Automatic effects (Always, OnTakeDamage, BelowHealth, etc.)
  - **Active Abilities**: Player-activated powers with cooldowns and targeting
  - **Aura Abilities**: Area effects that buff allies or debuff enemies
  - **Flexible Effects**: Attack/defense bonuses, healing, damage, resistances, movement
- **Multiple Unit Types**: 
  - Humans: Warrior, Archer, Mage, Paladin
  - Elves: Warrior, Archer, Mage, Paladin
  - Dwarves: Warrior, Archer, Mage, Paladin
  - Orcs: Warrior, Archer, Mage, Paladin
- **Combat System**: Separated combat crate with damage types and resistances
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

### AI Crate (NEW!)
- **GOAP Planning**: Goal-Oriented Action Planning system
- **Forward A* Planner**: Efficient pathfinding through action space
- **Action Templates**: Reusable action definitions
- **World State**: Fact-based state representation
- **Team Planning**: Multi-unit coordination support
- **Action Execution**: Runtime action execution system

### Encyclopedia Crate (NEW!)
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
- ✅ **Multi-Layer Rendering**: Terrain (z=0.0), Units (z=-0.5), Items (z=-0.6)
- ✅ **Smart Item Positioning**: Items shift to corner when unit occupies same hex
- ✅ **Trait Polymorphism**: Units as `Box<dyn Unit>` for flexible gameplay
- ✅ **Ability System**: Passive, Active, and Aura abilities with flexible effects
- ✅ **Equipment Bonuses**: Weapons and armor modify unit stats
- ✅ **Damage Type System**: 6 damage types with resistance calculations
- ✅ **Range Modifiers**: Equipment can extend attack range
- ✅ **GOAP AI**: Goal-oriented planning for intelligent unit behavior
- ✅ **Dynamic Encyclopedia**: Runtime-generated comprehensive game documentation
- ✅ **Serialization**: Full serde support for save/load functionality
- ✅ **Comprehensive Tests**: 170+ tests across all crates (100% passing)
- ✅ **Type Safety**: Strong typing throughout with custom types
- ✅ **Factory Pattern**: Flexible unit creation with UnitFactory

## 🧪 Test Coverage

**Total: 170+ tests across all crates, 100% passing**

**Units Crate: 79+ tests**
- Unit creation and factory patterns
- Ability system (passive, active, aura)
- Combat stats and calculations
- Experience and leveling
- Equipment and inventory
- Team affiliation
- Movement and positioning
- Damage types and resistances

**AI Crate: 9 tests**
- GOAP planning algorithms
- Action templates and instances
- World state management
- Goal achievement

**Encyclopedia Crate: 7 tests**
- Entry generation
- Search and filtering
- Display formatting

**Combat Crate: Tests**
- Combat resolution
- Damage calculations
- Resistance modifiers

**Graphics Crate: Tests**
- Coordinate conversion
- Rendering validation

**Game Crate: Tests**
- World management
- Object interactions

**QuestApp: Tests**
- Game state management
- Scene transitions
- User interactions

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
- **Quick start guide** with examples
- **Links to all sub-crates**
- **Design patterns** and data flow diagrams

## 📚 Documentation Overview

### Comprehensive Rust Documentation
**All crates are fully documented with Rust doc comments!**

Each crate includes:
- � **Crate-level documentation** (`//!`) explaining purpose and features
- 📝 **Module-level documentation** for all public modules
- 🔧 **Struct/Enum documentation** for all public types
- ⚙️ **Function documentation** with parameters, returns, and examples
- 💡 **Usage examples** embedded in doc comments
- 🔗 **Cross-references** between related types and functions

### Documentation Coverage by Crate

#### **AI Crate** (`ai`)
Goal-Oriented Action Planning system for intelligent unit behavior.

**Key Documentation:**
- `WorldState`: Fact-based world state representation
- `ActionTemplate`: Reusable action definitions
- `ActionInstance`: Grounded actions with specific parameters
- `plan()`: Forward A* GOAP planner
- `ActionExecutor`: Runtime action execution
- `Goal`: Goal definitions and achievement conditions

**Core Concepts:**
- GOAP (Goal-Oriented Action Planning)
- Forward state-space search
- Action preconditions and effects
- Cost-based planning
- Team coordination

#### **Encyclopedia Crate** (`encyclopedia`)
Dynamic runtime-generated game encyclopedia.

**Key Documentation:**
- `Encyclopedia`: Main encyclopedia system
- `UnitEntry`: Unit statistics and capabilities
- `TerrainEntry`: Terrain types and effects
- `MechanicEntry`: Game mechanics documentation
- Search and filtering system
- Display formatters

**Core Concepts:**
- Runtime content generation
- Automatic unit discovery
- Comprehensive game documentation
- Search and navigation
- Formatted console output

#### **Game Crate** (`game`)
Manages the game world and all entities within it.

**Key Documentation:**
- `GameObject` trait: Base interface for all world entities
- `GameWorld`: Central world management system
- `TerrainTile`: Hex grid terrain with movement costs
- `GameUnit`: Wrapper integrating units into the game world
- `InteractiveObject`: Items, NPCs, and world objects
- `Team` enum: Unit affiliation system
- `PendingCombat`: Combat confirmation dialog system

**Core Concepts:**
- Hex-based world with coordinate system
- Multi-entity management (terrain, units, objects)
- Combat initiation and resolution flow
- Team-based movement validation
- Position tracking and querying

#### **Combat Crate** (`combat`)
Turn-based combat system with damage types and resistance.

**Key Documentation:**
- `CombatStats`: Comprehensive unit combat statistics
- `DamageType`: Six damage types (Slash, Pierce, Blunt, Crush, Fire, Dark)
- `Resistances`: Percentage-based damage reduction
- `RangeCategory`: Melee, Range, and Siege combat
- `CombatAction`: Available combat actions
- `CombatResult`: Combat outcome data
- `resolve_combat()`: Main combat resolution algorithm

**Core Concepts:**
- Multi-attack system (attacks per round)
- Alternating turn-based combat
- Resistance modifiers (0-100%)
- Counter-attack mechanics
- Hit chance rolls with terrain modifiers

#### **Units Crate** (`units`)
Trait-based polymorphic unit system with comprehensive ability support.

**Key Documentation:**
- `Unit` trait: Core unit interface
- `BaseUnit`: Shared unit data structure
- `UnitFactory`: Factory pattern for unit creation
- `Ability` system: Passive, Active, and Aura abilities
- `UnitRegistry`: Centralized unit type management
- `Team`: Unit affiliation system
- Race implementations (Human, Elf, Dwarf, Orc)
- Equipment and inventory systems
- Experience and leveling mechanics

**Core Concepts:**
- Trait-based polymorphism
- Three-tier ability system (Passive/Active/Aura)
- Race and class combinations
- Equipment stat bonuses
- Dynamic unit creation
- Combat stat calculations
- Team-based mechanics

**Additional Documentation:**
- `docs/ABILITIES.md`: Complete ability system guide
- `docs/ABILITIES_QUICK_REF.md`: Quick reference
- `docs/ARCHITECTURE.md`: Crate architecture

#### **Items Crate** (`items`)
Equipment and consumable item system.

**Key Documentation:**
- Item types and properties
- Equipment system with slots
- Stat modifiers and bonuses
- Item definitions and templates

#### **Graphics Crate** (`graphics`)
OpenGL rendering with hex grid visualization.

**Key Documentation:**
- Hexagonal coordinate system (axial)
- Multi-layer rendering architecture
- Camera system with view culling
- Texture management
- Shader system
- UI components and text rendering

**Core Concepts:**
- Flat-top hexagon mathematics
- Three rendering layers (terrain/units/items)
- Efficient coordinate conversions
- Depth-based layer ordering

### Documentation Features
- ✅ **100% Public API Coverage**: Every public item is documented
- 💡 **Practical Examples**: Real usage patterns in doc comments
- 🔗 **Type Cross-References**: Links between related types
- 📊 **Visual Formatting**: Clear sections for Args, Returns, Examples
- 🧪 **Doc Tests**: Examples verified during `cargo test`
- 🎯 **Clear Explanations**: Behavior, constraints, and use cases
- 📝 **Field Documentation**: All struct fields explained
- ⚠️ **Edge Cases**: Special behaviors and limitations noted

### Documentation Location
Generated HTML documentation is placed in `target/doc/`:
```
target/doc/
├── questquest/index.html     # 🌟 START HERE: Project overview + architecture
│   ├── index.html            # Complete README content
│   └── architecture/         # ARCHITECTURE.md content
├── game/index.html           # Game world management
├── combat/index.html         # Combat system
├── units/index.html          # Unit system
├── items/index.html          # Item system
├── graphics/index.html       # Rendering engine
└── questapp/index.html       # Main application
```

**🎯 Recommended Reading Order:**
1. `questquest` - High-level overview and architecture
2. `game` - Game world and entity management
3. `combat` - Battle mechanics
4. `units` - Unit system and factory
5. `graphics` - Rendering engine
6. `items` - Equipment system

### Reading the Documentation

1. **Start with Crate-Level Docs**: Read the module overview (`lib.rs`)
2. **Explore Core Types**: Click through key structs and traits
3. **Check Examples**: Copy example code from doc comments
4. **Follow Cross-References**: Navigate between related types
5. **Read Method Docs**: Understand parameters and return values

### Documentation Best Practices Used

- **Consistent Structure**: All docs follow same format
- **Examples First**: Show usage before explaining details
- **Clear Sections**: Args, Returns, Examples clearly marked
- **Concise Descriptions**: One-line summaries + detailed explanations
- **Type Safety Notes**: Explain constraints and valid ranges
- **Integration Examples**: Show how crates work together

### Run Applications
```bash
# Main interactive game application (recommended!)
cargo run -p questapp

# Encyclopedia demo - Browse all units, terrain, and mechanics
cd Encyclopedia
cargo run --example encyclopedia_demo

# AI planning demo - See GOAP in action
cd AI
cargo run --example [example_name]

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

### System Architecture: How Everything Ties Together

QuestQuest uses a layered architecture where each crate has specific responsibilities and well-defined interfaces:

```
┌─────────────────────────────────────────────────────────────┐
│                        QuestApp                              │
│              (Main Application Entry Point)                  │
│  • Window management and event loop                         │
│  • User input handling (mouse, keyboard)                    │
│  • Game loop coordination                                   │
└──────────────────┬──────────────────────────────────────────┘
                   │
    ┌──────────────┼──────────────┬──────────────┐
    │              │              │              │
┌───▼────┐   ┌────▼─────┐   ┌───▼────┐   ┌────▼─────┐
│ Game   │   │ Graphics │   │ Units  │   │  Items   │
│ Crate  │   │  Crate   │   │ Crate  │   │  Crate   │
└───┬────┘   └────┬─────┘   └───┬────┘   └────┬─────┘
    │              │              │              │
    │              │         ┌────▼──────┐       │
    │              │         │  Combat   │       │
    │              │         │   Crate   │       │
    │              │         └───────────┘       │
    │              │                             │
    └──────────────┴─────────────────────────────┘
              Common Types & Traits
```

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

**Responsibilities:**
- Track all entities (terrain, units, objects)
- Validate movements and positions
- Manage combat flow (initiation → confirmation → resolution)
- Handle team affiliations
- Process entity interactions

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

**Responsibilities:**
- Turn-based combat resolution
- Damage calculation with resistances
- Hit chance rolls
- Multi-attack sequencing
- Combat result reporting

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

**Responsibilities:**
- Define `Unit` trait interface
- Implement race/class combinations
- Handle equipment and inventory
- Manage experience/leveling
- Provide combat stats

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

**Responsibilities:**
- Define item types
- Equipment stat bonuses
- Item templates
- Consumable effects

**Key Types:**
- `Item`: Core item structure
- `Equipment`: Wearable items
- Item properties and modifiers

**Dependencies:**
- Standalone definitions
- Used by other crates

#### **Graphics Crate** - Rendering Engine
**Role:** Visual presentation layer

**Responsibilities:**
- OpenGL rendering
- Hexagonal coordinate math
- Camera management
- Texture/sprite handling
- UI rendering
- Input coordinate conversion

**Key Types:**
- `HexCoord`: Axial coordinates
- `HexGrid`: Hex world representation
- `Camera`: View management
- `Renderer`: OpenGL interface

**Dependencies:**
- Standalone rendering
- Provides types to other crates

### Design Patterns Used

#### **Trait-Based Polymorphism**
```rust
// Units stored as trait objects for flexibility
pub struct GameWorld {
    pub units: HashMap<Uuid, GameUnit>,  // GameUnit contains Box<dyn Unit>
}

// Any type implementing Unit can be stored
let warrior: Box<dyn Unit> = Box::new(HumanWarrior::new(...));
let archer: Box<dyn Unit> = Box::new(ElfArcher::new(...));
```

#### **Factory Pattern**
```rust
// Create units dynamically by race and class
let unit = UnitFactory::create_unit(
    Race::Human,
    UnitClass::Warrior,
    "Thorin".to_string(),
    HexCoord::new(0, 0)
);
```

#### **Separation of Concerns**
- **Combat Logic**: Isolated in Combat crate, no game state dependencies
- **Rendering**: Graphics crate doesn't know about game rules
- **Game Logic**: Game crate orchestrates but doesn't render
- **Data Definitions**: Items and Units define structure, not behavior

#### **Dependency Inversion**
```rust
// Game depends on Unit trait, not concrete types
impl GameUnit {
    pub fn new(unit: Box<dyn Unit>) -> Self { ... }
    pub fn unit(&self) -> &dyn Unit { ... }
}
```

### Modular Design
- **Graphics**: Handles all rendering, hexagonal math, camera systems, and UI
- **Units**: Trait-based unit system with race/class combinations
- **Combat**: Separated combat resolution with damage types and resistances
- **Game**: World management, unit tracking, and interactive objects
- **Items**: Equipment and consumable definitions
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
1. **ai** - GOAP-based AI planning system
2. **encyclopedia** - Dynamic runtime encyclopedia
3. **graphics** - Rendering engine
4. **units** - Trait-based unit system with abilities
5. **combat** - Combat resolution
6. **game** - World management
7. **items** - Item and equipment system
8. **questapp** - Main application
9. **questquest** (root) - Workspace examples

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

### Completed Migration
- ✅ Migrated from monolithic Unit struct to trait-based system
- ✅ Created 12 concrete unit implementations
- ✅ Separated combat into dedicated crate
- ✅ Implemented damage types and resistances
- ✅ Created comprehensive new test suite (12 tests, 100% passing)
- ✅ Updated all examples and demos

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