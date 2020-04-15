#version 330 core

// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/shaders/fb_to_texture.frag - Calculate pixel color by calculating position in root image

struct DrawInstruction {
                     //  Alignment
    uint kind;        //  16
    float coords[4]; //  16 * 4 + 16 = 80
    float color[4];  //  16 * 4 + 80 = 144
};

in vec2 tex_coords;
out vec4 tex_coords;

uniform short s_width;
uniform short s_height; 
uniform DrawInstruction s_instructions[255]; // todo: raise limit if needed

void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0);
}
