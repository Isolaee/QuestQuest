//! # Architecture Documentation
//!
//! This module provides comprehensive architectural documentation for the QuestQuest engine.
//!
//! ## Table of Contents
//!
//! - [Crate Organization](#crate-organization)
//! - [Data Flow](#data-flow)
//! - [Integration Points](#integration-points)
//! - [Performance Considerations](#performance-considerations)
//!
//! # Crate Organization
//!
//! ## Game Crate: World State Management
//!
//! **Purpose:** Central authority for game state
//!
//! **Responsibilities:**
//! - Track all entities (terrain, units, objects)
//! - Validate movements and positions
//! - Manage combat flow (initiation → confirmation → resolution)
//! - Handle team affiliations
//! - Process entity interactions
//!
//! **Key Types:**
//! - `GameWorld`: Central state container
//! - `GameObject` trait: Unified entity interface
//! - `GameUnit`: Game-aware unit wrapper
//! - `TerrainTile`: Hex terrain data
//!
//! ## Combat Crate: Battle Mechanics
//!
//! **Purpose:** Isolated combat calculation engine
//!
//! **Responsibilities:**
//! - Turn-based combat resolution
//! - Damage calculation with resistances
//! - Hit chance rolls
//! - Multi-attack sequencing
//! - Combat result reporting
//!
//! **Key Types:**
//! - `CombatStats`: All combat-related numbers
//! - `DamageType`: Type of damage (6 variants)
//! - `Resistances`: Damage reduction values
//! - `resolve_combat()`: Main algorithm
//!
//! ## Units Crate: Unit System
//!
//! **Purpose:** Polymorphic unit definitions
//!
//! **Responsibilities:**
//! - Define `Unit` trait interface
//! - Implement race/class combinations
//! - Handle equipment and inventory
//! - Manage experience/leveling
//! - Provide combat stats
//!
//! **Key Types:**
//! - `Unit` trait: Core interface
//! - `BaseUnit`: Shared data
//! - `UnitFactory`: Creation pattern
//! - Race-specific implementations
//!
//! ## Graphics Crate: Rendering Engine
//!
//! **Purpose:** Visual presentation layer
//!
//! **Responsibilities:**
//! - OpenGL rendering
//! - Hexagonal coordinate math
//! - Camera management
//! - Texture/sprite handling
//! - UI rendering
//!
//! **Key Types:**
//! - `HexCoord`: Axial coordinates
//! - `HexGrid`: Hex world representation
//! - `Camera`: View management
//! - `Renderer`: OpenGL interface
//!
//! # Data Flow
//!
//! ## Complete Combat Sequence
//!
//! ### 1. User Input (QuestApp)
//! ```ignore
//! // Mouse click on unit
//! if right_click {
//!     let hex_coord = screen_to_hex(mouse_pos);
//!     if let Some(unit_id) = world.get_unit_at(hex_coord) {
//!         selected_unit = Some(unit_id);
//!         show_movement_range();
//!     }
//! }
//! ```
//!
//! ### 2. Movement Validation (Game Crate)
//! ```ignore
//! pub fn move_unit(&mut self, unit_id: Uuid, new_position: HexCoord) -> Result<(), String> {
//!     // Check for enemy at target position
//!     let enemy_at_target = self.get_units_at_position(new_position)
//!         .iter()
//!         .find(|u| u.team() != moving_unit.team());
//!     
//!     if let Some(enemy) = enemy_at_target {
//!         // Initiate combat confirmation
//!         return self.request_combat(unit_id, enemy.id());
//!     }
//!     
//!     // Normal movement validation
//!     unit.set_position(new_position);
//!     Ok(())
//! }
//! ```
//!
//! ### 3. Combat Resolution (Combat Crate)
//! ```ignore
//! let result = resolve_combat(
//!     &mut attacker_stats,
//!     &mut defender_stats,
//!     damage_type
//! );
//! ```
//!
//! ### 4. Visual Update (Graphics Crate)
//! ```ignore
//! // Update hex grid with new positions
//! for (id, unit) in world.units() {
//!     let pos = unit.position();
//!     grid.set_unit_at(pos.q, pos.r, Some(unit_sprite));
//! }
//! ```
//!
//! # Integration Points
//!
//! ## Between Crates
//!
//! ### Game ↔ Units
//! ```ignore
//! pub struct GameUnit {
//!     unit: Box<dyn Unit>,  // From Units crate
//! }
//!
//! impl GameWorld {
//!     pub fn update(&mut self, delta_time: f32) {
//!         for unit in self.units.values_mut() {
//!             unit.unit_mut().update(delta_time);
//!         }
//!     }
//! }
//! ```
//!
//! ### Game ↔ Combat
//! ```ignore
//! impl GameWorld {
//!     fn initiate_combat(&mut self, attacker_id: Uuid, defender_id: Uuid) {
//!         let mut attacker_stats = /* get from unit */;
//!         let mut defender_stats = /* get from unit */;
//!         
//!         // Combat crate resolves
//!         let result = resolve_combat(&mut attacker_stats, &mut defender_stats, damage_type);
//!         
//!         // Game handles result
//!         self.apply_combat_result(result);
//!     }
//! }
//! ```
//!
//! ### Graphics ↔ Game
//! ```ignore
//! use graphics::HexCoord;
//!
//! impl TerrainTile {
//!     position: HexCoord,  // Graphics coordinate type
//! }
//! ```
//!
//! ## Data Ownership Strategy
//!
//! ### Game Owns Core State
//! ```ignore
//! pub struct GameWorld {
//!     pub terrain: HashMap<HexCoord, TerrainTile>,
//!     pub units: HashMap<Uuid, GameUnit>,
//!     pub interactive_objects: HashMap<Uuid, InteractiveObject>,
//! }
//! ```
//!
//! ### Graphics Owns Rendering State
//! ```ignore
//! pub struct HexGrid {
//!     hexagons: HashMap<(i32, i32), Hexagon>,
//!     vertex_buffer: VertexBuffer,
//! }
//! ```
//!
//! ### QuestApp Coordinates
//! ```ignore
//! pub struct QuestApp {
//!     world: GameWorld,      // Game state
//!     grid: HexGrid,         // Visual state
//!     camera: Camera,        // View state
//! }
//! ```
//!
//! # Performance Considerations
//!
//! ## 1. View Culling
//!
//! Only process visible hexagons:
//!
//! ```ignore
//! pub fn is_in_view(&self, hex_coord: HexCoord) -> bool {
//!     let center = self.screen_to_hex(self.position);
//!     center.distance(hex_coord) <= self.view_distance
//! }
//!
//! for (coord, hex) in grid.hexagons() {
//!     if camera.is_in_view(coord) {
//!         renderer.render_hex(hex);
//!     }
//! }
//! ```
//!
//! **Impact:**
//! - O(n) → O(visible_hexes)
//! - Typically 50-200 hexes vs 1000+ total
//! - 80%+ performance improvement
//!
//! ## 2. HashMap Lookups
//!
//! O(1) entity lookup by ID and position:
//!
//! ```ignore
//! pub struct GameWorld {
//!     units: HashMap<Uuid, GameUnit>,              // By ID
//!     terrain: HashMap<HexCoord, TerrainTile>,     // By position
//! }
//! ```
//!
//! **Impact:**
//! - Constant time entity queries
//! - Efficient collision detection
//! - Fast selection and interaction
//!
//! ## 3. Batch Rendering
//!
//! Group similar draw calls:
//!
//! ```ignore
//! impl Renderer {
//!     pub fn render_layer(&mut self, layer: Layer) {
//!         self.use_shader(layer.shader());     // Once
//!         
//!         for sprite in layer.sprites() {
//!             vertex_buffer.add(sprite.vertices());
//!         }
//!         
//!         vertex_buffer.draw();                // Single call
//!     }
//! }
//! ```
//!
//! **Impact:**
//! - Minimize OpenGL state changes
//! - Reduce CPU-GPU communication
//! - 3-5x rendering performance
//!
//! ## 4. Lazy Calculation
//!
//! Cache stats until equipment changes:
//!
//! ```ignore
//! impl BaseUnit {
//!     pub fn combat_stats(&self) -> &CombatStats {
//!         if self.stats_dirty {
//!             self.recalculate_stats();
//!             self.stats_dirty = false;
//!         }
//!         &self.cached_stats
//!     }
//! }
//! ```
//!
//! **Impact:**
//! - Avoid redundant calculations
//! - Significant CPU savings
//!
//! # Design Patterns
//!
//! ## Trait-Based Polymorphism
//!
//! ```ignore
//! pub trait Unit: Send + Sync {
//!     fn combat_stats(&self) -> &CombatStats;
//!     fn take_damage(&mut self, damage: u32);
//! }
//!
//! pub struct GameUnit {
//!     unit: Box<dyn Unit>,  // Runtime polymorphism
//! }
//! ```
//!
//! **Benefits:**
//! - Runtime flexibility
//! - Extensible without modifying existing code
//! - Clean interface/implementation separation
//!
//! **Trade-offs:**
//! - Dynamic dispatch (slight performance cost)
//! - No serialization of trait objects
//! - Heap allocation required
//!
//! ## Factory Pattern
//!
//! ```ignore
//! impl UnitFactory {
//!     pub fn create_unit(
//!         race: Race,
//!         class: UnitClass,
//!         name: String,
//!         position: HexCoord,
//!     ) -> Box<dyn Unit> {
//!         match (race, class) {
//!             (Race::Human, UnitClass::Warrior) =>
//!                 Box::new(HumanWarrior::new(name, position)),
//!             // ... other combinations
//!         }
//!     }
//! }
//! ```
//!
//! **Benefits:**
//! - Consistent initialization
//! - Easy to extend
//! - Type-safe at compile time
//!
//! ## Separation of Concerns
//!
//! ```ignore
//! // Combat crate: Pure calculation
//! pub fn resolve_combat(
//!     attacker: &mut CombatStats,
//!     defender: &mut CombatStats,
//!     damage_type: DamageType,
//! ) -> CombatResult {
//!     // Only numbers, no game state
//! }
//!
//! // Game crate: Orchestration
//! impl GameWorld {
//!     pub fn initiate_combat(&mut self, ...) {
//!         // Retrieves units, calls combat, updates state
//!     }
//! }
//! ```
//!
//! **Benefits:**
//! - Testability (pure functions)
//! - Reusability (combat is standalone)
//! - Maintainability (changes localized)
//!
//! ## Type Safety
//!
//! ```ignore
//! #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//! pub struct HexCoord {
//!     pub q: i32,
//!     pub r: i32,
//! }
//!
//! fn move_unit(position: HexCoord) { ... }  // Type-safe!
//! ```
//!
//! **Benefits:**
//! - Compiler-enforced correctness
//! - Self-documenting code
//! - No runtime checks needed
//!
//! # Extension Guide
//!
//! ## Adding New Unit Types
//!
//! 1. **Define the struct:**
//! ```ignore
//! pub struct CustomRaceWarrior {
//!     base_unit: BaseUnit,
//!     special_ability: SpecialAbility,
//! }
//! ```
//!
//! 2. **Implement Unit trait:**
//! ```ignore
//! impl Unit for CustomRaceWarrior {
//!     fn name(&self) -> &str { &self.base_unit.name }
//!     fn combat_stats(&self) -> &CombatStats { &self.base_unit.combat_stats }
//!     // ... other methods
//! }
//! ```
//!
//! 3. **Update factory:**
//! ```ignore
//! impl UnitFactory {
//!     pub fn create_unit(...) -> Box<dyn Unit> {
//!         match (race, class) {
//!             (Race::CustomRace, UnitClass::Warrior) =>
//!                 Box::new(CustomRaceWarrior::new(...)),
//!             // ... other cases
//!         }
//!     }
//! }
//! ```
//!
//! ## Adding New Damage Types
//!
//! 1. **Extend enum:**
//! ```ignore
//! pub enum DamageType {
//!     // ... existing types
//!     Lightning,
//!     Poison,
//! }
//! ```
//!
//! 2. **Update Resistances:**
//! ```ignore
//! pub struct Resistances {
//!     // ... existing
//!     pub lightning: u8,
//!     pub poison: u8,
//! }
//! ```
//!
//! 3. **Update methods:**
//! ```ignore
//! impl Resistances {
//!     pub fn get_resistance(&self, damage_type: DamageType) -> u8 {
//!         match damage_type {
//!             // ... existing cases
//!             DamageType::Lightning => self.lightning,
//!             DamageType::Poison => self.poison,
//!         }
//!     }
//! }
//! ```
//!
//! ## Adding New Terrain Types
//!
//! 1. **Extend SpriteType:**
//! ```ignore
//! pub enum SpriteType {
//!     // ... existing
//!     Lava,
//!     Ice,
//! }
//! ```
//!
//! 2. **Update movement costs:**
//! ```ignore
//! impl TerrainTile {
//!     pub fn new(position: HexCoord, sprite_type: SpriteType) -> Self {
//!         let movement_cost = match sprite_type {
//!             // ... existing
//!             SpriteType::Lava => 5.0,
//!             SpriteType::Ice => 0.5,
//!         };
//!         // ...
//!     }
//! }
//! ```
//!
//! 3. **Add textures:**
//! - Add sprite files to `terrain_sprites/`
//! - Update texture loading in Graphics crate
//!
//! # Best Practices
//!
//! ## Documentation
//! - Document all public APIs with `///`
//! - Use `//!` for module-level docs
//! - Include examples in doc comments
//! - Run `cargo doc --open` regularly
//!
//! ## Testing
//! - Write unit tests for pure functions
//! - Test edge cases and error conditions
//! - Use integration tests for cross-crate functionality
//! - Maintain >80% test coverage
//!
//! ## Error Handling
//! - Use `Result<T, E>` for fallible operations
//! - Provide descriptive error messages
//! - Use custom error types when appropriate
//! - Log errors for debugging
//!
//! ## Type Safety
//! - Use newtypes for domain concepts
//! - Leverage Rust's type system
//! - Avoid primitive obsession
//! - Make invalid states unrepresentable
//!
//! ## Performance
//! - Profile before optimizing
//! - Use appropriate data structures
//! - Minimize allocations in hot paths
//! - Consider caching for expensive calculations
