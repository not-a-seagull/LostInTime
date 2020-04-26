// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// lit-gl-wrapper/src/frame_buffer.rs - An OpenGL FrameBuffer

use super::{GlCall, GlError, check_gl_error};
use gl::types::GLuint;

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    fbo: GLuint,
    gl: gl::Gl,
}

impl FrameBuffer {
    pub fn new(gl: &gl::Gl) -> Result<Self, GlError> {
        // generate open gl frame buffers
        let mut fbo: GLuint = 0;
        unsafe { gl.GenFramebuffers(1, &mut fbo) };
        check_gl_error(gl, GlCall::GenFramebuffers)?;

        Ok(Self {
            gl: gl.clone(),
            fbo,
        })
    }

    #[inline]
    pub fn fbo(&self) -> GLuint {
        self.fbo
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindFramebuffer(gl::FRAMEBUFFER, self.fbo()) };
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindFramebuffer(gl::FRAMEBUFFER, 0) };
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteFramebuffers(1, &self.fbo) };
    }
}
