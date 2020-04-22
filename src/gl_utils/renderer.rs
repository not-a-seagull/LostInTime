// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/renderer.rs - Renderer based on SDL2 and OpenGL

use super::{Program, Quad, Shader, ShaderType};
use crate::{Game, ImgTexture, LitError, Renderer};
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
use std::os::raw::c_void;

// sprite renderer program
lazy_static::lazy_static! {
    static ref SPRITE: Program = {
        let vert_source = include_str!("./shaders/sprite.vert");
        let frag_source = include_str!("./shaders/sprite.frag");
        let vert = Shader::from_source(&vert_source, ShaderType::Vertex)
            .expect("Vertex shader compilation failed");
        let frag = Shader::from_source(&frag_source, ShaderType::Fragment)
            .expect("Fragment shader compilation failed");
        Program::new(&[vert, frag]).expect("Shader linking failed")
    };
}

pub struct GlRenderer {
    sdl_context: Sdl,
    window: Window,
    _gl_context: GLContext,

    quad: Quad,
}

impl GlRenderer {
    // create a new GlRenderer
    pub fn init() -> Result<GlRenderer, LitError> {
        const WIDTH: u32 = 800;
        const HEIGHT: u32 = 600;

        // create the SDL2 context
        let sdl_context = sdl2::init().map_err(LitError::Msg)?;

        // access the video subsystem
        let video_context = sdl_context.video().map_err(LitError::Msg)?;

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
        let gl_context = window.gl_create_context().map_err(LitError::Msg)?;
        gl::load_with(|s| video_context.gl_get_proc_address(s) as *const c_void);

        // initialize the sprite render process
        let ortho =
            Orthographic3::<GLfloat>::new(0.0, WIDTH as GLfloat, HEIGHT as GLfloat, 0.0, -1.0, 1.0);
        SPRITE.set_uniform("ortho", ortho.into_inner());

        let mut quad = Quad::new();
        quad.bind();
        quad.unbind();

        Ok(GlRenderer {
            sdl_context,
            window,
            _gl_context: gl_context,
            quad,
        })
    }
}

impl Renderer for GlRenderer {
    fn main_loop(&mut self, mut game: Game) -> Result<(), LitError> {
        let mut event_pump = self
            .sdl_context
            .event_pump()
            .map_err(LitError::Msg)?;

        // set clear color
        unsafe { gl::ClearColor(1.0, 1.0, 1.0, 1.0) };

        // main loop
        'lit: loop {
            for event in event_pump.poll_iter() {
                // process the event
                match event {
                    Event::Quit { .. } => break 'lit,
                    _ => {}
                }
            }

            unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

            self.draw_sprite(
                game.get_resource::<ImgTexture>(0)?,
                Point2::new(40.0, 40.0),
                Point2::new(100.0, 100.0),
                0.0,
            )?;

            self.window.gl_swap_window();
        }

        Ok(())
    }

    fn draw_sprite(
        &mut self,
        img: &ImgTexture,
        position: Point2<GLfloat>,
        size: Point2<GLfloat>,
        rotation: GLfloat,
    ) -> Result<(), LitError> {
        SPRITE.activate();

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

        SPRITE.set_uniform("transf", transform);

        unsafe { gl::ActiveTexture(gl::TEXTURE0) };
        img.bind();

        self.quad.bind();
        self.quad.draw();
        self.quad.unbind();

        Ok(())
    }
}
