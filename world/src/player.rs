use util::shapes::Point;

/// This represents a player in the world.
/// 
pub struct Player {
    /// The players location in the world.
    /// 
    pub position: Point<u32>,
}

impl Player {
    /// Creates a new player at the location given.
    /// 
    pub fn new(
        x: u32,
        y: u32,
    ) -> Player {
        Player {
            position: Point::<u32> {
                x: x,
                y: y,
            },
        }
    }
}
