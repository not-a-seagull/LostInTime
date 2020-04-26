// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// error.rs - Error handling struct.

use crate::{
    script::{DataType, ParserState},
    GlCall, GlErrorType,
};
use lit_gl_wrapper::GlError;
use sdl2::video::WindowBuildError;
use std::{
    fmt,
    io::Error as IoError,
    num::TryFromIntError,
    string::FromUtf8Error,
    sync::{MutexGuard, PoisonError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LitError {
    #[error("{0}")]
    Msg(String),
    #[error("{0}")]
    StaticMsg(&'static str),
    #[error("{0}")]
    GlError(#[from] GlError),
    #[error("Unexpected byte while reading bytecode: {0:X?}")]
    BytecodeRead8(u8),
    #[error("Unexpected word while reading bytecode: {0:X?}")]
    BytecodeRead16(u16),
    #[error("Unexpected dword while reading bytecode: {0:X?}")]
    BytecodeRead32(u32),
    #[error("An IO error occurred: {0}")]
    Io(#[from] IoError),
    #[error("Error converting from UTF-8: {0}")]
    FromUtf8(#[from] FromUtf8Error),
    #[error("Unable to find variable with reference {0:X?}")]
    VariableNotFound(u32),
    #[error("Color map does not contain ID {0}")]
    ColorIdObjectNotFound(u32),
    #[error("Color map at {0} does not contain color {1}")]
    ColorIdNotFound(u32, u8),
    #[error("Expected data type {1:?}, found {0:?}")]
    IncorrectDataType(DataType, DataType),
    #[error("Expected a numerical data type, found {0:?}")]
    ExpectedNumericalDataType(DataType),
    #[error("Unable to find data file")]
    NoDataFile,
    #[error("Mutex has been poisoned - this is likely an internal issue")]
    PoisonedMutex,
    #[error("Conversion error: {0}")]
    TryFromInt(#[from] TryFromIntError),
}

impl<'a, T> From<PoisonError<MutexGuard<'a, T>>> for LitError {
    fn from(_f: PoisonError<MutexGuard<'a, T>>) -> Self {
        Self::PoisonedMutex
    }
}

impl From<LitError> for fmt::Error {
    fn from(_f: LitError) -> fmt::Error {
        fmt::Error
    }
}
