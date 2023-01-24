#version 450
layout (location = 0) out vec4 f_color;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in vec4 vertex_color;
layout (set = 0, binding = 0) uniform sampler2D tex;

// vec3 RGBtoHSL(vec3 color) {
//     vec3 hsl;
//     float r = color.r;
//     float g = color.g;
//     float b = color.b;
//     float cmin = min(min(r, g), b);
//     float cmax = max(max(r, g), b);
//     float delta = cmax - cmin;

//     hsl.z = (cmax + cmin) / 2.0;
//     if (delta == 0.0) {
//         hsl.x = 0.0;
//         hsl.y = 0.0;
//     } else {
//         if (hsl.z < 0.5) {
//             hsl.y = delta / (cmax + cmin);
//         } else {
//             hsl.y = delta / (2.0 - cmax - cmin);
//         }

//         float deltaR = (((cmax - r) / 6.0) + (delta / 2.0)) / delta;
//         float deltaG = (((cmax - g) / 6.0) + (delta / 2.0)) / delta;
//         float deltaB = (((cmax - b) / 6.0) + (delta / 2.0)) / delta;

//         if (r == cmax) {
//             hsl.x = deltaB - deltaG;
//         } else if (g == cmax) {
//             hsl.x = (1.0 / 3.0) + deltaR - deltaB;
//         } else if (b == cmax) {
//             hsl.x = (2.0 / 3.0) + deltaG - deltaR;
//         }

//         if (hsl.x < 0.0) {
//             hsl.x += 1.0;
//         } else if (hsl.x > 1.0) {
//             hsl.x -= 1.0;
//         }
//     }
//     return hsl;
// }

// vec3 HSLtoRGB(vec3 hsl) {
//     vec3 rgb;
//     if (hsl.y == 0.0) {
//         rgb = vec3(hsl.z);
//     } else {
//         float c = (1.0 - abs(2.0 * hsl.z - 1.0)) * hsl.y;
//         float x = c * (1.0 - abs(fract(hsl.x + 1.0) * 6.0 - 3.0));
//         float m = hsl.z - c * 0.5;
//         if (hsl.x < 1.0/6.0) {
//             rgb = vec3(c + m, x + m, m);
//         } else if (hsl.x < 2.0/6.0) {
//             rgb = vec3(x + m, c + m, m);
//         } else if (hsl.x < 3.0/6.0) {
//             rgb = vec3(m, c + m, x + m);
//         } else if (hsl.x < 4.0/6.0) {
//             rgb = vec3(m, x + m, c + m);
//         } else if (hsl.x < 5.0/6.0) {
//             rgb = vec3(x + m, m, c + m);
//         } else {
//             rgb = vec3(c + m, m, x + m);
//         }
//     }
//     return rgb;
// }

void main() {
    vec4 color = texture(tex, tex_coords / 2 + 0.5);
    f_color = color;// * 2.2;
}

