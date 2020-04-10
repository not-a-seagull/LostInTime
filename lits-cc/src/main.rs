// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// main.rs - Entry point for compiler

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
