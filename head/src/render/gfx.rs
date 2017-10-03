
use sdl2::render::WindowCanvas;

use world::colour::Colour;
use render::colour;
use to_sdl2::*;

use util::shapes::Rect;

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
        rectable : Rect<f32>,
    ) -> ();

    fn rectangle_outline(
        &mut self,
        colour   : Colour,
        rectable : Rect<f32>,
    ) -> ();

    fn finished_drawing( &mut self ) -> ();
}

impl GFX for WindowCanvas {

    ///
    /// Call this before you begin drawing.
    ///
    fn clear( &mut self ) {
        self.set_draw_color( colour::BLACK.to_sdl2() );
        self.clear();
    }

    ///
    /// Draws a rectangle at the location given,
    /// with the given colour.
    ///
    fn rectangle(
        &mut self,
        colour : Colour,
        rect   : Rect<f32>,
    ) {
        self.set_draw_color( colour.to_sdl2() );
        self.fill_rect( rect.to_sdl2() );
    }

    ///
    /// Draws a rectangle at the location given,
    /// with the given colour.
    ///
    fn rectangle_outline(
        &mut self,
        colour : Colour,
        rect   : Rect<f32>,
    ) {
        self.set_draw_color( colour.to_sdl2() );
        self.draw_rect( rect.to_sdl2() );
    }

    ///
    /// Call when all the drawing is over for the current loop.
    ///
    fn finished_drawing( &mut self ) {
        self.present();
    }
}

