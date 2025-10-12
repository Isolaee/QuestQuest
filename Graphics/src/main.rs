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

#[derive(Clone, Copy)]
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

#[derive(Clone)]
struct Hexagon {
    position: Vec2,
    color: [f32; 3],
}

struct Camera {
    position: Vec2,
    view_distance: f32,
}

impl Camera {
    fn new() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            view_distance: 2.0, // Can see 2 units in each direction
        }
    }

    fn can_see(&self, hex_position: Vec2) -> bool {
        let distance = self.position.distance(&hex_position);
        distance <= self.view_distance
    }
}

struct HexGrid {
    hexagons: Vec<Hexagon>,
    camera: Camera,
    hex_size: f32,
}

impl HexGrid {
    fn new() -> Self {
        let hex_size = 0.08;
        let mut hexagons = Vec::new();

        // Generate hex grid
        let hex_width = hex_size * 2.0 * 0.866; // sqrt(3)/2 * 2 * radius
        let hex_height = hex_size * 1.5;

        for row in -20..20 {
            for col in -25..25 {
                let x_offset = if row % 2 == 0 { 0.0 } else { hex_width * 0.5 };
                let x = col as f32 * hex_width + x_offset;
                let y = row as f32 * hex_height;

                // Vary colors based on position
                let color = [
                    0.3 + 0.3 * ((row + col) % 3) as f32 / 3.0,
                    0.5 + 0.3 * (row % 4) as f32 / 4.0,
                    0.4 + 0.4 * (col % 5) as f32 / 5.0,
                ];

                hexagons.push(Hexagon {
                    position: Vec2::new(x, y),
                    color,
                });
            }
        }

        Self {
            hexagons,
            camera: Camera::new(),
            hex_size,
        }
    }

    fn get_visible_hexagons(&self) -> Vec<&Hexagon> {
        self.hexagons
            .iter()
            .filter(|hex| self.camera.can_see(hex.position))
            .collect()
    }

    fn move_camera(&mut self, dx: f32, dy: f32) {
        self.camera.position.x += dx;
        self.camera.position.y += dy;
    }
}

struct App {
    window: Option<Window>,
    gl_context: Option<glutin::context::PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    vao: GLuint,
    shader_program: GLuint,
    hex_grid: HexGrid,
    dynamic_vbo: GLuint, // For dynamic vertex data
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title("Hexagon Grid")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600));

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
            std::num::NonZeroU32::new(800).unwrap(),
            std::num::NonZeroU32::new(600).unwrap(),
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
            gl::Viewport(0, 0, 800, 600);
            gl::ClearColor(0.1, 0.1, 0.2, 1.0);
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
                    match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowUp) => {
                            self.hex_grid.move_camera(0.0, 0.1);
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowDown) => {
                            self.hex_grid.move_camera(0.0, -0.1);
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft) => {
                            self.hex_grid.move_camera(-0.1, 0.0);
                        }
                        winit::keyboard::PhysicalKey::Code(
                            winit::keyboard::KeyCode::ArrowRight,
                        ) => {
                            self.hex_grid.move_camera(0.1, 0.0);
                        }
                        _ => {}
                    }
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                // Check if we have the required OpenGL objects first
                if self.gl_context.is_some() && self.gl_surface.is_some() {
                    // Call render first (which needs mutable access)
                    self.render();

                    // Then handle the swap buffers (which only needs immutable access)
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

            let center_x = hex.position.x - cam_x;
            let center_y = hex.position.y - cam_y;

            // Center vertex with color
            vertices.extend_from_slice(&[
                center_x,
                center_y,
                0.0,
                hex.color[0],
                hex.color[1],
                hex.color[2],
            ]);

            // Outer vertices
            for i in 0..=6 {
                let angle = (i as f32) * std::f32::consts::PI / 3.0;
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
