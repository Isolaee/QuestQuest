// Maps will be given as JSON files. Each JSON entry has the following structure:
// {
//   "HexCoord": {"q": 0, "r": 0},
//   "SpriteType": "Forest",
//   "Unit": "Warrior" // can be none
// },

// Hex lookup table will be generated based on JSON file.
// 1, Place hexes into world based on HexCoord.
// 2, Define world edges as world coordinates witch is pixel grid.
// 3, Define hexsize in pixels.
// 4, For each pixel in world, determine which hex it falls into.

use crate::core::hexagon::SpriteType;
use crate::core::HexCoord;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapHexEntry {
    #[serde(rename = "HexCoord")]
    pub hex_coord: HexCoord,
    #[serde(rename = "SpriteType")]
    pub sprite_type: SpriteType,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
}

#[derive(Debug)]
pub struct WorldHexLookupTable {
    // World bounds in pixels
    world_min_x: f32,
    world_min_y: f32,
    world_max_x: f32,
    world_max_y: f32,

    // Hex size in pixels
    _hex_size: f32,

    // Resolution (pixels per lookup entry)
    resolution: f32,

    // Actual lookup grid: pixel_lookup[y][x] = Option<HexCoord>
    pixel_lookup: Vec<Vec<Option<HexCoord>>>,
    width: usize,
    height: usize,

    // Hex data from JSON
    hex_entries: HashMap<HexCoord, MapHexEntry>,
}

impl WorldHexLookupTable {
    /// Load JSON and return just the HashMap
    pub fn from_json_hashmap(
        json_path: &str,
    ) -> Result<HashMap<HexCoord, MapHexEntry>, Box<dyn std::error::Error>> {
        println!("📖 Loading map from: {}", json_path);

        // Load and parse JSON
        let json_content = fs::read_to_string(json_path)?;
        println!("📄 JSON file size: {} bytes", json_content.len());

        let hex_entries: Vec<MapHexEntry> = serde_json::from_str(&json_content)?;
        println!("📍 Loaded {} hex entries from map", hex_entries.len());

        // Debug: Show first few entries
        if !hex_entries.is_empty() {
            println!("🔍 First few entries:");
            for (i, entry) in hex_entries.iter().take(3).enumerate() {
                println!(
                    "   [{}] {:?} -> {:?} (unit: {:?})",
                    i, entry.hex_coord, entry.sprite_type, entry.unit
                );
            }
            if hex_entries.len() > 3 {
                println!("   ... and {} more entries", hex_entries.len() - 3);
            }
        }

        // Convert to HashMap for fast lookup
        let mut hex_map = HashMap::new();
        for entry in hex_entries {
            println!("🗂️  Adding hex {:?} to map", entry.hex_coord);
            hex_map.insert(entry.hex_coord, entry);
        }
        println!("✅ HashMap created with {} entries", hex_map.len());

        Ok(hex_map)
    }

    /// Create WorldHexLookupTable from HashMap
    pub fn from_hashmap(
        hex_map: HashMap<HexCoord, MapHexEntry>,
        hex_size: f32,
        resolution: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        println!(
            "🔧 Creating WorldHexLookupTable from HashMap with hex_size={:.1}, resolution={:.2}",
            hex_size, resolution
        );
        println!("📍 Processing {} hexes from HashMap", hex_map.len());

        // Calculate world bounds and build lookup table
        Self::calculate_map(hex_map, hex_size, resolution)
    }

    /// Complete method: Load JSON and create lookup table
    pub fn from_json_map(
        json_path: &str,
        hex_size: f32,
        resolution: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Step 1: Load JSON into HashMap
        let hex_map = Self::from_json_hashmap(json_path)?;

        // Step 2: Create lookup table from HashMap
        Self::from_hashmap(hex_map, hex_size, resolution)
    }

    /// Build the actual lookup table (fixed to return Self)
    fn calculate_map(
        hex_map: HashMap<HexCoord, MapHexEntry>,
        hex_size: f32,
        resolution: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // DYNAMIC BOUNDS CALCULATION from actual map coordinates
        println!("📏 Calculating dynamic world bounds...");
        let bounds = Self::calculate_dynamic_world_bounds(&hex_map, hex_size);
        let (min_x, min_y, max_x, max_y) = bounds;

        println!(
            "🌍 Calculated bounds: ({:.1}, {:.1}) to ({:.1}, {:.1})",
            min_x, min_y, max_x, max_y
        );
        println!(
            "📐 World dimensions: {:.1} x {:.1} pixels",
            max_x - min_x,
            max_y - min_y
        );

        // Calculate lookup grid dimensions based on actual world size
        let width = ((max_x - min_x) / resolution).ceil() as usize;
        let height = ((max_y - min_y) / resolution).ceil() as usize;

        println!("🗺️  Building dynamic lookup table:");
        println!(
            "   📏 Map coordinate range: q={} to {}, r={} to {}",
            hex_map.keys().map(|c| c.q).min().unwrap_or(0),
            hex_map.keys().map(|c| c.q).max().unwrap_or(0),
            hex_map.keys().map(|c| c.r).min().unwrap_or(0),
            hex_map.keys().map(|c| c.r).max().unwrap_or(0)
        );
        println!(
            "   🌍 World bounds: ({:.1}, {:.1}) to ({:.1}, {:.1})",
            min_x, min_y, max_x, max_y
        );
        println!("   📐 Grid size: {}x{} entries", width, height);
        println!("   🔍 Resolution: {:.2} pixels per entry", resolution);
        println!(
            "   💾 Estimated memory: {:.2} MB",
            (width * height * std::mem::size_of::<Option<HexCoord>>()) as f32 / (1024.0 * 1024.0)
        );

        // Build the lookup grid
        println!("🔨 Building lookup grid...");
        let mut pixel_lookup = vec![vec![None; width]; height];
        let mut filled_count = 0;
        let mut debug_sample_count = 0;

        for (y, row) in pixel_lookup.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let world_x = min_x + (x as f32 * resolution);
                let world_y = min_y + (y as f32 * resolution);

                // Debug: Show progress every 10% and sample some lookups
                if y % (height / 10).max(1) == 0 && x == 0 {
                    println!(
                        "   ⏳ Progress: {:.0}% (row {} of {})",
                        (y as f32 / height as f32) * 100.0,
                        y,
                        height
                    );
                }

                // Find which hex contains this world position
                if let Some(hex_coord) =
                    Self::find_hex_at_world_pos(world_x, world_y, &hex_map, hex_size)
                {
                    *cell = Some(hex_coord);
                    filled_count += 1;

                    // Debug: Sample first few successful lookups
                    if debug_sample_count < 5 {
                        println!(
                            "   🎯 Sample lookup: world({:.1}, {:.1}) -> grid[{}][{}] -> {:?}",
                            world_x, world_y, y, x, hex_coord
                        );
                        debug_sample_count += 1;
                    }
                } else {
                    // Debug: Sample first few failed lookups
                    if debug_sample_count < 10 && filled_count > 0 {
                        println!(
                            "   ❌ Failed lookup: world({:.1}, {:.1}) -> grid[{}][{}] -> None",
                            world_x, world_y, y, x
                        );
                    }
                }
            }
        }

        println!("✅ Dynamic lookup table built!");
        println!("   📊 Total entries: {}", width * height);
        println!(
            "   📊 Filled entries: {} ({:.1}%)",
            filled_count,
            (filled_count as f32 / (width * height) as f32) * 100.0
        );
        println!("   📊 Hexagons in map: {}", hex_map.len());
        println!(
            "   📊 Average fills per hex: {:.1}",
            filled_count as f32 / hex_map.len() as f32
        );

        // Debug: Show some world coordinate samples
        if !hex_map.is_empty() {
            println!("🔍 Hex world position samples:");
            for (i, hex_coord) in hex_map.keys().take(3).enumerate() {
                let world_pos = hex_coord.to_world_pos(hex_size);
                println!(
                    "   [{}] {:?} -> world({:.1}, {:.1})",
                    i, hex_coord, world_pos.x, world_pos.y
                );
            }
        }

        let result = Self {
            world_min_x: min_x,
            world_min_y: min_y,
            world_max_x: max_x,
            world_max_y: max_y,
            _hex_size: hex_size,
            resolution,
            pixel_lookup,
            width,
            height,
            hex_entries: hex_map,
        };

        println!("🎉 WorldHexLookupTable created successfully!");

        Ok(result)
    }

    /// Calculate world bounds dynamically from actual hex coordinates in the map
    fn calculate_dynamic_world_bounds(
        hex_map: &HashMap<HexCoord, MapHexEntry>,
        hex_size: f32,
    ) -> (f32, f32, f32, f32) {
        if hex_map.is_empty() {
            // Fallback for empty map
            return (-hex_size, -hex_size, hex_size, hex_size);
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        // Calculate bounds from actual hex coordinates in the map
        for hex_coord in hex_map.keys() {
            let world_pos = hex_coord.to_world_pos(hex_size);

            // Account for hex radius when calculating bounds
            let hex_radius = hex_size; // removed padding *1.1

            min_x = min_x.min(world_pos.x - hex_radius);
            max_x = max_x.max(world_pos.x + hex_radius);
            min_y = min_y.min(world_pos.y - hex_radius);
            max_y = max_y.max(world_pos.y + hex_radius);
        }

        // Add extra padding around the entire map for smooth boundary handling
        let map_padding = hex_size; // removed map padding, that was * 2.0
        min_x -= map_padding;
        min_y -= map_padding;
        max_x += map_padding;
        max_y += map_padding;

        (min_x, min_y, max_x, max_y)
    }

    // Add method to expose bounds for debugging
    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (
            self.world_min_x,
            self.world_min_y,
            self.world_max_x,
            self.world_max_y,
        )
    }

    fn find_hex_at_world_pos(
        world_x: f32,
        world_y: f32,
        hex_map: &HashMap<HexCoord, MapHexEntry>,
        hex_size: f32,
    ) -> Option<HexCoord> {
        let mut closest_hex = None;
        let mut min_distance = f32::MAX;

        // Check all hexagons in the map
        for hex_coord in hex_map.keys() {
            let hex_world_pos = hex_coord.to_world_pos(hex_size);
            let distance =
                ((world_x - hex_world_pos.x).powi(2) + (world_y - hex_world_pos.y).powi(2)).sqrt();

            // Check if point is within this hexagon
            if Self::point_in_hexagon(world_x, world_y, hex_world_pos, hex_size) {
                return Some(*hex_coord);
            }

            // Track closest for potential fallback
            if distance < min_distance {
                min_distance = distance;
                closest_hex = Some(*hex_coord);
            }
        }

        // Only return closest if it's reasonably close (within hex boundary)
        if min_distance <= hex_size * 1.2 {
            closest_hex
        } else {
            None
        }
    }

    fn point_in_hexagon(px: f32, py: f32, hex_center: crate::math::Vec2, hex_size: f32) -> bool {
        // Distance check - approximate hexagon as circle
        let dx = px - hex_center.x;
        let dy = py - hex_center.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance <= hex_size * 0.9 // Hexagon inscribed circle radius ≈ 0.866 * hex_size
    }

    /// Get hex coordinate for world position (INSTANT LOOKUP)
    pub fn get_hex_at_world_pos(&self, world_x: f32, world_y: f32) -> Option<HexCoord> {
        // Check bounds
        if world_x < self.world_min_x
            || world_x > self.world_max_x
            || world_y < self.world_min_y
            || world_y > self.world_max_y
        {
            return None;
        }

        // Convert to lookup grid indices
        let x = ((world_x - self.world_min_x) / self.resolution) as usize;
        let y = ((world_y - self.world_min_y) / self.resolution) as usize;

        if x < self.width && y < self.height {
            self.pixel_lookup[y][x]
        } else {
            None
        }
    }

    /// Get map entry for hex coordinate
    pub fn get_hex_entry(&self, hex_coord: &HexCoord) -> Option<&MapHexEntry> {
        self.hex_entries.get(hex_coord)
    }

    /// Get all hex coordinates in the map
    pub fn get_all_hex_coords(&self) -> Vec<HexCoord> {
        self.hex_entries.keys().cloned().collect()
    }

    pub fn get_stats(&self) -> String {
        let total_entries = self.width * self.height;
        let filled_entries = self
            .pixel_lookup
            .iter()
            .flatten()
            .filter(|entry| entry.is_some())
            .count();

        // Calculate memory usage
        let lookup_grid_size = total_entries * std::mem::size_of::<Option<HexCoord>>();
        let hex_entries_size =
            self.hex_entries.len() * std::mem::size_of::<(HexCoord, MapHexEntry)>();
        let struct_overhead = std::mem::size_of::<Self>();
        let total_memory = lookup_grid_size + hex_entries_size + struct_overhead;

        format!(
            "Lookup Table Stats:\n   📊 Total entries: {}\n   📊 Filled entries: {} ({:.1}%)\n   📊 Hexagons in map: {}\n   💾 Memory usage:\n      - Lookup grid: {:.2} MB\n      - Hex entries: {:.1} KB\n      - Struct overhead: {} bytes\n      - Total memory: {:.2} MB",
            total_entries,
            filled_entries,
            (filled_entries as f32 / total_entries as f32) * 100.0,
            self.hex_entries.len(),
            lookup_grid_size as f32 / (1024.0 * 1024.0),      // MB
            hex_entries_size as f32 / 1024.0,                  // KB  
            struct_overhead,                                    // bytes
            total_memory as f32 / (1024.0 * 1024.0)           // MB
        )
    }

    /// Get detailed memory breakdown
    pub fn get_memory_info(&self) -> String {
        let total_entries = self.width * self.height;
        let lookup_grid_size = total_entries * std::mem::size_of::<Option<HexCoord>>();
        let hex_entries_size =
            self.hex_entries.len() * std::mem::size_of::<(HexCoord, MapHexEntry)>();
        let struct_overhead = std::mem::size_of::<Self>();

        format!(
            "📊 Lookup Table Memory Breakdown:\n\
             ├─ Grid dimensions: {}x{} = {} entries\n\
             ├─ Entry size: {} bytes (Option<HexCoord>)\n\
             ├─ Lookup grid: {} entries × {} bytes = {:.2} MB\n\
             ├─ Hex entries: {} × {} bytes = {:.1} KB\n\
             ├─ Struct fields: {} bytes\n\
             └─ Total memory: {:.2} MB\n\
             \n\
             🎯 Efficiency: {:.1}% of grid is filled\n\
             📐 Resolution: {:.2} pixels per lookup entry\n\
             🌍 World area: {:.1}×{:.1} = {:.0} square units",
            self.width,
            self.height,
            total_entries,
            std::mem::size_of::<Option<HexCoord>>(),
            total_entries,
            std::mem::size_of::<Option<HexCoord>>(),
            lookup_grid_size as f32 / (1024.0 * 1024.0),
            self.hex_entries.len(),
            std::mem::size_of::<(HexCoord, MapHexEntry)>(),
            hex_entries_size as f32 / 1024.0,
            struct_overhead,
            (lookup_grid_size + hex_entries_size + struct_overhead) as f32 / (1024.0 * 1024.0),
            (self
                .pixel_lookup
                .iter()
                .flatten()
                .filter(|e| e.is_some())
                .count() as f32
                / total_entries as f32)
                * 100.0,
            self.resolution,
            self.world_max_x - self.world_min_x,
            self.world_max_y - self.world_min_y,
            (self.world_max_x - self.world_min_x) * (self.world_max_y - self.world_min_y)
        )
    }
}
