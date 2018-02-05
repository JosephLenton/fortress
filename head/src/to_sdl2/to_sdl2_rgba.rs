use super::ToSDL2;
use util::colour::RGBA;

use sdl2::pixels::Color as SDL2Color;

impl ToSDL2<SDL2Color> for RGBA {
    fn to_sdl2(self) -> SDL2Color {
        return SDL2Color::RGBA(self.red, self.green, self.blue, self.alpha);
    }
}
