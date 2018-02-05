//!
//! This crate represents modelling the world.
//!
//! Creatures, land, places, weather, time, and more.
//!
extern crate util;

/// How time and dates work in the world.
pub mod calendar;

/// How weather works in the world.
pub mod weather;

/// The player management in the world.
pub mod player;

/// The world's tiles.
pub mod tiles;

/// Worlds maps.
pub mod map;

/// How to describe a new world.
pub mod world_setup;

/// How to load things in the world.
#[deprecated(note="needs to be moved out of fortress/world")]
pub mod load;
