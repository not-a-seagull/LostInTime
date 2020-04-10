// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// script/bytecode.rs - Trait for reading from bytecode.

use crate::LitError;
use std::io::prelude::*;

pub trait Bytecode: Sized {
    fn read<T: Read>(stream: &mut T) -> Result<Self, LitError>;
}
