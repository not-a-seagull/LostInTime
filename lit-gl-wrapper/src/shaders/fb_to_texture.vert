#version 330 core

// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/shaders/fb_to_texture.vert - Basic vertex shader that gets the texture coordinates

layout (location = 0) in vec4 vertex;

flat out ivec2 pix_coords;

uniform int s_width;
uniform int s_height;

int determine_pix(float scale, int length) {
    float real_scale = (scale + 1.0) / 2;
    return int(real_scale * length);
}

void main() {
    pix_coords = ivec2(determine_pix(vertex.z, s_width), determine_pix(vertex.w, s_height));
    gl_Position = vec4(vertex.xy, 0.0, 1.0);
}
