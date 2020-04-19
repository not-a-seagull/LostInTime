// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// color.rs - Defines a struct which can be used to contain colors.

use crate::LitError;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub is_transparent: bool,
}

impl Color {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            is_transparent: false,
        }
    }

    #[inline]
    pub fn from_arr(arr: &[i32]) -> Result<Self, LitError> {
        match arr.len() {
            3 => Ok(Self::new(
                arr[0].try_into()?,
                arr[1].try_into()?,
                arr[3].try_into()?,
            )),
            4 => match arr[3] {
                0 => Ok(Self::transparent()),
                _ => Ok(Self::new(
                    arr[0].try_into()?,
                    arr[1].try_into()?,
                    arr[3].try_into()?,
                )),
            },
            _ => Err(LitError::StaticMsg("Colors can only have 3 or 4 members")),
        }
    }

    #[inline]
    pub fn transparent() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            is_transparent: true,
        }
    }

    #[inline]
    pub fn as_gl_color(&self) -> [f32; 4] {
        [
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
            if self.is_transparent { 0.0 } else { 1.0 },
        ]
    }
}
