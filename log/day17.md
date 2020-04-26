# Day 17

I went through and rewrote the `lit-gl-wrapper` crate to rely on structual GL context. This should help make things safer. I also went though the main crate and fixed it to rely on `lit-gl-wrapper`.

Of note, I reworked the resource dictionary to load everything up front.

*In src/resource/dictionary.md*

```rust
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
```

I also begun work on a draw buffer before I ended the day.
