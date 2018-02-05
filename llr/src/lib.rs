//! Low Level Renderer.
//!
//! It's low level as in 'draw a rectangle', or 'clear the screen'.
//!
//! This is the logic for actual drawing operations. It also contains the
//! implementations for this. i.e. for drawing to SDL2, or the terminal.

extern crate sdl2;
extern crate util;

/// The generic LLR trait.
mod llr;

/// Options for setting up an LLR.
mod llr_options;

/// SDL2 implementation of the LLR.
mod llr_sdl2;

/// LLR was created to move code out of the `head` crate as a part of
/// refactoring. Until that is done, the `head` still needs some SDL2 bits.
/// `temp` exists to provide these bits.
pub mod temp;

/// The common LLR interface.
pub use llr::LLR;

/// Options for setting up a LLR.
pub use llr_options::LLROptions;

/// An SDL2 specific LLR.
pub use llr_sdl2::LLRSDL2;
