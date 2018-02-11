
/// We have rectangles, and colours, and things like that. When we want to talk
/// to SDL2 directly, it cannot use these types.
///
/// So this trait is for wrapping up the conversation. For converting from our
/// types, to SDL2's.
pub(crate) trait ToSDL2<S> {
    /// Returns the SDL2 equivalent of this item.
    fn to_sdl2(self) -> S;
}
