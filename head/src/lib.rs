
//!
//! This crate is things specific to how the world is drawn and run.
//! It also does things like create windows.
//!
//! Anything visual is here.
//!

extern crate sdl2;
extern crate game;
extern crate util;
extern crate world;

/// Contains the colour information.
/// i.e. what are the colour of tiles.
pub mod theme;

/// How to render world stuff.
pub mod render;

/// How to draw to the screen in an SDL2 specific way.
pub mod to_sdl2;

/// The native heads you can render to.
/// These are the real implementations.
/// No abstractions.
pub mod native;

