// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// draw/mod.rs - Draw handle for an object that can be drawn on.

mod buffer;
mod instruction;

pub use buffer::*;
pub use instruction::DrawInstruction;

use crate::{Color, ImgTexture, LitError};
use std::fmt;

pub trait DrawHandle: fmt::Display + fmt::Debug {
    fn new(width: u32, height: u32, background_color: Color) -> Self;

    // draw a single pixel
    fn draw_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<(), LitError>;

    // draw a rectangle
    fn draw_rectangle(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
    ) -> Result<(), LitError> {
        for i in x..(width - x + 1) {
            for j in y..(height - y + 1) {
                self.draw_pixel(i, j, color)?;
            }
        }

        Ok(())
    }

    // draw a square
    fn draw_square(&mut self, x: u32, y: u32, length: u32, color: Color) -> Result<(), LitError> {
        self.draw_rectangle(x, y, length, length, color)
    }
}
