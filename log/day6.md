# Day 6

Now that we have shader compilation code, let's draw some images. First, here is an enum to store instructions for drawing:

*In src/draw/instruction.rs*

```rust
use crate::Color;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DrawInstruction {
    // draw a single pixel
    Pixel {
        x: i16,
        y: i16,
        color: Color,
    },
    // draw a rectangle
    Rectangle {
        x: i16,
        y: i16,
        w: i16,
        h: i16,
        color: Color,
    },
    // draw a square of texture
    Square {
        x: i16,
        y: i16,
        l: i16,
        color: Color,
    },
}

#[inline]
pub fn translate_draw_instruction(di: &DrawInstruction) -> u32 {
    match di {
        &DrawInstruction::Pixel { .. } => 1,
        &DrawInstruction::Rectangle { .. } => 2,
        &DrawInstruction::Square { .. } => 3,
    }
}
```

They are convertible to `u32` so we can pass them into OpenGL shaders.

Next, let's construct wrappers around OpenGL textures and frame buffers.

*In src/gl_utils/texture.rs*

```rust
use gl::types::{GLint, GLuint};
use std::ffi::c_void;

#[derive(Debug)]
pub struct Texture {
    id: GLuint,
    width: i16,
    height: i16,
}

impl Texture {
    pub fn from_raw(width: i16, height: i16, data: *const u8) -> Self {
        let mut id: GLuint = 0;

        // generate and bind the texture
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
        }

        // fill the texture with the data
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as GLint,
                height as GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data as *const c_void,
            )
        };

        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) };

        Self { id, width, height }
    }

    #[inline]
    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}
```

*In src/gl_utils/frame_buffer.rs*

```rust
// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/frame_buffer.rs - An OpenGL FrameBuffer

use crate::{
    draw::{DrawHandle, DrawInstruction},
    Color, LitError,
};
use gl::types::GLuint;
use std::fmt;

#[derive(Debug)]
pub struct FrameBuffer {
    fbo: GLuint,
    draws: Vec<DrawInstruction>,
    height: i16,
    width: i16,
    background_color: Color,
}

impl fmt::Display for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FrameBuffer with ID {}", self.fbo)
    }
}

impl DrawHandle for FrameBuffer {
    fn new(width: i16, height: i16, background_color: Color) -> Self {
        // generate open gl frame buffers
        let mut fbo: GLuint = 0;
        unsafe { gl::GenFramebuffers(1, &mut fbo) };

        Self {
            fbo,
            draws: vec![],
            width,
            height,
            background_color,
        }
    }

    fn draw_pixel(&mut self, x: i16, y: i16, color: Color) -> Result<(), LitError> {
        self.draws.push(DrawInstruction::Pixel { x, y, color });
        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
        color: Color,
    ) -> Result<(), LitError> {
        self.draws.push(DrawInstruction::Rectangle {
            x,
            y,
            w: width,
            h: height,
            color,
        });
        Ok(())
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &self.fbo) };
    }
}
```

The wonder of Rust is that, because of the `Drop` trait, we can automatically delete the buffers when they go out of scope. This prevents a lot of the headaches of C-like memory allocation.

We still have to make it so we can take those instructions stored in the frame buffer, and convert it over to a Texture that we can render. Here is the code used to do that:

*Modifications to src/gl_utils/texture.rs*

```rust
impl Texture {
    /* ... */

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }
}

lazy_static::lazy_static! {
    static ref TEXTURE_RENDERER: Program = {
        let vert_source = include_str!("./shaders/fb_to_texture.vert");
        let frag_source = include_str!("./shaders/fb_to_texture.frag");
        let vert = Shader::from_source(&vert_source, ShaderType::Vertex)
            .expect("Vertex shader compilation failed");
        let frag = Shader::from_source(&frag_source, ShaderType::Fragment)
            .expect("Fragment shader compilation failed");

        Program::new(&[vert, frag]).expect("Shader linking failed")
    };
}

// TODO: make into TryFrom and check for errors
impl From<FrameBuffer> for Texture {
    fn from(fb: FrameBuffer) -> Texture {
        // create the texture
        let tex = Texture::from_raw(fb.width(), fb.height(), ptr::null());

        // bind the frame buffer
        fb.bind();

        // TODO: probably adjust the texture buffer a bit

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

        // TODO: draw commands
        TEXTURE_RENDERER.activate();

        tex
    }
}
```

*Modifications to src/gl_utils/frame_buffer.rs*

```rust
impl FrameBuffer {
    #[inline]
    pub fn fbo(&self) -> GLuint {
        self.fbo
    }

    #[inline]
    pub fn width(&self) -> i16 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i16 {
        self.height
    }

    pub fn bind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo()) };
    }

    pub fn unbind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) };
    }
}
```

*Modifications to src/gl_utils/program.rs*

```rust
impl Program {
    /* ... */

    #[inline]
    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn activate(&self) {
        unsafe { gl::UseProgram(self.id) }
    }
}
```

The logic here is that we'll bind the frame buffer to the texture, draw on the frame buffer, then dispose of the frame buffer and return the texture.

However, we did reference the "fb_to_texture.{vert/frag}" files in this. Meaning, this is the first time in this series that we will dive into GLSL. I'm going to make some basic shaders that pass through the coordinate input and output nothing but the color red.

*Inside of src/gl_utils/shaders/fb_to_texture.vert*

```glsl
#version 330 core

layout (location = 0) in vec4 vertex;

out vec2 tex_coords;

void main() {
    tex_coords = vertex.zw;
    gl_Position = vec4(vertex.xy, 0.0, 1.0);
}
```

*Inside of src/gl_utils/shaders/fb_to_texture.frag*

```glsl
#version 330 core

// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/shaders/fb_to_texture.frag - Calculate pixel color by calculating position in root image

in vec2 tex_coords;
out vec4 tex_coords;

void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0);
}
```

Now, aside from the problem of drawing, we need to find a way to pass the drawing instructions through to the shaders. We'll need a format that GLSL can really understand. We can do this with a `struct` with an `#repr(C)` tag.

```rust
#[repr(C)]
struct ShaderDrawInstruction {
    kind: GLuint,
    coords: [GLshort; 4],
    color: [GLfloat; 4],
}

impl From<DrawInstruction> for ShaderDrawInstruction {
    fn from(di: DrawInstruction) -> ShaderDrawInstruction {
        let mut sdi = ShaderDrawInstruction {
            kind: translate_draw_instruction(di),
            coords: [0; 4],
            color: [0; 4],
        };

        match di {
            DrawInstruction::Pixel { x, y, color } => {
                sdi.coords[0] = x;
                sdi.coords[1] = y;
                sdi.color = color.as_gl_color();
            },
            DrawInstruction::Rectangle { x, y, w, h, color } => {
                sdi.coords[0] = x;
                sdi.coords[1] = y;
                sdi.coords[2] = w;
                sdi.coords[3] = h;
                sdi.color = color.as_gl_color();
            },
            DrawInstruction::Square { x, y, l, color } => {
                sdi.coords[0] = x;
                sdi.coords[1] = y;
                sdi.coords[2] = l;
                sdi.coords[3] = l;
                sdi.color = color.as_gl_color();
            },
        }

        sdi
    }
}
```

Now, we implement the drawing code:

*Modifications to src/gl_utils/texture.rs*

```rust

const VERTICES: GLfloat[] = [
//  Pos       Tex
    0.0, 1.0, 0.0, 1.0,
    1.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 1.0,
    1.0, 1.0, 1.0, 1.0,
    1.0, 0.0, 1.0, 0.0,
];

impl From<FrameBuffer> for Texture {
    fn from(fb: FrameBuffer) -> Texture {
        /* ... */

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // fill the buffer with the required vertices
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(VERTICES), VERTICES, gl::STATIC_DRAW);

            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 4 * mem::size_of<GLfloat>(), 0 as *const GLvoid);
        }

        TEXTURE_RENDERER.activate();
        let prog_id = TEXTURE_RENDERER.id();

        // set up uniforms
        // image width
        let width_loc = unsafe { gl::GetUniformLocation(prog_id, cify_str("s_width")) };
        if width_loc != -1 {
            unsafe { gl::Uniform1i(width_loc, fb.width() as GLint) };
        }

        let height_loc = unsafe { gl::GetUniformLocation(prog_id, cify_str("s_height")) };
        if height_loc != -1 {
            unsafe { gl::Uniform1i(height_loc, fb.height() as GLint) };
        }

        // create shader draw instructions
        fb.draws().iter().reverse().enumerate().for_each(|i, di| {
            let di_name = format!("s_instructions[{}]", i);
            let kind_name = format!("{}.kind", di_name);
            let coords_name = format!("{}.coords", di_name);
            let color_name = format!("{}.color", di_name);
            let sdi: ShaderDrawInstruction = di.into();

            let kind_loc = unsafe { gl::GetUniformLocation(prog_id, cify_str(&kind_name)) };
            if kind_loc != -1 {
                unsafe { gl::Uniform1i(kind_loc, sdi.kind) };
            }

            let coords_loc = unsafe { gl::GetUniformLocation(prog_id, cify_str(&coords_name)) };
            if coords_loc != -1 {
                unsafe { gl::Uniform1fv(coords_loc, sdi.coords) };
            }

            let color_loc = unsafe { gl::GetUniformLocation(prog_id, cify_str(&color_name)) };
            if color_loc != -1 {
                unsafe { gl::Uniform1fv(colors_loc, sdi.color) };
            }
        });

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }

        /* ... */
    }
}
```

(Note: Tere are a lot of bottlenecks here that I should finish later)
