
//! Low Level Renderer.
//!
//! It's low level as in 'draw a rectangle', or 'clear the screen'.
//!
//! This is the logic for actual drawing operations. It also contains the
//! implementations for this. i.e. for drawing to SDL2, or the terminal.

extern crate sdl2;
extern crate util;
extern crate world;

/// The generic LLR trait.
mod llr;

/// Options for setting up an LLR.
mod llr_options;

/// SDL2 implementation of the LLR.
mod llr_sdl2;

/// The common LLR interface.
pub use llr::LLR;

/// Options for setting up a LLR.
pub use llr_options::LLROptions;

/// An SDL2 specific LLR.
pub use llr_sdl2::LLRSDL2;

