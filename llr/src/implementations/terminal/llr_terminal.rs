use LLR;
use LLROptions;
use LLRPixel;
use LLREvent;
use LLRKey;
use util::shapes::Point;
use util::shapes::Size;
use util::collections::Matrix;
use implementations::terminal::colour::pixel_to_cmd_code;
use implementations::terminal::colour::to_background_colour_code;
use terminal_size;

/// An LLR that renders to the terminal.
pub struct LLRTerminal {

    /// The options used to create this LLR.
    /// We'll need this later for use at runtime.
    /// Like the `clear_colour`.
    options : LLROptions,

    /// Internal buffer that holds that data we will be drawing.
    screen : Matrix<Option<LLRPixel>>,

    /// Holds the output to be printed.
    /// This is not the source of truth. It holds the contents of the matrix,
    /// once it's been converted to a string.
    ///
    /// We hold onto the string here to avoid re-creating it every time we go
    /// to draw.
    out_buffer : String,

}

impl LLRTerminal {
    /// Trivial constructor.
    pub fn new( options : LLROptions ) -> LLRTerminal {
        let matrix_size = options.window_size / options.tile_size.to::<u16>();
        let estimated_out_capacity = matrix_size.to_clamped::<usize>().area()*4;

        LLRTerminal {
            options : options,
            screen : Matrix::new( matrix_size, None ),
            out_buffer : String::with_capacity( estimated_out_capacity ),
        }
    }
}

impl LLR for LLRTerminal {
    fn clear(&mut self) {
        self.out_buffer.clear();
        self.out_buffer += "\x1B[2J";
    }

    fn pixel(
        &mut self,
        pixel: LLRPixel,
        pos: Point<u16>,
    ) -> Result<(), String> {
        if self.screen.size().contains( pos ) {
            self.screen[pos] = Some(pixel)
        }

        Ok(())
    }

    fn finished_drawing(&mut self) {
        let mut out_buffer = String::new();
        out_buffer += "\x1B[2J";

        self.screen.iter().for_each(|(maybe_pixel, pos)| -> () {
            match maybe_pixel {
                Some(pixel) => {
                    out_buffer += & pixel_to_cmd_code(pixel);
                },
                None => {
                    out_buffer += & to_background_colour_code(self.options.clear_colour);
                    out_buffer += " ";
                },
            };

            if pos.x == self.screen.size().width-1 {
                out_buffer += "\n";
            }
        });

        print!( "{}", out_buffer );
    }

    fn size(&self) -> Size<u16> {
        let size = terminal_size::terminal_size();

        if let Some((terminal_size::Width(w), terminal_size::Height(h))) = size {
            Size::new( w, h )

        // We'll just pretend we have a proper size.
        } else {
            self.options.window_size
        }
    }

    fn poll(&mut self) -> Option<LLREvent> {
        // TODO
        None
    }
}

