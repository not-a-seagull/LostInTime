// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// resource/mod.rs - Define resources and resource dictionaries.

mod dictionary;

pub use dictionary::ResourceDictionary;

use crate::{BytecodeObject, ImgTexture, LitError};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum ResourceType {
    ImgTexture,
}

pub trait Resource: Sized {
    fn get_subdict(dict: &ResourceDictionary) -> &HashMap<u32, Self>;
    fn get_subdict_mut(dict: &mut ResourceDictionary) -> &mut HashMap<u32, Self>;
}

impl Resource for ImgTexture {
    #[inline]
    fn get_subdict(dict: &ResourceDictionary) -> &HashMap<u32, Self> {
        dict.res_img_subdict()
    }

    #[inline]
    fn get_subdict_mut(dict: &mut ResourceDictionary) -> &mut HashMap<u32, Self> {
        dict.res_img_subdict_mut()
    }
}
