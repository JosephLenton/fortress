//! Adds the SDL libraries to the link path.
//! Otherwise they don't get found.
//! 

use std::env;

/// Trivial entry point.
/// Adds SDL to the link.
fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}/lib/sdl", out_dir);
}
