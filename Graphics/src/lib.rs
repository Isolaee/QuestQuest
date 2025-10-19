pub mod core;
pub mod math;
pub mod rendering;
pub mod ui;

// Re-export commonly used types
pub use core::{
    Camera, HexCoord, HexGrid, Hexagon, HighlightType, SpriteType, WorldHexLookupTable,
};
pub use math::Vec2;
pub use rendering::{
    setup_dynamic_hexagons, GuideBuilder, GuideDisplay, GuideEntry, GuideLibrary, MenuAction,
    Renderer,
};
pub use ui::{UiPanel, UnitDisplayInfo};
