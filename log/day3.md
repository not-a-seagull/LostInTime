# Day 3

I've changed the testing input file to this, to manage everything we've done so far:

```
gamedef "Lost in Time"                # define this game as "Lost in Time"
def MY_VAR 1                          # define MY_VAR as being equal to one
log "MY_VAR is equal to {}" (@MY_VAR) # print "MY_VAR is equal to 1" to the console
```

First, we need a decent way of handling comments. The most intuitive way of doing this would be to just split the line by the '#' character and only using the first entry.

*Modifications to lits-cc/src/main.rs*

```rust
for line in in_file.lines() {
    let line_ref = &line.unwrap();
    let processed_line = line_ref.split("#").next().unwrap_or_else(|| line_ref);
    compile::compile_line(processed_line, &mut out_file).unwrap();
}
```

From this, we get the following result:

```
Ident { sym: gamedef }
Literal { lit: "Lost in Time" }
Ident { sym: def }
Ident { sym: MY_VAR }
Literal { lit: 1 }
Ident { sym: log }
Literal { lit: "MY_VAR is equal to {}" }
Group { delimiter: Parenthesis, stream: TokenStream [Punct { op: '@', spacing: Alone }, Ident { sym: MY_VAR }] }
```

It seems that the processing loop I will use is something similar to:

1). Read an `Ident` object to determine which command we are using.
2). Read over the remaining tokens, and determine their types and values.

Of course, there is some extra complexity (for instance, the "def" command needs an identifier after the command). However, we can compensate for this.

*Inside of lits-cc/src/state.rs*

```rust
use crate::LitsCcError;
use std::collections::HashMap;

pub struct CompilerState {
    variables: HashMap<String, u32>,
    current_id: u32,
}

impl CompilerState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            current_id: 1,
        }
    }

    pub fn register_variable(&mut self, name: &str) -> u32 {
        let id = self.current_id;
        self.variables.insert(String::from(name), id);
        self.current_id += 1;
        id
    }

    pub fn get_variable_id(&self, name: &str) -> Result<u32, LitsCcError> {
        match self.variables.get(name) {
            Some(u) => Ok(*u),
            None => Err(LitsCcError::VariableNotFound(String::from(name))),
        }
    }
}
```

*Modifications to lits-cc/src/main.rs*

```rust
mod command;
pub use command::process_command;

/* ... */

mod literals;
pub use literals::process_literals;

mod state;
pub use state::CompilerState;

/* ... */

for (index, line) in in_file.lines().enumerate() {
    let line_ref = &line.unwrap();
    let processed_line = line_ref.split("#").next().unwrap_or_else(|| line_ref);
    if let Err(e) = compile::compile_line(processed_line, &mut out_file, &mut state) {
        eprintln!("Error occurred on line {}: {}", index, e);
        process::exit(1);
    }
}
```

*Modifications to lits-cc/src/compile.rs*

```rust
pub fn compile_line<T: Write>(
    line: &str,
    stream: &mut T,
    state: &mut CompilerState,
) -> Result<(), LitsCcError> {
    // parse the line into tokens
    let tokens: TokenStream = line.parse()?;
    let mut iter = tokens.into_iter();

    // get the first token in the stream, which must be an identifier
    match iter.next() {
        Some(TokenTree::Ident(ref i)) => {
            process_command(i, &mut iter, stream, state)?;
        }
        None => return Ok(()), // empty line, write nothing
        _ => return Err(LitsCcError::ExpectedIdent),
    }

    process_literals(&mut iter, stream, state)
}
```

*Inside of lits-cc/src/command.rs*

```rust
use crate::{CompilerState, LitsCcError};
use proc_macro2::{Ident, TokenTree};
use std::io::prelude::*;

pub fn process_command<TStream: Write, TIter: Iterator<Item = TokenTree>>(
    ident: &Ident,
    iter: &mut TIter,
    stream: &mut TStream,
    state: &mut CompilerState,
) -> Result<(), LitsCcError> {
    let name = format!("{}", ident);

    match name.as_ref() {
        "gamedef" => {
            stream.write(&[1])?;
            Ok(())
        }
        "def" => {
            stream.write(&[2])?;

            // also read in an ident
            match iter.next() {
                None => Err(LitsCcError::ExpectedIdent),
                Some(TokenTree::Ident(i)) => {
                    let var_name = format!("{}", i);
                    let id = state.register_variable(&var_name);
                    stream.write(&id.to_be_bytes())?;
                    Ok(())
                }
                _ => Err(LitsCcError::ExpectedIdent),
            }
        }
        "log" => {
            stream.write(&[3])?;
            Ok(())
        }
        _ => Err(LitsCcError::UnknownCommand(name)),
    }
}
```

*Inside of lits-cc/src/literals.rs*

```rust
use crate::{CompilerState, LitsCcError};
use proc_macro2::{Ident, TokenTree};
use std::io::prelude::*;

pub fn process_literals<TStream: Write, TIter: Iterator<Item = TokenTree>>(
    iter: &mut TIter,
    stream: &mut TStream,
    state: &mut CompilerState,
) -> Result<(), LitsCcError> {
    while let Some(token) = iter.next() {
        match token {
            TokenTree::Literal(l) => {
                let l = format!("{}", l);

                if l.starts_with('"') && l.ends_with('"') {
                    // this is a string!
                    let inner = l.split("\"").skip(1).next().unwrap();
                    stream.write(&[4, inner.len() as u8])?;
                    stream.write(inner.as_bytes())?;
                } else if let Ok(i) = l.parse::<i32>() {
                    if i >= std::u8::MIN as i32 && i <= std::u8::MAX as i32 {
                        stream.write(&[1])?;
                        stream.write(&[i as u8])?;
                    } else if i >= std::i16::MIN as i32 && i <= std::i16::MAX as i32 {
                        stream.write(&[2])?;
                        stream.write(&(i as i16).to_be_bytes())?;
                    } else {
                        stream.write(&[3])?;
                        stream.write(&i.to_be_bytes())?;
                    }
                } else {
                    return Err(LitsCcError::Msg(format!("Unexpected literal: {}", l)));
                }
            }
            TokenTree::Punct(p) => {
                if p.as_char() == '@' {
                    // next should be an identifier
                    match iter.next() {
                        Some(TokenTree::Ident(i)) => {
                            let name = format!("{}", i);
                            stream.write(&[6])?;
                            stream.write(&state.get_variable_id(&name)?.to_be_bytes())?;
                        }
                        _ => {
                            return Err(LitsCcError::ExpectedIdent);
                        }
                    }
                } else {
                    return Err(LitsCcError::Msg(format!("Unexpected punctuation: {}", p)));
                }
            }
            TokenTree::Group(g) => {
                // TODO
            }
            _ => return Err(LitsCcError::Msg(format!("Unexpected token: {}", token))),
        }
    }

    Ok(())
}
```

*Modifications to lits-cc/src/error.rs*

```rust
#[error("{0}")]
Msg(String),
#[error("{0}")]
StaticMsg(&'static str),
#[error("Unable to find variable with id {0}")]
VariableNotFound(String),
#[error("Expected identifier")]
ExpectedIdent,
#[error("Unknown command: {0}")]
UnknownCommand(String),
```

This ended up being a lot bigger of a task than I was expecting. I added a better way of reporting compilation errors, as well as most types of literals. However, I left out tuples, because I didn't know how to approach it. Looking back, I think this is how I should go about it:

*Modifications to lits-cc/src/literals.rs*

```rust
TokenTree::Group(g) => {
    if g.delimiter() != Delimiter::Parenthesis {
        return Err(LitsCcError::StaticMsg(
		"The only group supported at the moment are parenthesis.",
	));
    }

    let mut cursor = Cursor::new(Vec::new());
    let length = process_literals(iter, &mut cursor, state)?;
    cursor.seek(SeekFrom::Start(0))?;
    stream.write(&[5, length as u8])?;

    // copy bytes from the cursor into the file
    io::copy(&mut cursor, stream)?;
}
```

This seems to work; however, a strange bug appears in the output:

```
$ xxd out.dat 
00000000: 0104 0c4c 6f73 7420 696e 2054 696d 6502  ...Lost in Time.
00000010: 0000 0001 0101 0304 154d 595f 5641 5220  .........MY_VAR 
00000020: 6973 2065 7175 616c 2074 6f20 7b7d 0500  is equal to {}..
```

Note that the last word of byte `0500` roughly corresponds to an empty tuple. I puzzled over want went wrong for a minute, before I realized I had done:

```rust
let length = process_literals(iter, &mut cursor, state)?;
```

I should not have used `iter` here. The `Group` proc macro type skips over the elements inside of the group. In reality, I should have done:

```rust
let mut group_iter = g.stream().into_iter();
let length = process_literals(&mut group_iter, &mut cursor, state)?;
```

This now gives me my desired result. Before anything else, let's go back to the main program and implement the log formatting function:

*Modifications to src/script/eval.rs*

```rust
3 => {
     // log statement, output something to the debug log
     // home grown format processor, could be improved
     let format = BytecodeObject::read(stream)?;
     let tuple = BytecodeObject::read(stream)?;
     let tuple = tuple.as_tuple()?;
     let formatted_str = format
         .as_string()?
         .split("{}")
         .enumerate()
         .map(|(i, part)| {
             if i != tuple.len() {
                 format!("{}{}", part, tuple[i])
             } else {
                 String::from(part)
             }
         })
         .collect::<Vec<String>>()
         .join("");
    println!("{}", formatted_str);

    Ok(())
}

```

*Modifications to src/script/types.rs*

```rust
impl fmt::Display for BytecodeObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BytecodeObject::Numeric8(u) => write!(f, "{}", u),
            &BytecodeObject::Numeric16(u) => write!(f, "{}", u),
            &BytecodeObject::Numeric32(u) => write!(f, "{}", u),
            &BytecodeObject::Str(ref u) => write!(f, "{}", u),
            &BytecodeObject::Tuple(ref s) => write!(f, "{:?}", s),
            &BytecodeObject::VarInvocation(u) => {
                Self::fmt(PARSER_STATE.lock().unwrap().get_variable(u).unwrap(), f)
            }
        }
    }
}
```

*Note: I made PARSER_STATE a global variable to avoid having to pass in the state during every Display call. Because of this, I needed to rewrite some borrow semantics. I don't think these efforts are worth detailing here.*

We get the following as output:

```
MY_VAR is equal to 1
GameData { name: "Lost in Time" }
```

We can see here that the `log` statement is now compiled and working properly. Now that we have a working compiler, we can now work on image processing.
