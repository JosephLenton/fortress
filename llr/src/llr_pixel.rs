use util::colour::RGBA;

/// All information to use when drawing.
/// Colour to use, characters, etc.
#[derive(Copy, Clone)]
pub struct LLRPixel {
    /// The background colour.
    pub background: RGBA,

    /// The foreground colour.
    pub foreground: RGBA,

    /// This is the display character to display the tile.
    pub character: &'static str,
}
