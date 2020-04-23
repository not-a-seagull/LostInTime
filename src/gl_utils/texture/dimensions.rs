// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture/dimensions.rs - Define traits and functions for texture dimensions

use super::{
    super::{check_gl_error, GlCall},
    TextureType,
};
use crate::LitError;
use gl::types::{GLbyte, GLenum, GLint};
use std::ffi::c_void;

#[derive(Debug, Clone)]
pub struct DIBufferType;

impl TextureType for DIBufferType {
    type ValueType = GLint;

    #[inline]
    fn bind_texture_location() -> GLenum {
        gl::TEXTURE_1D
    }

    #[inline]
    fn tex_type() -> GLenum {
        gl::INT
    }

    fn tex_image(dimensions: &[i16], data: *const GLint) -> Result<(), LitError> {
        if dimensions.len() != 1 {
            return Err(LitError::ImproperDimensions(1, dimensions.len()));
        }

        unsafe {
            gl::TexImage1D(
                gl::TEXTURE_1D,
                0,
                gl::RGBA as GLint,
                dimensions[0] as GLint,
                0,
                gl::RGBA,
                gl::INT,
                data as *const c_void,
            )
        };

        check_gl_error(GlCall::TexImage1D)
    }
}

#[derive(Debug, Clone)]
pub struct ImgTextureType;

impl TextureType for ImgTextureType {
    type ValueType = GLfloat;

    #[inline]
    fn bind_texture_location() -> GLenum {
        gl::TEXTURE_2D
    }

    #[inline]
    fn tex_type() -> GLenum {
        gl::FLOAT
    }

    fn tex_image(dimensions: &[i16], data: *const GLfloat) -> Result<(), LitError> {
        if dimensions.len() != 2 {
            return Err(LitError::ImproperDimensions(2, dimensions.len()));
        }

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                dimensions[0] as GLint,
                dimensions[1] as GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data as *const c_void,
            )
        };

        check_gl_error(GlCall::TexImage2D)
    }
}
