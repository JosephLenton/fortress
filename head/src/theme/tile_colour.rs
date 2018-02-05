
use util::colour::RGBA;

/// The colour for a tile.
///
/// It combines a background and a foreground.
#[derive(Copy, Clone)]
pub struct TileColour {
    /// The background colour.
    pub background: RGBA,

    /// The foreground colour.
    pub foreground: RGBA,
}

