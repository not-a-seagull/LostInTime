// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// script/eval.rs - Evaluate a bytecode statement

use super::{Bytecode, BytecodeObject, GameData, ParserState};
use crate::LitError;
use std::io::prelude::*;

pub fn eval<T: Read>(
    stream: &mut T,
    data: &mut GameData,
    state: &mut ParserState,
) -> Result<(), LitError> {
    // read a single word from the stream
    let mut buffer = [0; 2];
    stream.read(&mut buffer)?;

    match u16::from_be_bytes(buffer) {
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
            // home grown format statement, could be improved
            let format = BytecodeObject::read(stream)?;
            let tuple = BytecodeObject::read(stream)?;
            let tuple = tuple.as_tuple(state)?;
            let formatted_str = format
                .as_string(state)?
                .split("{}")
                .enumerate()
                .map(|(i, part)| {
                    if i != tuple.len() {
                        format!("{}{}", part, tuple[i].stringify(state).unwrap())
                    } else {
                        String::from(part)
                    }
                })
                .collect::<Vec<String>>()
                .join("");
            println!("{}", formatted_str);

            Ok(())
        }
        _ => Err(LitError::BytecodeRead8(buffer[0])),
    }
}
