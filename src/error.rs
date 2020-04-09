// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// error.rs - Error handling struct.

use sdl2::video::WindowBuildError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LitError {
    #[error("{0}")]
    Msg(String),
    #[error("{0}")]
    StaticMsg(&'static str),
    #[error("An error occurred while building the SDL2 window: {0}")]
    WindowBuildError(#[from] WindowBuildError),
}
