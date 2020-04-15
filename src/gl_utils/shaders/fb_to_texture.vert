#version 330 core

// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/shaders/fb_to_texture.vert - Basic vertex shader that gets the texture coordinates

layout (location = 0) in vec4 vertex;

out vec2 tex_coords;

void main() {
    tex_coords = vertex.zw;
    gl_Position = vec4(vertex.xy, 0.0, 1.0);
}
