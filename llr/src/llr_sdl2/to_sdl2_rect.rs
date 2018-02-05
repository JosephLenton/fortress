use super::to_sdl2::ToSDL2;
use sdl2::rect::Rect as SDL2Rect;
use util::shapes::Rect;

impl ToSDL2<SDL2Rect> for Rect<f32> {
    fn to_sdl2(self) -> SDL2Rect {
        let mut x = self.x;
        let mut y = self.y;
        let mut w = self.width;
        let mut h = self.height;

        if w < 0.0 {
            x -= w;
            w = -w;
        }

        if h < 0.0 {
            y -= h;
            h = -h;
        }

        SDL2Rect::new(x.round() as i32, y.round() as i32, w.round() as u32, h.round() as u32)
    }
}

impl ToSDL2<SDL2Rect> for Rect<u32> {
    fn to_sdl2(self) -> SDL2Rect {
        let mut x = self.x as i32;
        let mut y = self.y as i32;
        let mut w = self.width as i32;
        let mut h = self.height as i32;

        if w < 0 {
            x -= w;
            w = -w;
        }

        if h < 0 {
            y -= h;
            h = -h;
        }

        SDL2Rect::new(x, y, w as u32, h as u32)
    }
}

impl ToSDL2<SDL2Rect> for Rect<i32> {
    fn to_sdl2(self) -> SDL2Rect {
        let mut x = self.x;
        let mut y = self.y;
        let mut w = self.width;
        let mut h = self.height;

        if w < 0 {
            x -= w;
            w = -w;
        }

        if h < 0 {
            y -= h;
            h = -h;
        }

        SDL2Rect::new(x, y, w as u32, h as u32)
    }
}
