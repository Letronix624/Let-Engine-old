#version 450
layout (location = 0) out vec4 f_color;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in vec4 vertex_color;
layout (set = 0, binding = 0) uniform sampler2D tex;

void main() {
    vec4 color = texture(tex, tex_coords / 2 + 0.5);
    f_color = color;// * 2.2;
}

