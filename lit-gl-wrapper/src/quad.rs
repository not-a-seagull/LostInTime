// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/quad.rs - A quadrilaterial shape to draw onto.

use super::{check_gl_error, vertices::QUAD_VERTICES, GlCall};
use crate::GlError;
use gl::types::{GLfloat, GLint, GLuint};
use std::{ffi::c_void, mem, ptr};

#[derive(Debug)]
pub struct Quad {
    vao: GLuint,
    _vbo: GLuint,
    has_been_bound: bool,
    gl: gl::Gl,
}

impl Quad {
    pub fn new(gl: &gl::Gl) -> Result<Self, GlError> {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            check_gl_error(gl, GlCall::GenVertexArrays)?;
            gl.GenBuffers(1, &mut vbo);
            check_gl_error(gl, GlCall::GenBuffers)?;

            // fill the buffer with the quad vertices
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            check_gl_error(gl, GlCall::BindBuffer)?;
            gl.BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&QUAD_VERTICES) as isize,
                QUAD_VERTICES.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            check_gl_error(gl, GlCall::BufferData)?;
        }

        Ok(Self {
            vao,
            _vbo: vbo,
            has_been_bound: false,
            gl: gl.clone(),
        })
    }

    pub fn bind(&self, store_vao: bool) -> Result<(), GlError> {
        unsafe { self.gl.BindVertexArray(self.vao) };
        check_gl_error(&self.gl, GlCall::BindVertexArray)?;

        if store_vao {
            unsafe {
                self.gl.EnableVertexAttribArray(0);
                check_gl_error(&self.gl, GlCall::EnableVertexAttribArray)?;
                self.gl.VertexAttribPointer(
                    0,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    (4 * mem::size_of::<GLfloat>()) as GLint,
                    ptr::null(),
                );
                check_gl_error(&self.gl, GlCall::VertexAttribPointer)?;
            }
        }

        Ok(())
    }

    pub fn draw(&self) -> Result<(), GlError> {
        unsafe {
            self.gl.DrawArrays(gl::TRIANGLES, 0, 6);
        }
        check_gl_error(&self.gl, GlCall::DrawArrays)
    }

    pub fn unbind(&self) -> Result<(), GlError> {
        unsafe { self.gl.BindVertexArray(0) };
        check_gl_error(&self.gl, GlCall::BindVertexArray)
    }
}
