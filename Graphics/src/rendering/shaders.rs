use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

/// Sets up OpenGL resources for dynamic hexagon rendering.
///
/// # Safety
///
/// This function makes direct OpenGL calls and must be called with a valid OpenGL context.
/// The caller must ensure:
/// - A valid OpenGL context is current on the calling thread
/// - OpenGL has been properly initialized
/// - The returned resources (VAO, shader program, VBO) are properly cleaned up
/// - No other thread is making conflicting OpenGL calls simultaneously
pub unsafe fn setup_dynamic_hexagons() -> (GLuint, GLuint, GLuint) {
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
