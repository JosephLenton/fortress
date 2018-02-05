
use theme::colours;
use sdl2::render::WindowCanvas;

use to_sdl2::*;

use util::shapes::Rect;
use util::colour::RGBA;

/// Represents the graphics state.
///
/// This handles all drawing logic.
pub trait GFX {
    /// Call this before you begin drawing.
    fn clear(&mut self) -> ();

    /// Draws a rectangle at the location given,
    /// with the given colour.
    fn rectangle(&mut self, colour: RGBA, rectable: Rect<f32>) -> ();

    /// Draws a rectangle at the location given,
    /// with the given colour.
    fn rectangle_outline(&mut self, colour: RGBA, rectable: Rect<f32>) -> ();

    /// Call when all the drawing is over for the current loop.
    fn finished_drawing(&mut self) -> ();
}

impl GFX for WindowCanvas {
    fn clear(&mut self) {
        self.set_draw_color(colours::BLACK.to_sdl2());
        self.clear();
    }

    fn rectangle(&mut self, colour: RGBA, rect: Rect<f32>) {
        self.set_draw_color(colour.to_sdl2());
        self.fill_rect(rect.to_sdl2());
    }

    fn rectangle_outline(&mut self, colour: RGBA, rect: Rect<f32>) {
        self.set_draw_color(colour.to_sdl2());
        self.draw_rect(rect.to_sdl2());
    }

    fn finished_drawing(&mut self) {
        self.present();
    }
}
