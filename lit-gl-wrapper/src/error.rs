// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture/error.rs - Check for GL errors.

use sdl2::video::WindowBuildError;
use std::{
    boxed::Box,
    io::Error as IoError,
    fmt,
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
    Unknown,
}

#[derive(Debug, Clone)]
pub struct GlCallError {
    call: GlCall,
    kind: GlErrorType,
}

impl GlCallError {
    #[inline]
    pub fn new(call: GlCall, kind: GlErrorType) -> Self {
        Self { call, kind }
    }

    #[inline]
    pub fn call(&self) -> GlCall {
        self.call
    }

    #[inline]
    pub fn kind(&self) -> GlErrorType {
        self.kind
    }
}

impl fmt::Display for GlCallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An OpenGl error occured in function {:?}: {:?}",
            self.call(),
            self.kind()
        )
    }
}

impl std::error::Error for GlCallError {}

#[derive(Debug, thiserror::Error)]
pub enum GlError {
    #[error("{0}")]
    Msg(String),
    #[error("An error occurred during compilation: {0}")]
    CompileError(String),
    #[error("The uniform {0} was not found")]
    UniformNotFound(&'static str),
    #[error("{0}")]
    GlCall(#[from] GlCallError),
    #[error("{0}")]
    WindowBuild(#[from] WindowBuildError),
    #[error("{0}")]
    GenericError(Box<dyn std::error::Error>),
    #[error("Expected an array with {0} dimensions, found {1} dimensions")]
    ImproperDimensions(usize, usize),
    #[error("{0}")]
    Io(#[from] IoError),
}

impl From<GlError> for fmt::Error {
    fn from(_gl: GlError) -> fmt::Error {
        fmt::Error
    }
}

pub fn check_gl_error(gl: &gl::Gl, call: GlCall) -> Result<(), GlError> {
    let err = unsafe { gl.GetError() };
    match err {
        gl::NO_ERROR => Ok(()),
        _ => Err(GlCallError::new(
            call,
            match err {
                gl::INVALID_ENUM => GlErrorType::InvalidEnum,
                gl::INVALID_VALUE => GlErrorType::InvalidValue,
                gl::INVALID_OPERATION => GlErrorType::InvalidOperation,
                gl::INVALID_FRAMEBUFFER_OPERATION => GlErrorType::InvalidFramebufferOperation,
                gl::OUT_OF_MEMORY => GlErrorType::OutOfMemory,
                _ => GlErrorType::Unknown,
            },
        )
        .into()),
    }
}
