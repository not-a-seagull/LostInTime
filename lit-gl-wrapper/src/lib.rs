// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// lit-gl-wrapper/lib.rs - OpenGL wrapper library

pub use gl;

mod error;
pub use error::*;

mod frame_buffer;
pub use frame_buffer::FrameBuffer;

mod gl_renderer;
pub use gl_renderer::*;

mod renderer;
pub use renderer::Renderer;

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

pub mod utils;
pub mod vertices;
