// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture/material.rs - Material used to create textures.

use super::DIBuffer;
use crate::{
    draw::{DrawHandle, DrawInstruction},
    Color, LitError, Material, ResourceDictionary,
};
use gl::types::{GLint, GLuint};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct ImgMaterial {
    width: i16,
    height: i16,
    bg_color: Color,
    draws: Vec<DrawInstruction>,
    buffer: Option<DIBuffer>,
}

impl ImgMaterial {
    #[inline]
    pub fn new(width: i16, height: i16, background_color: Color) -> Self {
        Self::from_draws(width, height, background_color, vec![])
    }

    #[inline]
    pub fn from_draws(
        width: i16,
        height: i16,
        bg_color: Color,
        draws: Vec<DrawInstruction>,
    ) -> Self {
        Self {
            width,
            height,
            draws,
            bg_color,
            buffer: None,
        }
    }

    #[inline]
    pub fn width(&self) -> i16 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i16 {
        self.height
    }

    #[inline]
    pub fn draws(&self) -> &[DrawInstruction] {
        &self.draws
    }

    #[inline]
    pub fn buffer(&self) -> Option<&DIBuffer> {
        (&self.buffer).as_ref()
    }

    #[inline]
    pub fn background_color(&self) -> Color {
        self.bg_color
    }
}

impl fmt::Display for ImgMaterial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{} Image", self.width, self.height)
    }
}

impl DrawHandle for ImgMaterial {
    fn draw_pixel(&mut self, x: i16, y: i16, color: Color) -> Result<(), LitError> {
        self.draws.push(DrawInstruction::Pixel { x, y, color });
        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        h: i16,
        color: Color,
    ) -> Result<(), LitError> {
        self.draws
            .push(DrawInstruction::Rectangle { x, y, w, h, color });
        Ok(())
    }
}

impl Material for ImgMaterial {
    #[inline]
    fn get_subdict(dict: &ResourceDictionary) -> &HashMap<u32, Self> {
        dict.mat_img_subdict()
    }

    #[inline]
    fn get_subdict_mut(dict: &mut ResourceDictionary) -> &mut HashMap<u32, Self> {
        dict.mat_img_subdict_mut()
    }

    fn prepare(&mut self) -> Result<(), LitError> {
        // this is just the buffer logic from earlier
        if self.buffer.is_some() {
            return Ok(());
        }

        let mut draws: Vec<GLint> = vec![];
        self.draws()
            .iter()
            .rev()
            .for_each(|d| draws.extend(&d.as_int_set()));

        // build a 1-dimensional texture for this instance
        self.buffer = Some(DIBuffer::from_raw(
            &[self.draws().len() as i16],
            draws.as_ptr(),
        )?);
        Ok(())
    }
}
