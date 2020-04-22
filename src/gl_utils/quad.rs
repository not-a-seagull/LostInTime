// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/quad.rs - A quadrilaterial shape to draw onto.

use super::vertices::QUAD_VERTICES;
use gl::types::{GLfloat, GLint, GLuint};
use std::{ffi::c_void, mem, ptr};

#[derive(Debug, Default)]
pub struct Quad {
    vao: GLuint,
    _vbo: GLuint,
    has_been_bound: bool,
}

impl Quad {
    pub fn new() -> Self {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // fill the buffer with the quad vertices
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&QUAD_VERTICES) as isize,
                QUAD_VERTICES.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }

        Self {
            vao,
            _vbo: vbo,
            has_been_bound: false,
        }
    }

    pub fn bind(&mut self) {
        unsafe { gl::BindVertexArray(self.vao) };

        if !self.has_been_bound {
            unsafe {
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(
                    0,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    (4 * mem::size_of::<GLfloat>()) as GLint,
                    ptr::null(),
                );
            }

            self.has_been_bound = true;
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}
