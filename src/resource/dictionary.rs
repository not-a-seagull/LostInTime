// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// resource/mod.rs - Define resource dictionaries.

use super::Resource;
use crate::{ImgTexture, LitError};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ResourceDictionary {
    next_id: u32,
    img_textures: HashMap<u32, ImgTexture>,
}

impl ResourceDictionary {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            loaded_ids: vec![],
            prev_loaded_ids: vec![],
            mat_img: HashMap::new(),
        }
    }

    #[inline]
    pub fn res_img_subdict(&self) -> &HashMap<u32, ImgTexture> {
        &self.res_img
    }
    #[inline]
    pub fn res_img_subdict_mut(&mut self) -> &mut HashMap<u32, ImgTexture> {
        &mut self.res_img
    }

    pub fn add_res<T: Resource>(&mut self, item: T) -> u32 {
        // generate id
        let id = self.next_id;
        self.next_id += 1;

        // insert item
        T::get_subdict_mut(self).insert(id, item);

        id
    }

    #[inline]
    pub fn get_res<T: Resource>(&self, id: u32) -> Option<&T> {
        T::get_subdict(self).get(&id)
    }
}
