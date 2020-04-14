# Day 5

Today, I wanted to correct some of the bad decisions I made before, before they become too badly ingrained into the program.

The first of these bad decisions was to make the parser state a global object. This would make copying common, which can slow down the program and increase memory consumption. In order to get around this, I had to rewrite the Display trait implementation with the much less slick:

```
pub fn stringify(&self, state: &ParserState) -> Result<String, LitError> {
    match self {
        &BytecodeObject::Numeric8(u) => Ok(format!("{}", u)),
        &BytecodeObject::Numeric16(u) => Ok(format!("{}", u)),
        &BytecodeObject::Numeric32(u) => Ok(format!("{}", u)); 
        &BytecodeObject::Str(ref u) => Ok(format!("{}", u)),
        &BytecodeObject::Tuple(ref s) => Ok(format!("{:?}", s)),
        &BytecodeObject::VarInvocation(u) => {
            Self::stringify(state.get_variable(u)?, state)
        }
    }
}
```

This may cause problems down the road (for instance, I think this means that we can't really put a `BytecodeObject` in the game state). For the time being, it's the most stable implementation.

The second is representing a command as a single byte. This gives us only 255 commands to create, which we may exceed in the future. To get around this, we must modify both the compiler and the interpreter to read 16 bits instead of 32.

*Modifications to src/script/eval.rs*

```
// read a single word from the stream
let mut buffer = [0; 2];
stream.read(&mut buffer)?;

match u16::from_be_bytes(&buffer) {
  /* ... */
}
```

*Modifications to lits-cc/src/command.rs*

```rust
#[inline]
fn write_word<T: Write>(stream: &mut T, word: u16) -> Result<(), LitsCcError> {
    let bytes = word.to_be_bytes();
    stream.write(&bytes)?;
    Ok(())
}

pub fn process_command<TStream: Write, TIter: Iterator<Item = TokenTree>>(
    ident: &Ident,
    iter: &mut TIter,
    stream: &mut TStream,
    state: &mut CompilerState,
) -> Result<(), LitsCcError> {
    let name = format!("{}", ident);

    match name.as_ref() {
        "gamedef" => {
            write_word(stream, 1)?;
            Ok(())
        }
        "def" => {
            write_word(stream, 2)?;
 
            /* ... */
        }
        "log" => {
            write_word(stream, 3)?;
            Ok(())
        }
        _ => Err(LitsCcError::UnknownCommand(name)),
    }
}
```

This seems to work properly, and should allow us to implement commands without worrying about hitting the limit.

Before we move on, I realized that you can hardware-accelerate drawing to textures. I also wanted to try to keep the program's RAM footprint low. I decided that it would be helpful to cache the drawing instructions in a struct, and then "expand" these drawing instructions to create the textures. 

However, to write hardware-accelerated code, we need to have a way of compiling it. To do that, we'll need to write objects to take care of that.

*In src/gl_utils/shader.rs*

```rust
use crate::LitError;
use gl::types::{GLchar, GLint, GLuint};
use std::{io::prelude::*, ffi::CString, path::Path, ptr};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new<T: Read>(stream: &mut T, kind: ShaderType) -> Result<Self, LitError> {
        // read the entire file into the string
        let mut source = String::new();
        stream.read_to_string(&mut source)?;
        let source = CString::new(source).unwrap();

        let id = unsafe { gl::CreateShader(kind as u32) };

        // process the source
        unsafe { gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null()) };
        unsafe { gl::CompileShader(id) };

        // check for errors
        let mut success: GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) };

        // if there is an error, report it
        if success == 0 {
            let mut err_len: GLint = 0;
            unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut err_len) };
            let buffer = crate::utils::create_cstring_buffer(err_len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, err_len, ptr::null_mut(), buffer.as_ptr() as *mut GLchar)
            };

            Err(LitError::Msg(buffer.to_string_lossy().into_owned()))
        } else {
            Ok(Self { id })
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}
```

*In src/gl_utils/program.rs*

```rust
use super::Shader;
use crate::LitError;
use gl::types::{GLchar, GLint, GLuint};
use std::{io::prelude::*, ptr};

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn new<T: Read>(shaders: &[Shader]) -> Result<Self, LitError> {
        // get the id
        let id = unsafe { gl::CreateProgram() };

        // attach every shader in the collection
        shaders
            .iter()
            .for_each(|s| unsafe { gl::AttachShader(id, s.id()) });

        // link together the program
        unsafe { gl::LinkProgram(id) };

        let mut success: GLint = 1;
        // test for errors
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut err_len: GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut err_len) };

            let buffer = crate::utils::create_cstring_buffer(err_len as usize);
            unsafe {
                gl::GetProgramInfoLog(id, err_len, ptr::null_mut(), buffer.as_ptr() as *mut GLchar)
            };

            return Err(LitError::Msg(buffer.to_string_lossy().into_owned()));
        }

        // detach every shader
        shaders
            .iter()
            .for_each(|s| unsafe { gl::DetachShader(id, s.id()) });

        Ok(Self { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}
```

*In src/utils.rs*

```rust
use std::ffi::CString;

pub fn create_cstring_buffer(len: usize) -> CString {
    let mut buffer = Vec::with_capacity(len);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
```
