// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/program.rs - Define a single OpenGL shader

use super::Shader;
use crate::LitError;
use gl::types::{GLchar, GLint, GLuint};
use std::{io::prelude::*, ptr};

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn new<T: Read>(shaders: &[Shader]) -> Result<Self, LitError> {
        // get the id
        let id = unsafe { gl::CreateProgram() };

        // attach every shader in the collection
        shaders
            .iter()
            .for_each(|s| unsafe { gl::AttachShader(id, s.id()) });

        // link together the program
        unsafe { gl::LinkProgram(id) };

        let mut success: GLint = 1;
        // test for errors
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut err_len: GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut err_len) };

            let buffer = crate::utils::create_cstring_buffer(err_len as usize);
            unsafe {
                gl::GetProgramInfoLog(id, err_len, ptr::null_mut(), buffer.as_ptr() as *mut GLchar)
            };

            return Err(LitError::Msg(buffer.to_string_lossy().into_owned()));
        }

        // detach every shader
        shaders
            .iter()
            .for_each(|s| unsafe { gl::DetachShader(id, s.id()) });

        Ok(Self { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}
