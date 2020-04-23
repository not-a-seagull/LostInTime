// Licensed under the BSD 3-Clause License. See the LICENSE file in the repository root for more information.
// lit-gl/build.rs - Build script for the OpenGL implementation

use gl_generator::{Api, DebugStructGenerator, Fallbacks, Profile, Registry, StructGenerator};
use std::{env, fs::File, path::Path};

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut outfile = File::create(&Path::new(&dest).join("gl.rs")).unwrap();

    let registry = Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, []);

    if env::var("PROFILE").unwrap() == "debug" {
        registry
            .write_bindings(DebugStructGenerator, &mut outfile)
            .unwrap();
    } else {
        registry
            .write_bindings(StructGenerator, &mut outfile)
            .unwrap();
    }
}
