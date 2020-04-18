# Day 9

Now that we have a form of resource management in place, it's time to finally write code that handles images. Here's what I imagine LitScript image creation looking like:

```
create_tex MY_SPRITE 5 5 (255 0 0 1)
color_id @MY_SPRITE 0 (0 0 0 1)
color_id @MY_SPRITE 1 (0 255 255 1)
draw_pixel @MY_SPRITE 1 1 0
draw_rect @MY_SPRITE 2 2 4 4 1
```

Let's add the compiler commands for this.

*Modifications to lits-cc/src/command.rs*

```rust
pub fn read_ident<TStream: Write, TIter: Iterator<Item = TokenTree>>(
    iter: &mut TIter,
    stream: &mut TStream,
    state: &mut CompilerState,
) -> Result<(), LitsCcError> {
    // read in an ident
    match iter.next() {
        Some(TokenTree::Ident(i)) => {
            let var_name = format!("{}", i);
            let id = state.register_variable(&var_name);
            stream.write(&id.to_be_bytes())?;
            Ok(())
        }
        _ => Err(LitsCcError::ExpectedIdent),
    }
}

pub fn process_command<TStream: Write, TIter: Iterator<Item = TokenTree>>(
    ident: &Ident,
    iter: &mut TIter,
    stream: &mut TStream,
    state: &mut CompilerState,
) -> Result<(), LitsCcError> {
    let name = format!("{}", ident);

    match name.as_ref() {
        "gamedef" => write_word(stream, 1),
        "def" => {
            write_word(stream, 2)?;
            read_ident(iter, stream, state)
        }
        "log" => write_word(stream, 3),
        "create_tex" => {
            write_word(stream, 4)?;
            read_ident(iter, stream, state)
        }
        "color_id" => write_word(stream, 5),
        "draw_pixel" => write_word(stream, 6),
        "draw_rect" => write_word(stream, 7),
        _ => Err(LitsCcError::UnknownCommand(name)),
    }
}
```

Now that we can produce image generation code, I've compiled this file:

```
gamedef "Lost in Time"                # define this game as "Lost in Time"
def MY_VAR 1                          # define MY_VAR as being equal to one
log "MY_VAR is equal to {}" (@MY_VAR) # print "MY_VAR is equal to 1" to the console
# just a comment

create_tex MY_SPRITE 5 5 (255 0 0 1)  # create a 5x5 image with background color of red
color_id @MY_SPRITE 0 (0 0 0 1)       # set color id 0 to black
color_id @MY_SPRITE 1 (0 255 255 1)   # set color id 1 to yellow
draw_pixel @MY_SPRITE 1 1 0           # draw pixel at (1, 1) with color id 0
draw_rect @MY_SPRITE 2 2 4 4 1        # draw rectangle at (2, 2, 4, 4) with color id 1
```

Unfortunately I got busy and could not complete the rest of this code.
