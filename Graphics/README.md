# Graphics Library

A modular Rust graphics library for hexagonal grid-based games using OpenGL.

## Structure

```
src/
├── lib.rs              # Library entry point
├── main.rs             # Binary application entry point
├── core/               # Core game logic
│   ├── mod.rs
│   ├── camera.rs       # Camera system
│   ├── grid.rs         # Hexagonal grid management
│   └── hexagon.rs      # Hexagon coordinates and sprites
├── math/               # Mathematical utilities
│   ├── mod.rs
│   └── vec2.rs         # 2D vector implementation
└── rendering/          # OpenGL rendering
    ├── mod.rs
    ├── renderer.rs     # Main renderer
    ├── shaders.rs      # Shader setup and management
    └── vertex_buffer.rs # Vertex buffer management
```

## Features

- **Hexagonal coordinate system** with axial coordinates
- **Camera system** with frustum culling for efficient rendering
- **Sprite system** for tile variations
- **Modular architecture** separating game logic from rendering
- **Clean OpenGL wrapper** for easy graphics programming

## Usage

### As a Library
Add to your `Cargo.toml`:
```toml
[dependencies]
graphics = { path = "../Graphics" }
```

### As a Binary
```bash
cargo run
```

Use arrow keys to move the camera around the hexagonal grid.

## Architecture Benefits

1. **Separation of Concerns**: Game logic is separate from rendering
2. **Reusability**: Core modules can be used in other projects
3. **Testability**: Individual modules can be unit tested
4. **Maintainability**: Clear structure makes code easy to navigate
5. **Extensibility**: Easy to add new modules (audio, physics, etc.)

## Future Extensions

- **Game Logic**: Add separate crate for game rules and mechanics  
- **Audio**: Add audio module for sound effects and music
- **Physics**: Add physics simulation for game objects
- **Networking**: Add multiplayer support
- **Input**: Expand input handling beyond basic keyboard