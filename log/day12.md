# Day 12

It took some finagling with the `nalgebra` crate, but I've now found a way to determine the transformation to be applied to the sprite:

*Modifications to src/gl_utils/renderer.rs*

```rust
fn draw_sprite(
    &mut self,
    position: Point2<GLfloat>,
    size: Point2<GLfloat>,
    rotation: GLfloat,
) -> Result<(), LitError> {
    let mut transform = Matrix3::<GLfloat>::identity();

    // shift position
    transform += Matrix3::from_diagonal(&Vector3::new(position.x, position.y, 0.0));

    // shift size
    transform += Matrix3::from_diagonal(&Vector3::new(0.5 * size.x, 0.5 * size.y, 0.0));

    // rotate matrix
    transform *= Rotation3::from_axis_angle(&Unit::new_normalize(Vector3::z()), rotation);
    transform += Matrix3::from_diagonal(&Vector3::new(-0.5 * size.x, -0.5 * size.y, 0.0));

    // resize with size
    transform *= Matrix3::from_diagonal(&Vector3::new(size.x, size.y, 1.0));

    Ok(())
}
```

Now, we need to be able to add uniforms to the program that we end up loading here. In the past, we had the `assign_uniform` macro. However, it feels somewhat hacky. I think that it would be more prudent to give the `Program` struct a `SetUniform` function. This function could take a generic parameter, derived from a trait `UniformType` in order to call the proper function.

*In src/gl_utils/uniform.rs*

```rust
use crate::utils::cify_str;
use gl::types::GLuint;

macro_rules! assign_uniform {
    ($id: expr, $name: expr => $call: ident <= $($val: expr),*) => {
        {
            let loc = unsafe { gl::GetUniformLocation($id, cify_str($name)) };
            if loc != -1 {
                unsafe { gl::$call(loc, $($val),*) };
            }
        }
    }
}

pub trait Uniform {
    fn set_uniform(self, uname: &'static str, prog_id: GLuint);
}

impl Uniform for i32 {
    #[inline]
    fn set_uniform(self, uname: &'static str, prog_id: GLuint) {
        assign_uniform!(prog_id, uname => Uniform1i <= self);
    }
}

impl Uniform for [f32; 4] {
    #[inline]
    fn set_uniform(self, uname: &'static str, prog_id: GLuint) {
        let [f1, f2, f3, f4] = self;
        assign_uniform!(prog_id, uname => Uniform4f <= f1, f2, f3, f4);
    }
}
```

*Modifications to src/gl_utils/program.rs*

```rust
impl Program {
    /* ... */

    #[inline]
    pub fn set_uniform<T: Uniform>(&self, uname: &'static str, uniform: T) {
        uniform.set_uniform(uname, self.id());
    }
}
```

*Modifications to src/gl_utils/texture/render.rs*

```rust
// set up uniforms
TEXTURE_RENDERER.set_uniform("s_width", mat.width() as GLint);
TEXTURE_RENDERER.set_uniform("s_height", mat.height() as GLint);
TEXTURE_RENDERER.set_uniform("s_draw_len", mat.draws().len() as GLint);
TEXTURE_RENDERER.set_uniform(mat.background_color().as_gl_color());
```

While I was here, I realized that the imaage rendering code and the sprite drawing code had a lot of code overlap, since they both involve drawing quads. Therefore, I created a `Quad` struct that can take care of this.

*In src/gl_utils/quad.rs*

```rust
use super::vertices::QUAD_VERTICES;
use gl::types::{GLfloat, GLint, GLuint};
use std::{ffi::c_void, mem, ptr};

#[derive(Debug)]
pub struct Quad {
    vao: GLuint,
    _vbo: GLuint,
    has_been_bound: bool,
}

impl Quad {
    pub fn new() -> Self {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // fill the buffer with the quad vertices
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&QUAD_VERTICES) as isize,
                QUAD_VERTICES.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }

        Self {
            vao,
            _vbo: vbo,
            has_been_bound: false,
        }
    }

    pub fn bind(&mut self) {
        unsafe { gl::BindVertexArray(self.vao) };

        if !self.has_been_bound {
            unsafe {
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(
                    0,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    (4 * mem::size_of::<GLfloat>()) as GLint,
                    ptr::null(),
                );
            }

            self.has_been_bound = true;
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}
```

*Modifications to src/gl_utils/texture/render.rs*

```rust
unsafe { gl::Viewport(0, 0, mat.width() as GLint, mat.height() as GLint) };

let mut quad = Quad::new();
quad.bind(); 

TEXTURE_RENDERER.activate();

// set up uniforms
TEXTURE_RENDERER.set_uniform("s_width", mat.width() as GLint);
TEXTURE_RENDERER.set_uniform("s_height", mat.height() as GLint);
TEXTURE_RENDERER.set_uniform("s_draw_len", mat.draws().len() as GLint);
TEXTURE_RENDERER.set_uniform("bg_color", mat.background_color().as_gl_color());

// bind the DI buffer to the context
let di_buffer = mat.buffer().unwrap();
di_buffer.bind();

quad.draw();
quad.unbind();
```

Now, we just need to add matrix support for this. We can do this for the `nalgebra` `Matrix4` class, with a similar trait.

*Modifications to src/gl_utils/uniform.rs*

```rust
impl Uniform for Matrix4<f32> {
    #[inline]
    fn set_uniform(self, uname: &'static str, prog_id: GLuint) {
        assign_uniform!(prog_id, uname => Uniform4fv <= 1, self.as_ptr());
    }
}

impl Uniform for Transform3<f32> {
    #[inline]
    fn set_uniform(self, uname: &'static str, prog_id: GLuint) {
        self.into_inner().set_uniform(uname, prog_id)
    }
}
```

*Modifications to src/gl_utils/renderer.rs*

```rust
impl GlRenderer {
    // create a new GlRenderer
    pub fn init() -> Result<GlRenderer, LitError> {
        /* ... */
 
        let mut quad = Quad::new();
        quad.bind();
        quad.unbind();

        Ok(GlRenderer {
            sdl_context,
            window,
            gl_context,
            quad,
        })
    }
}
```
