use LLRPixel;
use util::shapes::Point2;
use util::shapes::Size;

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
        pos: Point2<i32>,
    ) -> Result<(), String>;

    /// Call when all the drawing is over for the current loop.
    fn finished_drawing(&mut self) -> ();

    /// Returns the size of the LLR in pixel tiles.
    fn size(&self) -> Size<u32>;
}
