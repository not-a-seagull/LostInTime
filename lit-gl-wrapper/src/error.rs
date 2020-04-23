// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture/error.rs - Check for GL errors.

use crate::LitError;
use gl::types::{GLchar, GLenum, GLsizei, GLuint};
use std::{
    ffi::{c_void, CStr},
    fmt,
    ptr,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GlCall {
    Unknown,
    GenTextures,
    BindTexture,
    GetTexImage,
    GenFramebuffers,
    BindFramebuffer,
    LinkProgram,
    GetProgramiv,
    GetProgramInfoLog,
    DetachShader,
    UseProgram,
    GenVertexArrays,
    GenBuffers,
    BindBuffer,
    BufferData,
    BindVertexArray,
    EnableVertexAttribArray,
    VertexAttribPointer,
    DrawArrays,
    ShaderSource,
    CompileShader,
    GetShaderiv,
    GetShaderInfoLog,
    TexImage1D,
    TexImage2D,
    FramebufferTexture2D,
    Viewport,
    Uniform1i,
    Uniform4f,
    Uniform4fv,
    UniformMatrix4fv,
    GetUniformLocation,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GlErrorType {
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    StackUnderflow,
    StackOverflow,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct GlError {
    call: GlCall,
    kind: GlErrorType,
}

impl GlError {
    #[inline]
    pub fn new(call: GlCall, kind: GlErrorType) -> GlError {
        GlError { call, kind }
    }

    #[inline]
    pub fn call(&self) -> GlCall { self.call }

    #[inline]
    pub fn kind(&self) -> GlErrorType { self.kind }
}

impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An OpenGl error occured in function {:?}: {:?}", self.call(), self.kind())
    } 
}

impl Error for GlError {}

pub fn check_gl_error(gl: &gl::Gl, call: GlCall) -> Result<(), GlError> {
    let err = unsafe { gl.GetError() };
    match err {
        gl::NO_ERROR => Ok(()),
        _ => Err(GlError::new(
            call,
            match err {
                gl::INVALID_ENUM => GlErrorType::InvalidEnum,
                gl::INVALID_VALUE => GlErrorType::InvalidValue,
                gl::INVALID_OPERATION => GlErrorType::InvalidOperation,
                gl::INVALID_FRAMEBUFFER_OPERATION => GlErrorType::InvalidFramebufferOperation,
                gl::OUT_OF_MEMORY => GlErrorType::OutOfMemory,
                gl::STACK_UNDERFLOW => GlErrorType::StackUnderflow,
                gl::STACK_OVERFLOW => GlErrorType::StackOverflow,
                _ => GlErrorType::Unknown,
            },
        )),
    }
}


