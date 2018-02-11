use world::map::Map;
use world::tiles::Tile;

use std::io;

use llr::LLRPixel;
use hlr::theme::Theme;
use util::colour::RGBA;

/// For stating if the colour is on or off.
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

    for (tile, pos) in map.slice_all() {
        // This is for the previous line, if we just ended it.
        if pos.x == 0 && pos.y > 0 {
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

/// Converts the pixel to it's colour, and then returns the command version
/// of that colour.
fn pixel_to_cmd_code(pixel: LLRPixel) -> String {
    let foreground = to_foreground_colour_code(pixel.foreground);
    let background = to_background_colour_code(pixel.background);

    format!("{}{}{}", foreground, background, pixel.character)
}

fn to_foreground_colour_code(colour: RGBA) -> String {
    format!("\x1B[38;2;{}m", to_colour_code(colour))
}

fn to_background_colour_code(colour: RGBA) -> String {
    format!("\x1B[48;2;{}m", to_colour_code(colour))
}

fn to_colour_code(colour: RGBA) -> String {
    format!("{};{};{}", colour.red, colour.green, colour.blue)
}
