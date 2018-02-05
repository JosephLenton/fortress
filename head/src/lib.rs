//! This crate is things specific to how the world is drawn and run.
//! It also does things like create windows.
//!
//! Anything visual is here.
//! 

extern crate game;
extern crate llr;
extern crate util;
extern crate world;

/// Contains the colour information.
/// i.e. what are the colour of tiles.
pub mod theme;

/// High Level Renderer.
///
/// It's high level as in 'draw these tiles', 'draw the player', or 'render the
/// weather information'.
///
/// It doesn't do any actual drawing.
pub mod render;

/// Contains code for printing.
#[deprecated(note = "This will be rolled into the llr as the cmd line interface")]
pub mod native;

/// Used for setting up the hl renderer.
pub use llr::LLROptions as RenderOptions;
