
use super::ToSDL2;
use render::colour::*;

use sdl2::pixels::Color as SDL2Color;
use world::colour::Colour;

use util::colour::RGBA;

impl ToSDL2<SDL2Color> for Colour {
    fn to_sdl2( self ) -> SDL2Color {
        return self.to_rgba().to_sdl2();
    }
}

impl ToSDL2<SDL2Color> for RGBA {
    fn to_sdl2( self ) -> SDL2Color {
        return SDL2Color::RGBA( self.red, self.green, self.blue, self.alpha );
    }
}

