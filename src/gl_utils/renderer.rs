// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_utils/renderer.rs - Renderer based on SDL2 and OpenGL

use super::vertices::QUAD_VERTICES;
use crate::{Game, ImgTexture, LitError, Renderer};
use gl::types::{GLfloat, GLuint};
use nalgebra::{
    base::{Matrix3, Matrix4, Vector2, Point2},
    geometry::{Rotation3, Translation3},
};
use sdl2::{
    event::Event,
    video::{GLContext, GLProfile, Window},
    Sdl,
};
use std::{mem, os::raw::c_void};

pub struct GlRenderer {
    sdl_context: Sdl,
    window: Window,
    gl_context: GLContext,

    quad_vao: GLuint,
}

impl GlRenderer {
    // create a new GlRenderer
    pub fn init() -> Result<GlRenderer, LitError> {
        // create the SDL2 context
        let sdl_context = sdl2::init().map_err(|e| LitError::Msg(e))?;

        // access the video subsystem
        let video_context = sdl_context.video().map_err(|e| LitError::Msg(e))?;

        // set OpenGL options
        let gl_attr = video_context.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // create the window
        let window = video_context
            .window("Lost in Time", 800, 600)
            .opengl()
            .build()?;

        // create the OpenGL context
        let gl_context = window.gl_create_context().map_err(|e| LitError::Msg(e))?;
        let gl_item = gl::load_with(|s| video_context.gl_get_proc_address(s) as *const c_void);

        // initialize the sprite render process
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&QUAD_VERTICES) as isize,
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
                4 * mem::size_of::<GLFloat>() as i32,
                ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Ok(GlRenderer {
            sdl_context,
            window,
            gl_context,
            quad_vao: vao,
        })
    }
}

impl Renderer for GlRenderer {
    fn main_loop(&self, game: &Game) -> Result<(), LitError> {
        let mut event_pump = self
            .sdl_context
            .event_pump()
            .map_err(|e| LitError::Msg(e))?;

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

            self.window.gl_swap_window();
        }

        Ok(())
    }

    fn draw_sprite(
        &mut self,
        position: Point2<GLfloat>,
        size: Point2<GLfloat>,
        rotation: GLfloat,
    ) -> Result<(), LitError> {
        let mut model = Matrix3::<GLfloat>::identity();

        // shift position
        model *= Transition3::from_vector(Vector3::new(position.x(), position.y(), 0.0));

        // shift size
        model *= Transition3::from_vector(Vector3::new(0.5 * size.x(), 0.5 * size.y(), 0.0));

        // rotate matrix
        model *= Rotation3::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), rotation);
        model *= Transition3::from_vector(Vector3::new(-0.5 * size.x(), -0.5 * size.y(), 0.0));

        // resize with size
        model *= Matrix3::from_diagonal(Vector3::new(size.x(), size.y(), 1.0));

        Ok(())
    }
}
