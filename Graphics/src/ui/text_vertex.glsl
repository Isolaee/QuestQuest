#version 330 core
layout (location = 0) in vec4 vertex; // <vec2 pos, vec2 tex>

out vec2 TexCoords;
uniform vec2 screenSize;

void main() {
    // Convert from screen coordinates to NDC
    vec2 ndc = (vertex.xy / screenSize) * 2.0 - 1.0;
    ndc.y = -ndc.y; // Flip Y
    gl_Position = vec4(ndc, 0.0, 1.0);
    TexCoords = vertex.zw;
}
