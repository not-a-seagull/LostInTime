// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// command.rs - Process a command in the LitScript

use crate::{CompilerState, LitsCcError};
use proc_macro2::{Ident, TokenTree};
use std::io::prelude::*;

#[inline]
fn write_word<T: Write>(stream: &mut T, word: u16) -> Result<(), LitsCcError> {
    let bytes = word.to_be_bytes();
    stream.write_all(&bytes)?;
    Ok(())
}

pub fn read_ident<TStream: Write, TIter: Iterator<Item = TokenTree>>(
    iter: &mut TIter,
    stream: &mut TStream,
    state: &mut CompilerState,
) -> Result<(), LitsCcError> {
    // read in an ident
    match iter.next() {
        Some(TokenTree::Ident(i)) => {
            let var_name = format!("{}", i);
            let id = state.register_variable(&var_name);
            stream.write_all(&id.to_be_bytes())?;
            Ok(())
        }
        _ => Err(LitsCcError::ExpectedIdent),
    }
}

pub fn process_command<TStream: Write, TIter: Iterator<Item = TokenTree>>(
    ident: &Ident,
    iter: &mut TIter,
    stream: &mut TStream,
    state: &mut CompilerState,
) -> Result<(), LitsCcError> {
    let name = format!("{}", ident);

    match name.as_ref() {
        "gamedef" => write_word(stream, 1),
        "def" => {
            write_word(stream, 2)?;
            read_ident(iter, stream, state)
        }
        "log" => write_word(stream, 3),
        "create_tex" => {
            write_word(stream, 4)?;
            read_ident(iter, stream, state)
        }
        "color_id" => write_word(stream, 5),
        "draw_pixel" => write_word(stream, 6),
        "draw_rect" => write_word(stream, 7),
        _ => Err(LitsCcError::UnknownCommand(name)),
    }
}
