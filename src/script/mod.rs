// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// script/mod.rs - Construct game data from bytecode

mod bytecode;
pub use bytecode::Bytecode;

mod eval;

mod types;
pub use types::{BytecodeObject, DataType};

use super::LitError;
use std::{
    collections::HashMap,
    io::prelude::*,
    sync::{Arc, Mutex},
};

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

lazy_static::lazy_static! {
    pub static ref PARSER_STATE: Arc<Mutex<ParserState>> = Arc::new(Mutex::new(ParserState::new()));
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

        /*loop {
            if let Err(e) = eval::eval(stream, &mut data) {
                eprintln!("Error encountered: {}", e);
                break;
            } // go until error is encountered
        }*/

        while let Ok(()) = eval::eval(stream, &mut data) {}

        Ok(data)
    }
}
