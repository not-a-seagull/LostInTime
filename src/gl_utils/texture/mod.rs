// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture/mod.rs - OpenGL texture

use crate::LitError;
use gl::types::{GLenum, GLuint};
use std::{fmt, marker::PhantomData};

mod dimensions;
mod material;
mod render;

pub use dimensions::*;
pub use material::*;

pub trait TextureType: fmt::Debug {
    type ValueType;

    fn bind_texture_location() -> GLenum;
    fn tex_image(dimensions: &[i16], data: *const Self::ValueType) -> Result<(), LitError>;
}

#[derive(Debug, Clone)]
pub struct Texture<T: TextureType> {
    id: GLuint,
    dimensions: Vec<i16>,
    _phantom: PhantomData<T>,
}

impl<T: TextureType> Texture<T> {
    pub fn from_raw(dimensions: &[i16], data: *const T::ValueType) -> Result<Self, LitError> {
        let mut id: GLuint = 0;

        // generate and bind the texture
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(T::bind_texture_location(), id);
        }

        // fill the texture with the data
        T::tex_image(dimensions, data)?;

        unsafe { gl::BindTexture(T::bind_texture_location(), 0) };

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

    pub fn bind(&self) {
        unsafe { gl::BindTexture(T::bind_texture_location(), self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(T::bind_texture_location(), 0) }
    }
}

impl<T: TextureType> Drop for Texture<T> {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}

// some specific types
pub type ImgTexture = Texture<ImgTextureType>;
pub type DIBuffer = Texture<DIBufferType>;
