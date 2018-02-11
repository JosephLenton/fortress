//! High Level Renderer.
//!
//! This performs the rendering for the game. It knows all about the game, and
//! how things in the game should be rendered. It knows where to draw things,
//! and how they should get drawn. This includes colours.
//!
//! It doesn't know how create a window, and it does not know the specific
//! implementation for drawing. That is handled by the LLR.
//!

extern crate game;
extern crate llr;
extern crate util;
extern crate world;

/// Contains the colour information.
/// i.e. what are the colour of tiles.
pub mod theme;

/// Contains the rendering logic.
///
/// It doesn't do any actual drawing.
pub mod render;

/// Used for setting up the hl renderer.
pub use llr::LLROptions as HLROptions;
