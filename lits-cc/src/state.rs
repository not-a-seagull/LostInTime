// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// state.rs - The current state of the application.

use crate::LitsCcError;
use std::collections::HashMap;

pub struct CompilerState {
    variables: HashMap<String, u32>,
    current_id: u32,
}

impl CompilerState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            current_id: 1,
        }
    }

    pub fn register_variable(&mut self, name: &str) -> u32 {
        let id = self.current_id;
        self.variables.insert(String::from(name), id);
        self.current_id += 1;
        id
    }

    pub fn get_variable_id(&self, name: &str) -> Result<u32, LitsCcError> {
        match self.variables.get(name) {
            Some(u) => Ok(*u),
            None => Err(LitsCcError::VariableNotFound(String::from(name))),
        }
    }
}
