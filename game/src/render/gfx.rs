
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use tiles::colour::Colour;
use render::colour;
use render::colour::ToSDL2Color;

///
/// Represents the graphics state.
///
/// This handles all drawing logic.
///
pub trait GFX {
    fn clear( &mut self ) -> ();

    fn rectangle(
        &mut self,
        colour   : Colour,
        rectable : (f32, f32, f32, f32),
    ) -> ();

    fn finished_drawing( &mut self ) -> ();
}

impl GFX for WindowCanvas {

    ///
    /// Call this before you begin drawing.
    ///
    fn clear( &mut self ) {
        self.set_draw_color( colour::BLACK.to_sdl2_color() );
        self.clear();
    }

    ///
    /// Draws a rectangle at the location given,
    /// with the given colour.
    ///
    fn rectangle(
        &mut self,
        colour       : Colour,
        (x, y, w, h) : (f32, f32, f32, f32),
    ) {
        let rect = Rect::new(x as i32, y as i32, w as u32, h as u32);

        self.set_draw_color( colour.to_sdl2_color() );
        self.fill_rect( rect );
    }

    fn finished_drawing( &mut self ) {
        self.present();
    }

}

