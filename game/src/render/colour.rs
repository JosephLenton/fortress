
extern crate tiles;

use tiles::tile::tile::Tile;
use tiles::tile::tile_colour::TileColour;

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! colour {
        ( $r:expr, $g:expr, $b:expr ) => {
            [($r as f32)/255.0, ($g as f32)/255.0, ($b as f32)/255.0, 1.0];
        }
    }
}

pub type RenderColour = [ f32; 4 ];

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
    let colour = tiles::tile::tile_colour::tile_to_colour( tile );

    return ( tile_colour_to_render_colour(colour.0), tile_colour_to_render_colour(colour.1) )
}

pub fn tile_colour_to_render_colour( colour : TileColour ) -> RenderColour {
    return match colour {
        TileColour::Black        => BLACK,
        TileColour::White        => WHITE,

        TileColour::LightRed     => LIGHT_RED,
        TileColour::Red          => RED,

        TileColour::Pink         => PINK,
        TileColour::Purple       => PURPLE,

        TileColour::Brown        => BROWN,
        TileColour::Yellow       => YELLOW,

        TileColour::DarkGrey     => DARK_GREY,
        TileColour::Grey         => GREY,
        TileColour::LightGrey    => LIGHT_GREY,

        TileColour::LightCyan    => LIGHT_CYAN,
        TileColour::Cyan         => CYAN,

        TileColour::LightBlue    => LIGHT_BLUE,
        TileColour::Blue         => BLUE,

        TileColour::LightGreen   => LIGHT_GREEN,
        TileColour::Green        => GREEN,
    }
}

