// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// literals.rs - Process literals

use crate::{CompilerState, LitsCcError};
use proc_macro2::{Delimiter, TokenTree};
use std::io::{self, prelude::*, Cursor, SeekFrom};

pub fn process_literals<TStream: Write, TIter: Iterator<Item = TokenTree>>(
    iter: &mut TIter,
    stream: &mut TStream,
    state: &mut CompilerState,
) -> Result<usize, LitsCcError> {
    let mut elements_processed = 0;

    while let Some(token) = iter.next() {
        match token {
            TokenTree::Literal(l) => {
                let l = format!("{}", l);

                if l.starts_with('"') && l.ends_with('"') {
                    // this is a string!
                    let inner = l.split('\"').nth(1).unwrap();
                    stream.write_all(&[4, inner.len() as u8])?;
                    stream.write_all(inner.as_bytes())?;
                } else if let Ok(i) = l.parse::<i32>() {
                    if i >= std::u8::MIN as i32 && i <= std::u8::MAX as i32 {
                        stream.write_all(&[1])?;
                        stream.write_all(&[i as u8])?;
                    } else if i >= std::i16::MIN as i32 && i <= std::i16::MAX as i32 {
                        stream.write_all(&[2])?;
                        stream.write_all(&(i as i16).to_be_bytes())?;
                    } else {
                        stream.write_all(&[3])?;
                        stream.write_all(&i.to_be_bytes())?;
                    }
                } else {
                    return Err(LitsCcError::Msg(format!("Unexpected literal: {}", l)));
                }
            }
            TokenTree::Punct(p) => {
                if p.as_char() == '@' {
                    // next should be an identifier
                    match iter.next() {
                        Some(TokenTree::Ident(i)) => {
                            let name = format!("{}", i);
                            stream.write_all(&[6])?;
                            stream.write_all(&state.get_variable_id(&name)?.to_be_bytes())?;
                        }
                        _ => {
                            return Err(LitsCcError::ExpectedIdent);
                        }
                    }
                } else {
                    return Err(LitsCcError::Msg(format!("Unexpected punctuation: {}", p)));
                }
            }
            TokenTree::Group(g) => {
                if g.delimiter() != Delimiter::Parenthesis {
                    return Err(LitsCcError::StaticMsg(
                        "The only group supported at the moment are parenthesis.",
                    ));
                }

                let mut cursor = Cursor::new(Vec::new());

                let mut group_iter = g.stream().into_iter();
                let length = process_literals(&mut group_iter, &mut cursor, state)?;

                cursor.seek(SeekFrom::Start(0))?;
                stream.write_all(&[5, length as u8])?;

                // copy bytes from the cursor into the file
                io::copy(&mut cursor, stream)?;
            }
            _ => return Err(LitsCcError::Msg(format!("Unexpected token: {}", token))),
        }

        elements_processed += 1;
    }

    Ok(elements_processed)
}
