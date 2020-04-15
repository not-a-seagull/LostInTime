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
    draws: Vec<DrawInstruction>,
    height: i16,
    width: i16,
    background_color: Color,
}

impl FrameBuffer {
    #[inline]
    pub fn fbo(&self) -> GLuint {
        self.fbo
    }

    #[inline]
    pub fn width(&self) -> i16 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i16 {
        self.height
    }

    #[inline]
    pub fn background_color(&self) -> Color {
        self.background_color
    }

    #[inline]
    pub fn draws(&self) -> &[DrawInstruction] {
        &self.draws
    }

    pub fn bind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo()) };
    }

    pub fn unbind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) };
    }
}

impl fmt::Display for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FrameBuffer with ID {}", self.fbo)
    }
}

impl DrawHandle for FrameBuffer {
    fn new(width: i16, height: i16, background_color: Color) -> Self {
        // generate open gl frame buffers
        let mut fbo: GLuint = 0;
        unsafe { gl::GenFramebuffers(1, &mut fbo) };

        Self {
            fbo,
            draws: vec![],
            width,
            height,
            background_color,
        }
    }

    fn draw_pixel(&mut self, x: i16, y: i16, color: Color) -> Result<(), LitError> {
        self.draws.push(DrawInstruction::Pixel { x, y, color });
        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
        color: Color,
    ) -> Result<(), LitError> {
        self.draws.push(DrawInstruction::Rectangle {
            x,
            y,
            w: width,
            h: height,
            color,
        });
        Ok(())
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &self.fbo) };
    }
}
