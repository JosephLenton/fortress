use world::map::Map;
use world::tiles::Tile;

use std::io;

use super::colour::pixel_to_cmd_code;
use llr::LLRPixel;
use theme::Theme;

// Re-export this as ours for ease of use.
pub use util::states::OnOff;

/// Iterates over the map given, and writes it to the writer.
/// Simple as that really.
///
/// If colour is set to on then it'll be colourful. This is used for command
/// applications. By 'colour' we mean special characters injected into the
/// output stream which will appear as colour on the terminal.
///
/// If colour is off then the tiles characters alone are printed.
pub fn print_map(
    has_colour: OnOff,
    map: &Map<Tile>,
    out: &mut io::Write,
) -> io::Result<()> {
    let theme = Theme::new();

    for (tile, x, y) in map.slice_all() {
        // This is for the previous line, if we just ended it.
        if x == 0 && y > 0 {
            print_end_of_line(out, has_colour)?;
        }

        print_pixel(out, theme.get_tile(tile), has_colour)?;
    }

    print_end_of_line(out, has_colour)
}

fn print_pixel(
    out: &mut io::Write,
    pixel: LLRPixel,
    has_colour: OnOff,
) -> io::Result<()> {
    if has_colour.is_on() {
        write!(out, "{}", pixel_to_cmd_code(pixel))?;
    }

    write!(out, "{}", pixel.character)
}

fn print_end_of_line(
    out: &mut io::Write,
    has_colour: OnOff,
) -> io::Result<()> {
    if has_colour.is_on() {
        write!(out, "\x1B[0m")?;
    }

    writeln!(out, "")
}
