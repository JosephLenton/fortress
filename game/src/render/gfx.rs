
use tiles::colour::Colour;

/// 
/// Represents the graphics state.
///
/// This handles all drawing logic.
///
pub struct GFX {
    //submissions : Vec<Box<GpuFuture>>,
}

impl GFX {
    pub fn new() -> GFX {
        return GFX {
            //submissions : Vec::new(),
        }
    }

    pub fn rectangle(
        &mut self,
        colour : Colour,
        _pos   : [f32; 4],
    ) {
        // todo
    }
}

