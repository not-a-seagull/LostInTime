// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// uniform.rs - Trait that defines how a uniform is called.

use crate::{check_gl_error, utils::cify_str, GlCall, GlError};
use gl::types::GLuint;
use nalgebra::{geometry::Transform3, Matrix4};

macro_rules! assign_uniform {
    ($gl: expr, $id: expr, $name: expr => $call: ident <= $($val: expr),*) => {
        {
            let loc = unsafe { $gl.GetUniformLocation($id, cify_str($name)) };
            check_gl_error($gl, GlCall::GetUniformLocation)?;
            if loc != -1 {
                unsafe { $gl.$call(loc, $($val),*) };
                check_gl_error($gl, GlCall::$call)
            } else {
                Err(GlError::UniformNotFound($name))
            }
        }
    }
}

pub trait Uniform {
    fn set_uniform(self, gl: &gl::Gl, uname: &'static str, prog_id: GLuint) -> Result<(), GlError>;
}

impl Uniform for i32 {
    #[inline]
    fn set_uniform(self, gl: &gl::Gl, uname: &'static str, prog_id: GLuint) -> Result<(), GlError> {
        assign_uniform!(gl, prog_id, uname => Uniform1i <= self)
    }
}

impl Uniform for [f32; 4] {
    #[inline]
    fn set_uniform(self, gl: &gl::Gl, uname: &'static str, prog_id: GLuint) -> Result<(), GlError> {
        let [f1, f2, f3, f4] = self;
        assign_uniform!(gl, prog_id, uname => Uniform4f <= f1, f2, f3, f4)
    }
}

impl Uniform for Matrix4<f32> {
    #[inline]
    fn set_uniform(self, gl: &gl::Gl, uname: &'static str, prog_id: GLuint) -> Result<(), GlError> {
        assign_uniform!(gl, prog_id, uname => UniformMatrix4fv <= 1, gl::FALSE, self.as_ptr())
    }
}

impl Uniform for Transform3<f32> {
    #[inline]
    fn set_uniform(self, gl: &gl::Gl, uname: &'static str, prog_id: GLuint) -> Result<(), GlError> {
        self.into_inner().set_uniform(gl, uname, prog_id)
    }
}
