#version 450
layout (location = 0) out vec4 f_color;
layout (location = 0) flat in int obj1;
layout (location = 2) in vec4 vertex_color;
layout (set = 0, binding = 0) uniform sampler2D tex;



void main() {
    vec2 texcoord = gl_FragCoord.xy / vec2(217, 237) - 0.5;
    vec4 texture = texture(tex, texcoord);

    if (obj1 == 1){
        f_color = vec4(0.0, 0.0, 0.0, 1.0);
    }
    else {
        f_color = vertex_color;
    }
    
}