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

impl DrawInstruction {
    #[inline]
    pub fn identifier(&self) -> i32 {
        match *self {
            DrawInstruction::Pixel { .. } => 1,
            DrawInstruction::Rectangle { .. } => 2,
            DrawInstruction::Square { .. } => 3,
        }
    }

    #[inline]
    pub fn x(&self) -> i16 {
        match *self {
            DrawInstruction::Pixel { x, .. } => x,
            DrawInstruction::Rectangle { x, .. } => x,
            DrawInstruction::Square { x, .. } => x,
        }
    }

    #[inline]
    pub fn y(&self) -> i16 {
        match *self {
            DrawInstruction::Pixel { y, .. } => y,
            DrawInstruction::Rectangle { y, .. } => y,
            DrawInstruction::Square { y, .. } => y,
        }
    }

    #[inline]
    pub fn width(&self) -> Option<i16> {
        match *self {
            DrawInstruction::Pixel { .. } => None,
            DrawInstruction::Rectangle { w, .. } => Some(w),
            DrawInstruction::Square { l, .. } => Some(l),
        }
    }

    #[inline]
    pub fn height(&self) -> Option<i16> {
        match *self {
            DrawInstruction::Pixel { .. } => None,
            DrawInstruction::Rectangle { h, .. } => Some(h),
            DrawInstruction::Square { l, .. } => Some(l),
        }
    }

    #[inline]
    pub fn color(&self) -> Color {
        match *self {
            DrawInstruction::Pixel { color, .. } => color,
            DrawInstruction::Rectangle { color, .. } => color,
            DrawInstruction::Square { color, .. } => color,
        }
    }

    #[inline]
    pub fn as_int_set(&self) -> [i32; 12] {
        let color = self.color();
        [
            self.identifier(),
            0,
            0,
            0,
            self.x() as i32,
            self.y() as i32,
            self.width().unwrap_or(0) as i32,
            self.height().unwrap_or(0) as i32,
            color.r as i32,
            color.g as i32,
            color.b as i32,
            if color.is_transparent { 0 } else { 1 },
        ]
    }
}
