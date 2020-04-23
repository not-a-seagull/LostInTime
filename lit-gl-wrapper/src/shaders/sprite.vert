#version 330 core

// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/shaders/sprite.vert - Render a sprite to the screen.

layout (location = 0) in vec4 vertex;

out vec2 tex_coords;

uniform mat4 transf;
uniform mat4 ortho;

void main() {
    tex_coords = vertex.zw;
    gl_Position = transf * ortho * vec4(vertex.xy, 0.0, 1.0);
}
