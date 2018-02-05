
mod to_sdl2;
mod to_sdl2_rgba;
mod to_sdl2_rect;

/// A generic trait for converting things to their SDL2 version.
pub use self::to_sdl2::*;

/// For converting our RGBA values to SDL2's Colour type.
pub use self::to_sdl2_rgba::*;

/// Adds conversation to SDL2's size types to our own.
pub use self::to_sdl2_rect::*;

