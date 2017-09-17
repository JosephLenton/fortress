
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use tiles::colour::Colour;
use render::colour::colour_to_sdl2_colour;

///
/// Represents the graphics state.
///
/// This handles all drawing logic.
///
pub struct GFX<'a> {
    canvas : &'a mut WindowCanvas,
}

impl<'a> GFX<'a> {
    pub fn new(
        canvas : &'a mut WindowCanvas,
    ) -> GFX<'a> {
        return GFX {
            canvas : canvas,
        }
    }

    ///
    /// Draws a rectangle at the location given,
    /// with the given colour.
    ///
    pub fn rectangle(
        &mut self,
        colour       : Colour,
        (x, y, w, h) : (f32, f32, f32, f32),
    ) {
        let sdl_colour = colour_to_sdl2_colour( colour );
        let rect = Rect::new(x as i32, y as i32, w as u32, h as u32);

        self.canvas.set_draw_color( sdl_colour );
        self.canvas.fill_rect( rect );
    }

    ///
    /// Call this before you begin drawing.
    ///
    pub fn pre_render( &mut self ) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    ///
    /// Call this when drawing has ended.
    ///
    pub fn post_render( &mut self ) {
        self.canvas.present();
    }
}

