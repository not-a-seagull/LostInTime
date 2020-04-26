// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// renderer.rs - Renderer trait.

use crate::ImgTexture;
use nalgebra::geometry::Point2;
use std::{boxed::Box, error::Error};

pub trait Renderer {
    type Error: Error;

    fn main_loop<F>(&self, f: F) -> Result<(), Self::Error>
    where
        F: FnMut(&Self) -> Result<(), Box<dyn Error>>;
    fn draw_sprite(
        &self,
        img: &ImgTexture,
        position: Point2<f32>,
        size: Point2<f32>,
        rotation: f32,
    ) -> Result<(), Self::Error>;
}
