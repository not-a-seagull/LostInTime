// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// renderer.rs - Renderer trait.

use crate::{Game, LitError};

pub trait Renderer {
    fn main_loop(&self, game: &Game) -> Result<(), LitError>;
}
