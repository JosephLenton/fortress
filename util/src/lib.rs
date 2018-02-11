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
pub mod num_types;

/// Some custom collections I find useful.
pub mod collections;

/// For representing points, sizes, and shapes.
pub mod shapes;

/// For representing colours.
pub mod colour;

/// Some extra enums for representing items.
pub mod states;
