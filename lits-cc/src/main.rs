// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// main.rs - Entry point for compiler

#![allow(clippy::new_without_default)]

mod command;
pub use command::process_command;

mod compile;

mod error;
pub use error::LitsCcError;

mod literals;
pub use literals::process_literals;

mod state;
pub use state::CompilerState;

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

    let mut state = CompilerState::new();

    for (index, line) in in_file.lines().enumerate() {
        let line_ref = &line.unwrap();
        let processed_line = line_ref.split('#').next().unwrap_or_else(|| line_ref);
        if let Err(e) = compile::compile_line(processed_line, &mut out_file, &mut state) {
            eprintln!("Error occurred on line {}: {}", index, e);
            process::exit(1);
        }
    }
}
