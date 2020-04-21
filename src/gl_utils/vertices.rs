// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/vertices.rs - Various sets of vertices.

use gl::types::GLfloat;

pub const QUAD_VERTICES: [GLfloat; 24] = [
    0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 0.0, 1.0, 0.0,
];
