use gl::types::*;
use std::collections::HashMap;

// Simple 5x7 bitmap font data for ASCII characters
const FONT_WIDTH: usize = 5;
const FONT_HEIGHT: usize = 7;

// Type alias to reduce complexity
type FontAtlasResult = Result<(GLuint, HashMap<char, (f32, f32, f32, f32)>), String>;

pub struct TextRenderer {
    shader_program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    font_texture: GLuint,
    char_uvs: HashMap<char, (f32, f32, f32, f32)>, // (u0, v0, u1, v1)
}

impl TextRenderer {
    pub fn new() -> Result<Self, String> {
        let (shader_program, vao, vbo) = unsafe { Self::setup_text_rendering()? };
        let (font_texture, char_uvs) = unsafe { Self::create_bitmap_font()? };

        Ok(Self {
            shader_program,
            vao,
            vbo,
            font_texture,
            char_uvs,
        })
    }

    unsafe fn setup_text_rendering() -> Result<(GLuint, GLuint, GLuint), String> {
        let vertex_src = r#"
            #version 330 core
            layout (location = 0) in vec4 vertex; // <vec2 pos, vec2 tex>
            out vec2 TexCoords;
            uniform vec2 screenSize;
            
            void main() {
                vec2 ndc = (vertex.xy / screenSize) * 2.0 - 1.0;
                ndc.y = -ndc.y;
                gl_Position = vec4(ndc, 0.0, 1.0);
                TexCoords = vertex.zw;
            }
        "#;

        let fragment_src = r#"
            #version 330 core
            in vec2 TexCoords;
            out vec4 FragColor;
            uniform sampler2D fontTexture;
            uniform vec4 textColor;
            
            void main() {
                float alpha = texture(fontTexture, TexCoords).r;
                FragColor = vec4(textColor.rgb, textColor.a * alpha);
            }
        "#;

        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str = std::ffi::CString::new(vertex_src).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str = std::ffi::CString::new(fragment_src).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let mut vao = 0;
        let mut vbo = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<f32>() as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(0);

        Ok((shader_program, vao, vbo))
    }

    unsafe fn create_bitmap_font() -> FontAtlasResult {
        let mut char_uvs = HashMap::new();

        // Create a simple 16x6 character grid texture (96 characters)
        let chars_per_row = 16;
        let char_rows = 6;
        let atlas_width = chars_per_row * FONT_WIDTH;
        let atlas_height = char_rows * FONT_HEIGHT;

        let mut atlas_data = vec![0u8; atlas_width * atlas_height];

        // Generate bitmap for printable ASCII characters (32-127)
        for i in 0..96 {
            let ch = (i + 32) as u8 as char;
            let row = i / chars_per_row;
            let col = i % chars_per_row;

            let bitmap = get_char_bitmap(ch);

            // Copy bitmap to atlas
            for (y, row_data) in bitmap.iter().enumerate().take(FONT_HEIGHT) {
                for (x, &pixel) in row_data.iter().enumerate().take(FONT_WIDTH) {
                    let atlas_x = col * FONT_WIDTH + x;
                    let atlas_y = row * FONT_HEIGHT + y;
                    let atlas_idx = atlas_y * atlas_width + atlas_x;
                    atlas_data[atlas_idx] = if pixel { 255 } else { 0 };
                }
            }

            // Calculate UV coordinates
            let u0 = (col * FONT_WIDTH) as f32 / atlas_width as f32;
            let v0 = (row * FONT_HEIGHT) as f32 / atlas_height as f32;
            let u1 = ((col + 1) * FONT_WIDTH) as f32 / atlas_width as f32;
            let v1 = ((row + 1) * FONT_HEIGHT) as f32 / atlas_height as f32;

            char_uvs.insert(ch, (u0, v0, u1, v1));
        }

        // Create OpenGL texture
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as i32,
            atlas_width as i32,
            atlas_height as i32,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            atlas_data.as_ptr() as *const _,
        );

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        Ok((texture, char_uvs))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        color: [f32; 4],
        screen_width: f32,
        screen_height: f32,
    ) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.font_texture);

            let screen_loc = gl::GetUniformLocation(self.shader_program, c"screenSize".as_ptr());
            gl::Uniform2f(screen_loc, screen_width, screen_height);

            let color_loc = gl::GetUniformLocation(self.shader_program, c"textColor".as_ptr());
            gl::Uniform4f(color_loc, color[0], color[1], color[2], color[3]);

            // Explicitly set texture sampler to use texture unit 0
            let texture_loc = gl::GetUniformLocation(self.shader_program, c"fontTexture".as_ptr());
            gl::Uniform1i(texture_loc, 0);

            let char_width = size * (FONT_WIDTH as f32 / FONT_HEIGHT as f32);
            let char_height = size;
            let mut offset_x = x;

            for ch in text.chars() {
                if let Some((u0, v0, u1, v1)) = self.char_uvs.get(&ch) {
                    let vertices: [f32; 24] = [
                        offset_x,
                        y,
                        *u0,
                        *v0,
                        offset_x,
                        y + char_height,
                        *u0,
                        *v1,
                        offset_x + char_width,
                        y + char_height,
                        *u1,
                        *v1,
                        offset_x,
                        y,
                        *u0,
                        *v0,
                        offset_x + char_width,
                        y + char_height,
                        *u1,
                        *v1,
                        offset_x + char_width,
                        y,
                        *u1,
                        *v0,
                    ];

                    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                        vertices.as_ptr() as *const _,
                        gl::DYNAMIC_DRAW,
                    );

                    gl::DrawArrays(gl::TRIANGLES, 0, 6);

                    offset_x += char_width + 1.0; // 1px spacing between characters
                }
            }

            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::Disable(gl::BLEND);
        }
    }
}

impl Drop for TextRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.font_texture);
            gl::DeleteProgram(self.shader_program);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

// Simple 5x7 bitmap font patterns
fn get_char_bitmap(ch: char) -> [[bool; FONT_WIDTH]; FONT_HEIGHT] {
    match ch {
        '0' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, true, true],
            [true, false, true, false, true],
            [true, true, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        '1' => [
            [false, false, true, false, false],
            [false, true, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, true, true, true, false],
        ],
        '2' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [false, false, false, false, true],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [true, true, true, true, true],
        ],
        '3' => [
            [true, true, true, true, false],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [false, true, true, true, false],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [true, true, true, true, false],
        ],
        '4' => [
            [false, false, false, true, false],
            [false, false, true, true, false],
            [false, true, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, true],
            [false, false, false, true, false],
            [false, false, false, true, false],
        ],
        '5' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, true, true, true, false],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        '6' => [
            [false, false, true, true, false],
            [false, true, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        '7' => [
            [true, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
        ],
        '8' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        '9' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, true, false],
            [false, true, true, false, false],
        ],
        'A' | 'a' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'B' | 'b' => [
            [true, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, false],
        ],
        'C' | 'c' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        'D' | 'd' => [
            [true, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, false],
        ],
        'E' | 'e' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, true],
        ],
        'F' | 'f' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ],
        'G' | 'g' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, false],
            [true, false, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, true],
        ],
        'H' | 'h' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'I' | 'i' => [
            [false, true, true, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, true, true, true, false],
        ],
        'J' | 'j' => [
            [false, false, true, true, true],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
        ],
        'K' | 'k' => [
            [true, false, false, false, true],
            [true, false, false, true, false],
            [true, false, true, false, false],
            [true, true, false, false, false],
            [true, false, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, true],
        ],
        'L' | 'l' => [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, true],
        ],
        'M' | 'm' => [
            [true, false, false, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'N' | 'n' => [
            [true, false, false, false, true],
            [true, true, false, false, true],
            [true, false, true, false, true],
            [true, false, false, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'O' | 'o' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        'P' | 'p' => [
            [true, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ],
        'Q' | 'q' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, true, false, true],
            [true, false, false, true, false],
            [false, true, true, false, true],
        ],
        'R' | 'r' => [
            [true, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, false],
            [true, false, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, true],
        ],
        'S' | 's' => [
            [false, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, true, true, true, false],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [true, true, true, true, false],
        ],
        'T' | 't' => [
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ],
        'U' | 'u' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        'V' | 'v' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
        ],
        'W' | 'w' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, true, false, true],
            [true, true, false, true, true],
            [true, false, false, false, true],
        ],
        'X' | 'x' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, true, false, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'Y' | 'y' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ],
        'Z' | 'z' => [
            [true, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, true],
        ],
        ' ' => [[false; FONT_WIDTH]; FONT_HEIGHT],
        ':' => [
            [false, false, false, false, false],
            [false, false, true, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, true, false, false],
            [false, false, false, false, false],
        ],
        '/' => [
            [false, false, false, false, true],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false],
        ],
        '-' => [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, true, true, true, true],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
        _ => [[false; FONT_WIDTH]; FONT_HEIGHT], // Unknown character
    }
}
