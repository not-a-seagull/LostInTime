// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// script/mod.rs - Construct game data from bytecode

mod bytecode;
pub use bytecode::Bytecode;

mod eval;

mod types;
pub use types::{BytecodeObject, DataType};

use super::LitError;
use std::{collections::HashMap, io::prelude::*};

pub struct ParserState {
    variables: HashMap<u32, BytecodeObject>,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn register_variable(&mut self, index: u32, object: BytecodeObject) {
        self.variables.insert(index, object);
    }

    pub fn get_variable(&self, index: u32) -> Result<&BytecodeObject, LitError> {
        self.variables
            .get(&index)
            .ok_or_else(|| LitError::VariableNotFound(index))
    }
}

#[derive(Debug)]
pub struct GameData {
    name: String,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            name: String::from("Unnamed"),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Bytecode for GameData {
    fn read<T: Read>(stream: &mut T) -> Result<Self, LitError> {
        let mut data = Self::new();
        let mut parse = ParserState::new();

        while let Ok(()) = eval::eval(stream, &mut parse, &mut data) {} // go until error is encountered

        Ok(data)
    }
}
