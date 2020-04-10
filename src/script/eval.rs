// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// script/eval.rs - Evaluate a bytecode statement

use super::{Bytecode, BytecodeObject, GameData, ParserState};
use crate::LitError;
use std::io::prelude::*;

pub fn eval<T: Read>(
    stream: &mut T,
    state: &mut ParserState,
    data: &mut GameData,
) -> Result<(), LitError> {
    // read a single byte from the stream
    let mut buffer = [0; 1];
    stream.read(&mut buffer)?;

    match buffer[0] {
        1 => {
            // gamedef statement, define the game's name
            data.set_name(String::from(
                BytecodeObject::read(stream)?.as_string(state)?,
            ));
            Ok(())
        }
        2 => {
            // def statement, define a runtime variable
            let mut buffer = [0; 4];
            stream.read(&mut buffer)?;
            let id = u32::from_be_bytes(buffer);
            state.register_variable(id, BytecodeObject::read(stream)?);
            Ok(())
        }
        3 => {
            // log statement, output something to the debug log
            println!("{}", BytecodeObject::read(stream)?.as_string(state)?);
            // TODO: process tuple into println
            let _tuple = BytecodeObject::read(stream)?.as_tuple(state)?;
            Ok(())
        }
        _ => Err(LitError::BytecodeRead8(buffer[0])),
    }
}
