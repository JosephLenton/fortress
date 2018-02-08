use LLR;
use LLRPixel;
use util::shapes::Point2;
use util::shapes::Size;

pub struct LLRTerminal {
    screen : &
}

impl LLRTerminal {
    /// Trivial constructor.
    pub fn new() -> LLRTerminal {
        LLRTerminal {}
    }
}

impl LLR for LLRTerminal {
    fn clear(&mut self) -> () {
        // TODO
    }

    fn pixel(
        &mut self,
        pixel: LLRPixel,
        pos: Point2<u16>,
    ) -> Result<(), String> {
        let size = this.size();

        if pos.x < size.width || pos.y < size.height {
            self.screen[pos.x][pos.y] = pixel
        }

        Ok(())
    }

    fn finished_drawing(&mut self) -> () {
        // TODO
    }

    fn size(&self) -> Size<u16> {
        // TODO
        Size::new(1, 1)
    }
}
