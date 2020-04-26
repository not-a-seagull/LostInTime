// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// main.rs - Program entry point

#![allow(clippy::new_without_default)]

pub use lit_gl_wrapper::*;

pub mod draw;

mod color;
mod error;
mod game;
mod resource;
mod script;

pub use color::Color;
pub use draw::*;
pub use error::LitError;
pub use game::Game;
pub use renderer::*;
pub use resource::*;
pub use script::*;

use std::{env, fs, io::BufReader, process};

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
    let mut renderer = GlRenderer::init()?;
    let mut data_file = BufReader::new(fs::File::open(
        env::args().nth(1).ok_or_else(|| LitError::NoDataFile)?,
    )?);
    let game_data = script::GameData::read(&mut data_file)?;
    println!("{:?}", &game_data);
    let mut game = Game::new(game_data);
    println!("{:?}", game.get_resource::<ImgTexture>(0)?);

    renderer.main_loop(game)
}
