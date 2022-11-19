#version 450

layout (location = 0) in vec2 position;
layout (location = 0) out int obj1;
layout (location = 1) out vec4 vertex_color;
layout (set = 1, binding = 0) uniform Data {
    vec2 position;
    vec2 size;
    float rotation;
} player;

vec4 orange = vec4(1.00, 0.50, 0.00, 1.0);
vec4 lavender = vec4(0.53, 0.4, 0.8, 1.0);
vec4 skyblue = vec4(0.10, 0.40, 1.00, 1.0);
vec4 maya = vec4(0.5, 0.75, 1.0, 0.8);


vec4 colors[] = vec4[](
    lavender, // top left
    orange, // bottom left
    lavender, // top right
    // vertex 2
    orange, // bottom left
    orange, // bottom right
    lavender, // top right
    // upper row
    skyblue, // top left
    lavender, // bottom left
    skyblue, // top right
    // vertex 2
    lavender, // bottom left
    lavender, // bottom right
    skyblue // top right
);
void main() {
    
    float hypo = sqrt(pow(position.x, 2) + pow(position.y, 2));
    vec2 rotatedpos = vec2(
        cos(
            atan(position.y, position.x) + player.rotation
        ) * hypo,
        sin(
            atan(position.y, position.x) + player.rotation
        ) * hypo
    );

    vertex_color = colors[gl_VertexIndex];
    obj1 = 0;
    if (colors.length() <= gl_VertexIndex){
        obj1 = 1;
        gl_Position = vec4(rotatedpos * player.size + player.position, 0.0, 1.0);
    }
    else {
        gl_Position = vec4(position, 0.0, 1.0);
    }
}
