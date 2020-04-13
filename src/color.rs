// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// color.rs - Defines a struct which can be used to contain colors.

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
