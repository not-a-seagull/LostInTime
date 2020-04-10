// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// main.rs - Program entry point

mod error;
mod game;
mod gl_renderer;
mod renderer;
mod script;

pub use error::LitError;
pub use game::Game;
pub use renderer::Renderer;

use gl_renderer::GlRenderer;
use script::Bytecode;

use std::{
    env, fs,
    io::{prelude::*, BufReader},
    process,
};

fn main() {
    process::exit(match classic_main() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("A fatal error occurred: {}", e);
            1
        }
    });
}

fn classic_main() -> Result<(), LitError> {
    let game = Game {};
    let renderer = GlRenderer::init()?;
    let mut data_file = BufReader::new(fs::File::open(
        env::args()
            .skip(1)
            .next()
            .ok_or_else(|| LitError::NoDataFile)?,
    )?);
    let game_data = script::GameData::read(&mut data_file)?;

    println!("{:?}", game_data);
    renderer.main_loop(&game)
}
