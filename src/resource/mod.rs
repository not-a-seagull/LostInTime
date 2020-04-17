// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// resource/mod.rs - Define resources and resource dictionaries.

mod dictionary;

pub use dictionary::ResourceDictionary;

use crate::LitError;
use std::collections::HashMap;

/// An object that can be used to build resources.
pub trait Material: Sized {
    fn prepare(&mut self) -> Result<(), LitError>;
    fn get_subdict(dict: &ResourceDictionary) -> &HashMap<u32, Self>;
    fn get_subdict_mut(dict: &mut ResourceDictionary) -> &mut HashMap<u32, Self>;
}

pub trait Resource: Sized {
    type TMat: Material;

    fn get_subdict(dict: &ResourceDictionary) -> &HashMap<u32, Self>;
    fn get_subdict_mut(dict: &mut ResourceDictionary) -> &mut HashMap<u32, Self>;

    fn load(mat: &Self::TMat) -> Result<Self, LitError>;
}
