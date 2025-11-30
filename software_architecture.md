Purpose
-------
This document describes the high-level architecture of the QuestQuest repository, explains the role of each top-level crate/folder, how the components interact, and gives concise guidance for building, testing, and contributing. The intent is to help new contributors understand where to make changes and how subsystems relate to each other.

Repository layout (top-level)
----------------------------
- `AI/` — Crate implementing non-player unit decision-making and planning. Contains core AI types, planners, actions, world state, and example agents.
- `Combat/` — Combat system: action resolution, combat results, statistics, and logic for resolving encounters and combat interactions.
- `Encyclopedia/` — In-game reference system for units, mechanics, items, and other documentation-driven content. Provides entry formatting and serializers where needed.
- `Game/` — Core game rules and systems such as the turn system, world management, object lifecycle, and game-level mechanics.
- `Graphics/` — Rendering layer for OpenGL. Contains rendering pipeline, UI components, sprite and animation logic.
- `Items/` — Item domain logic: item definitions, properties, equipment handling, and any item-related utilities.
- `Maps/` — Map resources and tooling; holds map JSONs used by the game and any future map-loading utilities.
- `QuestApp/` — Application entrypoint and scene management for running the game as an app. Contains menus, scene manager, and glue between the crates for an executable build.
- `Units/` — Unit definitions, abilities, and related data structures that represent characters, enemies, and NPCs.
- `scripts/` — Utility scripts used by developers (format-and-stage, CI helpers, etc.). See `fmt-and-stage.ps1` and `fmt-and-stage.sh`.
- `src/` — Legacy crate; historically contained a top-level `architecture.md`.

Crate responsibilities and interactions
------------------------------------
- Separation of concerns: each folder is a Rust crate with a focused responsibility (AI, combat, rendering, etc.). Crates expose clean APIs via `lib.rs` and keep implementation details private when possible.
- Data and domain models (units, items, maps) live in `Units/`, `Items/`, and `Maps/` and are consumed by `Combat/`, `AI/`, and `Game/`.
- `QuestApp/` composes subsystems to produce runnable binaries. The `Graphics/` crate provides rendering and input integration; `Game/` contains the game loop and uses `Combat/`, `AI/`, and domain crates.
- `Encyclopedia/` provides tooling used by `QuestApp/` to build in-game help and reference data.

Build, run, and test
--------------------
- The repository uses Cargo with workspace crates declared in the top-level `Cargo.toml`.
- To build everything: `cargo build --workspace`.
- To run the app (when `QuestApp` has a binary): `cargo run -p QuestApp`.
- Tests are colocated in each crate under `tests/` and can be run per-crate: `cargo test -p Game` or across the workspace: `cargo test --workspace`.

Conventions and patterns
------------------------
- Crate API surface: prefer small, well-documented public types and functions in `lib.rs` and keep implementation modules private.
- Error handling: use `Result<T, E>` and crate-local error types where useful. Avoid panics in library crates.
- Serialization: when data must be persisted (maps, encyclopedia entries), prefer deterministic formats (JSON) with explicit versioning where appropriate.
- Tests: unit tests belong alongside modules; integration tests live in the crate `tests/` directory. Keep tests fast and deterministic.

How to add a new subsystem or crate
----------------------------------
1. Create a new folder at the repository root with a `Cargo.toml` and `src/lib.rs`.
2. Add the crate to the workspace in the top-level `Cargo.toml` under `[workspace] members`.
3. Keep public API minimal and documented; add examples in the crate's `examples/` folder if helpful.
4. Add unit and integration tests to `tests/` and CI scripts to `scripts/` when necessary.

Design notes and data flow
-------------------------
- The project adopts a modular approach: pure data crates (Units, Items, Maps) do not depend on rendering or app code. Systems (AI, Combat, Game) depend on domain crates and expose engine-agnostic logic.
- Rendering is decoupled from game logic: `Graphics/` exposes drawing primitives and a renderer interface. `Game/` or `QuestApp/` is responsible for translating game state into render commands.
