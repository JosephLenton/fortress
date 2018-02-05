use super::to_sdl2::ToSDL2;
use sdl2::pixels::Color;
use util::colour::RGBA;

impl ToSDL2<Color> for RGBA {
    fn to_sdl2(self) -> Color {
        Color::RGBA(self.red, self.green, self.blue, self.alpha)
    }
}
