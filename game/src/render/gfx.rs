
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
        rectable : (i32, i32, i32, i32),
    ) -> ();

    fn rectangle_outline(
        &mut self,
        colour   : Colour,
        rectable : (i32, i32, i32, i32),
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
        colour : Colour,
        xywh : (i32, i32, i32, i32),
    ) {
        let rect = to_sdl2_rect( xywh );

        self.set_draw_color( colour.to_sdl2_color() );
        self.fill_rect( rect );
    }

    ///
    /// Draws a rectangle at the location given,
    /// with the given colour.
    ///
    fn rectangle_outline(
        &mut self,
        colour : Colour,
        xywh : (i32, i32, i32, i32),
    ) {
        let rect = to_sdl2_rect( xywh );

        self.set_draw_color( colour.to_sdl2_color() );
        self.draw_rect( rect );
    }

    ///
    /// Call when all the drawing is over for the current loop.
    ///
    fn finished_drawing( &mut self ) {
        self.present();
    }
}

fn to_sdl2_rect(
    (mut x, mut y, mut w, mut h) : (i32, i32, i32, i32),
) -> Rect {
    if w < 0 {
        x -= w;
        w = -w;
    }

    if h < 0 {
        y -= h;
        h = -h;
    }

    return Rect::new(x, y, w as u32, h as u32);
}

