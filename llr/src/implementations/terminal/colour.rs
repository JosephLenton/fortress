use LLRPixel;
use util::colour::RGBA;

/// Returns an ASCII escape code which sets the terminal back to default
/// background and foreground colour.
///
/// What is this colour? Whatever the user has set as their default.
pub fn to_default_colour_code() -> &'static str {
    "\x1B[0;m"
}

/// Converts the pixel to it's colour, and then returns the command version
/// of that colour.
pub fn pixel_to_cmd_code(pixel: LLRPixel) -> String {
    let foreground = to_foreground_colour_code(pixel.foreground);
    let background = to_background_colour_code(pixel.background);

    format!("{}{}{}", foreground, background, pixel.character)
}

/// Converts the RGBA into an escape code. When you write this to the terminal,
/// the terminal will turn any future text into that colour.
pub fn to_foreground_colour_code(colour: RGBA) -> String {
    format!("\x1B[38;2;{}m", to_colour_code(colour))
}

/// Converts the RGBA into an escape code. When you write this to the terminal,
/// the terminal will change the background of any future text into that colour.
pub fn to_background_colour_code(colour: RGBA) -> String {
    format!("\x1B[48;2;{}m", to_colour_code(colour))
}

fn to_colour_code(colour: RGBA) -> String {
    format!("{};{};{}", colour.red, colour.green, colour.blue)
}

