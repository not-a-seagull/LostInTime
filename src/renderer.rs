// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// renderer.rs - Renderer trait.

use crate::{Game, ImgTexture, LitError};
use nalgebra::geometry::Point2;

pub trait Renderer {
    fn main_loop(&mut self, game: Game) -> Result<(), LitError>;
    fn draw_sprite(
        &mut self,
        img: &ImgTexture,
        position: Point2<f32>,
        size: Point2<f32>,
        rotation: f32,
    ) -> Result<(), LitError>;
}
