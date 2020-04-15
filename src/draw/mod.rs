// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// draw/mod.rs - Draw handle for an object that can be drawn on.

mod instruction;

pub use instruction::DrawInstruction;

use crate::{Color, LitError};
use std::{fmt, ops::Deref};

pub trait DrawHandle: Sized + fmt::Display + fmt::Debug {
    fn new(width: i16, height: i16, background_color: Color) -> Self;

    // draw a single pixel
    fn draw_pixel(&mut self, x: i16, y: i16, color: Color) -> Result<(), LitError>;

    // draw a rectangle
    fn draw_rectangle(
        &mut self,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
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
    fn draw_square(&mut self, x: i16, y: i16, length: i16, color: Color) -> Result<(), LitError> {
        self.draw_rectangle(x, y, length, length, color)
    }
}
