// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// utils.rs - Various utility functions.

use std::ffi::CString;

pub fn create_cstring_buffer(len: usize) -> CString {
    let mut buffer = Vec::with_capacity(len);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
