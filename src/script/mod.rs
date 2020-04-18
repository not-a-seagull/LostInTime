// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// script/mod.rs - Construct game data from bytecode

mod bytecode;
pub use bytecode::Bytecode;

mod eval;

mod types;
pub use types::{BytecodeObject, DataType};

use super::{Color, LitError, ResourceDictionary};
use std::{
    collections::HashMap,
    io::prelude::*,
    sync::{Arc, Mutex},
};

pub struct ParserState {
    variables: HashMap<u32, BytecodeObject>,
    color_ids: HashMap<u32, HashMap<u8, Color>>,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            color_ids: HashMap::new(),
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

    pub fn register_color_id(&mut self, object: u32, index: u8, clr: Color) {
        let dict = self.color_ids.get_mut(&object).unwrap_or_else(|| {
            self.color_ids.insert(object, HashMap::new());
            self.color_ids.get_mut(&object).unwrap()
        });

        dict.insert(index, clr);
    }

    pub fn get_color_id(&self, object: u32, index: u8) -> Result<&Color, LitError> {
        self.color_ids.get(&object).ok_or_else(|| LitError::ColorIdObjectNotFound(object))?
            .get(&index).ok_or_else(|| LitError::ColorIdNotFound(object, index))
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
        let mut state = ParserState::new();

        /*loop {
            if let Err(e) = eval::eval(stream, &mut data) {
                eprintln!("Error encountered: {}", e);
                break;
            } // go until error is encountered
        }*/

        while let Ok(()) = eval::eval(stream, &mut data, &mut state) {}

        Ok(data)
    }
}
