# Day 2

For storage of game data, I am planning on using a scripting format. Similar to DOOM WADs, this format will be loaded into the LostInTime program to play the game. However, rather than containing game data that the program reads, this data file will contain instructions on how to construct the game data. In a way, this will make the LostInTime program an interpreter.

What I'm forseeing is that I will have a source file containing something similar to the following contents:

```
gamedef "Lost in Time"                # define this game as "Lost in Time"
def MY_VAR 1                          # define MY_VAR as being equal to one
log "MY_VAR is equal to {}" (@MY_VAR) # print "MY_VAR is equal to 1" to the console
create_img SPRITE 20 20               # create an image named "SPRITE" that is
                                      # 20 pixels by 20 pixels
```

This will be compiled to bytecode to the tune of (in terms of decimal numbers):

```
01 03 11 76 111 115 32 105 110 32 84 105 109 101
02 01 01 01
03 03 21 77 89 95 86 65 82 32 105 115 32 101 113 117 97 108 32 116 111 32 123 125 04 01 02 01
04 02 20 20
```

(I'll explain my rationalle for this later).

First, it would be wise to build an interpreter for this bytecode. The first step in this would be to build a way to read objects from bytecode. To this end, I've created a "Bytecode" trait.

*Modifications to src/error.rs*

```rust
use std::{
   io::Error as IoError,
   string::FromUtf8Error,
};

/* Inside of the LitError enum */
#[error("Unexpected byte while reading bytecode: {0:X?}")]
BytecodeRead8(u8),
#[error("Unexpected word while reading bytecode: {0:X?}")]
BytecodeRead16(u16),
#[error("Unexpected dword while reading bytecode: {0:X?}")]
BytecodeRead32(u32),
#[error("An IO error occurred: {0:?}")]
Io(#[from] IoError),
#[error("Error converting from UTF-8: {0:?}")]
FromUtf8(#[from] FromUtf8Error),
```

*Inside of src/script/bytecode.rs*

```
use crate::LitError;
use std::io::prelude::*;

pub trait Bytecode: Sized {
    fn read<T: Read>(stream: &mut T) -> Result<Self, LitError>;
}
```

*Inside of src/script/mod.rs*

```
mod bytecode;
pub use bytecode::Bytecode;
```

*Modifications to src/main.rs*

```
mod script;
```

This compiles. Now, it's time to add some types. For now, we'll add this set of types:

* **8-bit numerical literals** - Numbers between 0 and 255
* **16-bit numerical literals** - Numbers between -32767 and 32767
* **32-bit numerical literals** - Numbers between -2147483647 and 2147483647
* **Strings** - An array of UTF-8 text.
* **Tuples** - A set of other data types.
* **Variable Invocations** - Using a type from a variable defined at interpretation time.

We can implement most of these:

*Inside of src/script/types.rs*

```
use super::Bytecode;
use crate::LitError;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Numeric8,
    Numeric16,
    Numeric32,
    Str,
    Tuple,
}

pub enum BytecodeObject {
    Numeric8(u8),
    Numeric16(i16),
    Numeric32(i32),
    Str(String),
    Tuple(Vec<BytecodeObject>),
    VarInvocation(i32, DataType),
}

impl BytecodeObject {
    #[inline]
    pub fn data_type(&self) -> DataType {
        match self {
            &BytecodeObject::Numeric8(_) => DataType::Numeric8,
            &BytecodeObject::Numeric16(_) => DataType::Numeric16,
            &BytecodeObject::Numeric32(_) => DataType::Numeric32,
            &BytecodeObject::Str(_) => DataType::Str,
            &BytecodeObject::Tuple(_) => DataType::Tuple,
            &BytecodeObject::VarInvocation(_, dt) => dt,
        }
    }
}

impl Bytecode for BytecodeObject {
    fn read<T: Read>(stream: &mut T) -> Result<Self, LitError> {
        // variable type is signified by an 8-bit number
        let mut buffer = [0; 1];
        stream.read(&mut buffer)?;

        // determine which variable to read further
        match buffer[0] {
            1 => {
                // 8-bit numerical value
                stream.read(&mut buffer)?;
                Ok(BytecodeObject::Numeric8(buffer[0]))
            }
            2 => {
                // 16-bit numerical value
                let mut buffer = [0; 2];
                stream.read(&mut buffer)?;
                let val = i16::from_be_bytes(buffer);
                Ok(BytecodeObject::Numeric16(val))
            }
            3 => {
                // 32-bit numerical value
                let mut buffer = [0; 4];
                stream.read(&mut buffer)?;
                let val = i32::from_be_bytes(buffer);
                Ok(BytecodeObject::Numeric32(val))
            }
            4 => {
                // UTF-8 string
                // first, get the length
                stream.read(&mut buffer)?;

                // then, read into buffer
                let mut buffer = vec![0; buffer[0] as usize];
                stream.read_exact(&mut buffer)?;

                // finally, convert the buffer to a string
                let val = String::from_utf8(buffer)?;
                Ok(BytecodeObject::Str(val))
            }
            5 => {
                // tuple
                // first, get the length of the tuple
                stream.read(&mut buffer)?;

                // then, read an element for each in the length
                let element_num = buffer[0];
                let mut buffer = Vec::with_capacity(element_num as usize);

                for _ in 0..element_num {
                    buffer.push(BytecodeObject::read(stream)?);
                }

                Ok(BytecodeObject::Tuple(buffer))
            }
            6 => {
                // variable invocation
                // consists of the ID, which is a 4-byte number
                let mut buffer = [0; 4];
                stream.read(&mut buffer)?;
                let val = u32::from_be_bytes(buffer);
                Ok(BytecodeObject::VarInvocation(val))
            }
            _ => Err(LitError::BytecodeRead8(buffer[0])),
        }
    }
}
```

*Modifications to src/script/mod.rs*

```
mod types;
pub use types::{BytecodeObject, DataType};
```

Now that we have a good way of reading objects from bytecode, we now need to be able to evaluate them as their interior components. However, we need to be able to also evaluate those variables. For that, we need to create an object to represent the state of the parser.

*Modifications to src/script/mod.rs*

```
use std::collections::HashMap;
use super::LitError;

pub struct ParserState {
    variables: HashMap<u32, BytecodeObject>,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn register_variable(&mut self, index: u32, object: BytecodeObject) {
        self.variables.insert(index, object);
    }

    pub fn get_variable(&self, index: u32) -> Result<&BytecodeObject, LitError> {
        self.variables.get(&index).ok_or_else(|| LitError::VariableNotFound(index))
    }
}
```

*Modifications to src/script/types.rs*

```rust
impl BytecodeObject {
    #[inline]
    pub fn data_type(&self, state: &ParserState) -> DataType {
        match self {
            &BytecodeObject::Numeric8(_) => DataType::Numeric8,
            &BytecodeObject::Numeric16(_) => DataType::Numeric16,
            &BytecodeObject::Numeric32(_) => DataType::Numeric32,
            &BytecodeObject::Str(_) => DataType::Str,
            &BytecodeObject::Tuple(_) => DataType::Tuple,
            &BytecodeObject::VarInvocation(i) => state.get_variable(i).unwrap().data_type(state),
        }
    }

    pub fn as_number(&self, state: &ParserState) -> Result<i32, LitError> {
        match self {
            &BytecodeObject::Numeric8(v) => Ok(v as i32),
            &BytecodeObject::Numeric16(v) => Ok(v as i32),
            &BytecodeObject::Numeric32(v) => Ok(v),
            &BytecodeObject::VarInvocation(i) => {
                let val = state.get_variable(i)?;
                val.as_number(state)
            },
            _ => Err(LitError::ExpectedNumericalDataType(self.data_type(state))), 
        }
    }

    pub fn as_string<'a>(&'a self, state: &'a ParserState) -> Result<&'a str, LitError> {
        match self {
            &BytecodeObject::Str(ref s) => Ok(s),
            &BytecodeObject::VarInvocation(i) => state.get_variable(i)?.as_string(state),
            _ => Err(LitError::IncorrectDataType(self.data_type(state), DataType::Str)),
        }
    }

    pub fn as_tuple<'a>(&'a self, state: &'a ParserState) -> Result<&'a [BytecodeObject], LitError> {
        match self {
            &BytecodeObject::Tuple(ref t) => Ok(t),
            &BytecodeObject::VarInvocation(i) => state.get_variable(i)?.as_tuple(state),
            _ => Err(LitError::IncorrectDataType(self.data_type(state), DataType::Tuple)),
        }
    }
}
```

*Note: I'm going to be using big endian format for this.*

Now that we have a good way of getting the values we need from the bytecode objects, it's time to write code that reads and then evaluates statements.

*Inside of src/script/eval.rs*

```rust
use super::{Bytecode, BytecodeObject, GameData, ParserState};
use crate::LitError;
use std::io::prelude::*;

pub fn eval<T: Read>(
    stream: &mut T,
    state: &mut ParserState,
    data: &mut GameData,
) -> Result<(), LitError> {
    // read a single byte from the stream
    let mut buffer = [0; 1];
    stream.read(&mut buffer)?;

    match buffer[0] {
        1 => {
            // gamedef statement, define the game's name
            data.set_name(String::from(
                BytecodeObject::read(stream)?.as_string(state)?,
            ));
            Ok(())
        }
        2 => {
            // def statement, define a runtime variable
            let mut buffer = [0; 4];
            stream.read(&mut buffer)?;
            let id = u32::from_be_bytes(buffer);
            state.register_variable(id, BytecodeObject::read(stream)?);
            Ok(())
        }
        3 => {
            // log statement, output something to the debug log
            println!("{}", BytecodeObject::read(stream)?.as_string(state)?);
            // TODO: process tuple into println
            let _tuple = BytecodeObject::read(stream)?.as_tuple(state)?;
            Ok(())
        }
        _ => Err(LitError::BytecodeRead8(buffer[0])),
    }
}
```

*Modifications to src/script/mod.rs*

```rust
mod eval;

/* ... */

#[derive(Debug)]
pub struct GameData {
    name: String,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            name: String::from("Unnamed"),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
```

We now have everything we need for a functioning interpreter. We just need to implement the "Bytecode" trait on "GameData" in order to tie everything together.

*Modifications to src/script/mod.rs*

```rust
impl Bytecode for GameData {
    fn read<T: Read>(stream: &mut T) -> Result<Self, LitError> {
        let mut data = Self::new();
        let mut parse = ParserState::new();

        while let Ok(()) = eval::eval(stream, &mut parse, &mut data) {} // go until error is encountered

        Ok(data)
    }
}
```

*Modifications to src/main.rs*

```rust
use script::Bytecode;

use std::{
    env, fs,
    io::{prelude::*, BufReader},
    process,
};

/* ... */

fn classic_main() -> Result<(), LitError> {
    let game = Game {};
    let renderer = GlRenderer::init()?;
    let mut data_file = BufReader::new(fs::File::open(
        env::args()
            .skip(1)
            .next()
            .ok_or_else(|| LitError::NoDataFile)?,
    )?);
    let game_data = script::GameData::read(&mut data_file)?;

    println!("{:?}", game_data);
    renderer.main_loop(&game)
}
```

*Modifications to src/error.rs*

```rust
#[error("Unable to find data file")]
NoDataFile,
```

Then, I created a file, "test.dat", with the following contents:

```
$ xxd test.dat 
00000000: 0104 0c48 656c 6c6f 2077 6f72 6c64 21    ...Hello world!
```

This is roughly equal to, in the above hypothetical "game script":

```
gamedef "Hello world!"
```

If we run the program via the following command:

```bash
$ cargo run -- test.dat
```

The program will output:

```
GameData { name: "Hello world!" }
```

Now that we have an interpreter for the bytecode, I figure that it's time to write the compiler that actually generates that bytecode. Since LostInTime will have no need of a compiler during its runtime, it makes sense to make this compiler its own separate crate.

```
$ cargo new --bin lits-cc
$ cd lits-cc
```

We'll use the "proc-macro2" create as a dependency, as it makes breaking down text into tokens relatively easy. We'll also use "thiserror" again, since we'll need error handling.

```
[dependencies]
proc-macro2 = "1"
thiserror = "1"
```

*Inside lits-cc/src/error.rs*

```rust
use proc_macro2::{LexError, TokenTree};
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LitsCcError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(TokenTree),
    #[error("An IO error occurred: {0}")]
    Io(#[from] IoError),
    #[error("A lexing error occurred: {0:?}")]
    Lex(LexError),
}

impl From<LexError> for LitsCcError {
    fn from(l: LexError) -> LitsCcError {
        LitsCcError::Lex(l)
    }
}
```

*Inside lits-cc/src/compile.rs*

```rust
use crate::LitsCcError;
use proc_macro2::TokenStream;
use std::io::prelude::*;

pub fn compile_line<T: Write>(line: &str, stream: &mut T) -> Result<(), LitsCcError> {
    // parse the line into tokens
    let tokens: TokenStream = line.parse()?;

    // iterate over each token
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
```

*Inside lits-cc/src/main.rs*

```rust
mod compile;

mod error;
pub use error::LitsCcError;

use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    process,
};

fn main() {
    // get input and output file
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("lits-cc expects at least two arguments.");
        process::exit(1);
    }

    let in_file = &args[1];
    let out_file = &args[2];

    // open each file as a bufferred reader/writer
    let in_file = BufReader::new(File::open(in_file).unwrap());
    let mut out_file = BufWriter::new(File::create(out_file).unwrap());

    for line in in_file.lines() {
        compile::compile_line(&line.unwrap(), &mut out_file).unwrap();
    }
}
```

This is a relatively good base to start from. It reads every line and outputs what the token reader reads. In this case, when we use this file as input:

```
gamedef "Hello world!"
```

We get:

```
Ident { sym: gamedef }
Literal { lit: "Hello world!" }
```
