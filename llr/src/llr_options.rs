use util::shapes::Size;
use util::colour::RGBA;

/// Describes how to setup a low level renderer.
///
/// A renderer may or may not use all of the items provided.
#[derive(Copy, Clone)]
pub struct LLROptions {
    pub title: &'static str,
    pub window_size: Size<u16>,
    pub tile_size: Size<u8>,
    pub clear_colour : RGBA,
}
