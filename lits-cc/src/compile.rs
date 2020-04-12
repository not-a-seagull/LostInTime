// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// compile.rs - Take a line of LitS and compile it.

use crate::{process_command, process_literals, CompilerState, LitsCcError};
use proc_macro2::{TokenStream, TokenTree};
use std::io::prelude::*;

pub fn compile_line<T: Write>(
    line: &str,
    stream: &mut T,
    state: &mut CompilerState,
) -> Result<(), LitsCcError> {
    // parse the line into tokens
    let tokens: TokenStream = line.parse()?;
    let mut iter = tokens.into_iter();

    // get the first token in the stream, which must be an identifier
    match iter.next() {
        Some(TokenTree::Ident(ref i)) => {
            process_command(i, &mut iter, stream, state)?;
        }
        None => return Ok(()), // empty line, write nothing
        _ => return Err(LitsCcError::ExpectedIdent),
    }

    let _ = process_literals(&mut iter, stream, state)?;
    Ok(())
}
