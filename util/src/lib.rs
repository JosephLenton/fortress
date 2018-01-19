
//!
//! This is a generic utility crate.
//!
//! It contains lots of random stuff. None of it is OS
//! or library specific.
//!
//! It's mostly maths related code. Like points, shapes,
//! and colours.
//!

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

pub mod shapes;
pub mod colour;
pub mod states;

