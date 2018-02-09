//! This is a generic utility crate.
//!
//! It contains lots of random stuff. None of it is OS
//! or library specific.
//!
//! It's mostly maths related code. Like points, shapes,
//! and colours.
//!

#![feature(associated_type_defaults)]
#![feature(const_fn)]

/// A module of traits to make it easier to work with numbers.
mod num_types;

pub mod collections;
pub mod shapes;
pub mod colour;
pub mod states;
