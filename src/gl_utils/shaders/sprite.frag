#version 330 core

// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/shaders/sprite.frag - Render a sprite to the screen.

in vec2 tex_coords;
out vec4 color;

uniform sampler2D image;
