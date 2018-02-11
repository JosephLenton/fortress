use util::colour::RGBA;
use util::shapes::Size;

/// Describes how to setup a low level renderer.
///
/// A renderer may or may not use all of the items provided.
#[derive(Copy, Clone)]
pub struct LLROptions {

    /// The title for the application.
    pub title: &'static str,

    /// Starting size for the display area.
    pub window_size: Size<u16>,

    /// How big the tiles are when rendered.
    /// This may be ignored.
    pub tile_size: Size<u8>,

    /// The background and clear colour.
    pub clear_colour: RGBA,

}
