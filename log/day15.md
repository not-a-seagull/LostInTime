# Day 15

I have had the sudden realization that it may be responsible to seperate my OpenGL implementation from my actual game. The two types of code have become intertwined, and the GL implementation now takes up most of the game code. We should fix that.

```bash
$ cargo new --vcs none --lib lit-gl-wrapper
$ mv src/gl_utils/* lit-gl-wrapper/src/
```

Secondly, it is useful to generate our own OpenGL bindings. Debugging was quite difficult in our implementation; it would be useful if we could build error output into our code proper.

```bash
$ cargo new --vcs none --lib lit-gl
```

Now, we can set up automatic bindings to the OpenGL runtime.

*In lit-gl/src/lib.rs*

```rust
use std::{ops::Deref, rc::Rc};

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/gl.rs"));
}

pub use bindings::*;

pub use bindings::Gl as GlContext;

/// Custom shared reference to the OpenGL context.
#[derive(Clone)]
pub struct Gl {
    inner: Rc<GlContext>,
}

impl Gl {
    pub fn load_with<F>(loader: F) -> Self
    where
        F: FnMut(&'static str) -> *const types::GLvoid,
    {
        Self {
            inner: Rc::new(GlContext::load_with(loader)),
        }
    }
}

impl Deref for Gl {
    type Target = GlContext;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
```

*In lit-gl/build.rs*

```rust
use gl_generator::{Api, DebugStructGenerator, Fallbacks, Profile, Registry, StructGenerator};
use std::{env, fs::File, path::Path};

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut outfile = File::create(&Path::new(&dest).join("gl.rs")).unwrap();

    let registry = Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, []);

    if env::var("PROFILE").unwrap() == "debug" {
        registry
            .write_bindings(DebugStructGenerator, &mut outfile)
            .unwrap();
    } else {
        registry
            .write_bindings(StructGenerator, &mut outfile)
            .unwrap();
    }
}
```

Now that we're using a struct-based context, let's go around and fix it for the `lit-gl-wrapper` crate. First, I created a real `GlError` struct.
