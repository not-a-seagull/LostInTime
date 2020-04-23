#version 330 core

// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/shaders/fb_to_texture.frag - Calculate pixel color by calculating position in root image

flat in ivec2 pix_coords;
out vec4 color;

uniform int s_draw_len;
uniform isampler1D s_draws;
uniform vec4 bg_color;

void main() {
    color = bg_color; 

    // iterate over draw instructions and see which ones involve this pixel
    int base_index;
    int k_type;
    ivec4 d_color;
    ivec4 coords;
    for (int i = 0; i < s_draw_len; i++) {
        base_index = i * 3;
        k_type = texelFetch(s_draws, base_index, 0).x;
        coords = texelFetch(s_draws, base_index + 1, 0);
        d_color = texelFetch(s_draws, base_index + 2, 0);

        // if our coords are in the designated zone, we are good
        if ((k_type == 1 && pix_coords.x == coords.x && pix_coords.y == coords.y) || // pixel
            ((k_type == 2 || k_type == 3) && pix_coords.x >= coords.x && pix_coords.x <= coords.x + coords.z
                       && pix_coords.y >= coords.y && pix_coords.y <= coords.y + coords.w)) {
            color = vec4(d_color.x / 255, d_color.y / 255, d_color.z / 255, d_color.w / 255);
            break;
        }
            
    }
}
