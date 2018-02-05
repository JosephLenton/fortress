use llr::LLRPixel;
use util::colour::RGBA;

/// Converts the pixel to it's colour, and then returns the command version
/// of that colour.
pub fn pixel_to_cmd_code(pixel : LLRPixel) -> String {
    let foreground = to_foreground_colour_code(pixel.foreground);
    let background = to_background_colour_code(pixel.background);

    format!("{}{}{}", foreground, background, pixel.character)
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
