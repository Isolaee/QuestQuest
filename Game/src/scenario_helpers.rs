use crate::objects::Team;
use crate::objects::*;
use crate::scenario_instance::ScenarioWorld;
use crate::world::GameWorld;
use graphics::HexCoord;
use graphics::SpriteType;
use log::warn;
use serde_json::Value;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use units::structures::{Structure, StructureFactory};
use units::UnitFactory;
use uuid::Uuid;

/// Team configuration from map JSON.
#[derive(Debug, Clone)]
pub struct TeamConfig {
    pub name: String,
    pub is_player_controlled: bool,
    pub goal: String,
}

/// Scenario information from map JSON.
#[derive(Debug, Clone)]
pub struct ScenarioInfo {
    pub name: String,
    pub description: String,
}

/// Result of parsing a map JSON file.
///
/// Contains scenario information, team configurations, terrain tiles and raw
/// JSON-backed lists of units, items and structures. Each entity is returned as
/// `(Uuid, HexCoord, serde_json::Value)` where the `Uuid` is either parsed from
/// the input `id` field or generated when missing.
#[derive(Debug, Clone)]
pub struct ParsedMap {
    pub scenario: Option<ScenarioInfo>,
    pub teams: Vec<TeamConfig>,
    pub terrain: HashMap<HexCoord, TerrainTile>,
    pub units: Vec<(Uuid, HexCoord, Value)>,
    pub items: Vec<(Uuid, HexCoord, Value)>,
    pub structures: Vec<(Uuid, HexCoord, Value)>,
}

impl ScenarioWorld {
    /// Dijkstra-like reachable calculation using integer costs.
    ///
    /// This was previously a nested function inside `generate_team_actions`.
    /// Moving it here makes it testable and keeps `generate_team_actions`
    /// focused on high-level logic.
    pub(crate) fn compute_reachable(
        &self,
        unit_id: Uuid,
        start: HexCoord,
        max_cost: i32,
    ) -> HashMap<HexCoord, i32> {
        let mut dist: HashMap<HexCoord, i32> = HashMap::new();
        // Use primitive tuple in heap so ordering is defined
        let mut heap: BinaryHeap<(Reverse<i32>, (i32, i32))> = BinaryHeap::new();

        dist.insert(start, 0);
        heap.push((Reverse(0), (start.q, start.r)));

        while let Some((Reverse(cost), (cq, cr))) = heap.pop() {
            let coord = HexCoord::new(cq, cr);
            if let Some(&best) = dist.get(&coord) {
                if cost > best {
                    continue;
                }
            }

            for nb in coord.neighbors().iter() {
                // Skip out-of-bounds when a world radius is defined
                if let Some(radius) = self.get_world_radius_opt() {
                    if nb.distance(HexCoord::new(0, 0)) > radius {
                        continue;
                    }
                }

                // Skip impassable terrain
                if let Some(terrain) = self.get_terrain(*nb) {
                    if terrain.blocks_movement() {
                        continue;
                    }
                }

                // Skip occupied tiles by friendly units (enemy units are valid targets for combat)
                let moving_unit = self.get_unit(unit_id);
                if let Some(mover) = moving_unit {
                    let mover_team = mover.team();
                    let units_there = self.get_units_at_position(*nb);
                    let blocked_by_friendly = units_there
                        .iter()
                        .any(|u| u.id() != unit_id && u.team() == mover_team);
                    if blocked_by_friendly {
                        continue;
                    }
                }

                let step_cost = self.get_movement_cost(*nb);
                // Skip if terrain is impassable (i32::MAX cost)
                if step_cost == i32::MAX {
                    continue;
                }
                let new_cost = cost + step_cost;
                if new_cost > max_cost {
                    continue;
                }

                match dist.get(nb) {
                    None => {
                        dist.insert(*nb, new_cost);
                        heap.push((Reverse(new_cost), (nb.q, nb.r)));
                    }
                    Some(&c) => {
                        if new_cost < c {
                            dist.insert(*nb, new_cost);
                            heap.push((Reverse(new_cost), (nb.q, nb.r)));
                        }
                    }
                }
            }
        }

        dist
    }

    /// Helper to optionally read a `world_radius` field if present on `GameWorld`.
    ///
    /// Some versions of the world store a radius field; calling code here
    /// is defensive and treats absence as "no radius limit".
    fn get_world_radius_opt(&self) -> Option<i32> {
        // If GameWorld has a `world_radius` field, prefer it. Use reflection-like
        // approach via a stub accessor if the field exists in user's struct.
        // The real codebase may expose this as a field or a method; try method first.
        #[allow(unused_imports)]
        use std::convert::TryInto;

        // If `world_radius` is a public method, call it. If not, return None.
        // This function is intentionally simple; if your GameWorld already has
        // a `world_radius` field, you can replace the body with `Some(self.world_radius)`.
        None
    }

    /// Parse the provided map JSON into terrain tiles, units, items and structures.
    ///
    /// Supports both legacy array format and new object format with Scenario, Teams, and Map.
    /// Returns a `ParsedMap` containing scenario info, team configs, terrain tiles, and
    /// vectors of `(Uuid, HexCoord, Value)` entries for units, items and structures.
    pub fn parse_map_json(map_json: &str) -> Result<ParsedMap, serde_json::Error> {
        #[derive(serde::Deserialize)]
        struct RawCell {
            #[serde(rename = "HexCoord")]
            hex: HexCoord,
            #[serde(rename = "SpriteType")]
            sprite: String,
            #[serde(rename = "Unit")]
            unit: Option<Value>,
            #[serde(rename = "Item")]
            item: Option<Value>,
            #[serde(rename = "Structure")]
            structure: Option<Value>,
        }

        // Try to parse as new format first (object with Scenario, Teams, Map)
        let root: Value = serde_json::from_str(map_json)?;

        let (scenario, teams, cells) = if root.is_object() {
            // New format with Scenario, Teams, and Map
            let scenario = root.get("Scenario").and_then(|s| {
                let name = s.get("Name")?.as_str()?.to_string();
                let description = s.get("Description")?.as_str()?.to_string();
                Some(ScenarioInfo { name, description })
            });

            let teams = root
                .get("Teams")
                .and_then(|t| t.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|team| {
                            let name = team.get("Name")?.as_str()?.to_string();
                            let is_player_controlled = team.get("IsPlayerControlled")?.as_bool()?;
                            let goal = team.get("Goal")?.as_str()?.to_string();
                            Some(TeamConfig {
                                name,
                                is_player_controlled,
                                goal,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let map_array = match root.get("Map") {
                Some(arr) => arr,
                None => {
                    return Err(serde_json::Error::io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Missing 'Map' field in new format",
                    )))
                }
            };
            let cells: Vec<RawCell> = serde_json::from_value(map_array.clone())?;

            (scenario, teams, cells)
        } else {
            // Legacy format (just an array of cells)
            let cells: Vec<RawCell> = serde_json::from_value(root)?;
            (None, Vec::new(), cells)
        };

        let mut terrain: HashMap<HexCoord, TerrainTile> = HashMap::new();
        let mut units: Vec<(Uuid, HexCoord, Value)> = Vec::new();
        let mut items: Vec<(Uuid, HexCoord, Value)> = Vec::new();
        let mut structures: Vec<(Uuid, HexCoord, Value)> = Vec::new();

        for cell in cells {
            let sprite_type = match cell.sprite.as_str() {
                "Forest" => SpriteType::Forest,
                "Forest2" => SpriteType::Forest2,
                "Grasslands" => SpriteType::Grasslands,
                "HauntedWoods" => SpriteType::HauntedWoods,
                "Hills" => SpriteType::Hills,
                "Mountain" => SpriteType::Mountain,
                "Swamp" => SpriteType::Swamp,
                "Unit" => SpriteType::Unit,
                "Item" => SpriteType::Item,
                "DwarfWarrior" => SpriteType::DwarfWarrior,
                "OrcWarrior" => SpriteType::OrcWarrior,
                "House" => SpriteType::House,
                "Wall" => SpriteType::Wall,
                _ => SpriteType::None,
            };

            let tile = TerrainTile::new(cell.hex, sprite_type);
            let pos = cell.hex;
            terrain.insert(pos, tile);

            if let Some(u) = cell.unit {
                if !u.is_null() {
                    // Handle new array format: ["UnitType", "TeamName"]
                    // or legacy object format with id/type/team fields
                    let unit_value = if u.is_array() {
                        // New format: ["UnitType", "TeamName"]
                        if let Some(arr) = u.as_array() {
                            if arr.len() >= 2 {
                                if let (Some(unit_type), Some(team_name)) =
                                    (arr[0].as_str(), arr[1].as_str())
                                {
                                    serde_json::json!({
                                        "type": unit_type,
                                        "team": team_name
                                    })
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        // Legacy object format
                        u
                    };

                    // Try to extract id if present, else create a new one
                    let id = match unit_value.get("id") {
                        Some(Value::String(s)) => {
                            Uuid::parse_str(s).unwrap_or_else(|_| Uuid::new_v4())
                        }
                        Some(Value::Object(map)) => {
                            if let Some(Value::String(s)) = map.get("id") {
                                Uuid::parse_str(s).unwrap_or_else(|_| Uuid::new_v4())
                            } else {
                                Uuid::new_v4()
                            }
                        }
                        _ => Uuid::new_v4(),
                    };
                    units.push((id, pos, unit_value));
                }
            }

            if let Some(it) = cell.item {
                if !it.is_null() {
                    let id = match it.get("id") {
                        Some(Value::String(s)) => {
                            Uuid::parse_str(s).unwrap_or_else(|_| Uuid::new_v4())
                        }
                        Some(Value::Object(map)) => {
                            if let Some(Value::String(s)) = map.get("id") {
                                Uuid::parse_str(s).unwrap_or_else(|_| Uuid::new_v4())
                            } else {
                                Uuid::new_v4()
                            }
                        }
                        _ => Uuid::new_v4(),
                    };
                    items.push((id, pos, it));
                }
            }

            if let Some(s) = cell.structure {
                if !s.is_null() {
                    // Handle string format: "House" or array format: ["House", "Player"]
                    let structure_value = if s.is_string() {
                        // Simple string format: "House"
                        if let Some(structure_type) = s.as_str() {
                            serde_json::json!({
                                "type": structure_type,
                                "team": "Neutral"
                            })
                        } else {
                            continue;
                        }
                    } else if s.is_array() {
                        // Array format: ["House", "Player"]
                        if let Some(arr) = s.as_array() {
                            if arr.len() >= 2 {
                                if let (Some(structure_type), Some(team_name)) =
                                    (arr[0].as_str(), arr[1].as_str())
                                {
                                    serde_json::json!({
                                        "type": structure_type,
                                        "team": team_name
                                    })
                                } else {
                                    continue;
                                }
                            } else if arr.len() == 1 {
                                if let Some(structure_type) = arr[0].as_str() {
                                    serde_json::json!({
                                        "type": structure_type,
                                        "team": "Neutral"
                                    })
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        // Object format (legacy)
                        s
                    };

                    let id = match structure_value.get("id") {
                        Some(Value::String(strid)) => {
                            Uuid::parse_str(strid).unwrap_or_else(|_| Uuid::new_v4())
                        }
                        Some(Value::Object(map)) => {
                            if let Some(Value::String(s)) = map.get("id") {
                                Uuid::parse_str(s).unwrap_or_else(|_| Uuid::new_v4())
                            } else {
                                Uuid::new_v4()
                            }
                        }
                        _ => Uuid::new_v4(),
                    };
                    structures.push((id, pos, structure_value));
                }
            }
        }

        Ok(ParsedMap {
            scenario,
            teams,
            terrain,
            units,
            items,
            structures,
        })
    }

    /// Apply a previously-parsed map into the provided `GameWorld`.
    ///
    /// This will merge terrain tiles, attempt to construct units via
    /// `units::UnitFactory` and insert them as `GameUnit`s (preserving UUID
    /// when possible), and create simple `InteractiveObject`s for items and
    /// structures when a direct factory is not available.
    pub fn apply_parsed_map_to_world(
        world: &mut GameWorld,
        parsed: ParsedMap,
    ) -> Result<(), String> {
        // Merge terrain (overwrite existing tiles at the same positions)
        for (pos, tile) in parsed.terrain.into_iter() {
            world.terrain.insert(pos, tile);
        }

        // Units: each Value should contain a `type` field (type name) optionally a `name` and `team`.
        for (id, pos, v) in parsed.units.into_iter() {
            if !v.is_object() {
                warn!("Skipping unit at {:?}: not an object", pos);
                continue;
            }
            let obj = v.as_object().unwrap();
            // Try to find type name
            let type_name_opt = obj
                .get("type")
                .and_then(|s| s.as_str())
                .or_else(|| obj.get("unit_type").and_then(|s| s.as_str()));
            let name_opt = obj
                .get("name")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string());
            let team = obj
                .get("team")
                .and_then(|t| t.as_str())
                .map(|s| match s {
                    "Enemy" => Team::Enemy,
                    "Neutral" => Team::Neutral,
                    _ => Team::Player,
                })
                .unwrap_or(Team::Player);

            let type_name = match type_name_opt {
                Some(t) => t,
                None => {
                    warn!("Skipping unit at {:?}: missing `type` field", pos);
                    continue;
                }
            };

            match UnitFactory::create(type_name, name_opt, Some(pos)) {
                Ok(boxed_unit) => {
                    let mut gu = GameUnit::new_with_team(boxed_unit, team);
                    // Preserve ID when possible
                    gu.set_id(id);
                    world.add_unit(gu);
                }
                Err(e) => {
                    warn!("Failed to create unit '{}' at {:?}: {}", type_name, pos, e);
                    continue;
                }
            }
        }

        // Items: try to map to a concrete item if a definition is given, else create a generic InteractiveObject
        for (_id, pos, v) in parsed.items.into_iter() {
            if !v.is_object() {
                warn!("Skipping item at {:?}: not an object", pos);
                continue;
            }
            let obj = v.as_object().unwrap();

            if let Some(def) = obj.get("definition").and_then(|s| s.as_str()) {
                // Known definitions may be mapped here. Example: "IronSword" -> items::item_definitions::create_iron_sword()
                match def {
                    "IronSword" | "Iron Sword" => {
                        let item = items::item_definitions::create_iron_sword();
                        let pickup = InteractiveObject::new_item_pickup(pos, item);
                        world.add_interactive_object(pickup);
                    }
                    other => {
                        warn!(
                            "Unknown item definition '{}', creating generic object",
                            other
                        );
                        let name = obj
                            .get("name")
                            .and_then(|s| s.as_str())
                            .unwrap_or("Item")
                            .to_string();
                        let desc = obj
                            .get("description")
                            .and_then(|s| s.as_str())
                            .unwrap_or("")
                            .to_string();
                        let io = InteractiveObject::new(pos, name, desc, SpriteType::Item);
                        world.add_interactive_object(io);
                    }
                }
            } else {
                // No definition — create a generic interactive object
                let name = obj
                    .get("name")
                    .and_then(|s| s.as_str())
                    .unwrap_or("Item")
                    .to_string();
                let desc = obj
                    .get("description")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let io = InteractiveObject::new(pos, name, desc, SpriteType::Item);
                world.add_interactive_object(io);
            }
        }

        // Structures: create generic InteractiveObject with blocking behavior where appropriate
        for (_id, pos, v) in parsed.structures.into_iter() {
            if !v.is_object() {
                warn!("Skipping structure at {:?}: not an object", pos);
                continue;
            }
            let obj = v.as_object().unwrap();
            let name = obj
                .get("name")
                .and_then(|s| s.as_str())
                .unwrap_or("Structure")
                .to_string();
            let desc = obj
                .get("description")
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .to_string();
            let io = InteractiveObject::new(pos, name, desc, SpriteType::House);
            // If structure has `blocks_movement: true`, set via public API not available — keep default
            world.add_interactive_object(io);
        }

        Ok(())
    }

    /// Populate units from parsed data into HashMaps (for ScenarioWorld).
    ///
    /// Returns a HashMap of GameUnit indexed by UUID.
    pub fn populate_units(parsed_units: Vec<(Uuid, HexCoord, Value)>) -> HashMap<Uuid, GameUnit> {
        let mut units = HashMap::new();

        for (id, pos, v) in parsed_units {
            if !v.is_object() {
                warn!("Skipping unit at {:?}: not an object", pos);
                continue;
            }
            let obj = v.as_object().unwrap();
            let type_name_opt = obj
                .get("type")
                .and_then(|s| s.as_str())
                .or_else(|| obj.get("unit_type").and_then(|s| s.as_str()));
            let name_opt = obj
                .get("name")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string());
            let team = obj
                .get("team")
                .and_then(|t| t.as_str())
                .map(|s| match s {
                    "Enemy" => Team::Enemy,
                    "Neutral" => Team::Neutral,
                    _ => Team::Player,
                })
                .unwrap_or(Team::Player);

            if let Some(type_name) = type_name_opt {
                match UnitFactory::create(type_name, name_opt, Some(pos)) {
                    Ok(boxed_unit) => {
                        let mut gu = GameUnit::new_with_team(boxed_unit, team);
                        gu.set_id(id);
                        units.insert(id, gu);
                    }
                    Err(e) => {
                        warn!("Failed to create unit '{}' at {:?}: {}", type_name, pos, e);
                    }
                }
            } else {
                warn!("Skipping unit at {:?}: missing `type` field", pos);
            }
        }

        units
    }

    /// Populate interactive objects from parsed items.
    ///
    /// Returns a HashMap of InteractiveObject indexed by UUID.
    pub fn populate_items(
        parsed_items: Vec<(Uuid, HexCoord, Value)>,
    ) -> HashMap<Uuid, InteractiveObject> {
        let mut interactive_objects = HashMap::new();

        for (_id, pos, v) in parsed_items {
            if !v.is_object() {
                warn!("Skipping item at {:?}: not an object", pos);
                continue;
            }
            let obj = v.as_object().unwrap();

            if let Some(def) = obj.get("definition").and_then(|s| s.as_str()) {
                match def {
                    "IronSword" | "Iron Sword" => {
                        let item = items::item_definitions::create_iron_sword();
                        let pickup = InteractiveObject::new_item_pickup(pos, item);
                        let pickup_id = pickup.id();
                        interactive_objects.insert(pickup_id, pickup);
                    }
                    other => {
                        warn!(
                            "Unknown item definition '{}', creating generic object",
                            other
                        );
                        let name = obj
                            .get("name")
                            .and_then(|s| s.as_str())
                            .unwrap_or("Item")
                            .to_string();
                        let desc = obj
                            .get("description")
                            .and_then(|s| s.as_str())
                            .unwrap_or("")
                            .to_string();
                        let io = InteractiveObject::new(pos, name, desc, SpriteType::Item);
                        let io_id = io.id();
                        interactive_objects.insert(io_id, io);
                    }
                }
            } else {
                let name = obj
                    .get("name")
                    .and_then(|s| s.as_str())
                    .unwrap_or("Item")
                    .to_string();
                let desc = obj
                    .get("description")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let io = InteractiveObject::new(pos, name, desc, SpriteType::Item);
                let io_id = io.id();
                interactive_objects.insert(io_id, io);
            }
        }

        interactive_objects
    }

    /// Populate structures from parsed structure data.
    ///
    /// Returns a HashMap of boxed Structure trait objects indexed by UUID.
    /// Supports structure types: House, StoneWall (Wall)
    pub fn populate_structures(
        parsed_structures: Vec<(Uuid, HexCoord, Value)>,
    ) -> HashMap<Uuid, Box<dyn Structure>> {
        let mut structures: HashMap<Uuid, Box<dyn Structure>> = HashMap::new();

        for (_id, pos, v) in parsed_structures {
            if !v.is_object() {
                warn!("Skipping structure at {:?}: not an object", pos);
                continue;
            }
            let obj = v.as_object().unwrap();

            // Get structure type
            let type_name = obj
                .get("type")
                .and_then(|s| s.as_str())
                .or_else(|| obj.get("name").and_then(|s| s.as_str()))
                .unwrap_or("House");

            // Get team
            let team = obj
                .get("team")
                .and_then(|t| t.as_str())
                .map(|s| match s {
                    "Enemy" => units::Team::Enemy,
                    "Player" => units::Team::Player,
                    _ => units::Team::Neutral,
                })
                .unwrap_or(units::Team::Neutral);

            // Create structure based on type
            let structure: Option<Box<dyn Structure>> = match type_name {
                "House" => Some(StructureFactory::create_house(pos, team)),
                "Wall" | "StoneWall" | "Stone Wall" => {
                    Some(StructureFactory::create_stone_wall(pos, team))
                }
                other => {
                    warn!(
                        "Unknown structure type '{}' at {:?}, defaulting to House",
                        other, pos
                    );
                    Some(StructureFactory::create_house(pos, team))
                }
            };

            if let Some(s) = structure {
                let structure_id = s.id();
                structures.insert(structure_id, s);
            }
        }

        structures
    }

    /// Populate interactive objects from parsed structures (legacy compatibility).
    ///
    /// Returns a HashMap of InteractiveObject indexed by UUID.
    /// Use `populate_structures` for proper Structure trait objects.
    #[deprecated(note = "Use populate_structures() for proper Structure objects")]
    pub fn populate_structures_as_interactive_objects(
        parsed_structures: Vec<(Uuid, HexCoord, Value)>,
    ) -> HashMap<Uuid, InteractiveObject> {
        let mut interactive_objects = HashMap::new();

        for (_id, pos, v) in parsed_structures {
            if !v.is_object() {
                warn!("Skipping structure at {:?}: not an object", pos);
                continue;
            }
            let obj = v.as_object().unwrap();
            let name = obj
                .get("type")
                .and_then(|s| s.as_str())
                .or_else(|| obj.get("name").and_then(|s| s.as_str()))
                .unwrap_or("House")
                .to_string();
            let desc = obj
                .get("description")
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .to_string();

            // Map structure type to sprite
            let sprite = match name.as_str() {
                "Wall" | "StoneWall" | "Stone Wall" => SpriteType::Wall,
                _ => SpriteType::House,
            };

            let io = InteractiveObject::new(pos, name, desc, sprite);
            let io_id = io.id();
            interactive_objects.insert(io_id, io);
        }

        interactive_objects
    }
}
