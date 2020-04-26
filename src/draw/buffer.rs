// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// draw/buffer.rs - Draw buffer implementation

use super::{DrawHandle, DrawInstruction};
use crate::{Color, ImgTexture, LitError};
use gl::types::GLfloat;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct DrawBuffer {
    instructions: Vec<DrawInstruction>,
    background_color: Color,
    width: u32,
    height: u32,
}

impl DrawBuffer {
    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn background_color(&self) -> Color {
        self.background_color
    }

    #[inline]
    fn instructions(&self) -> &[DrawInstruction] {
        &self.instructions
    }
}

impl DrawHandle for DrawBuffer {
    fn new(width: u32, height: u32, background_color: Color) -> Self {
        Self {
            instructions: vec![],
            width,
            height,
            background_color,
        }
    }

    fn draw_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<(), LitError> {
        self.instructions
            .push(DrawInstruction::Pixel { x, y, color });
        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
    ) -> Result<(), LitError> {
        self.instructions.push(DrawInstruction::Rectangle {
            x,
            y,
            width,
            height,
            color,
        });
        Ok(())
    }
}

impl<T: DrawHandle> TryFrom<DrawBuffer> for T {
    type Error = LitError;

    fn try_from(db: DrawBuffer) -> Result<T, LitError> {
        let mut res = T::new(db.width(), db.height(), db.background_color());
        res.instructions().iter().try_for_each(|i| match *i {
            DrawInstruction::Pixel { x, y, color } => res.draw_pixel(x, y, color),
            DrawInstruction::Rectangle { x, y, w, h, color } => {
                res.draw_rectangle(x, y, w, h, color)
            }
            DrawInstruction::Square { x, y, l, color } => res.draw_square(x, y, l, color),
        })?;

        Ok(res)
    }
}

impl DrawBuffer {
    fn create_texture(&self) -> Result<ImgTexture, LitError> {
        // create a map of the pixels we are using
        let mut pixel_map = [[self.background_color(); self.width()]; self.height()];

        // for each x and y coordinate, determine the pixel's color
        (0..self.width()).into_iter().for_each(|i| {
            (0..self.height()).into_iter().for_each(|j| {
                // iterate backward until we find something corresponding to our pixel
                if let Err(()) = self.instructions().iter().rev().try_for_each(|i| match *i {
                    DrawInstruction::Pixel { x, y, color } => {
                        if i == x && j == y {
                            pixel_map[j][i] = color;
                            Err(())
                        } else {
                            Ok(())
                        }
                    }
                    DrawInstruction::Rectangle { x, y, w, h, color } => {
                        if x <= i && i >= x + w - 1 && y <= j && j >= y + h - 1 {
                            pixel_map[j][i] = color;
                            Err(())
                        } else {
                            Ok(())
                        }
                    }
                    DrawInstruction::Square { x, y, l, color } => {
                        if x <= i && i >= x + l - 1 && y <= j && j >= y + l - 1 {
                            pixel_map[j][i] = color;
                            Err(())
                        } else {
                            Ok(())
                        }
                    }
                }) {}
            })
        });

        // convert pixel_map to a list of GL floats
        let mut floats: Vec<GLfloat> = vec![];
        pixel_map.into_iter().flatten().for_each(|f| floats.extend(f.as_gl_color()));
         
        // create a texture from these floats
        let tex = ImgTexture::from_raw(&self.gl, &[self.width(), self.height()], floats.as_ptr())?;
        Ok(tex)
    }
}
