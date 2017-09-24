
use super::ToSDL2;
use render::colour::*;

use sdl2::pixels::Color as SDL2Color;
use fortress::colour::Colour;

impl ToSDL2<SDL2Color> for Colour {
    fn to_sdl2( self ) -> SDL2Color {
        return self.to_render_colour().to_sdl2();
    }
}

impl ToSDL2<SDL2Color> for RenderColour {
    fn to_sdl2( self ) -> SDL2Color {
        return SDL2Color::RGBA( self.r, self.g, self.b, self.a );
    }
}

