use game::{GameObject, GameWorld, Team};
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use graphics::{setup_dynamic_hexagons, HexCoord, HexGrid, HighlightType, Renderer, SpriteType};
use raw_window_handle::HasWindowHandle;
use std::ffi::CString;
use uuid::Uuid;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

fn main() {
    // Build a small world and compute joint plans
    let mut world = GameWorld::new(5);
    world.turn_system.start_game();

    // Add two friendly units (AI team) and two enemies
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

    // We'll visualize plans for the Enemy team (AI-controlled). Ensure Enemy is AI.
    world.turn_system.set_team_control(Team::Enemy, false);
    // Make it Enemy turn
    world.turn_system.end_turn();

    // Prepare AI input
    let ws = world.extract_world_state_for_team(Team::Enemy);
    let actions = world.generate_team_actions(Team::Enemy);

    // Build goals similar to run_ai_for_current_team: adjacent-kill goals
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
        goals_per_agent.insert(aid, goals);
    }

    let plans = ai::plan_for_team(&ws, &actions, &goals_per_agent, &agent_order, 5000);

    // Now set up rendering using the same GL setup as the Graphics binary
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window_attributes = winit::window::WindowAttributes::default()
        .with_title("Joint Planning Visual Demo")
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

    // Populate unit sprites on hex grid
    for (_id, unit) in &world.units {
        let coord = unit.position();
        let sprite = if unit.team() == Team::Enemy {
            SpriteType::Unit
        } else {
            SpriteType::Item
        };
        hex_grid.set_unit_at(coord, sprite);
    }

    // Highlight planned moves and attacks
    for (agent, plan) in &plans {
        // Find agent unit
        if let Ok(uuid) = Uuid::parse_str(agent) {
            if world.get_unit(uuid).is_some() {
                // Build agent_actions list same way run_ai_for_current_team does
                let agent_actions: Vec<ai::ActionInstance> = actions
                    .iter()
                    .filter(|a| a.agent.as_ref().map(|s| s == agent).unwrap_or(false))
                    .cloned()
                    .collect();
                // For visualization use different highlight per agent
                for &idx in plan {
                    if let Some(a) = agent_actions.get(idx) {
                        if a.name.starts_with("Move-") {
                            if let Some((_, ai::FactValue::Str(dest))) = a.effects.first() {
                                let parts: Vec<&str> = dest.split(',').collect();
                                if parts.len() == 2 {
                                    if let (Ok(q), Ok(r)) =
                                        (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                                    {
                                        hex_grid.highlight_hex(
                                            HexCoord::new(q, r),
                                            HighlightType::MovementRange,
                                        );
                                    }
                                }
                            }
                        } else if a.name.starts_with("Attack-") {
                            if let Some((k, _)) = a.effects.first() {
                                if k.starts_with("Unit:") && k.ends_with(":Alive") {
                                    let mid = &k[5..k.len() - 6];
                                    if let Ok(target_uuid) = Uuid::parse_str(mid) {
                                        if let Some(target_unit) = world.get_unit(target_uuid) {
                                            hex_grid.highlight_hex(
                                                target_unit.position(),
                                                HighlightType::Selected,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Main event loop: render until window closed
    let _ = event_loop.run_app(&mut SimpleApp {
        gl_context: Some(gl_context),
        gl_surface: Some(gl_surface),
        renderer,
        hex_grid,
    });
}

struct SimpleApp {
    gl_context: Option<glutin::context::PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    renderer: Renderer,
    hex_grid: HexGrid,
}

impl winit::application::ApplicationHandler for SimpleApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // nothing
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                _event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.renderer.render(&self.hex_grid);
                if let (Some(gl_context), Some(gl_surface)) = (&self.gl_context, &self.gl_surface) {
                    gl_surface.swap_buffers(gl_context).unwrap();
                }
            }
            _ => {}
        }
    }
}
