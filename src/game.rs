// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// game.rs - Object for holding the game's current state.

use crate::{GameData, LitError, Resource};

pub struct Game {
    data: GameData,
}

impl Game {
    pub fn new(data: GameData) -> Self {
        Self { data }
    }

    pub fn get_resource<T: Resource>(&mut self, id: u32) -> Result<&T, LitError> {
        self.data.get_resource(id)
    }
}
