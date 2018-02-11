use super::terminal_keys::u8_to_key;
use LLR;
use LLREvent;
use LLROptions;
use LLRPixel;
use getch::Getch;
use implementations::terminal::colour;
use std::io::{self, Write};
use terminal_size;
use util::collections::Matrix;
use util::shapes::Point;
use util::shapes::Size;

/// An LLR that renders to the terminal.
pub struct LLRTerminal {
    /// The options used to create this LLR.
    /// We'll need this later for use at runtime.
    /// Like the `clear_colour`.
    options: LLROptions,

    /// Internal buffer that holds that data we will be drawing.
    screen: Matrix<Option<LLRPixel>>,

    /// We use this for reading in user input.
    getch: Getch,

    /// Where we are writing to.
    out: io::Stdout,

    /// The output is first concatonated into a giant string.
    /// This allows us to perform a single write to send it out.
    ///
    /// To avoid rebuilding that string every single time, we instead place it
    /// here.
    out_buffer : String,
}

impl LLRTerminal {
    /// Trivial constructor.
    pub fn new(options: LLROptions) -> Self {
        let matrix_size = options.window_size / options.tile_size.to::<u16>();
        let estimated_out_capacity = matrix_size.to_clamped::<usize>().area() * 4;

        let mut llr = Self {
            options: options,
            screen: Matrix::new(matrix_size, None),
            out_buffer: String::with_capacity(estimated_out_capacity),
            getch: Getch::new(),
            out: io::stdout(),
        };

        llr.set_title( & options.title );

        llr
    }
}

impl LLR for LLRTerminal {
    fn set_title( &mut self, title : &str ) {
        write!( self.out, "\x1B]2;{}\x1b\x5c", title );
    }

    fn clear(&mut self) {
        for x in 0..self.screen.size().width {
            for y in 0..self.screen.size().height {
                self.screen[Point::new(x, y)] = None;
            }
        }

        self.out_buffer.clear();
    }

    fn pixel(
        &mut self,
        pixel: LLRPixel,
        pos: Point<u16>,
    ) -> Result<(), String> {
        if self.screen.size().contains(pos) {
            self.screen[pos] = Some(pixel);
        }

        Ok(())
    }

    fn finished_drawing(&mut self) {
        let out_buffer = &mut self.out_buffer;
        let clear_colour = self.options.clear_colour;
        let clear_colour_str = & colour::to_background_colour_code(self.options.clear_colour);
        let mut maybe_last_pixel : Option<LLRPixel> = None;

        // Start us off in the top left with the clear colour.
        *out_buffer += & "\x1B[0;0H";
        *out_buffer += & clear_colour_str;

        self.screen.iter().for_each(|(maybe_pixel, pos)| -> () {
            if pos.x == 0 && pos.y > 0 {
                *out_buffer += & "\n";
            }

            match (maybe_pixel, maybe_last_pixel) {
                (Some(pixel), None) => {
                    if pixel.background != clear_colour {
                        *out_buffer += & colour::to_background_colour_code(pixel.background);
                    }

                    *out_buffer += & colour::to_foreground_colour_code(pixel.foreground);
                    *out_buffer += & pixel.character;

                    maybe_last_pixel = Some(pixel);
                },
                (Some(pixel), Some(last_pixel)) => {
                    if pixel.background != last_pixel.background {
                        *out_buffer += & colour::to_background_colour_code(pixel.background);
                    }

                    if pixel.foreground != last_pixel.foreground {
                        *out_buffer += & colour::to_foreground_colour_code(pixel.foreground);
                    }

                    *out_buffer += & pixel.character;
                    maybe_last_pixel = Some(pixel);
                },
                (None, None) => {
                    *out_buffer += & " ";
                },
                (None, _) => {
                    *out_buffer += & clear_colour_str;
                    *out_buffer += & " ";
                    maybe_last_pixel = None;
                },
            };
        });

        // Padding.
        // This is to hide the extra characters we get printed.
        *out_buffer += & clear_colour_str;
        *out_buffer += & "\n\x1B[2K";
        *out_buffer += & "\n\x1B[2K";
        *out_buffer += & "\x1B[1A";

        write!( self.out, "{}", &out_buffer );
    }

    fn on_start(&mut self) {
        let output = &mut String::new();

        // Clear.
        *output += & "\x1B[2J";

        // Default colours.
        *output += & colour::to_default_colour_code();

        // Hide cursor.
        *output += & "\x1B[?25l";

        write!( self.out, "{}", &output );
        self.out.flush();
    }

    fn on_quit(&mut self) {
        let output = &mut String::new();

        // Clear.
        *output += & "\x1B[2J";

        // Default colours.
        *output += & colour::to_default_colour_code();

        // Show cursor.
        *output += & "\x1B[?25h";

        // Restore to initial state.
        // May not actually make the cursor shown again, but should provide
        // any additional resets we missed.
        *output += & "\x1Bc";

        write!( self.out, "{}", &output );
        self.out.flush();
    }

    fn size(&self) -> Size<u16> {
        let size = terminal_size::terminal_size();

        if let Some((terminal_size::Width(w), terminal_size::Height(h))) = size {
            Size::new(w, h)

        // We'll just pretend we have a proper size.
        } else {
            self.options.window_size / self.options.tile_size.to::<u16>()
        }
    }

    fn poll(&mut self) -> Option<LLREvent> {
        match self.getch.getch() {
            Ok(c) => u8_to_key(c).map(|key| LLREvent::KeyPress(key)),
            Err(err) => {
                eprintln!("Error reading getch {}", err);
                None
            },
        }
    }
}
