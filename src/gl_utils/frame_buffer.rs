// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/frame_buffer.rs - An OpenGL FrameBuffer

use crate::{
    draw::{DrawHandle, DrawInstruction},
    Color, LitError,
};
use gl::types::GLuint;
use std::fmt;

#[derive(Debug)]
pub struct FrameBuffer {
    fbo: GLuint,
}

impl FrameBuffer {
    pub fn new() -> Self {
        // generate open gl frame buffers
        let mut fbo: GLuint = 0;
        unsafe { gl::GenFramebuffers(1, &mut fbo) };

        Self { fbo }
    }

    #[inline]
    pub fn fbo(&self) -> GLuint {
        self.fbo
    }

    pub fn bind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo()) };
    }

    pub fn unbind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) };
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &self.fbo) };
    }
}
