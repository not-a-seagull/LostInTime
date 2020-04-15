# Day 7

I've done some research, and I've come to the realization that a 1-dimensional texture is probably the best way to pass a series of numbers into the shader. At least, this is probably better than an array of structures, which requires a lot of (probably laggy) string formatting.

We can translate a single draw command into three four-dimensional vectors: one that just contains the kind, one that contains the coordinates for the draw command, and one that contains the color. Let's try implementing that.

First of all, I moved `src/gl_utils/texture.rs` to `src/gl_utils/texture/mod.rs`, and I moved all of the rendering code to `src/gl_utils/texture/fb_to_tex.rs`.

Then, I crated a macro to make assignments to uniforms a little bit easier.

*Modifications to src/gl_utils/texture/fb_to_tex.rs*

```rust
macro_rules! assign_uniform {
    ($name: expr => $call: ident <= $($val: expr),*) => {
        {
            let loc = unsafe { gl::GetUniformLocation(TEXTURE_RENDERER.id(), cify_str($name)) };
            if loc != -1 {
                unsafe { gl::$call(loc, $($val),*) };
            }
        }
    }
}
```

Let's see what happens when we replace the old logic with the new.

*Modifications to src/gl_utils/texture/fb_to_tex.rs*

```rust
TEXTURE_RENDERER.activate();

// set up uniforms
assign_uniform!("s_width" => Uniform1i <= fb.width() as GLint);
assign_uniform!("s_height" => Uniform1i <= fb.height() as GLint);
assign_uniform!("s_draw_len" => Uniform1i <= fb.draws().len() as GLint);

// build list of data for texture
let mut draws: Vec<GLint> = vec![];
fb.draws().iter().rev().for_each(|d| draws.extend(&d.as_int_set()));

// build a 1-dimensional texture for this instance
let mut tid: GLuint = 0;
unsafe {
    gl::GenTextures(1, &mut tid);
    gl::BindTexture(gl::TEXTURE_1D, tid);

    gl::TexImage1D(
	gl::TEXTURE_1D,
	0,
	gl::RGBA as GLint,
	fb.draws().len() as GLint,
	0,
	gl::RGBA,
	gl::INT,
	draws.as_ptr() as *const c_void,
    );
};

// draw the image
unsafe {
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
    gl::BindVertexArray(0);
    gl::BindTexture(gl::TEXTURE_1D, 0);
}
```

Let's also modify the shader to reflect this.

*Modifications to src/gl_utils/shaders/fb_to_texture.frag*

```glsl
uniform int s_width;
uniform int s_height;
uniform int s_draw_len;
uniform sampler1d s_draws;

int determine_pix(float scale, int length) {
    float real_scale = (scale + 1.0) / 2;
    return int(real_scale * length);
}

void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0); // TODO: set background color

    // determine which pixel we are
    vec2 pix_coords = vec2(determine_pix(tex_coords.x), determine_pix(tex_coords.y));

    // iterate over draw instructions and see which ones involve this pixel
    int base_index;
    int kind;
    ivec4 d_color;
    ivec4 coords;
    for (int i = 0; i < s_draw_len; i++) {
        base_index = i * 3;
        kind = texelFetch(s_draws, base_index, 0).x;
        coords = texelFetch(s_draws, base_index + 1, 0);
        d_color = texelFetch(s_draws, base_index + 2, 0);

        // if our coords are in the designated zone, we are good
        if ((kind == 1 && pix_coords.x == coords.x && pix_coords.y == coords.y) || // pixel
            ((kind == 2 || kind == 3) && pix_coords.x >= coords.x && pix_coords.x <= coords.x + coords.z
                       && pix_coords.y >= coords.y && pix_coords.y <= coords.y + coords.w) || // rectangle) {
            color = vec4(d_color.x / 255, d_color.y / 255, d_color.z / 255, d_color.w / 255);
            break;
        }
     
    }
}
```

The main thing missing here is that we don't have a uniform set to the background color. Let's fix that.

*Modifications to src/gl_utils/texture/fb_to_tex.rs*

```rust
let bg_clr = fb.background_color().as_gl_color();
assign_uniform!("bg_color" => Uniform4f <= bg_clr[0], bg_clr[1], bg_clr[2], bg_clr[3]);
```

*Modifications to src/gl_utils/shaders/fb_to_texture.rs*

```glsl
color = bg_color;
```

Finally, I also moved `src/gl_renderer.rs` to `src/gl_utils/renderer.rs` since it feels like it belongs there.

As you can see, this recreates the buffer whenever the image is rerendered. This will probably make the loading process slower. Later, I should move the buffer code so that it is only called once, then stored. Another limitation is that buffers have a maximum of, at minimum, 1024 texels. One of these instructions is 3 texels, so our images are limited to 341 draw commands. Hopefully, this should not be a problem.

Here's how I imagine the pipeline:

* A list of draw commands are read from the file.
* These draw commands are used to generate the draw command buffers. I haven't decided whether or not these will be generated at the same time as the list of draw commands or lazily generated when they are needed.
* A frame buffer loaded with the draw commands is created when the image is needed.
* The image is rendered into the frame buffer and used in the game.

Given that this will involve the transport of 1D textures as well as 2D textures, it will be useful to abstract the Texture class with multiple dimensions.

*Modifications to src/gl_utils/texture/mod.rs*

```rust
pub trait TextureType {
    type ValueType;

    fn bind_texture_location() -> GLenum;
    fn tex_image(dimensions: &[i16], data: *const Self::ValueType) -> Result<(), LitError>;
}

#[derive(Debug)]
pub struct Texture<T: TextureType> {
    id: GLuint,
    dimensions: Vec<i16>,
    _phantom: PhantomData<T>,
}

impl<T: TextureType> Texture<T> {
    pub fn from_raw(dimensions: &[i16], data: *const T::ValueType) -> Result<Self, LitError> {
        let mut id: GLuint = 0;

        // generate and bind the texture
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(T::bind_texture_location(), id);
        }

        // fill the texture with the data
        T::tex_image(dimensions, data)?;

        unsafe { gl::BindTexture(T::bind_texture_location(), 0) };

        Ok(Self {
            id,
            dimensions: dimensions.iter().map(|i| *i).collect(),
            _phantom: PhantomData,
        })
    }

    /* ... */ 

    pub fn bind(&self) {
        unsafe { gl::BindTexture(T::bind_texture_location(), self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(T::bind_texture_location(), 0) }
    }
}

/* ... */

// some specific types
pub type ImgTexture = Texture<ImgTextureType>;
pub type DIBuffer = Texture<DIBufferType>;
```

*In src/gl_utils/texture/dimensions.rs*

```rust
use super::TextureType;
use crate::LitError;
use gl::types::{GLbyte, GLenum, GLint};
use std::ffi::c_void;

pub struct DIBufferType;

impl TextureType for DIBufferType {
    type ValueType = GLint;

    #[inline]
    fn bind_texture_location() -> GLenum {
        gl::TEXTURE_1D
    }

    fn tex_image(dimensions: &[i16], data: *const GLint) -> Result<(), LitError> {
        if dimensions.len() != 1 {
            return Err(LitError::ImproperDimensions(1, dimensions.len()));
        }

        unsafe {
            gl::TexImage1D(
                gl::TEXTURE_1D,
                0,
                gl::RGBA as GLint,
                dimensions[0] as GLint,
                0,
                gl::RGBA,
                gl::INT,
                data as *const c_void,
            )
        };

        Ok(())
    }
}

pub struct ImgTextureType;

impl TextureType for ImgTextureType {
    type ValueType = GLbyte;

    #[inline]
    fn bind_texture_location() -> GLenum {
        gl::TEXTURE_2D
    }

    fn tex_image(dimensions: &[i16], data: *const GLbyte) -> Result<(), LitError> {
        if dimensions.len() != 2 {
            return Err(LitError::ImproperDimensions(2, dimensions.len()));
        }

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                dimensions[0] as GLint,
                dimensions[1] as GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data as *const c_void,
            )
        };

        Ok(())
    }
}
```

*Modifications to src/gl_utils/texture/fb_to_tex.rs*

```rust
impl TryFrom<FrameBuffer> for ImgTexture {
    type Error = LitError;

    fn try_from(fb: FrameBuffer) -> Result<ImgTexture, LitError> {
        // create the texture
        let tex = Texture::from_raw(&[fb.width(), fb.height()], ptr::null())?;

        /* ... */

        // build a 1-dimensional texture for this instance
        let mut tid: GLuint = 0;
        let di_buffer = DIBuffer::from_raw(&[fb.draws().len() as i16], draws.as_ptr())?;
        di_buffer.bind();

        // draw the image
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
