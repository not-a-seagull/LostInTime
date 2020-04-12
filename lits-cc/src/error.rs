// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// error.rs - Error handling for the compilation process

use proc_macro2::{LexError, TokenTree};
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LitsCcError {
    #[error("{0}")]
    Msg(String),
    #[error("{0}")]
    StaticMsg(&'static str),
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(TokenTree),
    #[error("An IO error occurred: {0}")]
    Io(#[from] IoError),
    #[error("A lexing error occurred: {0:?}")]
    Lex(LexError),
    #[error("Unable to find variable with id {0}")]
    VariableNotFound(String),
    #[error("Expected identifier")]
    ExpectedIdent,
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
}

impl From<LexError> for LitsCcError {
    fn from(l: LexError) -> LitsCcError {
        LitsCcError::Lex(l)
    }
}
