/// The SDL2 implementation for graphics.
mod llr_sdl2_impl;

/// How to draw to the screen in an SDL2 specific way.
mod to_sdl2;

/// Converting RGBA to SDL2 Colour.
mod to_sdl2_rgba;

/// Converting Rect to SDL2 Rect.
mod to_sdl2_rect;

/// The SDL2 implementation for graphics.
/// tl;dr an instance of this to get a window.
pub use self::llr_sdl2_impl::LLRSDL2Impl as LLRSDL2;
