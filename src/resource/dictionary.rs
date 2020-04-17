// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// resource/mod.rs - Define resource dictionaries.

use super::{Material, Resource};
use crate::{ImgMaterial, ImgTexture, LitError};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ResourceDictionary {
    next_id: u32,
    loaded_ids: Vec<u32>,
    prev_loaded_ids: Vec<u32>,

    mat_img: HashMap<u32, ImgMaterial>,

    res_img: HashMap<u32, ImgTexture>,
}

impl ResourceDictionary {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            loaded_ids: vec![],
            prev_loaded_ids: vec![],
            mat_img: HashMap::new(),
            res_img: HashMap::new(),
        }
    }

    #[inline]
    pub fn mat_img_subdict(&self) -> &HashMap<u32, ImgMaterial> {
        &self.mat_img
    }
    #[inline]
    pub fn res_img_subdict(&self) -> &HashMap<u32, ImgTexture> {
        &self.res_img
    }
    #[inline]
    pub fn mat_img_subdict_mut(&mut self) -> &mut HashMap<u32, ImgMaterial> {
        &mut self.mat_img
    }
    #[inline]
    pub fn res_img_subdict_mut(&mut self) -> &mut HashMap<u32, ImgTexture> {
        &mut self.res_img
    }

    pub fn load_mat<T: Material>(&mut self, id: u32) -> Result<&T, LitError> {
        match T::get_subdict_mut(self).get_mut(&id) {
            None => Err(LitError::MissingMaterial(id)),
            Some(t) => {
                t.prepare()?;
                Ok(t)
            }
        }
    }

    pub fn add_mat<T: Material>(&mut self, item: T) -> u32 {
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

    pub fn load_res<T: Resource>(&mut self, id: u32) -> Result<&T, LitError> {
        let mat = self.load_mat::<T::TMat>(id)?;
        let item = T::load(mat)?;

        // put item in loaded items
        T::get_subdict_mut(self).insert(id, item);

        // add item to list of loaded items
        self.loaded_ids.push(id);

        let item = self.get_res(id).unwrap(); // should be impossible to fail this

        Ok(item)
    }

    #[inline]
    pub fn swap_loaded_ids(&mut self) {
        self.prev_loaded_ids.clear();
        self.prev_loaded_ids.extend(&self.loaded_ids);
        self.loaded_ids.clear();
    }

    #[inline]
    pub fn unload_res<T: Resource>(&mut self, id: u32) {
        T::get_subdict_mut(self).remove(&id);
    }

    pub fn clean(&mut self) {
        // TODO: performance issues inbound!
        let mut prev_loaded_ids = self.prev_loaded_ids.clone();

        for i in prev_loaded_ids {
            if !self.loaded_ids.contains(&i) {
                self.unload_res::<ImgTexture>(i);
            }
        }    
 
        self.prev_loaded_ids.clear();
    }
}
