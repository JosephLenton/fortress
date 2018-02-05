
use LLR;
use LLRPixel;
use util::shapes::Rect;

pub struct LLRTerminal {
}

impl LLRTerminal {
    /// Trivial constructor.
    pub fn new() -> LLRTerminal {
        LLRTerminal {
        }
    }
}

impl LLR for LLRTerminal {
    fn clear(&mut self) -> () {
        // TODO
    }

    fn pixel(
        &mut self,
        pixel: LLRPixel,
        rectable: Rect<f32>,
    ) -> Result<(), String> {
        // TODO
    }

    fn finished_drawing(&mut self) -> () {
        // TODO
    }
}

