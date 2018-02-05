
use util::shapes::Size;

/// Describes how to setup a low level renderer.
///
/// A renderer may or may not use all of the items provided.
#[derive(Copy, Clone)]
pub struct LLROptions {
    pub title: &'static str,
    pub window_size: Size<u32>,
    pub tile_size: Size<u32>,
}

