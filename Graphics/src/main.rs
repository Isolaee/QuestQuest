use gl::types::*;
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use std::ffi::CString;
use std::mem;
use std::ptr;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Vec2) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// Axial coordinates for hexagonal grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct HexCoord {
    q: i32, // column
    r: i32, // row
}

impl HexCoord {
    fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    // Convert axial coordinates to world position (POINTY-TOP hexagons) - CORRECTED
    // Fixed: take self by value since HexCoord is Copy
    fn to_world_pos(self, hex_size: f32) -> Vec2 {
        let x = hex_size * (3.0_f32.sqrt() * (self.q as f32 + self.r as f32 / 2.0));
        let y = hex_size * (3.0 / 2.0 * self.r as f32);
        Vec2::new(x, y)
    }

    // Get distance between two hex coordinates
    #[allow(dead_code)]
    fn distance(self, other: HexCoord) -> i32 {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r).abs())
            / 2
    }

    // Get neighboring coordinates (corrected for POINTY-TOP hexagons)
    #[allow(dead_code)]
    fn neighbors(self) -> [HexCoord; 6] {
        [
            HexCoord::new(self.q + 1, self.r - 1), // Northeast
            HexCoord::new(self.q + 1, self.r),     // East
            HexCoord::new(self.q, self.r + 1),     // Southeast
            HexCoord::new(self.q - 1, self.r + 1), // Southwest
            HexCoord::new(self.q - 1, self.r),     // West
            HexCoord::new(self.q, self.r - 1),     // Northwest
        ]
    }
}

#[derive(Clone)]
struct Hexagon {
    coord: HexCoord,
    world_pos: Vec2,
    color: [f32; 3],
}

impl Hexagon {
    fn new(coord: HexCoord, hex_size: f32) -> Self {
        let world_pos = coord.to_world_pos(hex_size);

        // Generate color based on coordinate for visual debugging
        let color = [
            0.3 + 0.4 * ((coord.q + coord.r) % 3) as f32 / 3.0,
            0.4 + 0.3 * (coord.q % 4) as f32 / 4.0,
            0.5 + 0.3 * (coord.r % 5) as f32 / 5.0,
        ];

        Self {
            coord,
            world_pos,
            color,
        }
    }
}

struct Camera {
    position: Vec2,
    view_distance: f32,
}

impl Camera {
    fn new() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            view_distance: 3.0,
        }
    }

    fn can_see(&self, world_pos: Vec2) -> bool {
        let distance = self.position.distance(&world_pos);
        distance <= self.view_distance
    }

    // Convert camera position to hex coordinate (CORRECTED for pointy-top)
    #[allow(dead_code)]
    fn to_hex_coord(&self, hex_size: f32) -> HexCoord {
        let q = (2.0 / 3.0 * self.position.x) / (hex_size * 3.0_f32.sqrt());
        let r = (-1.0 / 3.0 * self.position.x + 3.0_f32.sqrt() / 3.0 * self.position.y)
            / (hex_size * 3.0 / 2.0);
        HexCoord::new(q.round() as i32, r.round() as i32)
    }
}

#[allow(dead_code)]
struct HexGrid {
    hexagons: std::collections::HashMap<HexCoord, Hexagon>,
    camera: Camera,
    hex_size: f32,
    #[allow(dead_code)]
    grid_radius: i32, // How far the grid extends
}

impl HexGrid {
    fn new() -> Self {
        let hex_size = 0.2; // Larger hexagons for better visibility
        let grid_radius = 15; // Grid extends 15 hexes in each direction
        let mut hexagons = std::collections::HashMap::new();

        // Generate hexagonal grid using axial coordinates
        for q in -grid_radius..=grid_radius {
            let r1 = (-grid_radius).max(-q - grid_radius);
            let r2 = grid_radius.min(-q + grid_radius);

            for r in r1..=r2 {
                let coord = HexCoord::new(q, r);
                let hexagon = Hexagon::new(coord, hex_size);
                hexagons.insert(coord, hexagon);
            }
        }

        Self {
            hexagons,
            camera: Camera::new(),
            hex_size,
            grid_radius,
        }
    }

    fn get_visible_hexagons(&self) -> Vec<&Hexagon> {
        // Get approximate camera hex coordinate for more efficient culling
        let cam_hex = self.camera.to_hex_coord(self.hex_size);
        let view_radius = (self.camera.view_distance / self.hex_size).ceil() as i32 + 1;

        self.hexagons
            .values()
            .filter(|hex| {
                // Quick hex-distance check first (cheaper than world distance)
                if cam_hex.distance(hex.coord) > view_radius {
                    return false;
                }
                // Then precise world distance check
                self.camera.can_see(hex.world_pos)
            })
            .collect()
    }

    fn move_camera(&mut self, dx: f32, dy: f32) {
        self.camera.position.x += dx;
        self.camera.position.y += dy;
    }

    // Get hexagon at specific coordinate (useful for game logic)
    #[allow(dead_code)]
    fn get_hex_at(&self, coord: HexCoord) -> Option<&Hexagon> {
        self.hexagons.get(&coord)
    }
}

struct App {
    window: Option<Window>,
    gl_context: Option<glutin::context::PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    vao: GLuint,
    shader_program: GLuint,
    hex_grid: HexGrid,
    dynamic_vbo: GLuint,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title("Hexagon Grid - Smooth Coordinates")
            .with_inner_size(winit::dpi::LogicalSize::new(1200, 800));

        let template = glutin::config::ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
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

        self.window = Some(window);
        self.gl_context = Some(gl_context);
        self.gl_surface = Some(gl_surface);
        self.vao = vao;
        self.shader_program = shader_program;
        self.dynamic_vbo = dynamic_vbo;
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
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    let move_speed = 0.1;
                    match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowUp) => {
                            self.hex_grid.move_camera(0.0, move_speed);
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowDown) => {
                            self.hex_grid.move_camera(0.0, -move_speed);
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft) => {
                            self.hex_grid.move_camera(-move_speed, 0.0);
                        }
                        winit::keyboard::PhysicalKey::Code(
                            winit::keyboard::KeyCode::ArrowRight,
                        ) => {
                            self.hex_grid.move_camera(move_speed, 0.0);
                        }
                        _ => {}
                    }
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if self.gl_context.is_some() && self.gl_surface.is_some() {
                    self.render();

                    if let (Some(gl_context), Some(gl_surface)) =
                        (&self.gl_context, &self.gl_surface)
                    {
                        gl_surface.swap_buffers(gl_context).unwrap();
                    }
                }
            }
            _ => (),
        }
    }
}

impl App {
    fn render(&mut self) {
        let visible_hexagons = self.hex_grid.get_visible_hexagons();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if !visible_hexagons.is_empty() {
                self.update_vertex_buffer(&visible_hexagons);

                gl::UseProgram(self.shader_program);
                gl::BindVertexArray(self.vao);

                // Draw all visible hexagons
                for (i, _) in visible_hexagons.iter().enumerate() {
                    let vertex_offset = (i * 8) as GLint; // 8 vertices per hexagon
                    gl::DrawArrays(gl::TRIANGLE_FAN, vertex_offset, 8);
                }
            }
        }
    }

    fn update_vertex_buffer(&self, visible_hexagons: &[&Hexagon]) {
        let mut vertices = Vec::new();

        for hex in visible_hexagons {
            // Generate hexagon vertices relative to camera
            let cam_x = self.hex_grid.camera.position.x;
            let cam_y = self.hex_grid.camera.position.y;

            let center_x = hex.world_pos.x - cam_x;
            let center_y = hex.world_pos.y - cam_y;

            // Center vertex with color
            vertices.extend_from_slice(&[
                center_x,
                center_y,
                0.0,
                hex.color[0],
                hex.color[1],
                hex.color[2],
            ]);

            // Outer vertices (6 points of POINTY-TOP hexagon)
            // Start at 30 degrees (π/6) to make pointy-top orientation
            for i in 0..=6 {
                let angle = (i as f32) * std::f32::consts::PI / 3.0 + std::f32::consts::PI / 6.0;
                let x = center_x + self.hex_grid.hex_size * angle.cos();
                let y = center_y + self.hex_grid.hex_size * angle.sin();
                vertices.extend_from_slice(&[x, y, 0.0, hex.color[0], hex.color[1], hex.color[2]]);
            }
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.dynamic_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
        }
    }
}

unsafe fn setup_dynamic_hexagons() -> (GLuint, GLuint, GLuint) {
    // Generate and bind VAO
    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // Generate VBO for dynamic data
    let mut vbo = 0;
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

    // Configure vertex attributes (position + color)
    // Position (3 floats)
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        6 * mem::size_of::<f32>() as GLsizei,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0);

    // Color (3 floats)
    gl::VertexAttribPointer(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        6 * mem::size_of::<f32>() as GLsizei,
        (3 * mem::size_of::<f32>()) as *const _,
    );
    gl::EnableVertexAttribArray(1);

    // Create shaders
    let vertex_shader_source = CString::new(
        r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;
        
        out vec3 vertexColor;
        
        void main() {
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            vertexColor = aColor;
        }
    "#,
    )
    .unwrap();

    let fragment_shader_source = CString::new(
        r#"
        #version 330 core
        in vec3 vertexColor;
        out vec4 FragColor;
        
        void main() {
            FragColor = vec4(vertexColor, 1.0);
        }
    "#,
    )
    .unwrap();

    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    gl::ShaderSource(
        vertex_shader,
        1,
        &vertex_shader_source.as_ptr(),
        ptr::null(),
    );
    gl::CompileShader(vertex_shader);

    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    gl::ShaderSource(
        fragment_shader,
        1,
        &fragment_shader_source.as_ptr(),
        ptr::null(),
    );
    gl::CompileShader(fragment_shader);

    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    (vao, shader_program, vbo)
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        window: None,
        gl_context: None,
        gl_surface: None,
        vao: 0,
        shader_program: 0,
        hex_grid: HexGrid::new(),
        dynamic_vbo: 0,
    };

    event_loop.run_app(&mut app).unwrap();
}
