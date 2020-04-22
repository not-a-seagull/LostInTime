// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/quad.rs - A quadrilaterial shape to draw onto.

use super::{check_gl_error, vertices::QUAD_VERTICES, GlCall};
use crate::LitError;
use gl::types::{GLfloat, GLint, GLuint};
use std::{ffi::c_void, mem, ptr};

#[derive(Debug, Default)]
pub struct Quad {
    vao: GLuint,
    _vbo: GLuint,
    has_been_bound: bool,
}

impl Quad {
    pub fn new() -> Result<Self, LitError> {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            check_gl_error(GlCall::GenVertexArrays)?;
            gl::GenBuffers(1, &mut vbo);
            check_gl_error(GlCall::GenBuffers)?;

            // fill the buffer with the quad vertices
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            check_gl_error(GlCall::BindBuffer)?;
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&QUAD_VERTICES) as isize,
                QUAD_VERTICES.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            check_gl_error(GlCall::BufferData)?;
        }

        Ok(Self {
            vao,
            _vbo: vbo,
            has_been_bound: false,
        })
    }

    pub fn bind(&mut self) -> Result<(), LitError> {
        unsafe { gl::BindVertexArray(self.vao) };
        check_gl_error(GlCall::BindVertexArray)?;

        if !self.has_been_bound {
            unsafe {
                gl::EnableVertexAttribArray(0);
                check_gl_error(GlCall::EnableVertexAttribArray)?;
                gl::VertexAttribPointer(
                    0,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    (4 * mem::size_of::<GLfloat>()) as GLint,
                    ptr::null(),
                );
                check_gl_error(GlCall::VertexAttribPointer)?;
            }

            self.has_been_bound = true;
        }

        Ok(())
    }

    pub fn draw(&self) -> Result<(), LitError> {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
        check_gl_error(GlCall::DrawArrays)
    }

    pub fn unbind(&self) -> Result<(), LitError> {
        unsafe { gl::BindVertexArray(0) };
        check_gl_error(GlCall::BindVertexArray)
    }
}
