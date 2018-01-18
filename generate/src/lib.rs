
#![crate_name = "generate"]

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rand;
extern crate world;

pub mod generate;
