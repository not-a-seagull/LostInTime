# Day 1

Today, I wanted to create a video game.

It's an idea I've been throwing around in my head for some time now: "Lost in Time". A game about time travel, where I aim to make fun of typical time travel tropes, and all around create a fun experience. Before we refine some of the game's details, it would be nice to have an engine, so I know my limitations.

Let's crate the base for the game. I'll be using Rust for its speed and general intuitiveness.

```
$ cargo new --bin lost_in_time
$ cd lost_in_time
```

Now that we've got a package, let's set up some of the dependencies we'll be using. For now, I will use the `sdl2` and `gl` packages for their GUI capacities, and the `nalgebra` package for its implementation of linear algebra.

With Rust's package manager, Cargo, this is relatively easy to set up.

```
[dependencies]
sdl2 = "0.33.0"
gl = "0.14.0"
nalgebra = "0.21.0"
```

Before anything else, I'll set up some of the basic structures behind the game. Although I will be using SDL and OpenGL for the initial implementation of the game, I'd like to be able to port it elsewhere with minimal code rewriting. Thus, any rendering interface should be able to be abstracted.

Rust allows us to do this with traits.

*In src/renderer.rs*

```
use crate::Game;

pub trait Renderer {
    fn main_loop(&self, game: &Game);
}
```

*In src/game.rs*

```
pub struct Game {
    // TODO: objects representative of the game's state
}
```

*In src/main.rs*

```
mod game;
mod renderer;

pub use game::Game;
pub use renderer::Renderer;

use std::process;

fn main() {
    process::exit(classic_main());
}

fn classic_main() -> i32 {
    let game = Game { };

    // TODO: create instance of renderer

    0
}
```

This compiles; albeit, it does nothing. Let's create a basic instance of an SDL renderer.

*In src/gl_renderer.rs*

```
use crate::{Game, Renderer};
use sdl2::{event::Event, video::Window, Sdl};

pub struct GlRenderer {
    sdl_context: Sdl,
    window: Window,
}

impl GlRenderer {
    // create a new GlRenderer
    pub fn init() -> GlRenderer {
        // create the SDL2 context
        let sdl_context = sdl2::init().unwrap();

        // access the video subsystem
        let video_context = sdl_context.video().unwrap();

        // create the window
        let window = video_context
            .window("Lost in Time", 800, 600)
            .build()
            .unwrap();

        GlRenderer {
            sdl_context,
            window,
        }
    }
}

impl Renderer for GlRenderer {
    fn main_loop(&self, game: &Game) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        // main loop
        'lit: loop {
            for event in event_pump.poll_iter() {
                // process the event
                match event {
                    Event::Quit { .. } => break 'lit,
                    _ => {}
                }
            }
        }
    }
}

```

*Modifications to src/main.rs*

```
mod game;
mod gl_renderer;
mod renderer;

/* ... */

use gl_renderer::GlRenderer;

/* ... */

fn classic_main() -> i32 {
    let game = Game {};
    let renderer = GlRenderer::init();

    renderer.main_loop(&game);

    0
}
```

This successfully displays a blank black window, with the title "Lost in Time".

Before we move on from boilerplate, I'd like to point out the liberal use of the `unwrap` function in `GlRenderer`. Personally, I don't like using `unwrap`, since it means that any error can panic the entire program without recovery. I'd let to set up a basic error handling system.

First, I download the `thiserror` crate, which reduces some of the boilerplate involved in setting up error handling.

```
[dependencies]
thiserror = "1"
```

This makes it relatively easy to set up the `LitError` enum.

*In src/error.rs*

```
use sdl2::video::WindowBuildError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LitError {
    #[error("{0}")]
    Msg(String),
    #[error("{0}")]
    StaticMsg(&'static str),
    #[error("An error occurred while building the SDL2 window: {0}")]
    WindowBuildError(#[from] WindowBuildError),
}
```

*In src/gl_renderer.rs*

```
use crate::{Game, LitError, Renderer};
use sdl2::{event::Event, video::Window, Sdl};

pub struct GlRenderer {
    sdl_context: Sdl,
    window: Window,
}

impl GlRenderer {
    // create a new GlRenderer
    pub fn init() -> Result<GlRenderer, LitError> {
        // create the SDL2 context
        let sdl_context = sdl2::init().map_err(|e| LitError::Msg(e))?;

        // access the video subsystem
        let video_context = sdl_context.video().map_err(|e| LitError::Msg(e))?;

        // create the window
        let window = video_context.window("Lost in Time", 800, 600).build()?;

        Ok(GlRenderer {
            sdl_context,
            window,
        })
    }
}

impl Renderer for GlRenderer {
    fn main_loop(&self, game: &Game) -> Result<(), LitError> {
        let mut event_pump = self
            .sdl_context
            .event_pump()
            .map_err(|e| LitError::Msg(e))?;

        // main loop
        'lit: loop {
            for event in event_pump.poll_iter() {
                // process the event
                match event {
                    Event::Quit { .. } => break 'lit,
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
```

*Modifications to src/renderer.rs*

```
fn main_loop(&self, game: &Game) -> Result<(), LitError>;
```

*Modifications to src/main.rs*

```
mod error;

/* ... */

pub use error::LitError;

/* ... */

fn main() {
    process::exit(match classic_main() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("A fatal error occurred: {:?}", e);
            1
        }
    });
}

fn classic_main() -> Result<(), LitError> {
    let game = Game {};
    let renderer = GlRenderer::init()?;

    renderer.main_loop(&game)
}
```

Finally, we might as well use OpenGL as a way of giving some color to the bleak window I've generated. We can modify `gl_renderer.rs` to express this.

*Modifications to src/gl_renderer.rs*

```
pub struct GlRenderer {
    sdl_context: Sdl,
    window: Window,
    gl_context: GLContext,
}

/* Inside of GlRenderer::init() */
// set OpenGL options
let gl_attr = video_context.gl_attr();
gl_attr.set_context_profile(GLProfile::Core);
gl_attr.set_context_version(3, 3);


// create the OpenGL context
let gl_context = window.gl_create_context().map_err(|e| LitError::Msg(e))?;
let gl_item = gl::load_with(|s| video_context.gl_get_proc_address(s) as *const c_void);

Ok(GlRenderer {
    sdl_context,
    window,
    gl_context,
})

/* Inside of main_loop() */
// set clear color
unsafe { gl::ClearColor(1.0, 1.0, 1.0, 1.0) };

/* Inside of the "'lit loop" */
unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

self.window.gl_swap_window();
```

We now have a white window; a blank canvas, ready to be filled.
