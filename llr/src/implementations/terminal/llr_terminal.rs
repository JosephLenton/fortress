use super::terminal_keys::u8_to_key;
use LLR;
use LLREvent;
use LLROptions;
use LLRPixel;
use getch::Getch;
use implementations::terminal::colour::pixel_to_cmd_code;
use implementations::terminal::colour::to_background_colour_code;
use std::io::{self, Write};
use std::time;
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

    /// Holds the output to be printed.
    /// This is not the source of truth. It holds the contents of the matrix,
    /// once it's been converted to a string.
    ///
    /// We hold onto the string here to avoid re-creating it every time we go
    /// to draw.
    out_buffer: String,

    /// We use this for reading in user input.
    getch: Getch,

    /// Where we are writing to.
    out: io::Stdout,
}

impl LLRTerminal {
    /// Trivial constructor.
    pub fn new(options: LLROptions) -> Self {
        let matrix_size = options.window_size / options.tile_size.to::<u16>();
        let estimated_out_capacity = matrix_size.to_clamped::<usize>().area() * 16;

        Self {
            options: options,
            screen: Matrix::new(matrix_size, None),
            out_buffer: String::with_capacity(estimated_out_capacity),
            getch: Getch::new(),
            out: io::stdout(),
        }
    }

    /// Cleares the terminal, and sets the drawing colour to the clear colour.
    fn reset_screen(&mut self) {
        self.out.write(b"\x1B[2J");
        write!(self.out, "{}", to_background_colour_code(self.options.clear_colour));
        self.out.flush();
    }
}

impl LLR for LLRTerminal {
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
        let matrix_size = self.screen.size();
        let estimated_out_capacity = matrix_size.to_clamped::<usize>().area() * 32;
        let mut out_buffer = String::with_capacity(estimated_out_capacity);

        self.screen.iter().for_each(|(maybe_pixel, pos)| -> () {
            match maybe_pixel {
                Some(pixel) => {
                    out_buffer += &format!("\x1B[{};{}H{}", pos.y, pos.x, pixel_to_cmd_code(pixel));
                },
                None => {
                    out_buffer += &format!(
                        "\x1B[{};{}H{} ",
                        pos.y,
                        pos.x,
                        to_background_colour_code(self.options.clear_colour)
                    );
                },
            };
        });

        self.out.write(&out_buffer.into_bytes());
    }

    fn on_start(&mut self) {
        self.reset_screen();
    }

    fn on_quit(&mut self) {
        self.reset_screen();
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
