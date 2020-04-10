// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// compile.rs - Take a line of LitS and compile it.

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
