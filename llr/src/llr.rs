
use util::colour::RGBA;
use util::shapes::Rect;

/// Represents rendering.
///
/// This handles all drawing logic.
pub trait LLR {
    /// Call this before you begin drawing.
    fn clear(&mut self) -> ();

    /// Draws a rectangle at the location given,
    /// with the given colour.
    fn rectangle(
        &mut self,
        colour: RGBA,
        rectable: Rect<f32>,
    ) -> Result<(), String>;

    /// Draws a rectangle at the location given,
    /// with the given colour.
    fn rectangle_outline(
        &mut self,
        colour: RGBA,
        rectable: Rect<f32>,
    ) -> Result<(), String>;

    /// Call when all the drawing is over for the current loop.
    fn finished_drawing(&mut self) -> ();
}

