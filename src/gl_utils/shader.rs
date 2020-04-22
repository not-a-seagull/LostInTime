// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/shader.rs - Define a single OpenGL shader

use super::{check_gl_error, GlCall};
use crate::LitError;
use gl::types::{GLchar, GLint, GLuint};
use std::{
    ffi::CString,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    ptr,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new<T: Read>(stream: &mut T, kind: ShaderType) -> Result<Self, LitError> {
        // read the entire file into the string
        let mut source = String::new();
        stream.read_to_string(&mut source)?;
        let source = CString::new(source).unwrap();

        let id = unsafe { gl::CreateShader(kind as u32) };

        // process the source
        unsafe { gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null()) };
        check_gl_error(GlCall::ShaderSource)?;
        unsafe { gl::CompileShader(id) };
        check_gl_error(GlCall::CompileShader)?;

        // check for errors
        let mut success: GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) };
        check_gl_error(GlCall::GetShaderiv)?;

        // if there is an error, report it
        if success == 0 {
            let mut err_len: GLint = 0;
            unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut err_len) };
            check_gl_error(GlCall::GetShaderiv)?;

            let buffer = crate::utils::create_cstring_buffer(err_len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, err_len, ptr::null_mut(), buffer.as_ptr() as *mut GLchar);
                check_gl_error(GlCall::GetShaderInfoLog)?;
            };

            Err(LitError::Msg(buffer.to_string_lossy().into_owned()))
        } else {
            Ok(Self { id })
        }
    }

    pub fn load<S: AsRef<Path>>(path: &S, kind: ShaderType) -> Result<Self, LitError> {
        let mut reader = BufReader::new(File::open(path)?);
        Self::new(&mut reader, kind)
    }

    pub fn from_source<S: AsRef<[u8]>>(source: &S, kind: ShaderType) -> Result<Self, LitError> {
        let mut reader = BufReader::new(source.as_ref());
        Self::new(&mut reader, kind)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}
