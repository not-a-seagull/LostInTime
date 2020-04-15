// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture.rs - OpenGL texture

use super::{FrameBuffer, Program, Shader, ShaderType};
use crate::{
    draw::{translate_draw_instruction, DrawInstruction},
    utils::cify_str,
};
use gl::types::{GLfloat, GLint, GLuint, GLvoid};
use std::{ffi::c_void, mem, ptr};

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

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
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

#[repr(C)]
struct ShaderDrawInstruction {
    kind: GLuint,
    coords: [GLint; 4],
    color: [GLfloat; 4],
}

impl From<&DrawInstruction> for ShaderDrawInstruction {
    fn from(di: &DrawInstruction) -> ShaderDrawInstruction {
        let mut sdi = ShaderDrawInstruction {
            kind: translate_draw_instruction(di),
            coords: [0; 4],
            color: [0.0; 4],
        };

        match di {
            &DrawInstruction::Pixel { x, y, color } => {
                sdi.coords[0] = x as GLint;
                sdi.coords[1] = y as GLint;
                sdi.color = color.as_gl_color();
            }
            &DrawInstruction::Rectangle { x, y, w, h, color } => {
                sdi.coords[0] = x as GLint;
                sdi.coords[1] = y as GLint;
                sdi.coords[2] = w as GLint;
                sdi.coords[3] = h as GLint;
                sdi.color = color.as_gl_color();
            }
            &DrawInstruction::Square { x, y, l, color } => {
                sdi.coords[0] = x as GLint;
                sdi.coords[1] = y as GLint;
                sdi.coords[2] = l as GLint;
                sdi.coords[3] = l as GLint;
                sdi.color = color.as_gl_color();
            }
        }

        sdi
    }
}

const VERTICES: [GLfloat; 24] = [
    0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 0.0, 1.0, 0.0,
];

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

        // initialize VAO and VBO
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        unsafe {
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
        fb.draws().iter().rev().enumerate().for_each(|(i, di)| {
            let di_name = format!("s_instructions[{}]", i);
            let kind_name = format!("{}.kind", di_name);
            let coords_name = format!("{}.coords", di_name);
            let color_name = format!("{}.color", di_name);
            let sdi: ShaderDrawInstruction = di.into();

            let kind_loc = unsafe { gl::GetUniformLocation(prog_id, cify_str(&kind_name)) };
            if kind_loc != -1 {
                unsafe { gl::Uniform1ui(kind_loc, sdi.kind) };
            }

            let coords_loc = unsafe { gl::GetUniformLocation(prog_id, cify_str(&coords_name)) };
            if coords_loc != -1 {
                unsafe { gl::Uniform1iv(coords_loc, 4, sdi.coords.as_ptr()) };
            }

            let color_loc = unsafe { gl::GetUniformLocation(prog_id, cify_str(&color_name)) };
            if color_loc != -1 {
                unsafe { gl::Uniform1fv(color_loc, 4, sdi.color.as_ptr()) };
            }
        });

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }

        tex
    }
}
