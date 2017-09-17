
use sdl2::pixels::Color;

use tiles::tile::tile::Tile;
use tiles::colour::Colour;
use tiles::tile::tile_colour;

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! colour {
        ( $r:expr, $g:expr, $b:expr ) => {
            ( $r as u8, $g as u8, $b as u8, 255 as u8 );
        }
    }
}

///
/// The colour used when rendering.
/// This matches the layout used by SDL.
///
pub type RenderColour = ( u8, u8, u8, u8 );

pub static BLACK        : RenderColour = colour!(   0,   0,   0 );
pub static WHITE        : RenderColour = colour!( 255, 255, 255 );

pub static LIGHT_RED    : RenderColour = colour!( 250, 128, 144 );
pub static RED          : RenderColour = colour!( 255,   0,   0 );

pub static PINK         : RenderColour = colour!( 255,   0, 255 );
pub static PURPLE       : RenderColour = colour!( 128,   0, 128 );

pub static BROWN        : RenderColour = colour!( 175,  90,  35 );
pub static YELLOW       : RenderColour = colour!( 255, 215,   0 );

pub static DARK_GREY    : RenderColour = colour!(  75,  75,  75 );
pub static GREY         : RenderColour = colour!( 120, 120, 120 );
pub static LIGHT_GREY   : RenderColour = colour!( 180, 180, 180 );

pub static LIGHT_CYAN   : RenderColour = colour!(   0, 255, 255 );
pub static CYAN         : RenderColour = colour!(  64, 224, 208 );

pub static LIGHT_BLUE   : RenderColour = colour!(  30, 144, 255 );
pub static BLUE         : RenderColour = colour!(   0,   0, 255 );

pub static LIGHT_GREEN  : RenderColour = colour!(   0, 255,   0 );
pub static GREEN        : RenderColour = colour!(  50, 205,  50 );

pub fn tile_to_colour( tile : Tile ) -> ( RenderColour, RenderColour ) {
    let colour = tile_colour::tile_to_colour( tile );

    return ( colour_to_render_colour(colour.0), colour_to_render_colour(colour.1) )
}

pub fn colour_to_render_colour( colour : Colour ) -> RenderColour {
    return match colour {
        Colour::Black        => BLACK,
        Colour::White        => WHITE,

        Colour::LightRed     => LIGHT_RED,
        Colour::Red          => RED,

        Colour::Pink         => PINK,
        Colour::Purple       => PURPLE,

        Colour::Brown        => BROWN,
        Colour::Yellow       => YELLOW,

        Colour::DarkGrey     => DARK_GREY,
        Colour::Grey         => GREY,
        Colour::LightGrey    => LIGHT_GREY,

        Colour::LightCyan    => LIGHT_CYAN,
        Colour::Cyan         => CYAN,

        Colour::LightBlue    => LIGHT_BLUE,
        Colour::Blue         => BLUE,

        Colour::LightGreen   => LIGHT_GREEN,
        Colour::Green        => GREEN,
    }
}

pub fn colour_to_sdl2_colour( colour : Colour ) -> Color {
    let (r, g, b, a) = colour_to_render_colour( colour );

    return Color::RGBA( r, g, b, a );
}

