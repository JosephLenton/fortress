//! Low Level Renderer.
//!
//! It's low level as in 'draw a rectangle', or 'clear the screen'.
//!
//! This is the logic for actual drawing operations. It also contains the
//! implementations for this. i.e. for drawing to SDL2, or the terminal.

extern crate sdl2;
extern crate util;
extern crate terminal_size;

/// Here lies the various llr implementations.
mod implementations;

/// The generic LLR trait.
mod llr;

/// Options for setting up an LLR.
mod llr_options;

/// Information for drawing a single pixel.
mod llr_pixel;

/// Event wrapper.
/// Aims to be a simpler common denominator.
mod llr_event;

/// Where the common keycodes live.
mod llr_key;

/// The translated event.
pub use llr_event::LLREvent;

/// Keycodes.
pub use llr_key::LLRKey;

/// The common LLR interface.
pub use llr::LLR;

/// Options for setting up a LLR.
pub use llr_options::LLROptions;

/// Describes the information needed to draw a single pixel.
/// Doesn't include the location.
pub use llr_pixel::LLRPixel;

/// An SDL2 specific LLR.
pub use implementations::LLRSDL2;

/// A terminal based LLR.
pub use implementations::LLRTerminal;
