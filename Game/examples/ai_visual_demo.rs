use game::{GameObject, GameWorld, Team};
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use graphics::{setup_dynamic_hexagons, HexCoord, HexGrid, Renderer, SpriteType};
use raw_window_handle::HasWindowHandle;
use std::ffi::CString;
use uuid::Uuid;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

/// Simple helper to draw a straight hex line between two hex coords (inclusive)
fn hex_linedraw(a: HexCoord, b: HexCoord) -> Vec<HexCoord> {
    // Convert to cube coordinates (x=q, z=r, y=-x-z)
    let ax = a.q as f32;
    let az = a.r as f32;
    let ay = -ax - az;
    let bx = b.q as f32;
    let bz = b.r as f32;
    let by = -bx - bz;

    let n = a.distance(b) as usize;
    let mut results = Vec::new();
    for i in 0..=n {
        let t = if n == 0 { 0.0 } else { i as f32 / n as f32 };
        let x = ax + (bx - ax) * t;
        let y = ay + (by - ay) * t;
        let z = az + (bz - az) * t;

        // Round cube coords
        let mut rx = x.round();
        let mut ry = y.round();
        let mut rz = z.round();

        let x_diff = (rx - x).abs();
        let y_diff = (ry - y).abs();
        let z_diff = (rz - z).abs();

        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry - rz;
        } else if y_diff > z_diff {
            ry = -rx - rz;
        } else {
            rz = -rx - ry;
        }

        // suppress unused-assignment lint for ry as in other code
        let _ = ry;

        // convert back to axial (q = x, r = z)
        results.push(HexCoord::new(rx as i32, rz as i32));
    }
    results
}

struct AnimatedUnit {
    _id: Uuid,
    path: Vec<HexCoord>, // sequence of hexes to step through
    idx: usize,          // current index in path (points to hex where unit currently is)
    step_timer: f32,     // accum time since last step
    step_duration: f32,  // seconds per step
    sprite: SpriteType,
}

fn main() {
    // Build a small world and compute joint plans
    let mut world = GameWorld::new(5);
    world.turn_system.start_game();

    // Add two enemy units and two player units
    let u1 = units::UnitFactory::create_goblin_grunt(
        "Goblin A".to_string(),
        HexCoord::new(3, 3),
        units::Terrain::Grasslands,
    );
    let u2 = units::UnitFactory::create_goblin_grunt(
        "Goblin B".to_string(),
        HexCoord::new(3, 4),
        units::Terrain::Grasslands,
    );
    let e1 = units::UnitFactory::create_human_warrior(
        "Human X".to_string(),
        HexCoord::new(-3, -3),
        units::Terrain::Grasslands,
    );
    let e2 = units::UnitFactory::create_human_warrior(
        "Human Y".to_string(),
        HexCoord::new(-3, -4),
        units::Terrain::Grasslands,
    );

    let _id1 = world.add_unit(game::GameUnit::new_with_team(u1, Team::Enemy));
    let _id2 = world.add_unit(game::GameUnit::new_with_team(u2, Team::Enemy));
    let _hid1 = world.add_unit(game::GameUnit::new_with_team(e1, Team::Player));
    let _hid2 = world.add_unit(game::GameUnit::new_with_team(e2, Team::Player));

    // Visualize Enemy plans — ensure Enemy is AI
    world.turn_system.set_team_control(Team::Enemy, false);
    world.turn_system.end_turn(); // make it Enemy turn

    // For the demo, give all units ample movement so the planner can choose moves
    for (_id, u) in world.units.iter_mut() {
        u.set_moves_left(10);
    }

    // Log positions before running AI
    println!("Unit positions BEFORE AI run:");
    for (id, u) in &world.units {
        println!(" - {} at {:?}", id, u.position());
    }

    // Run the AI which will execute actions (move/attack) and populate ai_event_queue
    world.run_ai_for_current_team();

    // Log positions after AI run
    println!("Unit positions AFTER AI run:");
    for (id, u) in &world.units {
        println!(" - {} at {:?}", id, u.position());
    }

    // Drain and log any AI events produced (for debug)
    if let Ok(q) = world.ai_event_queue.lock() {
        if !q.is_empty() {
            println!("AI events emitted (draining): {}", q.len());
            for ev in q.iter() {
                println!(" - event: {:?}", ev);
            }
        } else {
            println!("No AI events emitted");
        }
    }

    // Prepare AI input and actions
    let ws = world.extract_world_state_for_team(Team::Enemy);
    let actions = world.generate_team_actions(Team::Enemy);

    use ai::FactValue as AiFactValue;
    use ai::Goal as AiGoal;
    use std::collections::HashMap as StdHashMap;

    let mut goals_per_agent: StdHashMap<String, Vec<AiGoal>> = StdHashMap::new();
    let mut agent_order: Vec<String> = Vec::new();

    for (id, unit) in &world.units {
        if unit.team() != Team::Enemy {
            continue;
        }
        let aid = id.to_string();
        agent_order.push(aid.clone());
        let mut goals: Vec<AiGoal> = Vec::new();
        for nb in unit.position().neighbors().iter() {
            for u in world.get_units_at_position(*nb) {
                if u.team() != unit.team() {
                    goals.push(AiGoal {
                        key: format!("Unit:{}:Alive", u.id()),
                        value: AiFactValue::Bool(false),
                    });
                }
            }
        }
        // If no explicit combat goals were found, add a simple movement goal to move toward origin
        if goals.is_empty() {
            goals.push(AiGoal {
                key: format!("Unit:{}:At", id),
                value: AiFactValue::Str(format!("{},{}", 0, 0)),
            });
        }
        goals_per_agent.insert(aid, goals);
    }

    let plans = ai::plan_for_team(&ws, &actions, &goals_per_agent, &agent_order, 5000);

    // Log plans and available actions for debugging
    println!("AI: computed {} plans", plans.len());
    println!("AI: total grounded actions available: {}", actions.len());
    for (agent, plan) in &plans {
        println!("Plan for agent {}: indices={:?}", agent, plan);
        // list visible actions that belong to this agent
        let agent_actions: Vec<&ai::ActionInstance> = actions
            .iter()
            .filter(|a| a.agent.as_ref().map(|s| s == agent).unwrap_or(false))
            .collect();
        println!("  visible actions ({}):", agent_actions.len());
        for (i, a) in agent_actions.iter().enumerate() {
            println!("    [{}] {}", i, a.name);
        }
        if plan.is_empty() && !agent_actions.is_empty() {
            // Try single-agent planner to see why no plan was found
            use ai::plan_instances;
            // Reconstruct the agent-visible instances vector (owned)
            let agent_instances: Vec<ai::ActionInstance> = actions
                .iter()
                .filter(|a| a.agent.as_ref().map(|s| s == agent).unwrap_or(false))
                .cloned()
                .collect();
            // use first goal for this agent if present
            if let Some(goals_vec) = goals_per_agent.get(agent) {
                if let Some(goal) = goals_vec.first() {
                    println!("  Trying single-agent planner for goal {:?}...", goal);
                    if let Some(idxvec) = plan_instances(&ws, &agent_instances, goal, 2000) {
                        println!("    single-agent plan indices: {:?}", idxvec);
                        let chosen: Vec<String> = idxvec
                            .iter()
                            .filter_map(|&i| agent_instances.get(i).map(|a| a.name.clone()))
                            .collect();
                        println!("    single-agent plan actions: {:?}", chosen);
                    } else {
                        println!("    single-agent planner found no plan (within limit)");
                    }
                }
            }
        }
    }

    // Set up GL rendering (same as joint_visual_demo)
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window_attributes = winit::window::WindowAttributes::default()
        .with_title("AI Visual Demo: animated moves")
        .with_inner_size(winit::dpi::LogicalSize::new(1200, 800));

    let template = glutin::config::ConfigTemplateBuilder::new();
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            configs.reduce(|a, _| a).unwrap()
        })
        .unwrap();

    let window = window.unwrap();
    let gl_display = gl_config.display();
    let context_attributes =
        ContextAttributesBuilder::new().build(Some(window.window_handle().unwrap().into()));
    let mut not_current_gl_context =
        Some(unsafe { gl_display.create_context(&gl_config, &context_attributes) }.unwrap());

    let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window.window_handle().unwrap().into(),
        std::num::NonZeroU32::new(1200).unwrap(),
        std::num::NonZeroU32::new(800).unwrap(),
    );

    let gl_surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &surface_attributes)
            .unwrap()
    };
    let gl_context = not_current_gl_context
        .take()
        .unwrap()
        .make_current(&gl_surface)
        .unwrap();

    gl::load_with(|symbol| {
        let symbol = CString::new(symbol).unwrap();
        gl_display.get_proc_address(symbol.as_c_str()).cast()
    });

    unsafe {
        gl::Viewport(0, 0, 1200, 800);
        gl::ClearColor(0.05, 0.05, 0.1, 1.0);
    }

    let (vao, shader_program, dynamic_vbo) = unsafe { setup_dynamic_hexagons() };
    let renderer =
        Renderer::new(vao, shader_program, dynamic_vbo, 1200.0, 800.0).expect("renderer");

    let mut hex_grid = HexGrid::new();

    // Place unit sprites on the grid based on world
    for (_id, unit) in &world.units {
        let coord = unit.position();
        let sprite = if unit.team() == Team::Enemy {
            SpriteType::Unit
        } else {
            SpriteType::Item
        };
        hex_grid.set_unit_at(coord, sprite);
    }

    // Build animated unit tasks from plans
    let mut anims: Vec<AnimatedUnit> = Vec::new();
    for (agent, plan) in &plans {
        if let Ok(uuid) = Uuid::parse_str(agent) {
            if let Some(unit) = world.get_unit(uuid) {
                // collect visible actions for this agent
                let agent_actions: Vec<ai::ActionInstance> = actions
                    .iter()
                    .filter(|a| a.agent.as_ref().map(|s| s == agent).unwrap_or(false))
                    .cloned()
                    .collect();
                let mut current = unit.position();
                for &local_idx in plan {
                    if let Some(a) = agent_actions.get(local_idx) {
                        if a.name.starts_with("Move") {
                            if let Some((_, ai::FactValue::Str(dest))) = a.effects.first() {
                                // dest format sometimes "q,r" or "q:r" in different places; support both
                                let dest = dest.replace(':', ",");
                                let parts: Vec<&str> = dest.split(',').collect();
                                if parts.len() == 2 {
                                    if let (Ok(q), Ok(r)) =
                                        (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                                    {
                                        let target = HexCoord::new(q, r);
                                        // create path from current -> target
                                        let path = hex_linedraw(current, target);
                                        // skip the first element of path to avoid duplicate
                                        let path_steps = if path.len() > 1 {
                                            path[1..].to_vec()
                                        } else {
                                            vec![]
                                        };
                                        if !path_steps.is_empty() {
                                            println!(
                                                "Creating animation for unit {} path {:?}",
                                                uuid, path_steps
                                            );
                                            anims.push(AnimatedUnit {
                                                _id: uuid,
                                                path: path_steps,
                                                idx: 0,
                                                step_timer: 0.0,
                                                step_duration: 0.18,
                                                sprite: if unit.team() == Team::Enemy {
                                                    SpriteType::Unit
                                                } else {
                                                    SpriteType::Item
                                                },
                                            });
                                        }
                                        current = target;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Event loop with simple per-frame updates to advance animations
    let _ = event_loop.run_app(&mut SimpleApp {
        gl_context: Some(gl_context),
        gl_surface: Some(gl_surface),
        renderer,
        hex_grid,
        anims,
    });
}

struct SimpleApp {
    gl_context: Option<glutin::context::PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    renderer: Renderer,
    hex_grid: HexGrid,
    anims: Vec<AnimatedUnit>,
}

impl winit::application::ApplicationHandler for SimpleApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // nothing
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // advance animations by a fixed tick (approximate)
                let dt = 1.0 / 60.0; // 60 FPS step
                                     // For each animated unit, advance timer and maybe step
                let mut to_remove: Vec<usize> = Vec::new();
                for (i, a) in self.anims.iter_mut().enumerate() {
                    a.step_timer += dt;
                    if a.step_timer >= a.step_duration {
                        a.step_timer = 0.0;
                        // move on hex grid from previous pos to next
                        if a.idx < a.path.len() {
                            // find previous position (either previous path entry or dst of previous)
                            let new_coord = a.path[a.idx];
                            // remove any unit at previous coordinate for this id
                            // naive: remove unit from all hexes and set at new_coord
                            // first clear existing sprite for this unit by scanning all hexes
                            for hex in self.hex_grid.hexagons.values_mut() {
                                if hex.unit_sprite == Some(a.sprite) {
                                    // Heuristic: clear unit sprites of this type when matching idempotent visuals.
                                    // This is simple but may remove other units of same sprite; acceptable for demo.
                                    hex.unit_sprite = None;
                                }
                            }
                            self.hex_grid.set_unit_at(new_coord, a.sprite);
                            a.idx += 1;
                        } else {
                            to_remove.push(i);
                        }
                    }
                }
                // remove finished animations (reverse order)
                for &i in to_remove.iter().rev() {
                    self.anims.remove(i);
                }

                self.renderer.render(&self.hex_grid);
                if let (Some(gl_context), Some(gl_surface)) = (&self.gl_context, &self.gl_surface) {
                    gl_surface.swap_buffers(gl_context).unwrap();
                }
            }
            _ => {}
        }
    }
}
