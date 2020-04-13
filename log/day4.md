# Day 4

Considering that our graphics will most likely be very simple, I have a clever idea for compressing our graphics. First, let's create a trait that represents an object that can be drawn on, since I forsee using a lot of objects that support drawing.

*Inside of src/color.rs*

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub is_transparent: bool,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            is_transparent: false,
        }
    }

    pub fn transparent() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            is_transparent: true,
        }
    }
}
```

*Inside of src/draw/mod.rs*

```rust
use crate::{Color, LitError};
use std::fmt;

pub trait DrawHandle: Sized + fmt::Display + fmt::Debug {
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
```

This should be a pretty good interface for when we need to draw on something. However, we're using OpenGL in this program. We should take advantage of the "texture" system OpenGL offers to draw upon.

Unfortunately I got busy so I had no time to do this.
