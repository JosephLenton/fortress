use LLRPixel;
use util::shapes::Rect;

/// Represents rendering.
///
/// This handles all drawing logic.
pub trait LLR {
    /// Clears everything.
    ///
    /// You can call this before you begin drawing.
    fn clear(&mut self) -> ();

    /// Draws a rectangle at the location given,
    /// with the given colour.
    fn pixel(
        &mut self,
        pixel: LLRPixel,
        rectable: Rect<f32>,
    ) -> Result<(), String>;

    /// Call when all the drawing is over for the current loop.
    fn finished_drawing(&mut self) -> ();
}
