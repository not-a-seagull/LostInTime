// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture/error.rs - Check for GL errors.

use crate::LitError;
use gl::types::{GLchar, GLenum, GLsizei, GLuint};
use std::{
    ffi::{c_void, CStr},
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

pub fn check_gl_error(call: GlCall) -> Result<(), LitError> {
    let err = unsafe { gl::GetError() };
    match err {
        gl::NO_ERROR => Ok(()),
        _ => Err(LitError::GlError(
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

#[macro_export]
macro_rules! gl_op {
    ($eval: expr) => {{
        let res = unsafe { $eval };
        crate::check_gl_error()?;
        Ok(res)
    }};
}

// a callback to receive opengl errors
#[cfg(debug_assertions)]
extern "system" fn opengl_print_error(
    _src: GLenum,
    _ty: GLenum,
    _id: GLuint,
    _severity: GLenum,
    _len: GLsizei,
    msg: *const GLchar,
    _user_param: *mut c_void,
) {
    let msg = unsafe { CStr::from_ptr(msg).to_string_lossy() };
    println!("GL Error: {}", msg);
}

#[cfg(debug_assertions)]
pub fn set_gl_error_callback() -> Result<(), LitError> {
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(opengl_print_error), ptr::null());
    };

    let mut unused_ids: GLuint = 0;
    unsafe {
        gl::DebugMessageControl(
            gl::DONT_CARE,
            gl::DONT_CARE,
            gl::DONT_CARE,
            0,
            &mut unused_ids,
            gl::TRUE,
        )
    };

    Ok(())
}
