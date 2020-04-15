// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture/fb_to_tex.rs - Convert a frame buffer to a texture.

use super::{
    super::{FrameBuffer, Program, Shader, ShaderType},
    DIBuffer, ImgTexture, Texture,
};
use crate::{draw::DrawInstruction, utils::cify_str, LitError};
use gl::types::{GLfloat, GLint, GLuint, GLvoid};
use std::{convert::TryFrom, ffi::c_void, mem, ptr};

lazy_static::lazy_static! {
    static ref TEXTURE_RENDERER: Program = {
        let vert_source = include_str!("../shaders/fb_to_texture.vert");
        let frag_source = include_str!("../shaders/fb_to_texture.frag");
        let vert = Shader::from_source(&vert_source, ShaderType::Vertex)
            .expect("Vertex shader compilation failed");
        let frag = Shader::from_source(&frag_source, ShaderType::Fragment)
            .expect("Fragment shader compilation failed");

        Program::new(&[vert, frag]).expect("Shader linking failed")
    };
}

const VERTICES: [GLfloat; 24] = [
    0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 0.0, 1.0, 0.0,
];

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

impl TryFrom<FrameBuffer> for ImgTexture {
    type Error = LitError;

    fn try_from(fb: FrameBuffer) -> Result<ImgTexture, LitError> {
        // create the texture
        let tex = Texture::from_raw(&[fb.width(), fb.height()], ptr::null())?;

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

        // initialize VAO and VBO
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        unsafe {
            gl::Viewport(0, 0, fb.width() as GLint, fb.height() as GLint);
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
        assign_uniform!("s_width" => Uniform1i <= fb.width() as GLint);
        assign_uniform!("s_height" => Uniform1i <= fb.height() as GLint);
        assign_uniform!("s_draw_len" => Uniform1i <= fb.draws().len() as GLint);

        let bg_clr = fb.background_color().as_gl_color();
        assign_uniform!("bg_color" => Uniform4f <= bg_clr[0], bg_clr[1], bg_clr[2], bg_clr[3]);

        // build list of data for texture
        let mut draws: Vec<GLint> = vec![];
        fb.draws()
            .iter()
            .rev()
            .for_each(|d| draws.extend(&d.as_int_set()));

        // build a 1-dimensional texture for this instance
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
