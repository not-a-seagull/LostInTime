// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// script/eval.rs - Evaluate a bytecode statement

use super::{Bytecode, BytecodeObject, GameData, ParserState};
use crate::{Color, ImgMaterial, LitError};
use std::{convert::TryInto, io::prelude::*};

pub fn eval<T: Read>(
    stream: &mut T,
    data: &mut GameData,
    state: &mut ParserState,
) -> Result<bool, LitError> {
    // read a single word from the stream
    let mut buffer = [0; 2];
    stream.read(&mut buffer)?;
    let res = u16::from_be_bytes(buffer);

    match res {
        1 => {
            // gamedef statement, define the game's name
            data.set_name(String::from(
                BytecodeObject::read(stream)?.as_string(state)?,
            ));
            Ok(true)
        }
        2 => {
            // def statement, define a runtime variable
            let mut buffer = [0; 4];
            stream.read(&mut buffer)?;
            let id = u32::from_be_bytes(buffer);
            state.register_variable(id, BytecodeObject::read(stream)?);
            Ok(true)
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

            Ok(true)
        }
        4 => {
            // create a new texture material
            let mut buffer = [0; 4];
            stream.read(&mut buffer)?;
            let id = u32::from_be_bytes(buffer);

            let width = BytecodeObject::read(stream)?;
            let width = width.as_number(state)?.try_into()?;

            let height = BytecodeObject::read(stream)?;
            let height = height.as_number(state)?.try_into()?;

            let bg_color = BytecodeObject::read(stream)?.as_color(state)?;

            let mat = ImgMaterial::new(width, height, bg_color);
            state.register_variable(id, BytecodeObject::ImgMaterial(mat));
            state.img_material_ids.push(id);

            Ok(true)
        }
        5 => {
            // assign a color id to an invocation
            let buf_id = BytecodeObject::read(stream)?;
            let buf_id = buf_id.get_var_id(state)?;

            let clr_id = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;

            let color = BytecodeObject::read(stream)?.as_color(state)?;

            state.register_color_id(buf_id, clr_id, color);
            Ok(true)
        }
        6 => {
            // draw a single pixel
            let mut draw_buffer = BytecodeObject::read(stream)?;
            let draw_id = draw_buffer.get_var_id(state)?;

            let x = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;
            let y = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;
            let color = match state.get_color(
                draw_id,
                BytecodeObject::read(stream)?.as_number(state)?.try_into()?,
            ) {
                Ok(c) => *c,
                Err(_e) => BytecodeObject::read(stream)?.as_color(state)?,
            };

            let draw_handle = draw_buffer.as_draw_handle_mut(state)?;

            draw_handle.draw_pixel(x, y, color)?;
            Ok(true)
        }
        7 => {
            // draw a rectangle
            let mut draw_buffer = BytecodeObject::read(stream)?;
            let draw_id = draw_buffer.get_var_id(state)?;

            let x = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;
            let y = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;
            let width = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;
            let height = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;

            let color = match state.get_color(
                draw_id,
                BytecodeObject::read(stream)?.as_number(state)?.try_into()?,
            ) {
                Ok(c) => *c,
                Err(_e) => BytecodeObject::read(stream)?.as_color(state)?,
            };

            let draw_handle = draw_buffer.as_draw_handle_mut(state)?;

            draw_handle.draw_rectangle(x, y, width, height, color)?;
            Ok(true)
        }
        0 => Ok(false),
        _ => Err(LitError::BytecodeRead16(res)),
    }
}
