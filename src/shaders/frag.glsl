#version 450
layout (location = 0) out vec4 f_color;
layout (location = 0) flat in int obj1;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in vec4 vertex_color;
layout (set = 0, binding = 0) uniform sampler2D tex;


void main() {
    vec4 texture = texture(tex, tex_coords);


    if (obj1 == 1){
        f_color = vec4(0.5, 0.75, 1.0, 0.3);//texture;
    }
    else {
        f_color = vertex_color;
    }
    
}