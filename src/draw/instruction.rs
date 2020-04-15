// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// draw/instruction.rs - Instructions for drawing an image.

use crate::Color;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DrawInstruction {
    // draw a single pixel
    Pixel {
        x: i16,
        y: i16,
        color: Color,
    },
    // draw a rectangle
    Rectangle {
        x: i16,
        y: i16,
        w: i16,
        h: i16,
        color: Color,
    },
    // draw a square of texture
    Square {
        x: i16,
        y: i16,
        l: i16,
        color: Color,
    },
}

#[inline]
pub fn translate_draw_instruction(di: &DrawInstruction) -> u32 {
    match di {
        &DrawInstruction::Pixel { .. } => 1,
        &DrawInstruction::Rectangle { .. } => 2,
        &DrawInstruction::Square { .. } => 3,
    }
}
