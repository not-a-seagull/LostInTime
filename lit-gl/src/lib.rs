// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// lit-gl/src/lib.rs - Container for OpenGL bindings

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
