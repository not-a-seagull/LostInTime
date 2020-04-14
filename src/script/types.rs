// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// script/types.rs - Types used in bytecode reading.

use super::{Bytecode, ParserState};
use crate::LitError;
use std::{borrow::Cow, fmt, io::prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Numeric8,
    Numeric16,
    Numeric32,
    Str,
    Tuple,
}

#[derive(Debug, Clone)]
pub enum BytecodeObject {
    Numeric8(u8),
    Numeric16(i16),
    Numeric32(i32),
    Str(String),
    Tuple(Vec<BytecodeObject>),
    VarInvocation(u32),
}

impl BytecodeObject {
    #[inline]
    pub fn data_type(&self, state: &ParserState) -> DataType {
        match self {
            &BytecodeObject::Numeric8(_) => DataType::Numeric8,
            &BytecodeObject::Numeric16(_) => DataType::Numeric16,
            &BytecodeObject::Numeric32(_) => DataType::Numeric32,
            &BytecodeObject::Str(_) => DataType::Str,
            &BytecodeObject::Tuple(_) => DataType::Tuple,
            &BytecodeObject::VarInvocation(i) => state.get_variable(i).unwrap().data_type(state),
        }
    }

    pub fn as_number(&self, state: &ParserState) -> Result<i32, LitError> {
        match self {
            &BytecodeObject::Numeric8(v) => Ok(v as i32),
            &BytecodeObject::Numeric16(v) => Ok(v as i32),
            &BytecodeObject::Numeric32(v) => Ok(v),
            &BytecodeObject::VarInvocation(i) => {
                let val = state.get_variable(i)?;
                val.as_number(state)
            }
            _ => Err(LitError::ExpectedNumericalDataType(self.data_type(state))),
        }
    }

    pub fn as_string<'a>(&'a self, state: &'a ParserState) -> Result<&'a str, LitError> {
        match self {
            &BytecodeObject::Str(ref s) => Ok(s),
            &BytecodeObject::VarInvocation(i) => state.get_variable(i)?.as_string(state),
            _ => Err(LitError::IncorrectDataType(
                self.data_type(state),
                DataType::Str,
            )),
        }
    }

    pub fn as_tuple<'a>(
        &'a self,
        state: &'a ParserState,
    ) -> Result<&'a [BytecodeObject], LitError> {
        match self {
            &BytecodeObject::Tuple(ref t) => Ok(t),
            &BytecodeObject::VarInvocation(i) => state.get_variable(i)?.as_tuple(state),
            _ => Err(LitError::IncorrectDataType(
                self.data_type(state),
                DataType::Tuple,
            )),
        }
    }

    pub fn stringify(&self, state: &ParserState) -> Result<String, LitError> {
        match self {
            &BytecodeObject::Numeric8(u) => Ok(format!("{}", u)),
            &BytecodeObject::Numeric16(u) => Ok(format!("{}", u)),
            &BytecodeObject::Numeric32(u) => Ok(format!("{}", u)),
            &BytecodeObject::Str(ref u) => Ok(format!("{}", u)),
            &BytecodeObject::Tuple(ref s) => Ok(format!("{:?}", s)),
            &BytecodeObject::VarInvocation(u) => Self::stringify(state.get_variable(u)?, state),
        }
    }
}

impl Bytecode for BytecodeObject {
    fn read<T: Read>(stream: &mut T) -> Result<Self, LitError> {
        // variable type is signified by an 8-bit number
        let mut buffer = [0; 1];
        stream.read(&mut buffer)?;

        // determine which variable to read further
        match buffer[0] {
            1 => {
                // 8-bit numerical value
                stream.read(&mut buffer)?;
                Ok(BytecodeObject::Numeric8(buffer[0]))
            }
            2 => {
                // 16-bit numerical value
                let mut buffer = [0; 2];
                stream.read(&mut buffer)?;
                let val = i16::from_be_bytes(buffer);
                Ok(BytecodeObject::Numeric16(val))
            }
            3 => {
                // 32-bit numerical value
                let mut buffer = [0; 4];
                stream.read(&mut buffer)?;
                let val = i32::from_be_bytes(buffer);
                Ok(BytecodeObject::Numeric32(val))
            }
            4 => {
                // UTF-8 string
                // first, get the length
                stream.read(&mut buffer)?;

                // then, read into buffer
                let mut buffer = vec![0; buffer[0] as usize];
                stream.read_exact(&mut buffer)?;

                // finally, convert the buffer to a string
                let val = String::from_utf8(buffer)?;
                Ok(BytecodeObject::Str(val))
            }
            5 => {
                // tuple
                // first, get the length of the tuple
                stream.read(&mut buffer)?;

                // then, read an element for each in the length
                let element_num = buffer[0];
                let mut buffer = Vec::with_capacity(element_num as usize);

                for _ in 0..element_num {
                    buffer.push(BytecodeObject::read(stream)?);
                }

                Ok(BytecodeObject::Tuple(buffer))
            }
            6 => {
                // variable invocation
                // consists of the ID, which is a 4-byte number
                let mut buffer = [0; 4];
                stream.read(&mut buffer)?;
                let val = u32::from_be_bytes(buffer);
                Ok(BytecodeObject::VarInvocation(val))
            }
            _ => Err(LitError::BytecodeRead8(buffer[0])),
        }
    }
}
