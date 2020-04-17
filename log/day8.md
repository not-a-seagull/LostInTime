# Day 8

![Diagram][diagram.png]

This is a diagram of the resource schema I plan on using. A resource dictionary should contain a hash map of the resources currently loaded, and have the ability to construct and deallocate resources on demand.

Before we add the ability to contain images in files, let's make a way to load resources in general.

*In src/resource/mod.rs*

```rust
mod dictionary;

pub use dictionary::ResourceDictionary;

use crate::LitError;
use std::collections::HashMap;

/// An object that can be used to build resources.
pub trait Material : Sized {
    fn prepare(&mut self) -> Result<(), LitError>;
    fn get_subdict(dict: &ResourceDictionary) -> &HashMap<u32, Self>;
    fn get_subdict_mut(dict: &mut ResourceDictionary) -> &mut HashMap<u32, Self>;
}

pub trait Resource : Sized {
    type TMat : Material;

    fn get_subdict(dict: &ResourceDictionary) -> &HashMap<u32, Self>;
    fn get_subdict_mut(dict: &mut ResourceDictionary) -> &mut HashMap<u32, Self>;

    fn load(&mut Material) -> Result<Self, LitError>;
}
```

*In src/resource/dictionary.rs*

```rust
use crate::{ImgMaterial, ImgTexture, LitError};
use super::Resource;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ResourceDictionary {
    next_id: u32,
    loaded_ids: Vec<u32>,
    prev_loaded_ids: Vec<u32>

    mat_img: HashMap<u32, ImgMaterial>;
    
    res_img: HashMap<u32, ImgTexture>;
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
    pub fn mat_img_subdict(&self) -> &HashMap<u32, ImgMaterial> { &self.mat_img }
    #[inline]
    pub fn res_img_subdict(&self) -> &HashMap<u32, ImgTexture>  { &self.res_img }
    #[inline]
    pub fn mat_img_subdict_mut(&mut self) -> &mut HashMap<u32, ImgMaterial> { &mut self.mat_img }
    #[inline]
    pub fn res_img_subdict_mut(&mut self) -> &mut HashMap<u32, ImgTexture>  { &mut self.res_img }

    pub fn load_mat<T: Material>(&mut self, id: u32) -> Result<&mut T, LitError> {
        match T::get_subdict_mut(self).get_mut(id) {
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
    pub fn get_res<T: Resource>(&mut self, id: u32) -> Option<&mut T> {
        T::get_subdict_mut(self).get_mut(id)
    }

    #[inline]
    pub fn swap_loaded_ids(&mut self) { 
        self.prev_loaded_ids.clear();
        self.prev_loaded_ids.extend(&self.loaded_ids);
        self.loaded_ids.clear();
    }

    pub fn load_res<T: Resource>(&mut self, id: u32) -> Result<&mut T, LitError> {
        match self.get_res(id) {
            Some(r) => Ok(r),
            None => {
                let mat = self.load_mat::<T::TMat>(id)?;
                let item = T::load(mat)?;
                
                // put item in loaded items
                T::get_subdict_mut(self).insert(id, item);
                let item = self.get_res(id).unwrap(); // should be impossible to fail this

                // add item to list of loaded items
                self.loaded_ids.push(id);

                Ok(item)
            }
        }
    }

    #[inline]
    pub fn unload_res<T: Resource>(&mut self, id: u32) {
        T::get_subdict_mut(self).remove(&id)
    }

    pub fn clean(&mut self) {
        self.prev_loaded_ids.iter().filter(|i| !self.loaded_ids.contains(i)).for_each(|id| {
             self.unload_res::<ImgTexture>(*id);
        });
    }
}
```

The idea is this: resources are built from materials. A map of materials and resources, both with the same ID, are kept for this purpose. Now, let's make `ImgMaterial`.

*Inside of src/gl_utils/texture/material.rs*

```rust
use super::DIBuffer;
use crate::{
    draw::{DrawHandle, DrawInstruction},
    Color, LitError, Material, ResourceDictionary,
};
use gl::types::GLuint;
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct ImgMaterial {
    width: i16,
    height: i16,
    bg_color: Color,
    draws: Vec<DrawInstruction>,
    buffer: Option<DIBuffer>,
}

impl ImgMaterial {
    #[inline]
    pub fn from_draws(
        width: i16,
        height: i16,
        bg_color: Color,
        draws: Vec<DrawInstruction>,
    ) -> Self {
        Self {
            width,
            height,
            draws,
            bg_color,
            buffer: None,
        }
    }

    #[inline]
    pub fn width(&self) -> i16 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i16 {
        self.height
    }

    #[inline]
    pub fn draws(&self) -> &[DrawInstruction] {
        &self.draws
    }
}

impl Display for ImgMaterial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{} Image", self.width, self.height)
    }
}

impl DrawHandle for ImgMaterial {
    #[inline]
    fn new(width: i16, height: i16, background_color: Color) -> Self {
        Self::from_draws(width, height, background_color, vec![])
    }

    fn draw_pixel(&mut self, x: i16, y: i16, color: Color) -> Result<(), LitError> {
        self.draws.push(DrawInstruction::Pixel { x, y, color });
        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        h: i16,
        color: Color,
    ) -> Result<(), LitError> {
        self.draws
            .push(DrawInstruction::Rectangle { x, y, w, h, color });
        Ok(())
    }
}

impl Material for ImgMaterial {
    #[inline]
    fn get_subdict(dict: &ResourceDictionary) -> &HashMap<u32, Self> {
        dict.mat_img_subdict()
    }

    #[inline]
    fn get_subdict_mut(dict: &mut ResourceDictionary) -> &mut HashMap<u32, Self> {
        dict.mat_img_subdict_mut()
    }

    fn prepare(&mut self) -> Result<(), LitError> {
        // this is just the buffer logic from earlier

        let mut draws: Vec<GLint> = vec![];
        fb.draws()
            .iter()
            .rev()
            .for_each(|d| draws.extend(&d.as_int_set()));

        // build a 1-dimensional texture for this instance
        self.buffer = Some(DIBuffer::from_raw(
            &[fb.draws().len() as i16],
            draws.as_ptr(),
        )?);
        Ok(())
    }
}
```

Come to think of it, it makes more sense to be able to translate an ImgMaterial to an ImgTexture, rather than using a FrameBuffer. Thankfully, this process will involve constructing a FrameBuffer from an ImgMaterial, so we can re-use most of the code that we already have.

I've renamed `fb_to_texture.rs` to `render.rs`, since it feels like it fits better.

*Modifications to src/gl_utils/texture/render.rs*

```rust
impl Resource for ImgTexture {
    type TMat = ImgMaterial;

    #[inline]
    fn get_subdict(dict: &ResourceDictionary) -> &HashMap<u32, Self> {
        dict.res_img_subdict()
    }

    #[inline]
    fn get_subdict_mut(dict: &mut ResourceDictionary) -> &mut HashMap<u32, Self> {
        dict.res_img_subdict_mut()
    }

    fn load(mat: &mut ImgMaterial) -> Result<Self, LitError> {
        // create a frame buffer and a texture
        let fb = FrameBuffer::new();
        let tex = Texture::from_raw(&[mat.width(), mat.height()], ptr::null())?;

        // bind the frame buffer to the current context
        fb.bind();

        // bind the frame buffer to the texture
        unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex.id(),
                0,
            )
        };

        // initialize VAO and VBO
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        unsafe {
            gl::Viewport(0, 0, mat.width() as GLint, mat.height() as GLint);
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // fill the buffer with the required vertices
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * mem::size_of::<GLfloat>()) as GLint,
                0 as *const GLvoid,
            );
        }

        TEXTURE_RENDERER.activate();

        // set up uniforms
        assign_uniform!("s_width" => Uniform1i <= mat.width() as GLint);
        assign_uniform!("s_height" => Uniform1i <= mat.height() as GLint);
        assign_uniform!("s_draw_len" => Uniform1i <= mat.draws().len() as GLint);

        let bg_clr = mat.background_color().as_gl_color();
        assign_uniform!("bg_color" => Uniform4f <= bg_clr[0], bg_clr[1], bg_clr[2], bg_clr[3]);

        // bind the DI buffer to the context
        let di_buffer = mat.buffer().unwrap();
        di_buffer.bind();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }

        di_buffer.unbind();
        fb.unbind();

        Ok(tex)
    }
}
```

Unfortunately, I run into the issue of the borrow checker here. Apparently, since we are using mutable borrows of the resource dictionary several times, this conflicts with the "once mutable" philosophy of Rust. I had to write some sketchy code. I had to separate `load_res` and `get_res`, but also:

```rust
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
```
