
use theme::TileColour;
use util::colour::RGBA;

/// Converts the tile to it's colour, and then returns the command version
/// of that colour.
pub fn tile_to_cmd(colour: TileColour) -> String {
    let foreground = to_foreground_colour_code( colour.foreground );
    let background = to_background_colour_code( colour.background );

    format!("{}{}", foreground, background)
}

pub fn to_foreground_colour_code(colour: RGBA) -> String {
    format!("\x1B[38;2;{}m", to_colour_code(colour))
}

pub fn to_background_colour_code(colour: RGBA) -> String {
    format!("\x1B[48;2;{}m", to_colour_code(colour))
}

fn to_colour_code(colour: RGBA) -> String {
    format!("{};{};{}", colour.red, colour.green, colour.blue)
}

