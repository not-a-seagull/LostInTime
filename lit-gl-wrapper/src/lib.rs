// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// lit-gl-wrapper/lib.rs - OpenGL wrapper library

extern crate gl;
pub use gl;

mod error;
pub use error::*;

mod frame_buffer;
pub use frame_buffer::FrameBuffer;

mod renderer;
pub use renderer::GlRenderer;

mod quad;
pub use quad::Quad;

mod shader;
pub use shader::{Shader, ShaderType};

mod texture;
pub use texture::*;

mod uniform;
pub use uniform::*;

mod program;
pub use program::Program;

pub mod vertices;
