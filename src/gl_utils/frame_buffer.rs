// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/frame_buffer.rs - An OpenGL FrameBuffer

use super::{check_gl_error, GlCall};
use crate::LitError;
use gl::types::GLuint;

#[derive(Debug, Default, Clone)]
pub struct FrameBuffer {
    fbo: GLuint,
}

impl FrameBuffer {
    pub fn new() -> Result<Self, LitError> {
        // generate open gl frame buffers
        let mut fbo: GLuint = 0;
        unsafe { gl::GenFramebuffers(1, &mut fbo) };
        check_gl_error(GlCall::GenFramebuffers)?;

        Ok(Self { fbo })
    }

    #[inline]
    pub fn fbo(&self) -> GLuint {
        self.fbo
    }

    pub fn bind(&self) -> Result<(), LitError> {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo()) };
        check_gl_error(GlCall::BindFramebuffer)
    }

    pub fn unbind(&self) -> Result<(), LitError> {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) };
        check_gl_error(GlCall::BindFramebuffer)
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &self.fbo) };
    }
}
