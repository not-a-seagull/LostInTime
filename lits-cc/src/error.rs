// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// error.rs - Error handling for the compilation process

use proc_macro2::{LexError, TokenTree};
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LitsCcError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(TokenTree),
    #[error("An IO error occurred: {0}")]
    Io(#[from] IoError),
    #[error("A lexing error occurred: {0:?}")]
    Lex(LexError),
}

impl From<LexError> for LitsCcError {
    fn from(l: LexError) -> LitsCcError {
        LitsCcError::Lex(l)
    }
}
