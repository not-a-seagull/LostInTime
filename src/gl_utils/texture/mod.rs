// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture/mod.rs - OpenGL texture

use crate::{check_gl_error, GlCall, LitError};
use gl::types::{GLenum, GLuint};
use std::{ffi::c_void, fmt, marker::PhantomData};

mod dimensions;
mod material;
mod render;

pub use dimensions::*;
pub use material::*;

pub trait TextureType: fmt::Debug {
    type ValueType: fmt::Debug + fmt::Display + Default;

    fn bind_texture_location() -> GLenum;
    fn tex_type() -> GLenum;
    fn tex_image(dimensions: &[i16], data: *const Self::ValueType) -> Result<(), LitError>;
}

#[derive(Clone)]
pub struct Texture<T: TextureType> {
    id: GLuint,
    dimensions: Vec<i16>,
    _phantom: PhantomData<T>,
}

impl<T: TextureType> Texture<T> {
    pub fn from_raw(dimensions: &[i16], data: *const T::ValueType) -> Result<Self, LitError> {
        let mut id: GLuint = 0;

        // generate and bind the texture
        unsafe { gl::GenTextures(1, &mut id) };
        check_gl_error(GlCall::GenTextures)?;
        unsafe { gl::BindTexture(T::bind_texture_location(), id) };
        check_gl_error(GlCall::BindTexture)?;

        // fill the texture with the data
        T::tex_image(dimensions, data)?;

        unsafe { gl::BindTexture(T::bind_texture_location(), 0) };
        check_gl_error(GlCall::BindTexture)?;

        Ok(Self {
            id,
            dimensions: dimensions.iter().copied().collect(),
            _phantom: PhantomData,
        })
    }

    #[inline]
    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&self) -> Result<(), LitError> {
        unsafe { gl::BindTexture(T::bind_texture_location(), self.id) };
        check_gl_error(GlCall::BindTexture)
    }

    pub fn unbind(&self) -> Result<(), LitError> {
        unsafe { gl::BindTexture(T::bind_texture_location(), 0) };
        check_gl_error(GlCall::BindTexture)
    }
}

impl<T: TextureType> Drop for Texture<T> {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}

impl<T: TextureType> fmt::Debug for Texture<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pixel_count = (self.dimensions.iter().product::<i16>() * 4) as usize;
        let mut pixel_buffer: Vec<T::ValueType> = Vec::with_capacity(pixel_count);
        self.bind()?;

        unsafe {
            gl::GetTexImage(
                T::bind_texture_location(),
                0,
                gl::RGBA,
                T::tex_type(),
                pixel_buffer.as_mut_ptr() as *mut c_void,
            );

            check_gl_error(GlCall::GetTexImage)?;

            pixel_buffer.set_len(pixel_count);
        };

        write!(f, "[")?;
        for (i, pixel) in pixel_buffer.iter().enumerate() {
            write!(f, "{}", pixel)?;
            if i != pixel_count - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

// some specific types
pub type ImgTexture = Texture<ImgTextureType>;
pub type DIBuffer = Texture<DIBufferType>;
