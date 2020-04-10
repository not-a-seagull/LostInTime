// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// gl_renderer.rs - Renderer based on SDL2 and OpenGL

use crate::{Game, LitError, Renderer};
use sdl2::{
    event::Event,
    video::{GLContext, GLProfile, Window},
    Sdl,
};
use std::os::raw::c_void;

pub struct GlRenderer {
    sdl_context: Sdl,
    window: Window,
    gl_context: GLContext,
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

        Ok(GlRenderer {
            sdl_context,
            window,
            gl_context,
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
}
