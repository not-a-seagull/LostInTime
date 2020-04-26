// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// lit-gl-wrapper/src/gl_renderer.rs - Renderer based on SDL2 and OpenGL

use super::{Program, Quad, Shader, ShaderType};
use crate::{GlError, ImgTexture, Renderer};
use gl::types::GLfloat;
use nalgebra::{
    base::{Matrix4, Unit, Vector3, Vector4},
    geometry::{Orthographic3, Point2, Rotation3, Transform3, Translation3},
};
use sdl2::{
    event::Event,
    video::{GLContext, GLProfile, Window},
    Sdl,
};
use std::{boxed::Box, os::raw::c_void};

pub struct GlRenderer {
    sdl_context: Sdl,
    window: Window,
    _gl_context: GLContext,
    gl: gl::Gl,
    sprite: Program,

    quad: Quad,
}

impl GlRenderer {
    // create a new GlRenderer
    pub fn init() -> Result<GlRenderer, GlError> {
        const WIDTH: u32 = 800;
        const HEIGHT: u32 = 600;

        // create the SDL2 context
        let sdl_context = sdl2::init().map_err(GlError::Msg)?;

        // access the video subsystem
        let video_context = sdl_context.video().map_err(GlError::Msg)?;

        // set OpenGL options
        let gl_attr = video_context.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // create the window
        let window = video_context
            .window("Lost in Time", WIDTH, HEIGHT)
            .opengl()
            .build()?;

        // create the OpenGL context
        let gl_context = window.gl_create_context().map_err(GlError::Msg)?;
        let gl = gl::Gl::load_with(|s| video_context.gl_get_proc_address(s) as *const c_void);

        let sprite = {
            let vert_source = include_str!("./../../shaders/sprite.vert");
            let frag_source = include_str!("./../../shaders/sprite.frag");
            let vert = Shader::from_source(&gl, &vert_source, ShaderType::Vertex)
                .expect("Vertex shader compilation failed");
            let frag = Shader::from_source(&gl, &frag_source, ShaderType::Fragment)
                .expect("Fragment shader compilation failed");
            Program::new(&gl, &[vert, frag]).expect("Shader linking failed")
        };

        // initialize the sprite render process
        let ortho =
            Orthographic3::<GLfloat>::new(0.0, WIDTH as GLfloat, HEIGHT as GLfloat, 0.0, -1.0, 1.0);
        sprite.activate()?;
        sprite.set_uniform("ortho", ortho.into_inner())?;

        let quad = Quad::new(&gl)?;
        quad.bind(true)?;
        quad.unbind()?;

        Ok(GlRenderer {
            sdl_context,
            window,
            _gl_context: gl_context,
            quad,
            sprite,
            gl,
        })
    }
}

impl Renderer for GlRenderer {
    type Error = GlError;

    fn main_loop<F>(&self, mut loop_function: F) -> Result<(), GlError>
    where
        F: FnMut(&Self) -> Result<(), Box<dyn std::error::Error>>,
    {
        let mut event_pump = self.sdl_context.event_pump().map_err(GlError::Msg)?;

        // set clear color
        unsafe { self.gl.ClearColor(1.0, 1.0, 1.0, 1.0) };

        // main loop
        'lit: loop {
            for event in event_pump.poll_iter() {
                // process the event
                match event {
                    Event::Quit { .. } => break 'lit,
                    _ => {}
                }
            }

            unsafe { self.gl.Clear(gl::COLOR_BUFFER_BIT) };

            loop_function(self).map_err(GlError::GenericError)?;

            self.window.gl_swap_window();
        }

        Ok(())
    }

    fn draw_sprite(
        &self,
        img: &ImgTexture,
        position: Point2<GLfloat>,
        size: Point2<GLfloat>,
        rotation: GLfloat,
    ) -> Result<(), GlError> {
        self.sprite.activate()?;

        let mut transform = Transform3::<GLfloat>::identity();

        // shift position
        transform *= Translation3::from(Vector3::new(position.x, position.y, 0.0));

        // shift size
        transform *= Translation3::from(Vector3::new(0.5 * size.x, 0.5 * size.y, 0.0));

        // rotate matrix
        transform *= Rotation3::from_axis_angle(&Unit::new_normalize(Vector3::z()), rotation);
        transform *= Translation3::from(Vector3::new(-0.5 * size.x, -0.5 * size.y, 0.0));

        // resize with size
        transform *= Transform3::from_matrix_unchecked(Matrix4::from_diagonal(&Vector4::new(
            size.x, size.y, 1.0, 1.0,
        )));

        self.sprite.set_uniform("transf", transform)?;

        unsafe { self.gl.ActiveTexture(gl::TEXTURE0) };
        img.bind()?;

        self.quad.bind(false)?;
        self.quad.draw()?;
        self.quad.unbind()?;

        Ok(())
    }
}
