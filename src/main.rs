// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// main.rs - Program entry point

mod error;
mod game;
mod gl_renderer;
mod renderer;

pub use error::LitError;
pub use game::Game;
pub use renderer::Renderer;

use gl_renderer::GlRenderer;

use std::process;

fn main() {
    process::exit(match classic_main() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("A fatal error occurred: {:?}", e);
            1
        }
    });
}

fn classic_main() -> Result<(), LitError> {
    let game = Game {};
    let renderer = GlRenderer::init()?;

    renderer.main_loop(&game)
}
