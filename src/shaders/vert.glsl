#version 450

layout (location = 0) in vec2 position;
layout (location = 0) out int obj1;
layout (location = 1) out vec2 tex_coords;
layout (location = 2) out vec4 vertex_color;

layout (push_constant) uniform PushConstant {
    lowp vec2 resolution;
    vec2 camera;
} pc;

vec4 orange = vec4(1.00, 0.50, 0.00, 1.0);
vec4 lavender = vec4(0.53, 0.4, 0.8, 1.0);
vec4 skyblue = vec4(0.10, 0.40, 1.00, 1.0);
vec4 maya = vec4(0.5, 0.75, 1.0, 1.0);


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
    // float hypo = sqrt(pow(position.x, 2) + pow(position.y, 2));

    // uint id = oi.index[gl_VertexIndex];

    // vec2 object = vec2(objects.x[id], objects.y[id]);
    // vec2 object_size = vec2(objects.sx[id], objects.sx[id]);
    
    // float object_rotation = objects.rotation[oi.index[gl_VertexIndex]];

    // vec2 rotatedpos = vec2(
    //     cos(
    //         atan(position.y, position.x) + object_rotation
    //     ) * hypo,
    //     sin(
    //         atan(position.y, position.x) + object_rotation
    //     ) * hypo
    // );

    // y bound (position + pc.camera / pc.resolution) * pc.resolution.y

    // y / (x + y)

    vertex_color = colors[gl_VertexIndex];

    //vec2 resolutionscaler = vec2(pc.resolution.y / (pc.resolution.x + pc.resolution.y), pc.resolution.x / (pc.resolution.x + pc.resolution.y));
    vec2 resolutionscaler = vec2(sin(atan(pc.resolution.y, pc.resolution.x)), cos(atan(pc.resolution.y, pc.resolution.x)))  / (sqrt(2) / 2);

    
    
    gl_Position = vec4((position - pc.camera / pc.resolution) * resolutionscaler, 0.0, 1.0);
    obj1 = 0;
    if (gl_VertexIndex >= colors.length()){
        tex_coords = (position - pc.camera / pc.resolution) * resolutionscaler;
        obj1 = 1;
    }
}
