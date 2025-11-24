use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

/// Sets up OpenGL resources for dynamic hexagon rendering with textures.
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

    // Configure vertex attributes (position + texture coords + texture ID + color)
    // Position (3 floats)
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        9 * mem::size_of::<f32>() as GLsizei,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0);

    // Texture coordinates (2 floats)
    gl::VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        9 * mem::size_of::<f32>() as GLsizei,
        (3 * mem::size_of::<f32>()) as *const _,
    );
    gl::EnableVertexAttribArray(1);

    // Texture ID (1 float)
    gl::VertexAttribPointer(
        2,
        1,
        gl::FLOAT,
        gl::FALSE,
        9 * mem::size_of::<f32>() as GLsizei,
        (5 * mem::size_of::<f32>()) as *const _,
    );
    gl::EnableVertexAttribArray(2);

    // Color (3 floats - RGB)
    gl::VertexAttribPointer(
        3,
        3,
        gl::FLOAT,
        gl::FALSE,
        9 * mem::size_of::<f32>() as GLsizei,
        (6 * mem::size_of::<f32>()) as *const _,
    );
    gl::EnableVertexAttribArray(3);

    // Create shaders
    let vertex_shader_source = CString::new(
        r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;
        layout (location = 2) in float aTextureId;
        layout (location = 3) in vec3 aColor;
        
        out vec2 TexCoord;
        out float TextureId;
        out vec3 Color;
        
        void main() {
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            TexCoord = aTexCoord;
            TextureId = aTextureId;
            Color = aColor;
        }
    "#,
    )
    .unwrap();

    let fragment_shader_source = CString::new(
        r#"
        #version 330 core
        in vec2 TexCoord;
        in float TextureId;
        in vec3 Color;
        out vec4 FragColor;
        
        uniform sampler2D textures[12];
        
        void main() {
            int texId = int(TextureId);
            if (texId >= 0 && texId < 12) {
                // Apply color tint to texture
                vec4 texColor = texture(textures[texId], TexCoord);
                FragColor = vec4(texColor.rgb * Color, texColor.a);
            } else if (texId == -1) {
                // Special case for generic units: render as colored circle
                vec2 center = vec2(0.5, 0.5);
                float dist = distance(TexCoord, center);
                if (dist < 0.4) {
                    FragColor = vec4(Color * 0.7 + vec3(0.9, 0.2, 0.2) * 0.3, 1.0); // Tint red with color
                } else {
                    discard; // Transparent outside circle
                }
            } else {
                FragColor = vec4(Color, 1.0); // Use color directly for non-textured
            }
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
