// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/program.rs - Define a single OpenGL shader

use super::{check_gl_error, GlCall, GlError, Shader, Uniform};
use gl::types::{GLchar, GLint, GLuint};
use std::ptr;

pub struct Program {
    gl: gl::Gl,
    id: GLuint,
}

impl Program {
    pub fn new(gl: &gl::Gl, shaders: &[Shader]) -> Result<Self, GlError> {
        // get the id
        let id = unsafe { gl.CreateProgram() };

        // attach every shader in the collection
        shaders
            .iter()
            .for_each(|s| unsafe { gl.AttachShader(id, s.id()) });

        // link together the program
        unsafe { gl.LinkProgram(id) };
        check_gl_error(gl, GlCall::LinkProgram)?;

        let mut success: GLint = 1;
        // test for errors
        unsafe { gl.GetProgramiv(id, gl::LINK_STATUS, &mut success) };
        check_gl_error(gl, GlCall::GetProgramiv)?;

        if success == 0 {
            let mut err_len: GLint = 0;
            unsafe { gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut err_len) };
            check_gl_error(gl, GlCall::GetProgramiv)?;

            let buffer = crate::utils::create_cstring_buffer(err_len as usize);
            unsafe {
                gl.GetProgramInfoLog(id, err_len, ptr::null_mut(), buffer.as_ptr() as *mut GLchar);
                check_gl_error(gl, GlCall::GetProgramInfoLog)?;
            };

            return Err(GlError::CompileError(buffer.to_string_lossy().into_owned()));
        }

        // detach every shader
        shaders
            .iter()
            .for_each(|s| unsafe { gl.DetachShader(id, s.id()) });
        check_gl_error(gl, GlCall::DetachShader)?;

        Ok(Self { id, gl: gl.clone() })
    }

    #[inline]
    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn activate(&self) -> Result<(), GlError> {
        unsafe { self.gl.UseProgram(self.id) };
        check_gl_error(&self.gl, GlCall::UseProgram)
    }

    #[inline]
    pub fn set_uniform<T: Uniform>(&self, uname: &'static str, uniform: T) -> Result<(), GlError> {
        uniform.set_uniform(&self.gl, uname, self.id())
    }
}
