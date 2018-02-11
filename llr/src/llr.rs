use LLREvent;
use LLRPixel;
use util::shapes::Point;
use util::shapes::Size;

/// Represents rendering.
///
/// This handles all drawing logic.
///
/// Note that no drawing calls will be displayed until you call
/// `finished_drawing`.
pub trait LLR {
    /// Sets the title on the LLR.
    ///
    /// This is probably the window title, or the window title if in the
    /// terminal.
    ///
    /// Not guaranteed to actually do anything.
    fn set_title(&mut self, title : &str);

    /// Clears everything.
    ///
    /// You can call this before you begin drawing, or during drawing to reset.
    /// Note the clear will not show until you call `finished_drawing`.
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
