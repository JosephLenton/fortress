use LLREvent;
use LLRPixel;
use util::shapes::Point;
use util::shapes::Size;

/// Represents rendering.
///
/// This handles all drawing logic.
pub trait LLR {
    /// Clears everything.
    ///
    /// You can call this before you begin drawing.
    fn clear(&mut self);

    /// Draws a rectangle at the location given,
    /// with the given colour.
    fn pixel(
        &mut self,
        pixel: LLRPixel,
        pos: Point<u16>,
    ) -> Result<(), String>;

    /// Call when all the drawing is over for the current loop.
    fn finished_drawing(&mut self);

    /// Called when we are first starting.
    fn on_start(&mut self);

    /// Called when we are done and the program is shutting down.
    /// An LLR may want to do something at this point.
    fn on_quit(&mut self);

    /// Returns the size of the LLR in pixel tiles.
    fn size(&self) -> Size<u16>;

    /// Polls for events.
    fn poll(&mut self) -> Option<LLREvent>;
}
