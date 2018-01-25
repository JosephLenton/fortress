use map::Map;
use tiles::Tile;
use tiles::tile_colour::tile_to_cmd;
use tiles::display_tile::tile_to_char;

use std::io;

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
pub fn print_map(colour: OnOff, map: &Map<Tile>, out: &mut io::Write) {
    let has_colour = colour == OnOff::On;

    for (tile, x, y) in map.slice_all() {
        let tile_char = tile_to_char(tile);

        // This is for the previous line, if we just ended it.
        if x == 0 && y > 0 {
            write_end_of_line(out, has_colour, "failed to write end of line during print");
        }

        if has_colour {
            let tile_colour = tile_to_cmd(tile);

            write!(out, "{}{}", tile_colour, tile_char).expect("failed to write tile");
        } else {
            write!(out, "{}", tile_char).expect("failed to write tile");
        }
    }

    write_end_of_line(
        out,
        has_colour,
        "failed to perform final end of line after print",
    );
}

fn write_end_of_line(out: &mut io::Write, has_colour: bool, msg: &'static str) {
    if has_colour {
        writeln!(out, "\x1B[0m").expect(msg);
    } else {
        writeln!(out, "").expect(msg);
    }
}
