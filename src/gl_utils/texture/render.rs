// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/texture/render.rs - Convert a frame buffer to a texture.

use super::{
    super::{vertices::QUAD_VERTICES, FrameBuffer, Program, Quad, Shader, ShaderType},
    DIBuffer, ImgMaterial, ImgTexture, Texture,
};
use crate::{draw::DrawInstruction, utils::cify_str, LitError, Resource, ResourceDictionary};
use gl::types::{GLfloat, GLint, GLuint, GLvoid};
use std::{collections::HashMap, convert::TryFrom, ffi::c_void, mem, ptr};

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

    fn load(mat: &ImgMaterial) -> Result<Self, LitError> {
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

        di_buffer.unbind();
        fb.unbind();

        Ok(tex)
    }
}
