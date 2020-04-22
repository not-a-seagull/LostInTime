// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// script/mod.rs - Construct game data from bytecode

mod bytecode;
pub use bytecode::Bytecode;

mod eval;

mod types;
pub use types::{BytecodeObject, DataType};

use super::{Color, ImgMaterial, LitError, Material, MaterialType, Resource, ResourceDictionary};
use std::{collections::HashMap, io::prelude::*};

#[derive(Debug, Copy, Clone)]
pub struct Dependancy {
    pub kind: MaterialType,
    pub id: u32,
}

pub struct ParserState {
    pub variables: HashMap<u32, BytecodeObject>,
    pub dependency_relations: HashMap<u32, Vec<Dependancy>>,
    color_ids: HashMap<u32, HashMap<u8, Color>>,

    // storage for various types of resources
    pub img_material_ids: Vec<u32>,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            color_ids: HashMap::new(),
            dependency_relations: HashMap::new(),
            variables: HashMap::new(),
            img_material_ids: vec![],
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

    pub fn add_dependencies(&mut self, index: u32, dependencies: Vec<Dependancy>) {
        self.dependency_relations.insert(index, dependencies);
    }

    pub fn get_variable_mut(&mut self, index: u32) -> Result<&mut BytecodeObject, LitError> {
        self.variables
            .get_mut(&index)
            .ok_or_else(|| LitError::VariableNotFound(index))
    }

    pub fn register_color_id(&mut self, object: u32, index: u8, clr: Color) {
        let dict = match self.color_ids.get_mut(&object) {
            Some(d) => d,
            None => {
                self.color_ids.insert(object, HashMap::new());
                self.color_ids.get_mut(&object).unwrap()
            }
        };

        dict.insert(index, clr);
    }

    pub fn get_color(&self, object: u32, index: u8) -> Result<&Color, LitError> {
        self.color_ids
            .get(&object)
            .ok_or_else(|| LitError::ColorIdObjectNotFound(object))?
            .get(&index)
            .ok_or_else(|| LitError::ColorIdNotFound(object, index))
    }

    pub fn into_resource_dict(self) -> Result<ResourceDictionary, LitError> {
        let mut rd = ResourceDictionary::new();
        let ParserState {
            mut variables,
            dependency_relations,
            img_material_ids,
            ..
        } = self;

        insert_materials::<ImgMaterial>(
            &mut rd,
            &dependency_relations,
            &mut variables,
            img_material_ids,
        )?;

        Ok(rd)
    }
}

// helper function: insert resources for a certain type
#[inline]
fn insert_material<T: Material>(
    rd: &mut ResourceDictionary,
    dep_rels: &HashMap<u32, Vec<Dependancy>>,
    variables: &mut HashMap<u32, BytecodeObject>,
    id: u32,
) -> Result<u32, LitError> {
    // loop through dependencies to get ids for these dependencies
    let dep_ids: Vec<Result<u32, LitError>> = if let Some(deps) = dep_rels.get(&id) {
        deps.iter()
            .map(|dep| match dep.kind {
                MaterialType::ImgMaterial => {
                    insert_material::<ImgMaterial>(rd, dep_rels, variables, dep.id)
                }
            })
            .collect()
    } else {
        vec![]
    };

    if let Some(mat) = variables.remove(&id) {
        Ok(rd.add_mat(T::from_bytecode_object(mat, &dep_ids)?))
    } else {
        Err(LitError::VariableNotFound(id))
    }
}

#[inline]
fn insert_materials<T: Material>(
    rd: &mut ResourceDictionary,
    dep_rels: &HashMap<u32, Vec<Dependancy>>,
    variables: &mut HashMap<u32, BytecodeObject>,
    ids: Vec<u32>,
) -> Result<(), LitError> {
    for id in ids {
        insert_material::<T>(rd, dep_rels, variables, id)?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct GameData {
    name: String,
    resource_dict: Option<ResourceDictionary>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            name: String::from("Unnamed"),
            resource_dict: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_resource<T: Resource>(&mut self, id: u32) -> Result<&T, LitError> {
        self.resource_dict.as_mut().unwrap().load_res(id)
    }
}

impl Bytecode for GameData {
    fn read<T: Read>(stream: &mut T) -> Result<Self, LitError> {
        let mut data = Self::new();
        let mut state = ParserState::new();

        'parse: loop {
            match eval::eval(stream, &mut data, &mut state) {
                Err(e) => {
                    eprintln!("Error encountered: {}", e);
                    break 'parse;
                }
                Ok(false) => break 'parse,
                Ok(true) => {}
            } // go until error is encountered
        }

        data.resource_dict = Some(state.into_resource_dict()?);

        Ok(data)
    }
}
